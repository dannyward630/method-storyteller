//! `mst`: deprecated alias of `method-storyteller`; runs the same code and notes the rename on stderr.

use std::process::ExitCode;

fn main() -> ExitCode {
    storyteller_cli::main(storyteller_cli::Invocation::Alias)
}
