# Measurements

## enwik7 — 10 MB DU merger POC

| stage | seconds |
|---|---:|
| parallel u16 DU encode | **0.105719** |
| merge dictionaries | 0.047124 |
| build gmap24 | 0.066142 |
| **global DU mapping ready** | **0.218985** |
| global-u24 conformance materialization | 0.067533 |
| restore through gmap | 0.172057 |
| restore global-u24 | 0.150887 |

```text
restore_gmap_exact=PASS
restore_global_u24_exact=PASS
```

| artifact | bytes |
|---|---:|
| local dictionaries | 1,810,006 |
| merged dictionary | **838,432** |
| gmap24 | 672,942 |
| merged dict + gmap | **1,511,374** |
| local u16 stream | 7,957,428 |
| global u24 stream | 11,936,142 |

The merged dictionary removes 53.68% of duplicated dictionary bytes.

Merged dictionary plus gmap remains 16.50% smaller than retaining every local
dictionary.

The global-u24 token stream costs exactly 50% more bytes than the local-u16
stream and is therefore a conformance representation, not the intended hot
path.

---

## enwik8 — 100 MB parallel DU

| stage | seconds |
|---|---:|
| parallel u16 DU encode | **0.650681** |
| merge dictionaries | 0.211210 |
| build gmap24 | 0.361544 |
| **global DU mapping ready** | **1.223435** |
| global-u24 conformance materialization | 0.401617 |
| restore through gmap | 0.997115 |
| restore global-u24 | 0.921565 |

```text
restore_gmap_exact=PASS
restore_global_u24_exact=PASS
```

| artifact | bytes |
|---|---:|
| local dictionaries | 11,330,612 |
| merged dictionary | **3,812,931** |
| gmap24 | 4,102,107 |
| merged dict + gmap | **7,915,038** |
| local u16 stream | 78,387,360 |
| global u24 stream | 117,581,040 |

Additional facts:

```text
tokens             = 39,193,680
representation chunks = 31
global vocab       = 426,714

merged dict reduction:
-66.35%

merged dict + gmap reduction vs local dict set:
-30.14%
```

---

## enwik8 — R1 t1/words projection

| metric | result |
|---|---:|
| DU vocab | 426,714 |
| singleton types | **234,696** |
| singleton type fraction | **55.0008%** |
| frequent types | **192,018** |
| singleton token fraction | **0.5988%** |
| literal singleton bytes | 1,972,346 |
| u16-length-prefixed lexical bytes | 2,441,738 |
| DU global→R1 map | 1,706,856 |
| stats wall | **0.150 s** |
| stats RSS | **12.1 MiB** |
| R1 plan wall | **0.330 s** |
| R1 plan RSS | **50.2 MiB** |

Paper highlight:

> Singleton pruning removes more than half of the modeled symbol types while
> diverting fewer than six token positions per thousand into the lexical
> exception path.

---

## enwik8 — current monolithic zRank baseline

| representation | bytes | encode total |
|---|---:|---:|
| raw LF + zstd1 | 40,675,947 | **0.095 s** |
| R1 + zstd1 | 39,642,839 | 5.680 s |
| zRank bundle + zstd1 | **34,206,369** | 7.845 s |

```text
zRank+zstd1 vs raw+zstd1:
-6,469,578 bytes
-15.90%

zRank+zstd1 vs R1+zstd1:
-5,436,470 bytes
-13.71%
```

The latency and size axes must be reported separately.

The DU parallel path establishes a route toward reducing structural latency;
it is not yet an end-to-end parallel zRank result.

<!-- ENWIK9_BEGIN -->
# enwik9 DU/R1 receipt

| metric | result |
|---|---:|
| input | 1,000,000,000 bytes |
| CPU threads | 16 |
| scheduler target tasks | 32 |
| macro size | 30 MiB |
| representation chunks | 188 |
| u16 rollovers | 156 |
| DU encode | **4.460 s** |
| DU throughput | **224.2 MB/s** |
| global DU mapping ready | **11.540 s** |
| DU stats | 1.070 s |
| R1 plan | 1.100 s |
| fused R1 scan | 1.190 s |
| DU encode max RSS | 2936.3 MiB |
| DU stats max RSS | 42.9 MiB |

## Vocabulary

| metric | value |
|---|---:|
| global DU types | 2,229,308 |
| singleton types | **1,380,346** |
| singleton type fraction | **61.92%** |
| frequent R1 types | **848,962** |
| singleton token fraction | **0.343%** |

## Artifacts

| artifact | bytes |
|---|---:|
| local dictionaries | 95,280,910 |
| merged dictionary | **20,856,995** |
| gmap24 | 34,537,734 |
| local u16 payload | 805,981,206 |
| pruned R1 vocabulary | 7,473,916 |

## Exactness

```text
restore_gmap_exact=PASS
```

The global-u24 token stream is intentionally not materialized in this run.

<!-- ENWIK9_END -->


<!-- FINAL_ENWIK9_BEGIN -->
## enwik9 — 1 GB full DU/R1 receipt

Input:

```text
1,000,000,000 bytes
SHA256=159b85351e5f76e60cbe32e04c677847a9ecba3adc79addab6f4c6c7aa3744bc
threads=16
scheduler target tasks=32
macro size=30 MiB
```

### Timing

| stage | wall | max RSS |
|---|---:|---:|
| parallel DU encode | **4.46 s** | **2,936 MiB** |
| merge dictionaries | 2.93 s | 200 MiB |
| build gmap24 | 4.15 s | 254 MiB |
| **global DU mapping ready** | **11.54 s** | — |
| exact restore through gmap | 7.72 s | 259 MiB |
| DU unigram stats | 1.07 s | 42.9 MiB |
| R1 t1/words plan | 1.10 s | 209 MiB |
| fused R1 scan | 1.19 s | 95.2 MiB |
| **reproduction harness wall** | **26.19 s** | **2,936 MiB peak** |

Exactness:

```text
restore_gmap_exact=PASS
```

Parallel DU throughput:

```text
~224.2 MB/s
```

### Structural topology

| metric | value |
|---|---:|
| scheduler target tasks | 32 |
| representation chunks | **188** |
| u16 vocabulary rollovers | **156** |

This reinforces that execution sharding and representation boundaries are
independent.

### Vocabulary / R1 projection

| metric | value |
|---|---:|
| global DU types | 2,229,308 |
| singleton types | **1,380,346** |
| singleton type fraction | **61.92%** |
| frequent R1 types | **848,962** |
| singleton token fraction | **0.343%** |
| literal singleton bytes | 12,002,741 |

The singleton projection removes nearly 62% of the modeled type space while
diverting approximately 3.4 token positions per thousand.

### Artifact accounting

| artifact | bytes |
|---|---:|
| local dictionaries | 95,280,910 |
| merged DU dictionary | **20,856,995** |
| gmap24 | 34,537,734 |
| local u16 payload | 805,981,206 |
| pruned R1 vocabulary | **7,473,916** |

The full global-u24 corpus representation was deliberately not materialized.
<!-- FINAL_ENWIK9_END -->

## Timing semantics — enwik9 reproduction run

The outer `/usr/bin/time -v` wrapped the complete reproduction script.

It therefore measures more than structural encode or decode.

| measurement | wall |
|---|---:|
| parallel DU encode | **4.46 s** |
| merge dictionaries | 2.93 s |
| build gmap24 | 4.15 s |
| **global DU mapping ready** | **11.54 s** |
| exact restore through local-u16 + gmap | **7.72 s** |
| DU stats | 1.07 s |
| R1 plan | 1.10 s |
| fused R1 scan | 1.19 s |
| **sum of explicitly measured stages** | **22.62 s** |
| **complete reproduction-harness wall** | **26.19 s** |

The approximately 3.57 s difference includes helper compilation and
non-stage harness/reporting/file-management overhead.

`restore_gmap = 7.72 s` is the measured structural text-restoration path.

It is **not** entropy-codec decompression time.

No claim should label 26.19 s as simply "decode", "restore", or
"encode + decode".

## 10 GB vocabulary-width crossover

Dataset:

```text
enwik10-20251101-prefix
bytes=10,000,000,000
sha256=4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

Observed global vocabulary:

```text
17,294,055 types
```

Packed u24 capacity:

```text
16,777,216 IDs
```

Therefore the measured corpus exceeds the u24 global-ID space by:

```text
516,839 IDs
3.08%
```

This is the first measured DUMBer dataset requiring a u32-capable canonical
global map.

### Important execution note

This particular 10 GB attempt did **not** use the experimental Europa
bounded-input binary.

The receipt identifies:

```text
redumb_sha256=b342ab57b059957a2b3035d3d842de5d86a7c271c14a8b30bdcd22054badc76c
macro_mb=597
```

The run therefore exercised the historical whole-file input path.

Observed values:

```text
DU encode      102.80 s
peak RSS       23,293,084 KiB
merge dicts     90.41 s
global vocab    17,294,055
```

These numbers are useful as a scale/crossover observation but are not an
Europa bounded-memory benchmark.

The run stopped deliberately before gmap serialization because the u24 guard
detected the overflow.

## Ganymede — 10 GB canonical u32 mapping

The 10 GB Wikipedia prefix crossed the u24 canonical-identity ceiling.

| metric | result |
|---|---:|
| input | 10,000,000,000 B |
| global DU vocabulary | **17,294,055** |
| u24 capacity | 16,777,216 |
| excess over u24 | **516,839 / 3.08%** |
| representation chunks | **1828** |
| local dictionary bytes | **1033808012** |
| local u16 payload bytes | **7605565792** |
| merged dictionary bytes | **214565572** |
| gmap32 bytes | **477768072** |
| build gmap32 | **47.27 s** |
| gmap32 peak RSS | **2,049,308 KiB** |
| exact structural restore | **6:09.62** |
| restore peak RSS | **1,178,196 KiB** |
| exactness | **PASS** |

The local payload remains u16. Ganymede widens only the canonical
local-to-global reconciliation map when the global vocabulary exceeds u24.

The restore benchmark ran from an external mechanical volume measured at
approximately 95.5 MB/s sequential read throughput.

SHA256 exactness:

```text
source:   4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
restored: 4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

## Ganymede canonical restore bundle

The complete working directory contains build intermediates that are not
required after canonical reconciliation.

Measured 10 GB accounting:

```text
raw source              10,000,000,000 B

local u16                7,605,565,792 B
merged dictionary          214,565,572 B
gmap32                     477,768,072 B
                         ----------------
canonical restore bundle 8,297,899,436 B

local dictionaries       1,033,808,012 B   build intermediate
```

Therefore the reversible canonical bundle is **17.021% smaller than the raw
source before applying a general-purpose entropy codec**.

`local_dicts/` is required to construct the canonical mapping but is not
required by `restore-u16-gmap32` after `merged.dict` and `gmap32/` exist.

The full working-artifact accounting of 9,331,707,448 bytes should therefore
not be confused with the retained canonical representation.

## 10 GB Ganymede + zstd1 Pareto point

The retained canonical representation was also passed through zstd level 1.

| representation | bytes | fraction of 10 GB source | measured wall |
|---|---:|---:|---:|
| raw source | 10,000,000,000 | 100.000% | — |
| Ganymede canonical, pre-entropy | **8,297,899,436** | **82.979%** | raw→canonical: **240.48 s*** |
| raw + zstd1 | **3,482,115,937** | **34.821%** | **23.16 s†** |
| Ganymede canonical + zstd1 | **3,280,838,574** | **32.808%** | **324.13 s†** |

Ganymede + zstd1 is **201,277,363 bytes (5.780%) smaller** than applying
zstd1 directly to the raw source.

Relative to the original 10 GB source, the resulting artifact is
**32.808% of source size**, or a **67.192% reduction**.

The zstd1 artifact is 39.538% of the 8,297,899,436-byte canonical structural
representation.

### Timing context

`*` The measured 10 GB raw-to-canonical path was:

```text
DU encode       102.80 s
merge dicts      90.41 s
build gmap32     47.27 s
                --------
total           240.48 s
```

This initial DU encode used the historical whole-file front end on the
external Transcend volume; it is a scale/conformance measurement, not an
Europa bounded-input performance result.

At exactly linear scaling, 240.48 seconds per 10 GB corresponds to
approximately **40.08 minutes per 100 GB**. This is an extrapolation, not a
measured 100 GB benchmark.

`†` The zstd wall times are not directly storage-fair. Raw zstd1 processed
10 GB in 23.16 s at 182% CPU, corresponding to an effective source-read rate
well above the previously measured physical-disk sequential rate. The source
was therefore substantially served from cache. The canonical tar+zstd1 run
used only 19% CPU and spent 324.13 s reading thousands of structural files
from the external volume.

The compressed **byte-size comparison remains valid**. Timing Pareto
measurements require controlled cache state or an SSD-backed rerun.

### Next Pareto measurements

1. Cold-cache raw vs canonical zstd on the same SSD.
2. Parallel compression of independently addressable structural chunks.
3. Separate throughput accounting for structural transform and entropy
   backend.
4. Preserve independently decompressible chunks for parallel restore and
   random access.
5. Repeat at larger scale after Europa/Io working-set reductions.

A natural parallel backend is:

```text
local structural chunks
    ├── worker 0 -> entropy frame
    ├── worker 1 -> entropy frame
    ├── worker 2 -> entropy frame
    └── ...
```

This trades some cross-chunk entropy context for parallel encode/decode,
bounded working sets, random access and scheduler-level scalability.

## 10 GB hard-entropy Pareto

The same 10 GB source and retained Ganymede canonical representation were
measured with zstd level 19.

| path | retained bytes | measured encode/build wall | peak RSS |
|---|---:|---:|---:|
| raw source | 10,000,000,000 | — | — |
| Ganymede canonical | 8,297,899,436 | 240.48 s* | stage-dependent |
| raw + zstd1 | 3,482,115,937 | 23.16 s† | 36,724 KiB |
| Ganymede + zstd1 | **3,280,838,574** | 324.13 s† | 32,956 KiB |
| raw + zstd19 | **2,321,300,290** | 853.83 s | 1,109,104 KiB |
| Ganymede + zstd19 | 2,554,960,620 | **505.05 s** | 1,204,668 KiB |

At level 1, the structural-first artifact is **201,277,363 bytes (5.780%)
smaller** than raw+zstd1.

At level 19, raw+zstd19 is **233,660,330 bytes smaller**; the structural-first
artifact is **10.066% larger** than raw+zstd19.

However, the level-19 entropy stage itself is substantially faster on the
Ganymede representation:

```text
raw zstd19          853.83 s
Ganymede zstd19     505.05 s
difference         -348.78 s
wall delta          -40.85%
```

Both level-19 runs saturated approximately 7-8 CPU cores, making this timing
comparison substantially cleaner than the level-1 external-disk run.

### Composite measured-stage path

The measured components of raw -> Ganymede -> zstd19 are:

```text
DU encode           102.80 s
merge dictionaries   90.41 s
build gmap32          47.27 s
zstd19 canonical     505.05 s
                    --------
sum                  745.53 s
```

Raw -> zstd19 measured 853.83 s.

The sum of separately measured structural-first stages is therefore
**108.30 s (12.68%) lower**, while producing an artifact 10.066% larger.

This is a composite sum of separately measured stages, not a single
end-to-end timed invocation.

### Decode semantics

The exact Ganymede canonical restore to the original 10 GB source measured:

```text
wall                 369.62 s
peak RSS           1,178,196 KiB
exact restore            PASS
```

This should be compared with entropy-codec decode time, not entropy encode
time.

A full compressed-Ganymede decode requires two stages and remains to be
measured:

```text
Ganymede.tar.zst
    -> decompress/extract canonical artifacts
    -> DUMBer structural restore
    -> original text
```

The current 10 GB run does not include R1 or zRank.

### 100 GB scale projection

Measured raw -> canonical construction at 10 GB:

```text
DU encode            102.80 s
merge dictionaries    90.41 s
build gmap32           47.27 s
                     --------
total                 240.48 s
```

Exact linear projection:

```text
100 GB ~= 2,404.8 s ~= 40.08 min
```

This is an extrapolation, not a measured 100 GB benchmark.

The measurement used an external mechanical volume and the historical
whole-file DU front end. The following remain to be tested:

- Europa bounded-input at 10 GB and 100 GB
- SSD-backed cold-cache throughput
- actual 100 GB scaling
- peak RSS under the bounded-input path
- dictionary-merge scalability
- auto u24/u32 pipeline selection
- chunk-parallel entropy coding
- parallel/random-access restore
- gmap32 support in R1 and zRank reducers

