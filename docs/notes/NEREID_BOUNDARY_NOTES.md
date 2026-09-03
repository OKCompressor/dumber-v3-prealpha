# Nereid boundary notes

Working claim:

```text
AGI built on next-token prediction
depends on token boundary choices
which are historically contingent engineering artifacts
not semantic necessity
```

Nereid separates:

```text
1. source byte identity
2. DU structural identity
3. target vocabulary validity
4. target model's preferred token-ID path
```

A valid target-ID stream can decode to the same bytes while not matching the
target tokenizer's preferred segmentation.

```text
same decoded bytes
different valid token-ID sequence
different embedding path
different model state
```

## v0.1337 result

Nereid v0.1337 passed on an enwik8 2 MB prefix using the existing dense
`cl100k_base` remap artifact.

```text
DU vocab types:              426,714
DU types segmentable:        426,714
DU types not segmentable:    0
DU decode == raw prefix:     PASS
bridge decode == raw prefix: PASS
```

This confirms the simple adapter shape:

```text
DU decoded dictionary entry
  -> target-token pieces
  -> cached target IDs
  -> stream expansion
```

Exact direct-tokenizer equivalence remains a span/window problem.
