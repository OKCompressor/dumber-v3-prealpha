# Nereid v0.1337 — DU lexeme to target-token stream

Nereid v0.1337 is the simplest working adapter path:

```text
decoded DU lexeme
  -> target tokenizer / target vocabulary pieces
  -> target ID sequence
```

Then the corpus is emitted by lookup:

```text
DU stream:
  DU_ID DU_ID DU_ID ...

adapter table:
  DU_ID -> target IDs

output:
  target ID stream
```

This is not BPE over DU IDs.

It is tokenization over decoded DU dictionary entries, cached once and reused
over the DU stream.

## Receipt

On the enwik8 DU run and the existing `bench-pre-v2` dense `cl100k_base`
remap artifact:

```text
DU vocab types:              426,714
target vocab entries:        71,161
DU types segmentable:        426,714
DU types not segmentable:    0
segmentable fraction:        1.0

prefix bytes emitted:        2,000,000
DU positions emitted:        781,080
target IDs emitted:          900,938

DU decode == raw prefix:     PASS
bridge decode == raw prefix: PASS
```

## Meaning

```text
DU canonical representation
  -> target-token stream
  -> original bytes
```

This shows that DUMBer can act as a reusable structural substrate beneath
model tokenizers.

## Limits

The current receipt uses a dense remap artifact:

```text
model.bin  = used token byte table
tokens.u32 = dense IDs into model.bin
```

That is enough for byte roundtrip and compression comparison.

Direct model input requires official target tokenizer IDs.

## Next

```text
v0.1337:
  DU lexeme table, text-equivalent

v1:
  span ledger over raw byte ranges

v2:
  context/window adapter for direct tokenizer-path equivalence

v3:
  compiled plugin packs per model family
```
