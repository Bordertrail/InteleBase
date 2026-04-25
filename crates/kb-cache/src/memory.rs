//! Memory cache stub - Moka (Phase 2)

pub struct MemoryCache;

impl MemoryCache {
    pub fn new() -> Self {
        Self
    }

    pub fn get<K, V>(&self, _key: &K) -> Option<V> {
        None
    }

    pub fn insert<K, V>(&self, _key: K, _value: V) {}
}
