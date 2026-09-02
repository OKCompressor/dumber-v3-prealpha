# R1 singleton plan

mode=words
threshold=1

global_du_vocab=2229308
frequent_types=848962
singleton_types=1380346
sentinel_id=848962
total_tokens=402990603

singleton_type_fraction=0.61918138
singleton_token_fraction=0.00342526

literal_singleton_bytes_raw=12002741
literal_singleton_bytes_u16_length_prefixed=14763433

mapping_bytes_u32=8917232

Important:
This creates the DU-global -> R1 projection and pruned R1 vocabulary.
It does not yet materialize the ordered singleton sideband.
The sideband is emitted during the later fused stream pass.
