#!/usr/bin/env python3
import array
import pathlib
import struct
import sys

if len(sys.argv) != 4:
    raise SystemExit(
        "usage: r1_singleton_plan.py COUNTS.u64le MERGED_DICT OUT_DIR"
    )

counts_path = pathlib.Path(sys.argv[1])
dict_path = pathlib.Path(sys.argv[2])
out = pathlib.Path(sys.argv[3])
out.mkdir(parents=True, exist_ok=True)

counts = array.array("Q")
with counts_path.open("rb") as f:
    counts.fromfile(f, counts_path.stat().st_size // 8)

if sys.byteorder != "little":
    counts.byteswap()

tokens = dict_path.read_bytes().splitlines()

if len(tokens) != len(counts):
    raise SystemExit(
        f"dict/count mismatch dict={len(tokens)} counts={len(counts)}"
    )

singleton = [c == 1 for c in counts]
singleton_types = sum(singleton)
frequent_types = len(counts) - singleton_types

# Dense frequent IDs first. Sentinel is one distinguished ID after them.
sentinel_id = frequent_types
next_id = 0
mapping = []

lexical_raw_bytes = 0
lexical_lenpref_bytes = 0

with (out / "r1_vocab.txt").open("wb") as vf:
    for tok, is_single in zip(tokens, singleton):
        if is_single:
            mapping.append(sentinel_id)
            lexical_raw_bytes += len(tok)
            lexical_lenpref_bytes += 2 + len(tok)
        else:
            mapping.append(next_id)
            vf.write(tok + b"\n")
            next_id += 1

    vf.write(b"<RARE1>\n")

with (out / "du_global_to_r1.u32le").open("wb") as f:
    for x in mapping:
        f.write(struct.pack("<I", x))

total_tokens = sum(counts)
singleton_fraction_types = singleton_types / len(counts) if counts else 0.0
singleton_fraction_tokens = singleton_types / total_tokens if total_tokens else 0.0

summary = f"""# R1 singleton plan

mode=words
threshold=1

global_du_vocab={len(counts)}
frequent_types={frequent_types}
singleton_types={singleton_types}
sentinel_id={sentinel_id}
total_tokens={total_tokens}

singleton_type_fraction={singleton_fraction_types:.8f}
singleton_token_fraction={singleton_fraction_tokens:.8f}

literal_singleton_bytes_raw={lexical_raw_bytes}
literal_singleton_bytes_u16_length_prefixed={lexical_lenpref_bytes}

mapping_bytes_u32={len(mapping) * 4}

Important:
This creates the DU-global -> R1 projection and pruned R1 vocabulary.
It does not yet materialize the ordered singleton sideband.
The sideband is emitted during the later fused stream pass.
"""

(out / "PLAN.md").write_text(summary)
print(summary)
