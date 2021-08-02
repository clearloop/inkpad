# ceres list

Lists loaded contracts

```
 𝝺 ceres list -h
ceres-list 0.2.0
Lists all contracts

USAGE:
    ceres list

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

for example

```
 𝝺 ceres list

	contract             code-hash
	---------------------------------------------------------------------------------------
	delegator            0x85970b066ab92b4495bba682917ded604019a6fce9b247c36b35e8c967287f4f
	accumulator          0x906811f0dca85909fd267df762c09de7a6af62cd1f1aa73d3778d58d308fd376
	adder                0xc60c90582f3e6767d9b8cca5b83f747b8b86afb8c425b2e23dfad5e4784409e1
	subber               0xcc9b22fda1a527754190c8833b919a26cf440bd9aca7f0e5810104f09a8e0b82
```

The code-hash displayed could be used in `call-contracts`
