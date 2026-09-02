# Redumb / DUMB v3preview alpha

A reversible, vocabulary-bounded structural tokenizer and local-ID stream.

This preview focuses on:

- parallel tokenization
- u16 local ID spaces
- vocabulary-driven rollover
- global dictionary reconciliation
- exact restoration
- reproducible throughput / memory measurements

It is intended as a latency, memory, representation, and downstream-ID
research artifact.

It does not claim BPE-equivalent semantics or universal compression superiority.

## 100 MB highlight

Measured on enwik8:

| metric | result |
|---|---:|
| input | 100,000,000 bytes |
| threads | 16 |
| DU encode | **0.650681 s** |
| throughput | **~153.7 MB/s** |
| global DU mapping ready | **1.223435 s** |
| structural chunks | **31** |
| global vocabulary | 426,714 |
| exact restore | **PASS** |

Approximately sixteen scheduler-sized input regions produced thirty-one
representation chunks because local dictionaries may independently reach the
u16 vocabulary ceiling.

Therefore:

```text
scheduler shard != representation chunk != downstream zRank chunk
```

Processor count is an execution parameter, not a file-format parameter.

## ID-width policy

```text
local storage:          u16
compact global maps:    u24
canonical compute/API:  u32
```

u24 is useful for compact serialization.

u32 is the preferred hydrated representation for arithmetic, SIMD-oriented
processing, LLM/NLP integration, graph operations, and future larger spaces.

The full token stream should not normally be materialized as global-u24.
Local u16 streams can instead be mapped while feeding downstream transforms.

## R1 singleton result

On the same 100 MB input:

| metric | result |
|---|---:|
| global DU vocabulary | 426,714 |
| singleton types | **234,696** |
| singleton share of types | **55.00%** |
| frequent types | **192,018** |
| token positions diverted | **0.599%** |
| DU counts scan | **0.150 s** |
| R1 plan | **0.330 s** |

Primary downstream candidate:

```text
threshold=1
rare-ones-mode=words
```

This reduces the contextual vocabulary from 426,714 symbols to approximately
192,019 modeled symbols including the sentinel, while lexical singleton
exceptions bypass that modeled space.

## Reproduce

```bash
tools/repro_dataset.sh /path/to/input DATASET 16 2
```

The final argument is scheduling tasks per worker.

For enwik9:

```bash
tools/repro_dataset.sh \
  /mnt/data_linux/IT/OKC/enwiks/enwik9 \
  enwik9 \
  16 \
  2
```
