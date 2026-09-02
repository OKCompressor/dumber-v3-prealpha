# DUMBer moon codenames

DUMBer development lanes use moon codenames.

The astronomical parent body is part of the alias map but has no effect on
format compatibility or reproducibility.

| moon | planet | technical lane | state |
|---|---|---|---|
| **Io** | Jupiter | hot-path / speed / per-worker RSS | queued |
| **Europa** | Jupiter | bounded streaming input | proof passed |
| **Ganymede** | Jupiter | large global ID space / u32 | active |
| **Callisto** | Jupiter | compatibility / interchange | reserved |
| **Mimas** | Saturn | minimal/core profile | reserved |
| **Enceladus** | Saturn | streaming model epochs | queued |
| **Dione** | Saturn | graph / metadata integration | reserved |
| **Rhea** | Saturn | reducer-tree architecture | queued |
| **Titan** | Saturn | giant-corpus / full-Wikipedia profile | queued |
| **Triton** | Neptune | reverse/transduction / DUMB2(b)PE | queued |

## Current trajectory

```text
Europa
bounded file windows
        |
        v
Ganymede
auto u24/u32 canonical identity space
        |
        v
Io
per-worker hot-path and RSS reduction
        |
        v
Titan
giant-corpus / full-Wikipedia runs
```

Independent research lane:

```text
Triton
DUMB2(b)PE
DU structural IDs
    ->
existing model token spaces
```

Streaming model evolution:

```text
Enceladus
M0 --Delta1--> M1 --Delta2--> M2
```

Codenames are human aliases only.

Receipts, binaries and public claims remain keyed by explicit versions,
Git commits, SHA256 values and dataset hashes.
