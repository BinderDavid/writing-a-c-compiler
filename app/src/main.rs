use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use tacky_lib::{
    codegen::compile_program, driver::{assemble_and_link, preprocess}, frontend::{lexer::lex, parser::parse_program},
    tacky::emit::EmitTacky,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Run the lexer, but stop before parsing.
    #[arg(long)]
    lex: bool,
    /// Run the parser, but stop before tacky generation.
    #[arg(long)]
    parse: bool,
    /// Generate tacky code, but stop before assembly generation.
    #[arg(long)]
    tacky: bool,
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
    let mut tokens = lex(&contents)?;
    if cli.lex {
        exit(0)
    };

    // Parse file
    let prog = parse_program(&mut tokens)?;
    if cli.parse {
        exit(0)
    };

    // Generate tacky code
    let mut gen: u64 = 0;
    let tacky = prog.emit_tacky(&mut gen);
    if cli.tacky {
        exit(0)
    }

    // Compile the file
    example.set_extension("s");
    let compiled = compile_program(tacky);
    fs::write(&example, format!("{}", compiled)).expect("Unable to write file");

    // Assemble and link the file.
    assemble_and_link(&example);

    Ok(())
}
