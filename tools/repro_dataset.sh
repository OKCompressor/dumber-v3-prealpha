#!/usr/bin/env bash
# SPDX-License-Identifier: LicenseRef-Luna-Non-Commons-1.1
set -euo pipefail

REL="$(cd "$(dirname "$0")/.." && pwd)"
REDUMB="$REL/dist/redumb-linux-x86_64"

INPUT="${1:?input path}"
NAME="${2:-dataset}"
THREADS="${3:-16}"
TASK_MULT="${4:-2}"
VERIFY="${VERIFY:-1}"

BYTES="$(stat -c %s "$INPUT")"
MIB=1048576
TASKS=$(( THREADS * TASK_MULT ))
MACRO_MB=$(( (BYTES + TASKS*MIB - 1) / (TASKS*MIB) ))

if [ "$MACRO_MB" -lt 1 ]; then
    MACRO_MB=1
fi

STAMP="$(date +%Y%m%d-%H%M%S)"
RUN="$REL/_runs/${NAME}-${STAMP}"
LOG="$RUN/logs"

mkdir -p "$RUN"/{local_dicts,local_u16,gmap24,du_stats,r1_plan,fused} "$LOG"

printf "stage\twall_s\tmax_rss_kb\tcpu\n" > "$RUN/TIMINGS.tsv"

run_stage() {
    name="$1"
    shift

    tf="$RUN/.time-${name}"

    /usr/bin/time \
        -f '%e	%M	%P' \
        -o "$tf" \
        "$@" \
        >"$LOG/${name}.log" 2>&1

    read -r wall rss cpu < "$tf"
    printf "%s\t%s\t%s\t%s\n" "$name" "$wall" "$rss" "$cpu" |
        tee -a "$RUN/TIMINGS.tsv"

    rm -f "$tf"
}

rustc -O "$REL/devtools/du_stats_reduce.rs" \
    -o "$RUN/du-stats-reduce"

rustc -O "$REL/devtools/fused_emit.rs" \
    -o "$RUN/fused-emit"

{
    echo "date=$(date -Iseconds)"
    echo "dataset=$NAME"
    echo "input=$INPUT"
    echo "input_bytes=$BYTES"
    echo "input_sha256=$(sha256sum "$INPUT" | awk '{print $1}')"
    echo "redumb_sha256=$(sha256sum "$REDUMB" | awk '{print $1}')"
    echo "threads=$THREADS"
    echo "task_multiplier=$TASK_MULT"
    echo "target_scheduler_tasks=$TASKS"
    echo "macro_mb=$MACRO_MB"
} | tee "$RUN/ENVIRONMENT.txt"

run_stage du_encode \
    "$REDUMB" encode-u16-auto-nosdict-par \
    "$INPUT" \
    "$RUN/local_dicts" \
    "$RUN/local_u16" \
    "$THREADS" \
    "$MACRO_MB"

run_stage merge_dicts \
    "$REDUMB" merge-dicts \
    "$RUN/local_dicts" \
    "$RUN/merged.dict"

run_stage build_gmap24 \
    "$REDUMB" build-gmap24 \
    "$RUN/local_dicts" \
    "$RUN/merged.dict" \
    "$RUN/gmap24"

if [ "$VERIFY" = "1" ]; then
    run_stage restore_gmap \
        "$REDUMB" restore-u16-gmap24 \
        "$RUN/merged.dict" \
        "$RUN/gmap24" \
        "$RUN/local_u16" \
        "$RUN/restored.txt"

    if cmp -s "$INPUT" "$RUN/restored.txt"; then
        echo "restore_gmap_exact=PASS" | tee "$RUN/RESTORE_STATUS"
    else
        echo "restore_gmap_exact=FAIL" | tee "$RUN/RESTORE_STATUS"
        exit 1
    fi

    rm -f "$RUN/restored.txt"
else
    echo "restore_gmap_exact=NOT_RUN" > "$RUN/RESTORE_STATUS"
fi

run_stage du_stats \
    "$RUN/du-stats-reduce" \
    "$RUN/local_u16" \
    "$RUN/gmap24" \
    "$RUN/du_stats"

run_stage r1_plan \
    python3 "$REL/devtools/r1_singleton_plan.py" \
    "$RUN/du_stats/COUNTS.u64le" \
    "$RUN/merged.dict" \
    "$RUN/r1_plan"

SENTINEL="$(
    awk -F= '/^sentinel_id=/{print $2; exit}' \
        "$RUN/r1_plan/PLAN.md"
)"

run_stage fused_r1_scan \
    "$RUN/fused-emit" \
    "$RUN/local_u16" \
    "$RUN/gmap24" \
    "$RUN/r1_plan/du_global_to_r1.u32le" \
    "$RUN/merged.dict" \
    "$SENTINEL" \
    "$RUN/fused" \
    0

LOCAL_DICTS="$(du -sb "$RUN/local_dicts" | awk '{print $1}')"
LOCAL_U16="$(du -sb "$RUN/local_u16" | awk '{print $1}')"
MERGED_DICT="$(stat -c %s "$RUN/merged.dict")"
GMAP="$(du -sb "$RUN/gmap24" | awk '{print $1}')"
R1_VOCAB="$(stat -c %s "$RUN/r1_plan/r1_vocab.txt")"
FUSED_SIDEBAND="$(du -sb "$RUN/fused" | awk '{print $1}')"
CHUNKS="$(find "$RUN/local_dicts" -type f | wc -l)"
ROLLOVERS="$(grep -c 'rollover=true' "$LOG/du_encode.log" || true)"

{
    printf "metric\tbytes_or_count\n"
    printf "input\t%s\n" "$BYTES"
    printf "local_dicts\t%s\n" "$LOCAL_DICTS"
    printf "local_u16\t%s\n" "$LOCAL_U16"
    printf "merged_dict\t%s\n" "$MERGED_DICT"
    printf "gmap24\t%s\n" "$GMAP"
    printf "r1_vocab\t%s\n" "$R1_VOCAB"
    printf "fused_output_dir\t%s\n" "$FUSED_SIDEBAND"
    printf "representation_chunks\t%s\n" "$CHUNKS"
    printf "rollovers\t%s\n" "$ROLLOVERS"
} > "$RUN/ACCOUNTING.tsv"

python3 - "$RUN" "$NAME" "$BYTES" "$THREADS" "$TASKS" "$MACRO_MB" <<'PY'
from pathlib import Path
import sys

run = Path(sys.argv[1])
name = sys.argv[2]
input_bytes = int(sys.argv[3])
threads = int(sys.argv[4])
tasks = int(sys.argv[5])
macro_mb = int(sys.argv[6])

timings = {}
rss = {}

for line in (run/"TIMINGS.tsv").read_text().splitlines()[1:]:
    if not line.strip():
        continue
    stage, wall, mem, cpu = line.split("\t")
    timings[stage] = float(wall)
    rss[stage] = int(mem)

acc = {}
for line in (run/"ACCOUNTING.tsv").read_text().splitlines()[1:]:
    k,v = line.split("\t")
    acc[k] = int(v)

plan = {}
for line in (run/"r1_plan"/"PLAN.md").read_text().splitlines():
    if "=" in line and not line.startswith("```"):
        k,v = line.split("=",1)
        try:
            plan[k.strip()] = float(v.strip())
        except:
            pass

fused = {}
for line in (run/"fused"/"SUMMARY.tsv").read_text().splitlines()[1:]:
    k,v = line.split("\t")
    fused[k] = int(v)

du_s = timings.get("du_encode", 0)
throughput = input_bytes / 1_000_000 / du_s if du_s else 0

mapped_s = sum(
    timings.get(x,0)
    for x in ("du_encode","merge_dicts","build_gmap24")
)

global_vocab = int(plan.get("global_du_vocab",0))
frequent = int(plan.get("frequent_types",0))
singletons = int(plan.get("singleton_types",0))
single_frac = plan.get("singleton_type_fraction",0)
single_token_frac = plan.get("singleton_token_fraction",0)

text = f"""# {name} DU/R1 receipt

| metric | result |
|---|---:|
| input | {input_bytes:,} bytes |
| CPU threads | {threads} |
| scheduler target tasks | {tasks} |
| macro size | {macro_mb} MiB |
| representation chunks | {acc['representation_chunks']} |
| u16 rollovers | {acc['rollovers']} |
| DU encode | **{timings.get('du_encode',0):.3f} s** |
| DU throughput | **{throughput:.1f} MB/s** |
| global DU mapping ready | **{mapped_s:.3f} s** |
| DU stats | {timings.get('du_stats',0):.3f} s |
| R1 plan | {timings.get('r1_plan',0):.3f} s |
| fused R1 scan | {timings.get('fused_r1_scan',0):.3f} s |
| DU encode max RSS | {rss.get('du_encode',0)/1024:.1f} MiB |
| DU stats max RSS | {rss.get('du_stats',0)/1024:.1f} MiB |

## Vocabulary

| metric | value |
|---|---:|
| global DU types | {global_vocab:,} |
| singleton types | **{singletons:,}** |
| singleton type fraction | **{single_frac*100:.2f}%** |
| frequent R1 types | **{frequent:,}** |
| singleton token fraction | **{single_token_frac*100:.3f}%** |

## Artifacts

| artifact | bytes |
|---|---:|
| local dictionaries | {acc['local_dicts']:,} |
| merged dictionary | **{acc['merged_dict']:,}** |
| gmap24 | {acc['gmap24']:,} |
| local u16 payload | {acc['local_u16']:,} |
| pruned R1 vocabulary | {acc['r1_vocab']:,} |

## Exactness

```text
{(run/'RESTORE_STATUS').read_text().strip()}
```

The global-u24 token stream is intentionally not materialized in this run.
"""

(run/"SUMMARY.md").write_text(text)
print(text)
PY

mkdir -p "$REL/receipts/$NAME"

cp -av \
    "$RUN/SUMMARY.md" \
    "$RUN/TIMINGS.tsv" \
    "$RUN/ACCOUNTING.tsv" \
    "$RUN/ENVIRONMENT.txt" \
    "$RUN/RESTORE_STATUS" \
    "$RUN/r1_plan/PLAN.md" \
    "$RUN/fused/SUMMARY.tsv" \
    "$REL/receipts/$NAME/"

python3 - "$REL/docs/MEASUREMENTS.md" "$RUN/SUMMARY.md" <<'PY'
from pathlib import Path
import sys

p=Path(sys.argv[1])
summary=Path(sys.argv[2]).read_text()

s=p.read_text()
a="<!-- ENWIK9_BEGIN -->"
b="<!-- ENWIK9_END -->"

block=a+"\n"+summary+"\n"+b

if a in s and b in s:
    i=s.index(a)
    j=s.index(b,i)+len(b)
    s=s[:i]+block+s[j:]
else:
    s += "\n\n"+block+"\n"

p.write_text(s)
PY

find "$RUN" -type f \
    -not -name SHA256SUMS \
    -print0 |
    sort -z |
    xargs -0 sha256sum > "$RUN/SHA256SUMS"

echo
echo "RUN=$RUN"
echo "SUMMARY=$RUN/SUMMARY.md"
