# Pretty-logs

Simple CLI to drop input until '{' is encountered (designed to be used in conjunction with https://github.com/TylerBrock/saw 
and https://github.com/stedolan/jq) for logs produced by an AWS Lambda with NodeJS runtime. 

```
saw watch /aws/lambda/loggroup | pretty-logs | jq
```

## Running sources

* `echo "aaaaa" | cargo run`
* `echo "      {}" | cargo run`

## Installation

`cargo install --path .`

## Release

* Update `Cargo.toml` / `Cargo.lock`
* `git commit -m 'Release(v1.2.0)'`
* `git tag v1.2.0`
* `git push && git push --tags`
