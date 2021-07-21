# Quickstart

## 0. Install `cargo-contract`

```
cargo install --git https://github.com/patractlabs/cargo-contract.git --branch tag-v0.12.1 --force
```

Recommend to use `patractlabs/cargo-contract` which could compile ink! contracts with debug info.

## 1. Install `ceres`

```
cargo install ceres
```

## 2. Run `cargo contract new`

```
cargo contract new flipper
```

Create a template flipper contract


### 3. Compile with debug info

```
cargo contract build --debug
```

Assume we are using `patractlabs/ceres`, compile `flipper` with debug info


### 4. Test flipper

```
ceres target/ink/flipper.contract info
```
