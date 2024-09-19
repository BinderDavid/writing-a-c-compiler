use std::path::PathBuf;

use clap::Parser;
use tacky_lib::{assemble_and_link, compile, preprocess};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Run the lexer, but stop before parsing.
    #[arg(long)]
    lex: bool,
    /// Run the parser, but stop before assembly generation.
    #[arg(long)]
    parse: bool,
    /// Generate assembly, but stop before code generation.
    #[arg(long)]
    codegen: bool,
    filepath: PathBuf,
}
fn main() {
    let cli = Cli::parse();
    exec(&cli)
}

fn exec(cli: &Cli) {
    let mut example: PathBuf = cli.filepath.clone();

    preprocess(&example);
    example.set_extension("i");
    compile(&example);
    example.set_extension("s");
    assemble_and_link(&example);
}
