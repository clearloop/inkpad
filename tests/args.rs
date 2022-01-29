use inkpad_ri::Instance;
use inkpad_runtime::Runtime;
use inkpad_support::types::Cache;
use parity_scale_codec::Encode;

fn t(f: fn(rt: &mut Runtime)) {
    let mut args = Runtime::from_contract(
        include_bytes!("../contracts/args.contract"),
        Cache::default(),
        Some(Instance),
    )
    .unwrap();

    // deploy
    args.deploy("default", vec![], None).unwrap();

    // run test
    f(&mut args);
}

#[test]
fn test_boolean() {
    t(|args: &mut Runtime| {
        assert_eq!(
            args.call("test_boolean", vec![true.encode()], None),
            Ok(Some(vec![1]))
        );
    })
}

#[test]
fn test_number() {
    t(|args: &mut Runtime| {
        assert_eq!(
            args.call("test_number", vec![0.encode()], None),
            Ok(Some(vec![0, 0, 0, 0]))
        );
    })
}

#[test]
fn test_hash() {
    t(|args: &mut Runtime| {
        let hash = [0; 32];
        assert_eq!(
            args.call("test_hash", vec![hash.to_vec()], None),
            Ok(Some(vec![0; 32]))
        );
    })
}

#[test]
fn test_boolean_and_number() {
    t(|args: &mut Runtime| {
        assert_eq!(
            args.call(
                "test_boolean_and_number",
                vec![true.encode(), 1.encode()],
                None
            ),
            Ok(Some(vec![1, 1, 0, 0, 0]))
        );
    })
}

#[test]
fn test_boolean_and_hash() {
    t(|args: &mut Runtime| {
        let hash = [0; 32];
        let mut res = true.encode();
        res.append(&mut hash.to_vec());
        assert_eq!(
            args.call(
                "test_boolean_and_hash",
                vec![true.encode(), hash.to_vec()],
                None
            ),
            Ok(Some(res))
        );
    })
}

#[test]
fn test_number_and_number() {
    t(|args: &mut Runtime| {
        assert_eq!(
            args.call("test_number_and_number", vec![0.encode(), 1.encode()], None),
            Ok(Some(vec![0, 0, 0, 0, 1, 0, 0, 0]))
        );
    })
}

#[test]
fn test_number_and_hash() {
    t(|args: &mut Runtime| {
        let hash = [0; 32];
        let mut res = 0.encode();
        res.append(&mut hash.to_vec());
        assert_eq!(
            args.call(
                "test_number_and_hash",
                vec![0.encode(), hash.to_vec()],
                None
            ),
            Ok(Some(res)),
        );
    })
}

#[test]
fn test_all() {
    t(|args: &mut Runtime| {
        let hash = [0; 32];
        let mut res = 0.encode();
        res.append(&mut hash.to_vec());
        res.append(&mut true.encode());
        assert_eq!(
            args.call(
                "test_all",
                vec![0.encode(), hash.to_vec(), true.encode()],
                None
            ),
            Ok(Some(res)),
        );
    })
}

#[test]
fn test_number_and_hash_with_numbers() {
    t(|args: &mut Runtime| {
        assert_eq!(
            args.call("test_number_and_hash", vec![0.encode(), 1.encode()], None),
            Ok(None)
        );
    })
}
