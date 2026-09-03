# Nereid plugin architecture

Nereid bridges DUMBer canonical DU streams into existing tokenizer and model-ID
spaces.

It is not BPE over DU IDs.

## Core plugin shape

```text
TargetTokenizerPlugin:
  name
  vocab_fingerprint
  token_bytes(id) -> bytes
  encode_bytes(bytes) -> list[target_id]
  decode_ids(list[target_id]) -> bytes
  can_feed_model_ids -> bool
```

## Public proof lane

```text
public proof:
  one reproducible adapter
  open receipt
  clear equivalence criteria
```

Good public targets:

```text
cl100k_base / o200k_base
GPT-2 byte-level BPE
SentencePiece local model
```

## Private / paid lane

```text
paid/private:
  optimized Rust emitters
  model-specific official-ID plugins
  validation receipts
  local inference integration packs
  corpus-specific caches
```

Target families:

```text
Qwen
DeepSeek
Llama
GLM
Mistral
Gemma
Claude only if tokenizer/ID path is accessible
```

## Adapter levels

```text
v0.1337:
  decoded DU lexeme -> target IDs
  text-equivalent

v1:
  span ledger
  byte-span alignment between DU and target tokens

v2:
  context/window adapter
  direct tokenizer-path equivalence

v3:
  compiled plugin pack
  optimized model-specific bridge
```

## Boundary doctrine

```text
source text identity != tokenizer ontology
valid decoded text    != same model computation
```

Nereid makes that boundary measurable.
