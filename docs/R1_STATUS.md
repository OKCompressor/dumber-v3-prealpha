# Rare1 / R1 implementation status

The DUMBer v3 pre-alpha repository contains the first integrated
`threshold=1, words` Rare1 path.

It is currently implemented as three components.

## 1. Global DU statistics

`devtools/du_stats_reduce.rs`

Input:

```text
local u16 streams
+
packed local-to-global DU maps
```

Operation:

```text
local u16
    |
    v
global DU ID
    |
    v
global unigram count
```

This establishes which lexical types occur exactly once globally.

It does not currently build zRank contextual pair statistics.

## 2. Singleton prune/reindex plan

`devtools/r1_singleton_plan.py`

Input:

```text
global DU counts
+
merged DU dictionary
```

Output:

```text
pruned frequent R1 vocabulary
DU-global -> R1 map
sentinel ID
singleton accounting
```

For the primary mode:

```text
count == 1
    -> SENTINEL

count > 1
    -> dense frequent R1 ID
```

The singleton lexical value is not represented by another rare dictionary ID.

It is carried lexically in the ordered sideband.

## 3. Fused projection / lexical sideband

`devtools/fused_emit.rs`

Hot path:

```text
local u16
    |
    | phi_i
    v
global DU ID
    |
    | rho
    v
R1 ID / SENTINEL
```

When the mapped value is the singleton sentinel, the original lexical token is
emitted to the ordered singleton sideband.

The measured enwik9 run used:

```text
main_materialized=0
```

The complete R1 main stream was intentionally not materialized as u32.

The intended next consumer is the zRank encoder itself.

## Current status

```text
global DU counts                         DONE
singleton classification                 DONE
frequent R1 vocabulary                   DONE
DU-global -> R1 map                      DONE
ordered singleton lexical sideband       DONE
standalone R1 package/decoder             NOT YET
global contextual zRank reducer           NOT YET
fused R1 -> zRank payload writer          NOT YET
```

Therefore the current release demonstrates and measures the R1 structural
projection, but should not be described as a finished standalone Rare1 codec.
