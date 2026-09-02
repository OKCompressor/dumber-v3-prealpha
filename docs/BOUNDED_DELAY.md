# Bounded-delay structural tokenization

Compression ratio is not the only useful axis for a deployable tokenizer.

Latency and memory can be treated as first-class representation properties.

DUMBer's u16-auto mode already bounds local vocabulary state:

```text
local vocabulary <= 65,536 symbols
```

When that space is exhausted, the representation rolls into a new local
chunk.

This establishes a vocabulary-state bound, but does not alone prove bounded
output delay.

For the lexical transducer, the remaining question is whether there exists a
corpus-independent constant `L` such that output associated with input
position `i` becomes irrevocable after reading at most `L` additional input.

If the scanner has a bounded unresolved suffix/lookahead, the structural
tokenizer approaches:

```text
vocabulary state = O(65,536)
unresolved lexical buffer = O(L)
streaming memory independent of corpus length
```

This provides a direct research bridge to bounded-delay streaming BPE /
tokenization work.

The bound should be derived from scanner behavior before being claimed as a
property of the implementation.
