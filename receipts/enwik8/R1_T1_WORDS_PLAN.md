# R1 singleton plan

mode=words
threshold=1

global_du_vocab=426714
frequent_types=192018
singleton_types=234696
sentinel_id=192018
total_tokens=39193680

singleton_type_fraction=0.55000773
singleton_token_fraction=0.00598811

literal_singleton_bytes_raw=1972346
literal_singleton_bytes_u16_length_prefixed=2441738

mapping_bytes_u32=1706856

Important:
This creates the DU-global -> R1 projection and pruned R1 vocabulary.
It does not yet materialize the ordered singleton sideband.
The sideband is emitted during the later fused stream pass.
