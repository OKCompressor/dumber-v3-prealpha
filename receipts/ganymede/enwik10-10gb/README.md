# DUMBer Ganymede — 10 GB exact receipt

Dataset:

```text
path=enwik10-20251101-prefix
bytes=10000000000
sha256=4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

This is a 10,000,000,000-byte uncompressed prefix extracted from the
2025-11-01 English Wikipedia pages/articles multistream XML dump.

## Canonical vocabulary

```text
global DU types     17,294,055
u24 capacity        16,777,216
overflow               516,839
overflow fraction        3.08%
```

This dataset therefore requires a canonical ID space wider than u24.

Local representation streams remain u16.

## Ganymede mapping

```text
representation chunks   1828
gmap32 files             1828
gmap32 bytes             477768072
merged dictionary bytes  214565572
```

A full local vocabulary contains at most 65,536 entries.

For a full chunk:

```text
65,536 local IDs × 4-byte global IDs
= 262,144-byte gmap32
```

The token payload itself remains local-u16.

## Timing

```text
build gmap32 wall        47.27 s
build gmap32 peak RSS    2,049,308 KiB

exact structural restore 6:09.62
restore peak RSS         1,178,196 KiB
```

The restore timing excludes the later SHA256 and direct byte-comparison
validation passes.

The test volume was a mechanical external drive previously measured at
approximately 95.5 MB/s sequential read throughput.

## Exactness

```text
restore_gmap32_rc=0
restore_gmap32_exact=PASS

input SHA256:
4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c

restored SHA256:
4a11a5ab0740c29fc1097aeafe9691e021238be5a1c1a93ebb8a4e89d85c7b6c
```

The restored 10 GB temporary file was removed after validation.

## Development binary

```text
codename=Ganymede
inherits=Europa bounded-input work

SHA256:
85aad7063a4d5361ff0f35d47b835de56c6ae0d988f2c6fb031a5c94b118ca9b

BuildID:
066152f5b827532c756047fc510b127e4d6453a7
```
