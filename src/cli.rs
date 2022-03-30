use structopt::StructOpt;

#[derive(StructOpt)]
/// Drop input until a '{' character is encountered.
/// Designed to be used in conjunction with https://github.com/TylerBrock/saw and https://github.com/stedolan/jq) for log produced by an AWS Lambda with NodeJS runtime.
///
/// Example usage :
/// saw watch /aws/lambda/loggroup | pretty-logs | jq
#[structopt(name = "pretty-logs")]
pub struct Cli {}
