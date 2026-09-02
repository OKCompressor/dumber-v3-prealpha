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

## Ganymede: measured u24 -> u32 crossover

A 10 GB Wikipedia-derived prefix produced **17,294,055 canonical DU lexical
types**, exceeding the 24-bit ID space by **516,839 types (3.08%)**.

Ganymede therefore widens only the reconciliation layer:

```text
local token payload       u16
local vocabulary          <= 65,536
canonical map             u24 or u32
compute/API identity      u32
```

The 10 GB run produced 1828 vocabulary-bounded local representation
chunks and a 477768072-byte gmap32 layer.

Exact restoration passed, with identical source/restored SHA256:

```text
4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

## Canonical bundle versus build workspace

At 10 GB, Ganymede's retained reversible representation is:

```text
local u16        7,605,565,792 B
merged vocab       214,565,572 B
gmap32              477,768,072 B
                  ----------------
canonical         8,297,899,436 B
```

This is **17.021% below the 10,000,000,000-byte source before entropy coding**.

The additional 1,033,808,012 bytes of local dictionaries are construction
state and can be discarded after the canonical map has been built.

## 10 GB: structural transform before entropy coding

At the measured 10 GB scale point:

| representation | bytes |
|---|---:|
| raw | 10,000,000,000 |
| canonical Ganymede | **8,297,899,436** |
| raw + zstd1 | **3,482,115,937** |
| Ganymede + zstd1 | **3,280,838,574** |

The canonical representation is **17.021% smaller than raw before entropy
coding**.

After zstd1, the structural-first representation remains **201,277,363 bytes,
or 5.780%, smaller** than raw+zstd1.

The current zstd timing comparison is intentionally not treated as a
throughput result because the raw source was substantially cache-served while
the canonical artifact was read as thousands of files from an external
mechanical volume.

The next performance comparison is SSD-backed and chunk-parallel.

## Entropy-backend Pareto at 10 GB

The 10 GB experiment exposes two different operating points.

```text
zstd1
raw                         3,482,115,937 B
Ganymede structural-first   3,280,838,574 B
gain                           -5.780%

zstd19
raw                         2,321,300,290 B
Ganymede structural-first   2,554,960,620 B
size cost                     +10.066%

zstd19 wall
raw                            853.83 s
Ganymede                       505.05 s
entropy-stage wall delta       -40.85%
```

At low entropy effort, the structural transform improves final size. At high
entropy effort, raw zstd19 achieves the smaller artifact, while zstd19 over
the structural representation completes substantially faster.

Using the separately measured construction stages, the composite
raw -> Ganymede -> zstd19 path sums to **745.53 s**, versus **853.83 s** for
raw -> zstd19, a **12.68% wall advantage** at a **10.066% size cost**.

These are Pareto points rather than a universal compressor-ranking claim.

The current 10 GB result covers DUMBer canonicalization only; R1 and zRank are
not part of this measurement.

## Ganymede release boundary

DUMBer v3 pre-alpha4 / Ganymede establishes a 10 GB exact-reversible
canonical layer beyond u24:

```text
10 GB source
 -> 1,828 local u16 representation chunks
 -> 17,294,055 global DU identities
 -> gmap32
 -> 8,297,899,436-byte retained canonical bundle
 -> exact restore PASS
```

Measured raw-to-canonical construction:

```text
DU encode           102.80 s
merge                90.41 s
gmap32                47.27 s
                    --------
total                240.48 s = 4:00.48
```

Linear 100 GB projection:

```text
40.08 minutes
```

This is an extrapolation only. SSD-backed bounded-input 10 GB/100 GB scaling
and actual 100 GB limits remain to be measured.

The 10 GB release result stops at DUMBer canonicalization. R1 and zRank were
not run on this dataset.

The next transform boundary is model/PLM token-space transduction, followed
by widened-map R1/zRank integration and parallel entropy containers.
