# Reference binary

`redumb-linux-x86_64` is the executable used for the published benchmark
receipts in this pre-alpha.

```text
SHA256:
b342ab57b059957a2b3035d3d842de5d86a7c271c14a8b30bdcd22054badc76c

BuildID:
ed8866f15efd43714c3ea324438fb2a2dd582357
```

The repository includes public helper/reducer source used around this binary.

This pre-alpha does **not** yet claim that the candidate historical Redumb
source snapshot rebuilds this ELF byte-for-byte.

The binary is therefore published as a measured reference/provenance artifact,
not as evidence of a reproducible source build.

A later release gate is:

```text
pinned source
-> clean build
-> behavioral conformance
-> exact restoration
-> benchmark receipt
-> binary/source provenance statement
```
