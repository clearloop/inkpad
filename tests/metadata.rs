use inkpad_support::types::Metadata;

#[test]
fn test_decoding_flipper() {
    assert!(
        serde_json::from_str::<Metadata>(&String::from_utf8_lossy(include_bytes!(
            "../contracts/flipper.contract"
        )))
        .is_ok()
    );
}
