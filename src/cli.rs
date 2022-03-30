use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
/// Ignore characters from input in order to transform each line into (hopefully) valid json.
/// Designed to be used in conjunction with https://github.com/TylerBrock/saw and
/// https://github.com/stedolan/jq for logs produced by an AWS Lambda with NodeJS runtime.
///
/// Example usage :
/// saw watch /aws/lambda/loggroup | pretty-logs | jq
#[structopt(name = "pretty-logs", setting = clap::AppSettings::DeriveDisplayOrder)]
#[structopt(set_term_width = 90)]
pub struct Cli {
    #[structopt(long)]
    /// don't remove characters at the start of the line
    pub preserve_start: bool,

    #[structopt(long)]
    /// don't remove characters at the end of the line
    pub preserve_end: bool,
}
