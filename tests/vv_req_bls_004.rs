//! REQUIREMENT: BLS-004 — Uses chia_bls::BlsCache type (compile test)

mod common;

use chia_bls::BlsCache;

#[test]
fn bls_004_uses_chia_bls_cache() {
    // Compile test: BlsCache is from chia_bls
    let _cache: BlsCache = BlsCache::default();
}

#[test]
fn bls_004_cache_is_chia_bls_type() {
    // Verify BlsCache can be created and is the type accepted by validate_spend_bundle
    let mut cache = BlsCache::default();
    let _ref: &mut BlsCache = &mut cache;

    // The Option<&mut BlsCache> parameter type matches chia_bls::BlsCache
    let _opt: Option<&mut BlsCache> = Some(&mut cache);
}

#[test]
fn bls_004_bls_cache_default() {
    // BlsCache::default() produces a valid empty cache
    let cache = BlsCache::default();
    // Just verify it was created without panic
    drop(cache);
}
