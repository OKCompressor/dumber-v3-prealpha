# DU merger / R1 / zRank architecture

## Spaces and maps

### Local DU space

Each representation chunk owns a local u16 vocabulary.

### Global DU space

The DU merger creates one canonical vocabulary.

Notation:

```text
φ_i(l) = g
```

Read:

```text
phi for chunk i maps local ID l to global DU ID g
```

No magic.

### R1 space

R1 is a projection of global DU space.

```text
ρ(g) = r
```

Read:

```text
rho maps global DU ID g to R1 ID r
```

For the t1/words candidate:

```text
global count == 1
    → RARE sentinel + lexical sideband

global count > 1
    → dense frequent R1 ID
```

The canonical DU vocabulary remains a reusable structural layer. R1 does not
replace its identity; it defines another address space above it.

## Current reducer

`du_stats_reduce.rs` reads:

```text
local u16 streams
+
packed gmap24
```

and performs:

```text
local u16
→ hydrated u32 global lookup
→ global unigram count
```

It also records first and last global IDs per chunk.

It does **not yet compute contextual/bigram counts**.

Therefore it is already sufficient for singleton pruning but not yet
sufficient by itself to build the complete zRank-v7 contextual model.

## Target execution

```text
DU workers                    ███████████████████
dictionary reducers             ████████████████
unigram/context reducers          ██████████████
R1 singleton planning               ███████████
zRank model preparation               █████████
                                         │
                                   model freeze
                                         █
parallel fused emission                 ███
```

Representation chunks, reducer nodes, scheduling shards, and zRank payload
chunks are deliberately separate concepts.

With 16 cores the scheduler may maintain 16 or 32 ready work units.

With 200 cores it may maintain hundreds.

The structural format does not change.

## Shared-model zRank target

The first target is:

```text
GLOBAL contextual model
+
chunk-local u16 source/fallback
```

A token can remain local-u16 in storage.

At execution:

```text
local u16
→ φ
→ global DU
→ ρ
→ global R1
→ contextual zRank lookup
```

A rank hit uses the shared global model.

A fallback may preserve the local-u16 identifier and resolve it through the
chunk map during decode.

This preserves a two-byte local lane without requiring a two-byte global
vocabulary.

## Boundary cost

Once the model is shared, dividing the rank payload into independent chunks
does not require duplicating the model.

The structural loss is approximately tied to the number of boundaries:

```text
chunk seeds
+ chunk index metadata
+ optionally one lost/reset context per boundary
```

rather than the number of corpus tokens.

If exact cross-boundary context matters, a chunk may carry the preceding R1
context as a seed.

For tens or hundreds of millions of tokens, dozens of such seeds are
negligible compared with the corpus payload.

Entropy-coder chunk boundaries are a separate measurement.

## Batch-global

```text
workers
→ continuously reduced global statistics
→ freeze one global model
→ parallel final rank emission
```

Goal: strongest archival representation.

## Stream-epoch research branch

A future streaming representation can freeze model epochs:

```text
M0
M1 = update(M0, Δ1)
M2 = update(M1, Δ2)
...
```

A chunk declares the model epoch under which it was encoded.

`Δ` means a serialized state/model update. It should not be interpreted as
ordinary arithmetic addition unless a future model representation explicitly
makes that operation additive.

The important property is:

```text
no previous rank payload needs rewriting
```

while later chunks can benefit from progressively richer global statistics.

## Plugin capability sketch

Future structural modules may expose properties such as:

```text
stream_effect = observer | remap | expand | reorder
stats_transform = exact | requires_materialization
```

R1-style one-to-one remapping can transform sufficient statistics exactly.

An expansion/reordering transform may require its resulting stream to be
materialized or separately summarized.

This is scheduler metadata, not a restriction on future transforms.

## What `du_stats_reduce.rs` actually does

The reducer does not create a global-width copy of the corpus.

For each representation chunk it reads:

```text
local u16 token stream
+
local-to-global packed gmap24
```

The packed map is hydrated into native u32 values in memory.

For every local token ID:

```text
local_id
    |
    v
gmap[local_id]
    |
    v
global_DU_id
    |
    +--> global_count[global_DU_id] += 1
```

It also records the first and last global IDs of each chunk.

The resulting state is sufficient for the current Rare1 singleton decision:

```text
count == 1  -> singleton
count > 1   -> frequent
```

No `global_u24` corpus stream is written.

### Current boundary

The reducer currently computes unigram counts.

It does not yet compute zRank-v7 contextual adjacent-pair counts.

The next contextual reducer extends the same scan:

```text
previous_global_R1_id
+
current_global_R1_id
    ->
context_pair_count++
```

Chunk boundaries require only the recorded last ID of chunk N and first ID of
chunk N+1.

Therefore contextual statistics can also be reduced without materializing a
global-width token stream.

## Current parallel front-end memory boundary

The recovered v0.3 parallel implementation currently begins with:

```rust
let input = fs::read_to_string(input_file)?;
```

It then builds macrochunk byte ranges over that in-memory `String` and feeds
those ranges to the Rayon worker pool.

Consequently, the current implementation has two different kinds of bounds:

```text
local vocabulary space:
    bounded to u16

whole-process input working set:
    not yet bounded independently of corpus size
```

Measured enwik9 behavior:

```text
input size               1,000,000,000 bytes
parallel DU peak RSS     ~2.87 GiB
```

By comparison:

```text
DU stats peak RSS        ~43 MiB
R1 plan peak RSS         ~209 MiB
fused R1 scan peak RSS   ~95 MiB
```

The principal memory-optimization target is therefore the current parallel
front end.

### Planned bounded-working-set path

Replace whole-file `read_to_string` with bounded file/mmap ranges:

```text
file
  |
  +--> scheduler range A --> worker --> DU chunks --> release input window
  +--> scheduler range B --> worker --> DU chunks --> release input window
  +--> scheduler range C --> worker --> DU chunks --> release input window
```

Representation chunk boundaries remain vocabulary-driven and independent of
scheduler input windows.

This is also the preferred architecture for large datasets and slow external
storage because it avoids retaining the complete source corpus in memory.
