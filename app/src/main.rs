use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use tacky_lib::{assemble_and_link, lexer::Lexer, parser::parse_program, preprocess};

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
    match exec(&cli) {
        Ok(_) => exit(0),
        Err(str) => {
            println!("{}", str);
            exit(2)
        }
    }
}

fn exec(cli: &Cli) -> Result<(), String> {
    let mut example: PathBuf = cli.filepath.clone();

    // Preprocess the C file and remove preprocessor directives.
    preprocess(&example);
    example.set_extension("i");

    // Read in file
    let contents = fs::read_to_string(&example).expect("Should have been able to read the file");

    // Lex file
    let mut tokens = Lexer::new(&contents);
    if cli.lex {
        exit(0)
    };

    // Parse file
    let _prog = parse_program(&mut tokens)?;
    if cli.parse {
        exit(0)
    };

    // Assemble and link the file.
    example.set_extension("s");
    assemble_and_link(&example);

    Ok(())
}
