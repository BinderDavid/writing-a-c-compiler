use std::{path::{Path, PathBuf}, process::Command};

fn main() {
    let mut example: PathBuf = PathBuf::from("examples/return_2.c");

    preprocess(&example);
    example.set_extension("i");
    compile(&example);
    example.set_extension("s");
    assemble_and_link(&example);
}

/// Preprocess a file `path/to/file.c` and return a file `path/to/file.i`.
/// The preprocessed file no longer contains C preprocessor directives.
fn preprocess(inputpath: &PathBuf) {
    let mut outputpath = inputpath.clone();
    outputpath.set_extension("i");
    Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(inputpath)
        .arg("-o")
        .arg(outputpath)
        .output()
        .expect("foo");
}

/// Compile a preprocessed C file `path/to/file.i` and return a file `path/to/file.s`
/// which contains assembly code.
/// Deletes the original `path/to/file.i`.
fn compile(_inputpath: &Path) {
    todo!()
}

/// Assemble and link the compiled file `path/to/file.s` and return an executable binary.
/// Deletes the original `path/to/file.s`.
fn assemble_and_link(inputpath: &PathBuf) {
    let mut outputpath = inputpath.clone();
    outputpath.set_extension("");
    Command::new("gcc").arg(inputpath).arg("-o").arg(outputpath).output().expect("foo");
}
