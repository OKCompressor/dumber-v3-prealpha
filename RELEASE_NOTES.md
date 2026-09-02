# DUMBer v3 pre-alpha

First public structural-tokenization preview.

Highlights:

- parallel vocabulary-bounded u16 tokenization
- exact reversible text restoration
- dynamic representation rollover
- global dictionary reconciliation
- packed local-to-global maps
- canonical u32 compute lane
- measured 10 MB / 100 MB receipts
- R1 singleton projection prototype
- DU -> BPE bridge design
- bounded-delay research note

Measured enwik8:

```text
100 MB input
16 threads
0.651 s DU encode
~153.7 MB/s
1.223 s global mapping ready
31 representation chunks
exact restore PASS
```

R1 t1/words planning on the same structural stream:

```text
426,714 DU types
234,696 singleton types
192,018 frequent types
55.00% type-space reduction
0.599% token positions diverted
```

Public binary provenance and source-rebuild conformance remain an explicit
future step.
