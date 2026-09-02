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
