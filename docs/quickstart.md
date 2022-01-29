# Quickstart

## 0. Install `cargo-contract`

```
cargo install cargo-contract
```


## 1. Install `inkpad`

```
cargo install inkpad
```


## 2. Run `cargo contract new`

```
cargo contract new flipper
```


### 3. Compile with debug info

```
cargo contract build
```

### 4. Test flipper

```
inkpad target/ink/flipper.contract info
```
