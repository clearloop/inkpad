## Ceres

![Rust](https://github.com/patractlabs/ceres/workflows/Ceres/badge.svg)

Run ink! contract anywhere.


## Design

``` text

- - - - - - - - -      - - - - -      - - - - - - -
| ink! contract | ---> | ceres | ---> |  anywhere |
- - - - - - - - -      - - - - -      - - - - - - -

```

## Example

```rust
use ceres_runtime::Runtime;

#[test]
fn test_flipper() {
    let mut rt = Runtime::from_contract(include_bytes!("../flipper.contract"))
        .expect("Create runtime failed");

    rt.deploy("default", &[]).expect("Deploy failed");
    assert_eq!(&rt.call("get", &[]).expect("Call contract failed"), &[0]);

    rt.deploy("new", &["true"]).expect("Deploy failed");
    assert_eq!(&rt.call("get", &[]).expect("Call contract failed"), &[1]);

    rt.call("flip", &[]).expect("Call contract failed");
    assert_eq!(&rt.call("get", &[]).expect("Call contract failed"), &[0]);
}
```

## LICENSE

MIT
