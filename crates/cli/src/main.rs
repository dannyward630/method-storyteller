//! `method-storyteller`: the Method Storyteller command line.

use std::process::ExitCode;

fn main() -> ExitCode {
    storyteller_cli::main(storyteller_cli::Invocation::Primary)
}
