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
