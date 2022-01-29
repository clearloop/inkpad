# inkpad deploy

The `inkpad deploy` command deploy the supplied contract with constructor, like `inkpad call`

```
 𝝺 inkpad deploy -h
inkpad-deploy
Calls a deploy method

USAGE:
    inkpad deploy [OPTIONS] <method>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --address <address>                        Contract callee
    -b, --balance <balance>                        contract balance
        --caller <caller>                          Contract caller
    -m, --minimum-balance <minimum-balance>        minimum balance
    -n, --now <now>                                current time
    -a, --args <string,>...                        Arguments
    -v, --value-transferred <value-transferred>    transferred value

ARGS:
    <method>    Calling method
```

The options of method `deploy` are destructed from `Transaction`


### address

Callee address



### balance

Contract balance



### caller

Caller address



### minimum-balance

`minimum_balance` in transaction



### now

Transaction time



### args

Transaction arguments, should be `parity-scale-codec` format



### value

Transferred value
