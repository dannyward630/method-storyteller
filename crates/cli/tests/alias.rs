#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

//! The `mst` alias runs the same code as `method-storyteller` and only adds a note on stderr.

use std::path::PathBuf;
use std::process::{Command, Output};

const PRIMARY: &str = env!("CARGO_BIN_EXE_method-storyteller");
const ALIAS: &str = env!("CARGO_BIN_EXE_mst");
const NOTE: &str = "note: `mst` is now `method-storyteller`; the old name will be removed in a future release.\n";

fn fixture() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../core/tests/fixtures/lighthouse.fountain").display().to_string()
}

fn run(bin: &str, args: &[&str]) -> Output {
    Command::new(bin).args(args).output().expect("binary runs")
}

fn assert_same(args: &[&str]) {
    let primary = run(PRIMARY, args);
    let alias = run(ALIAS, args);
    assert_eq!(primary.stdout, alias.stdout, "stdout differs for {args:?}");
    assert_eq!(primary.status.code(), alias.status.code(), "exit status differs for {args:?}");
    let primary_err = String::from_utf8(primary.stderr).expect("utf-8 stderr");
    let alias_err = String::from_utf8(alias.stderr).expect("utf-8 stderr");
    assert!(!primary_err.contains("note:"), "method-storyteller must not print the alias note: {primary_err}");
    assert_eq!(alias_err, format!("{NOTE}{primary_err}"), "alias stderr is the note followed by the normal stderr");
}

#[test]
fn alias_matches_primary_on_stdout_and_notes_rename_on_stderr() {
    let fixture = fixture();
    assert_same(&[]);
    assert_same(&["--help"]);
    assert_same(&["--version"]);
    assert_same(&["commands"]);
    assert_same(&["info", &fixture]);
    assert_same(&["paginate", &fixture, "--text"]);
    assert_same(&["no-such-command"]);
}

#[test]
fn version_names_the_command_and_the_product() {
    let output = run(PRIMARY, &["--version"]);
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert_eq!(stdout, format!("method-storyteller {} (Method Storyteller)\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_names_method_storyteller() {
    let output = run(PRIMARY, &["--help"]);
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).expect("utf-8 help");
    assert!(help.contains("method-storyteller convert"), "{help}");
    assert!(!help.contains("mst "), "{help}");
}

#[test]
fn errors_are_prefixed_with_method_storyteller() {
    let output = run(PRIMARY, &["no-such-command"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf-8 stderr");
    assert!(stderr.starts_with("method-storyteller: unknown command `no-such-command`"), "{stderr}");
}
