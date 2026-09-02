# DUMBer -> BPE model bridge

DUMBer and BPE do not need to be competing terminal representations.

A canonical DUMBer vocabulary can act as an intermediate lexical space.

For each global DUMBer token:

```text
DU_TOKEN[g]
    ->
target_tokenizer.encode(DU_TOKEN[g])
    ->
[BPE_ID_0, BPE_ID_1, ...]
```

The result is stored as a compact expansion table:

```text
offset[g]
length[g]
bpe_ids[offset[g] : offset[g] + length[g]]
```

A DUMBer ID stream can then be translated by lookup rather than by rerunning
the target tokenizer's lexical matching over the original raw text.

## enwik8 motivation

Measured data:

```text
raw input bytes        100,000,000
merged DUMBer vocab      3,812,931
```

Thus the lexical material that must be submitted to a target BPE tokenizer is
only about 3.81% of the raw corpus bytes for this dataset.

The later ID expansion still traverses the structural stream, but lexical
tokenization itself can be cached per target model.

## Long-term bridges

The same canonical vocabulary can support:

```text
DUMBer -> BPE
DUMBer -> SentencePiece
DUMBer -> spaCy annotations
DUMBer -> NLTK features
DUMBer -> embedding tables
DUMBer -> graph/entity layers
DUMBer -> model-specific lexical spaces
```

This makes the structural tokenizer an upstream representation rather than a
closed tokenizer island.

Direct BPE -> DUMBer is a separate problem because a BPE sequence need not
map uniquely to a DUMBer lexical boundary.

Safe reverse baseline:

```text
BPE IDs -> text -> DUMBer
```
