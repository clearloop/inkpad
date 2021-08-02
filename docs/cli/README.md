# CLI

`ceres` has an CLI implementation to help you debug contracts in command line.


```
 𝝺 ceres
ceres 0.2.0
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

## Arguments

Once we use `*.contract` as the argument of `ceres`, ceres will load and record
the target contract in database(~/.ceres) by default.
