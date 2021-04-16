# Ceres

![Rust](https://github.com/patractlabs/ceres/workflows/Ceres/badge.svg)

Run ink! contract anywhere.


## Installation

This Ceres CLI can be installed on Linux and macOS with a small install script:

```
$ cargo install --git https://github.com/patractlabs/ceres --bin ceres
```

## Usage

```
ceres 0.1.0
Ceres command tool

USAGE:
    ceres [*.contract | name | code-hash] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <*.contract | name | code-hash>    If empty, ceres will load the last contract which has been executed

SUBCOMMANDS:
    call      Calls a call method
    deploy    Calls a deploy method
    help      Prints this message or the help of the given subcommand(s)
    info      Prints info of *.contract
    list      Lists all contracts
```

## Example

```shell
$ ceres flipper.contract deploy default
Deploy contract succeed!

$ ceres list

	contract             code-hash
	---------------------------------------------------------------------------------------
	flipper              0x97994513522c4cdf681c3377ef043ccefd865df0bd3ce86c599aab7b23de211f

$ ceres info

	name: flipper
	code_hash: 0x97994513522c4cdf681c3377ef043ccefd865df0bd3ce86c599aab7b23de211f
	contructors:
		 - default [  ]
		 - new [ bool ]
	methods:
		 - flip [  ]
		 - get [  ]

$ ceres call get
result: [0]

$ ceres call flip
result: []

$ ceres call get 
result: [1] 
```


## Features

Ceres is an independent ink! contract environment, the [runtime][rt] of Ceres
supports both `wasmtime` and `wasmi`, with the `wasmi` feature of Ceres, you
can run your ink! contract anywhere, even in the browser.



## LICENSE

MIT

[rt]: crates/runtime
