# Nereid v0 span ledger — fixed DU restore

This receipt aligns an existing dense `cl100k_base` remap stream against the
canonical DU stream by raw byte spans.

The DU lexical placeholders are restored before comparison.

Result:

```text
target decode == raw prefix: PASS
DU decode == raw prefix:     PASS
bytes tested:                2,000,000
target tokens tested:        518,791
DU tokens covering prefix:   781,080
exact DU-boundary targets:   375,821
cross/cut DU-boundary:       142,970
fraction exact boundary:     0.724417
```

This is an alignment/debug receipt, not an official model-ID adapter.
