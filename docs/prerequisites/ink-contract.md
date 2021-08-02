# ink! contract

Since `ceres` is an execution of ink! contract, we need to have an ink! contract
first of all.


### 0. Download `cargo-contract`

For generating an ink! contract, we need to download `cargo-contract`

```
# Download the official `cargo-contract`
cargo install cargo-contract --force

# Donwload patract version `cargo-contract` witch supports debug info
cargo install https://github.com/patractlabs/cargo-contract
```


### 1. New ink! contract

Once we have `cargo-contract` installed in our machine, we can run 

```
cargo new my-ink-contract
```

to generate a template ink contract.


### 2. Compile ink! contract

```
cd my-ink-contract
cargo contract build
```
