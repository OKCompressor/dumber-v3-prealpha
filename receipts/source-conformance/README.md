# Recovered-source conformance receipt

A recovered Redumb v0.3.0 Rust worktree was rebuilt and compared with the
reference executable used for the published DUMBer measurements.

Recovered source state:

```text
git HEAD:
75c89feb7090e0debc4413b0b242e93d9b35c955

describe:
v0.4-preview-local-8-g75c89fe-dirty
```

Reference executable:

```text
SHA256:
b342ab57b059957a2b3035d3d842de5d86a7c271c14a8b30bdcd22054badc76c

BuildID:
ed8866f15efd43714c3ea324438fb2a2dd582357
```

Fresh rebuild:

```text
SHA256:
371f3dc910f73252044f0b2a0a4cc6fb0a436e2c175ee7932237388cdd2431e5

BuildID:
3b58397f7e80ae8eefb107be4f9c5a82f5ce2b78
```

The binaries are therefore not claimed to be byte-identical.

Measured conformance:

```text
help_exact=PASS
ref_restore_exact=PASS
rebuilt_restore_exact=PASS
merged_dict_exact=PASS
fixture_artifacts_exact=PASS
```

This establishes behavioral and artifact-level conformance for the tested
CLI path. It does not establish bit-reproducible compilation.

The recovered source worktree was dirty. The exact worktree used for this
test is separately pinned in release provenance storage together with its Git
HEAD, working-tree patch and file hashes.
