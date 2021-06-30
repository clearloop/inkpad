//! Memory Cache
use crate::traits;
use ceres_std::{BTreeMap, Vec};
use core::iter::Iterator;

/// Memory cache implementation
pub struct Cache<Memory> {
    map: BTreeMap<Vec<u8>, Vec<u8>>,
    memory: Memory,
}

impl<Memory> Iterator for Cache<Memory> {
    type Item = (Vec<u8>, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.map.iter().next().map(|(k, v)| (k.clone(), v.clone()))
    }
}

impl<'i, Memory> Iterator for &'i Cache<Memory> {
    type Item = (&'i Vec<u8>, &'i Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.map.iter().next()
    }
}

impl<Memory> traits::Storage for Cache<Memory> {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.map.insert(key, value)
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.map.get(key).map(|v| v.as_ref())
    }
}

impl<Memory> traits::Frame for Cache<Memory> {
    // const PREFIX: [u8; 4] = [0; 4];
    fn frame_prefix(&self) -> &[u8] {
        &[0, 0, 0, 0]
    }
}

impl<Memory> traits::State<Memory> for Cache<Memory> {
    fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }
}

impl<Memory> traits::Cache<Memory> for Cache<Memory> {}
