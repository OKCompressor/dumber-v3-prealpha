# Nereid v0.1337 — DU lexeme table receipt

This receipt tests the minimal adapter shape:

```text
DU decoded lexeme
  -> target token pieces
  -> cached target IDs
  -> stream expansion
  -> target decode
  -> original bytes
```

Result on a 2 MB enwik8 prefix using the existing dense `cl100k_base` remap
artifact:

```text
DU vocab types:              426,714
DU types segmentable:        426,714
DU types not segmentable:    0
segmentable fraction:        1.0
DU decode == raw prefix:     PASS
bridge decode == raw prefix: PASS
```

The dense remap IDs are sufficient for roundtrip and compression comparison.
Official model-input adapters must preserve official target tokenizer IDs.
