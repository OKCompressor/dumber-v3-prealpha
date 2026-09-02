# DUMBer v3 pre-alpha4 — Ganymede

Ganymede is the first DUMBer development release validated beyond the u24
canonical identity space.

Parent body: Jupiter  
Inherited milestone: Europa bounded-input work

## 10 GB scale point

Dataset:

```text
name=enwik10-20251101-prefix
bytes=10,000,000,000
sha256=4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

Measured global vocabulary:

```text
17,294,055 canonical DU types
```

u24 capacity:

```text
16,777,216
```

Measured excess:

```text
516,839 IDs
3.08%
```

This corpus therefore requires a canonical global ID space wider than u24.

Local representation chunks remain u16.

## Retained canonical representation

| component | bytes |
|---|---:|
| local u16 | 7,605,565,792 |
| merged global dictionary | 214,565,572 |
| gmap32 | 477,768,072 |
| **canonical reversible bundle** | **8,297,899,436** |
| raw source | 10,000,000,000 |

The retained canonical representation is **17.021% smaller than raw before
entropy coding**.

The additional 1,033,808,012 bytes of local dictionaries are construction
state and are not required after canonical reconciliation.

Representation chunks:

```text
1,828
```

A full local chunk remains bounded to:

```text
65,536 local vocabulary entries
```

and therefore:

```text
local token IDs      u16
full gmap32 table    65,536 × 4 = 262,144 bytes
```

## Exactness

```text
restore_gmap32_rc=0
restore_gmap32_exact=PASS
```

Source and restored SHA256:

```text
4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

Validated Ganymede ELF:

```text
SHA256
85aad7063a4d5361ff0f35d47b835de56c6ae0d988f2c6fb031a5c94b118ca9b

BuildID
066152f5b827532c756047fc510b127e4d6453a7
```

## 10 GB construction timing

Measured raw -> canonical construction:

```text
DU encode            102.80 s
merge dictionaries    90.41 s
build gmap32           47.27 s
                     --------
total                 240.48 s
```

Therefore:

```text
10 GB measured        240.48 s = 4:00.48
100 GB linear model  2404.80 s = 40.08 min
```

The 100 GB figure is an extrapolation, not a measured 100 GB result.

This 10 GB run used the historical whole-file front end for the DU stage on
an external mechanical volume. Europa's bounded-input implementation has
been validated separately but has not yet been benchmarked at 10 GB or
100 GB on SSD.

Still to be measured:

- Europa bounded-input at 10 GB and 100 GB
- SSD-backed cold-cache throughput
- actual 100 GB scaling
- peak RSS under the bounded-input path
- merge scalability
- automatic u24/u32 pipeline selection
- chunk-parallel entropy backends
- parallel restore / random access

## Entropy Pareto

| 10 GB path | output bytes | measured encode/build wall | peak RSS |
|---|---:|---:|---:|
| Raw | 10,000,000,000 | — | — |
| Canonical Ganymede | 8,297,899,436 | **240.48 s*** | stage-dependent |
| Raw + zstd1 | 3,482,115,937 | **23.16 s†** | 36,724 KiB |
| Ganymede + zstd1 | **3,280,838,574** | 324.13 s† | 32,956 KiB |
| Raw + zstd19 | **2,321,300,290** | **853.83 s** | 1,109,104 KiB |
| Ganymede + zstd19 | 2,554,960,620 | **505.05 s** | 1,204,668 KiB |
| Ganymede -> exact raw | 10,000,000,000 | **369.62 s** | 1,178,196 KiB |

`*` Raw-to-canonical is the sum of the measured DU, merge and gmap32 stages.

`†` The zstd1 wall-time comparison is storage/cache contaminated and is not
used as a codec-throughput claim.

### Low-effort entropy point

```text
raw + zstd1           3,482,115,937 B
Ganymede + zstd1      3,280,838,574 B
difference             -201,277,363 B
                       -5.780%
```

At zstd1, structural-first encoding produces the smaller artifact.

### High-effort entropy point

```text
raw + zstd19          2,321,300,290 B
Ganymede + zstd19     2,554,960,620 B

raw zstd19 wall            853.83 s
Ganymede zstd19 wall       505.05 s
entropy-stage delta        -40.85%
```

At zstd19, direct raw compression produces the smaller artifact while the
entropy stage over the canonical representation completes substantially
faster.

Using separately measured stages:

```text
raw -> Ganymede

DU                    102.80 s
merge                  90.41 s
gmap32                  47.27 s

Ganymede -> zstd19     505.05 s
                      --------
composite              745.53 s

raw -> zstd19          853.83 s
```

The structural-first composite is **108.30 seconds (12.68%) lower in measured
stage wall time**, at a **10.066% final-size cost**.

This is a Pareto point, not a universal compressor-ranking claim.

## Scope boundary

The 10 GB measurements above cover:

```text
raw
 -> local DU/u16
 -> global vocabulary merge
 -> canonical gmap32
 -> optional general-purpose entropy backend
```

They do **not** include:

```text
R1
zRank
```

No 10 GB R1 or zRank size/performance claim is implied by this release.

The canonical Ganymede stream is intended to be reusable input to those
downstream transforms once their global-map readers support the widened
identity space.

## Roadmap from Ganymede

Current sequence:

```text
Europa
bounded input
    ✓

Ganymede
u32 canonical identity space
    ✓

Triton
model / PLM token-space transduction
DUMB2(b)PE proof
    ↓

R1 integration
gmap24/gmap32-aware reducers
10 GB without re-tokenization
    ↓

zRank integration
contextual structural coding over canonical/R1 IDs
    ↓

parallel entropy containers
independently addressable structural frames
parallel encode/decode
bounded working sets
random access
    ↓

broader Pareto matrix
zstd and additional general-purpose / archival backends
    ↓

custom OKC entropy transforms
```

The long-term target is not to permanently place zstd behind DUMBer.
General-purpose codecs are baselines and optional backends.

The structural stream provides a stable boundary at which model-specific
transduction, R1, contextual zRank and future entropy transforms can operate
without re-tokenizing the original corpus.


## R1 evidence and 10 GB planning bound

R1 and zRank were not executed on the 10 GB Ganymede corpus.

Existing measured R1 evidence:

| stage | enwik8 / 100 MB | enwik9 / 1 GB |
|---|---:|---:|
| DU global unigram statistics | 0.150 s | 1.07 s |
| singleton/R1 planning | 0.330 s | 1.10 s |
| fused R1 scan | — | 1.19 s |

The 10 GB Ganymede local-u16 representation contains:

```text
local_u16_bytes=7,605,565,792
token_positions=3,802,782,896
global_vocabulary=17,294,055
```

Relative to enwik9:

```text
token-position scale = 9.436x
global-vocab scale   = 7.758x
```

A simple extrapolation of the already-measured stages gives:

```text
DU stats             ~10.1 s
R1 plan               ~8.5 s
fused R1 scan        ~11.2 s
                     -------
compute model        ~29.9 s
```

These are planning estimates, not 10 GB benchmark results.

On the external mechanical test volume, storage is expected to dominate:
one sequential pass over the 7.606 GB local-u16 stream has a physical
read-time floor of roughly 80 seconds at the previously measured
~95.5 MB/s rate.

A 10 GB R1 run therefore remains a small follow-up once the reducers accept
gmap32. It does not require re-tokenizing the source.

The current Python singleton planner is not the intended large-corpus
implementation. A Rust planner is the next implementation target.

No 10 GB zRank time is extrapolated here. Context-pair reduction and its
working-set/storage behavior will be measured directly after widened-map R1
integration.
