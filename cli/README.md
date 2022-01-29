# Inkpad

![Rust](https://github.com/patractlabs/inkpad/workflows/Inkpad/badge.svg)
[![crate](https://img.shields.io/crates/v/inkpad-cli.svg)](https://crates.io/crates/inkpad-cli)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/inkpad-cli/)
[![downloads](https://img.shields.io/crates/d/inkpad-cli.svg)](https://crates.io/crates/inkpad-cli)
[![LICENSE](https://img.shields.io/crates/l/inkpad-cli.svg)](https://choosealicense.com/licenses/apache-2.0/)

Run ink! contract anywhere.


## Installation

This Inkpad CLI can be installed on Linux and macOS with a small install script:

```
$ cargo install --git https://github.com/patractlabs/inkpad --bin inkpad
```

## Usage

```text
inkpad 0.2.0
Inkpad command tool

USAGE:
    inkpad [*.contract | name | code-hash] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <*.contract | name | code-hash>    If empty, inkpad will load the last contract which has been executed

SUBCOMMANDS:
    call      Calls a call method
    deploy    Calls a deploy method
    help      Prints this message or the help of the given subcommand(s)
    info      Prints info of *.contract
    list      Lists all contracts
```

## Example

```text
$ inkpad flipper.contract deploy default
Deploy contract succeed!

$ inkpad list

	contract             code-hash
	---------------------------------------------------------------------------------------
	flipper              0x97994513522c4cdf681c3377ef043ccefd865df0bd3ce86c599aab7b23de211f

$ inkpad info

	name: flipper
	code_hash: 0x97994513522c4cdf681c3377ef043ccefd865df0bd3ce86c599aab7b23de211f
	contructors:
		 - default [  ]
		 - new [ bool ]
	methods:
		 - flip [  ]
		 - get [  ]

$ inkpad call get
result: [0]

$ inkpad call flip
result: []

$ inkpad call get 
result: [1] 
```


## Features

Inkpad is an independent ink! contract environment, the [runtime][rt] of Inkpad
supports both `wasmtime` and `wasmi`, with the `wasmi` feature of Inkpad, you
can run your ink! contract anywhere, even in the browser.



## LICENSE

Apache-2.0

[rt]: crates/runtime
