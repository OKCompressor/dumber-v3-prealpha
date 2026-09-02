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
| DU encode | **4.240 s** |
| DU throughput | **235.8 MB/s** |
| global DU mapping ready | **10.650 s** |
| DU stats | 0.950 s |
| R1 plan | 1.090 s |
| fused R1 scan | 1.140 s |
| DU encode max RSS | 2916.6 MiB |
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
