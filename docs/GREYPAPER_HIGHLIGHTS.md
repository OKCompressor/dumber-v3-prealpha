# DUMBer v3 pre-alpha — grey-paper highlights

## Parallel reversible structural tokenization

DUMBer maps text into vocabulary-bounded local u16 token streams.

Local representation spaces can be reconciled into a canonical global
vocabulary without materializing the entire corpus as global-width IDs.

### Measured enwik7 — 10 MB

| metric | result |
|---|---:|
| parallel DU encode | **0.105719 s** |
| global DU mapping ready | **0.218985 s** |
| exact restore | **PASS** |

### Measured enwik8 — 100 MB

| metric | result |
|---|---:|
| threads | 16 |
| parallel DU encode | **0.650681 s** |
| throughput | **~153.7 MB/s** |
| global DU mapping ready | **1.223435 s** |
| representation chunks | **31** |
| exact restore | **PASS** |

Approximately sixteen scheduling regions produced thirty-one representation
chunks because local u16 vocabularies can roll independently.

Therefore:

```text
scheduler shard
!= representation chunk
!= downstream zRank payload chunk
```

Processor count is an execution parameter rather than a file-format
parameter.

## Vocabulary reconciliation

### enwik8

| artifact | bytes |
|---|---:|
| local dictionaries | 11,330,612 |
| merged dictionary | **3,812,931** |
| packed gmap24 | 4,102,107 |
| merged dictionary + gmap | **7,915,038** |

The merged dictionary removes approximately 66.35% of duplicate local
dictionary bytes.

The merged dictionary plus packed map remains approximately 30.14% below the
collection of local dictionaries.

## Width policy

```text
u16 = local structural storage
u24 = compact serialized global mapping
u32 = hydrated compute / API / vector lane
```

The complete corpus need not be materialized as global-u24.

## Singleton projection

On enwik8:

| metric | result |
|---|---:|
| global DU types | 426,714 |
| singleton types | **234,696** |
| singleton share of types | **55.00%** |
| frequent R1 types | **192,018** |
| token positions diverted | **0.599%** |
| DU stats scan | **0.150 s** |
| R1 plan | **0.330 s** |

The primary R1 candidate is:

```text
threshold=1
rare-ones-mode=words
```

This removes more than half of the modeled symbol types while diverting fewer
than six token positions per thousand into the lexical singleton path.

## Current structural-compression baseline

enwik8, 100 MB:

| representation | bytes | encode total |
|---|---:|---:|
| raw LF + zstd1 | 40,675,947 | **0.095 s** |
| R1 + zstd1 | 39,642,839 | 5.680 s |
| zRank + zstd1 | **34,206,369** | 7.845 s |

The current monolithic zRank representation is 15.90% smaller than raw+zstd1
on this run, but currently pays substantial encode latency.

Parallel DU/reducer work targets that latency.

## Reducer critical path

```text
DU workers                    ███████████████████
dictionary reducers             ████████████████
unigram/context reducers          ██████████████
R1 singleton planning               ███████████
zRank model preparation               █████████
                                         │
                                   model freeze
                                         █
parallel fused emission                 ███
```

The target is to overlap most structural preparation with the slowest worker
wave, leaving a small global barrier plus a highly parallel final emission
pass.

## Batch and streaming model evolution

Batch-global:

```text
all reducer state
-> one frozen global model
-> parallel payload emission
```

Future stream epochs:

```text
M0 --Δ1--> M1 --Δ2--> M2
```

A chunk identifies the frozen model epoch under which it was encoded.

`Δ` denotes a model-state update, not necessarily ordinary numerical
addition.

This allows later chunks to use richer global state without rewriting earlier
payloads.

## 1 GB scaling result

A 1 GB enwik9 run using 16 worker threads and approximately 32 scheduling
regions produced:

```text
DU encode                4.46 s
DU throughput          ~224.2 MB/s
global mapping ready    11.54 s
reproduction harness wall      26.19 s
exact restore            PASS

representation chunks      188
u16 rollovers               156
```

The same run produced 2,229,308 globally unique DU types.

Rare1 singleton projection identified:

```text
1,380,346 singleton types = 61.92% of type space
0.343% of token positions
848,962 remaining frequent types
```

Thus vocabulary growth with corpus size strengthens the motivation for
singleton pruning: a majority of lexical types can be excluded from the main
contextual model while affecting only a small fraction of token positions.

## Measurement semantics for the 1 GB run

The 26.19 s number is the wall time of the complete reproducibility harness,
not a codec decode measurement.

The principal structural milestones are:

```text
parallel DU encode              4.46 s
canonical DU mapping ready     11.54 s
exact DU structural restore     7.72 s
stats + R1 plan + fused scan    3.36 s
measured stage sum             22.62 s
reproduction harness wall      26.19 s
```

Peak memory during the harness was approximately 2.87 GiB, occurring in the
parallel DU encode stage.

### Vocabulary scaling

```text
enwik8:
  global types       426,714
  singleton types    234,696   55.00%
  singleton tokens               0.599%

enwik9:
  global types     2,229,308
  singleton types  1,380,346   61.92%
  singleton tokens               0.343%
```

The larger corpus increases the share of lexical types excluded by singleton
pruning while decreasing the fraction of token positions that take the
exception path.

## Memory bound versus representation bound

The u16 vocabulary ceiling is currently a bound on each local representation
space, not a claim that the present encoder has constant process memory.

On the 1 GB run:

```text
parallel DU encode peak RSS    ~2.87 GiB
DU statistics peak RSS          ~43 MiB
R1 planning peak RSS           ~209 MiB
fused R1 scan peak RSS          ~95 MiB
```

The dominant memory target is therefore the current parallel DUMBer front end,
not the singleton reducer.

Future streaming work should distinguish:

```text
representation-state bound
scheduler working-set bound
scanner unresolved-input bound
whole-process RSS
```

A bounded local vocabulary alone does not prove all four.

## Release boundary

The distributed Redumb/DUMBer ELF is the exact executable used for the
published receipts.

This pre-alpha publishes it as a measured reference binary.

Before a v0.1-style release, one of the following should hold:

```text
A. pinned source -> rebuilt binary -> behavioral/conformance receipt

or

B. source release with the historical reference ELF clearly separated
```

The current pre-alpha does not claim byte-reproducible source-to-ELF
provenance.
