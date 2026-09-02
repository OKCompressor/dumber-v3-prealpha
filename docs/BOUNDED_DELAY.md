# Bounded-delay tokenizer note

A useful research axis for structural token streams is not only bits per token
but latency and memory as input grows.

Redumb's u16-auto representation already imposes a bound on local vocabulary
state:

```text
local vocabulary <= 65,536 symbols
```

When that space is exhausted, representation rolls into a new local chunk.

This establishes a vocabulary-state bound.

It does not by itself prove a bounded output delay.

For a lexical transducer, bounded delay asks whether there exists a
corpus-independent constant `L` such that, after consuming input position `i`,
the tokenizer needs at most `L` additional input before the output associated
with that region becomes irrevocable.

The next source-level question is therefore the scanner's maximum unresolved
suffix / lookahead.

If this can be bounded independently of input length, the resulting design has
the form:

```text
vocabulary state = O(65,536)
unresolved lexical buffer = O(L)
total streaming state independent of corpus length
```

This connects vocabulary-bounded structural tokenization with streaming BPE /
finite-delay transduction work, while remaining a separate question from
compression ratio.

This note should be treated as a research direction until the scanner delay is
formally derived from the implementation.
