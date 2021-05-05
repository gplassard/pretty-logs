# Pretty-logs

Simple CLI to drop input until '{' is encountered (designed to be used in conjuction with https://github.com/TylerBrock/saw 
and https://github.com/stedolan/jq) for log produced by an AWS Lambda with NodeJS runtime. 

```
saw watch /aws/lambda/loggroup | pretty-logs | jq
```

## Running sources

* `echo "aaaaa" | cargo run`
* `echo "      {}" | cargo run`

## Installation

`cargo install --path .`
