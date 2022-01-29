# inkpad call

The `inkpad call` command call a method of the supplied contract.

```
 𝝺 inkpad call -h
inkpad-call
Calls a call method

USAGE:
    inkpad call [OPTIONS] <method>

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

The options of method `call` are destructed from `Transaction`


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
