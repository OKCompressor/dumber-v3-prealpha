# DUMBer v3 pre-alpha3

**Parallel reversible structural tokenization with bounded local vocabularies.**

DUMBer produces vocabulary-bounded local u16 token streams, merges their
lexical spaces into a canonical global representation, and supports exact
restoration.

It is designed as an upstream structural representation for compression,
language-model token bridges, NLP, graphs, and downstream rank coding.

## Highlights

| dataset | input | DU encode | global mapping ready | exact |
|---|---:|---:|---:|---|
| enwik7 | 10 MB | **0.106 s** | **0.219 s** | PASS |
| enwik8 | 100 MB | **0.651 s** | **1.223 s** | PASS |
| enwik9 | 1 GB | **4.46 s** | **11.54 s** | PASS |

Measured parallel DU throughput:

```text
enwik8   ~153.7 MB/s
enwik9   ~224.2 MB/s
```

The 100 MB run produced 31 vocabulary-bounded representation chunks while
using 16 CPU threads.

```text
scheduler shard != representation chunk != downstream payload chunk
```

<!-- ENWIK9_HERO_BEGIN -->
## 1 GB result

On a 1 GB input, DUMBer encoded the local vocabulary-bounded token stream in
**4.46 s** and reached a globally reconciled DU identity space in **11.54 s**
on 16 worker threads, restoring the original text exactly.

| metric | result |
|---|---:|
| input | 1,000,000,000 bytes |
| DU encode | **4.46 s** |
| DU throughput | **224.2 MB/s** |
| global DU mapping ready | **11.54 s** |
| exact structural restore | **7.72 s / PASS** |
| representation chunks | 188 |
| u16 vocabulary rollovers | 156 |
| peak RSS | ~2.87 GiB |

The global vocabulary contained **2,229,308 types**. Of these,
**1,380,346 (61.92%)** were global singletons while representing only
**0.343% of token positions**.

The complete reproducibility harness took 26.19 s. This number includes
helper compilation, encode, merge, mapping, exact restore, statistics, R1
planning, fused scanning, and harness overhead; it is not an encode or decode
latency measurement.
<!-- ENWIK9_HERO_END -->

## R1 singleton projection

enwik8:

| metric | result |
|---|---:|
| global DU vocabulary | 426,714 |
| singleton types | **234,696 / 55.00%** |
| frequent R1 vocabulary | **192,018** |
| singleton token positions | **0.599%** |
| DU stats | **0.150 s** |
| R1 plan | **0.330 s** |

Primary downstream candidate:

```text
threshold=1
rare-ones-mode=words
```

## ID-width policy

```text
u16 = local structural storage
u24 = compact serialized mapping
u32 = hydrated compute/API/vector lane
```

A global-u24 corpus stream is not required.

Local u16 streams can be mapped directly into downstream global spaces.

## Structural handoff

```text
text
-> local u16 DUMBer streams
-> canonical DU identity space
-> optional R1 pruning
-> zRank / NLP / graph / model bridges
```

## Model-tokenizer bridge

A target BPE/SentencePiece tokenizer can be applied once to each entry in the
merged DUMBer vocabulary:

```text
DU global ID
-> [target tokenizer IDs...]
```

On enwik8 the merged vocabulary is only 3.81 MB for a 100 MB source corpus.

See:

```text
docs/DUMBER_BPE_BRIDGE.md
```

## Reproduce

Run from an environment with Rust available:

```bash
tools/repro_dataset.sh /path/to/input DATASET THREADS TASK_MULTIPLIER
```

Example:

```bash
tools/repro_dataset.sh \
  /mnt/data_linux/IT/OKC/enwiks/enwik8 \
  enwik8 \
  16 \
  2
```

The distributed executable remains named `redumb` internally for provenance
and CLI compatibility.

## Scope

This preview does not claim:

- BPE-equivalent semantics
- universal compression superiority
- byte-reproducible source/binary provenance
- end-to-end parallel zRank results

Those are separate measurements.

## License

DUMBer v3 pre-alpha is distributed under the **Luna Non-Commons License v1.1**
(`LicenseRef-Luna-Non-Commons-1.1`).

Human learning, research, modification, preservation, self-hosting and forking
remain available under its terms.

Commercial organizational use requires a separate commercial license or
reciprocity agreement.

See `LICENSE.md` and `docs/LICENSE.md`.

## 10 GB Ganymede scale point

A 10,000,000,000-byte Wikipedia-derived prefix crossed the u24 canonical
identity limit.

| metric | result |
|---|---:|
| raw input | 10,000,000,000 B |
| local u16 payload | 7,605,565,792 B |
| merged global dictionary | 214,565,572 B |
| gmap32 | 477,768,072 B |
| **canonical reversible bundle** | **8,297,899,436 B** |
| **pre-entropy delta vs raw** | **-17.021%** |
| global DU vocabulary | **17,294,055** |
| representation chunks | 1,828 |
| exact gmap32 restore | **PASS** |

The 1,033,808,012 bytes of local dictionaries are construction state and are
not required once the merged dictionary and canonical gmap32 layer exist.

The local token streams remain u16. Only the local-to-global reconciliation
map widens to u32.

The measured 10 GB construction path used the historical whole-file front end
for its initial DU pass, so its raw-to-canonical wall time is retained as a
scale/conformance result rather than an Europa bounded-input performance
claim.

### Structural + entropy result at 10 GB

| path | retained bytes |
|---|---:|
| raw | 10,000,000,000 |
| Ganymede canonical | **8,297,899,436** |
| raw + zstd1 | **3,482,115,937** |
| Ganymede + zstd1 | **3,280,838,574** |

At this scale, structural transformation reduces the source by **17.021%**
before entropy coding. After zstd1, the structural-first artifact is
**5.780% smaller than raw+zstd1**.

The current wall-time comparison is not used as a codec throughput claim:
the raw zstd input was substantially cache-served, while the structural
artifact was read from thousands of files on an external mechanical volume.
An SSD-backed, chunk-parallel Pareto run is planned.

### 10 GB entropy Pareto

At zstd level 1, the Ganymede structural representation produced a
**3,280,838,574-byte** artifact, **5.780% smaller** than raw+zstd1.

At zstd level 19, raw+zstd19 produced the smaller artifact:

```text
raw+zstd19          2,321,300,290 B
Ganymede+zstd19     2,554,960,620 B
```

The zstd19 entropy stage over Ganymede completed in **505.05 s**, versus
**853.83 s** over raw input, a **40.85% reduction in measured entropy-stage
wall time**.

The measured raw-to-canonical construction path was:

```text
DU encode        102.80 s
merge             90.41 s
gmap32             47.27 s
                 --------
total             240.48 s
```

A linear 100 GB projection is approximately **40.08 minutes**. This is not a
measured 100 GB result. SSD-backed bounded-input scaling remains to be tested.

The 10 GB measurements cover DUMBer canonicalization, not the downstream R1
or zRank stages.

### What pre-alpha4 does — and does not — measure

Ganymede's 10 GB result measures DUMBer canonicalization through gmap32 and
exact structural restoration.

It does **not** include R1 or zRank.

```text
10 GB raw -> canonical
DU        102.80 s
merge      90.41 s
gmap32     47.27 s
          --------
          240.48 s = 4:00.48
```

A strictly linear projection is approximately **40.08 minutes per 100 GB**.
That is not a measured 100 GB result; SSD-backed bounded-input scaling remains
to be tested.

See `docs/releases/PREALPHA4_GANYMEDE.md` for the full 10 GB size/time Pareto,
scope boundary and downstream roadmap.
