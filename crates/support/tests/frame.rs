//! Frame tests
use ceres_std::BTreeMap;
use ceres_support::traits::{Frame, Storage};

#[derive(Default)]
struct Test(BTreeMap<Vec<u8>, Vec<u8>>);

impl Storage for Test {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.0.insert(key, value)
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.0.get(key).map(|v| v.as_ref())
    }
}

impl Frame for Test {}

#[test]
fn active_is_none_when_init() {
    let t = Test::default();
    assert_eq!(t.active(), None);
}

#[test]
fn push_works() {
    let mut t = Test::default();
    assert_eq!(t.active(), None);
    assert_eq!(t.push([1; 32]), None);
    assert_eq!(t.active().map(|v| v.to_vec()), Some(vec![1; 32]));
}

#[test]
fn pop_works() {
    let mut t = Test::default();
    assert_eq!(t.active(), None);
    assert_eq!(t.push([1; 32]), None);
    assert_eq!(t.active().map(|v| v.to_vec()), Some(vec![1; 32]));
    assert_eq!(t.pop().map(|v| v.to_vec()), Some(vec![1; 32]));
    assert_eq!(t.active(), None);
}
