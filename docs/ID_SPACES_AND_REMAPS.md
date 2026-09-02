# ID spaces and remaps

DUMBer deliberately separates local storage IDs from canonical and
transform-specific identity spaces.

## Symbols

| symbol | meaning |
|---|---|
| `l` | local u16 ID inside one DUMBer chunk |
| `φᵢ` | local-to-global DU remap for chunk `i` |
| `g` | canonical global DU ID |
| `ρ` | global-DU to Rare1-pruned remap |
| `r` | resulting R1 ID or rare sentinel |

The basic composition is:

```text
l --φᵢ--> g --ρ--> r
```

or:

```text
r = ρ(φᵢ(l))
```

`φᵢ` reconciles independently constructed local vocabularies.

`ρ` is a later optional structural projection. For the primary
`threshold=1, words` configuration, globally singleton lexical types are
removed from the frequent modeled vocabulary and represented by a sentinel
plus lexical sideband.

These two maps should remain conceptually distinct:

```text
φ : identity reconciliation
ρ : pruning / structural reindexing
```

Downstream transforms may introduce further named spaces without modifying the
canonical DU identity layer.

## Width policy

```text
local structural lane      u16
compact persistent remap   u24
canonical compute lane     u32
```

The width of a serialized map is not required to equal the arithmetic width
used by downstream computation.
