use std::path::PathBuf;

use tacky_lib::{assemble_and_link, compile, preprocess};

fn main() {
    let mut example: PathBuf = PathBuf::from("examples/return_2.c");

    preprocess(&example);
    example.set_extension("i");
    compile(&example);
    example.set_extension("s");
    assemble_and_link(&example);
}
