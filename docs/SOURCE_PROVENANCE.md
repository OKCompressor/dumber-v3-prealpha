# Source / binary provenance

Distributed preview binary:

```text
/mnt/data_linux/IT/OKC-releases/prestage/dumb-rare/_private_ref/redumb-v3preview-2026-09-02/bin/redumb
SHA256=b342ab57b059957a2b3035d3d842de5d86a7c271c14a8b30bdcd22054badc76c
BuildID=ed8866f15efd43714c3ea324438fb2a2dd582357
```

Candidate Rust implementation located at:

```text
/mnt/data/OKC/redumb_sprint_20260528_191328/redumb_github
git_head=75c89feb7090e0debc4413b0b242e93d9b35c955
```

The candidate implementation contains the measured CLI and Rayon parallel
u16-auto / merge / gmap functionality.

The current distributed ELF and older preview ELFs share BuildID lineage but
do not share identical SHA256 hashes.

Therefore this release does **not** yet claim byte-reproducible
source-to-binary provenance.

A later conformance step should rebuild the candidate Rust source in an
isolated target directory and compare CLI behavior, exact restoration,
artifact structure, and benchmark receipts against the distributed ELF.

## Recovered source conformance — 2026-09-02

The recovered Rust source was rebuilt from the worktree identified as:

```text
75c89feb7090e0debc4413b0b242e93d9b35c955
v0.4-preview-local-8-g75c89fe-dirty
crate version: redumb 0.3.0
```

The fresh executable is not byte-identical to the distributed reference ELF.

However, the tested implementation path produced:

```text
CLI help equality             PASS
reference exact restore       PASS
rebuilt exact restore         PASS
merged dictionary equality    PASS
fixture artifact equality     PASS
```

The current statement is therefore **behavioral/artifact conformance**, not
bit-reproducible binary provenance.

See `receipts/source-conformance/`.

## Interpretation of the recovered-source test

The reference executable and fresh rebuild are not byte-identical executables.

However, on the tested path both implementations:

```text
expose identical CLI help
restore the fixture exactly
produce identical merged dictionary
produce identical tested DU/gmap artifacts
```

This strongly supports behavioral equivalence of the recovered implementation
for the tested functionality.

It does not prove that the historical reference ELF was compiled with the same
compiler, linker, flags, dependency build, or packaging environment.

The recovered worktree was marked dirty because it contains staged and
untracked historical research material.

The exact filesystem snapshot used for conformance has therefore been
preserved separately with a complete file-hash manifest.

The source snapshot, rather than a Git patch alone, is the authoritative
preservation object because untracked files are not represented by
`git diff HEAD`.
