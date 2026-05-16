use std::path::PathBuf;

use clap::{ArgAction, Parser};

mod job;
mod template;

/// Test harness that runs tasks according to a job and collects their results. Can be used standalone or with Beaker.
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Run job from file
    #[arg(short = 'j', long = "job", value_name = "FILE")]
    job: Option<PathBuf>,

    /// Resume interrupted job from DIR
    #[arg(short = 'r', long = "run", value_name = "DIR")]
    run: Option<PathBuf>,

    /// Skip XML validation
    #[arg(short = 'n', long = "novalidate")]
    novalidate: bool,

    /// Override host for a recipe id
    #[arg(short = 't', long = "host", value_name = "<recipe_id>=[<user>@]<host>")]
    host: Vec<String>,

    /// Number of reconnection retries
    #[arg(short = 'c', long = "conn-retries", default_value_t = 15)]
    conn_retries: u32,

    /// Port for restraintd HTTP server
    #[arg(short = 'p', long = "port", value_name = "PORT")]
    port: Option<u16>,

    /// Increase verbosity (up to three times)
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count)]
    verbose: u8,

    /// Remote shell command [default: ssh -o ServerAliveInterval=60 -o ServerAliveCountMax=<--timeout>]
    #[arg(short = 'e', long = "rsh")]
    rsh: Option<String>,

    /// Path to restraintd on the remote machine
    #[arg(long = "restraint-path", default_value = "restraintd")]
    restraint_path: String,

    /// Connection timeout in minutes (used when `--rsh` is not set)
    #[arg(long = "timeout", default_value_t = 5)]
    timeout: u32,
}

impl Args {
    fn parse_with_defaults() -> Self {
        let mut args = Self::parse();
        if args.rsh.is_none() {
            args.rsh = Some(format!(
                "ssh -o ServerAliveInterval=60 -o ServerAliveCountMax={}",
                args.timeout
            ));
        }
        args
    }
}

fn main() {
    Args::parse_with_defaults();
}
