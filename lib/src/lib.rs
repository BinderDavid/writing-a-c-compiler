use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub mod ast;
pub mod lexer;

/// Preprocess a file `path/to/file.c` and return a file `path/to/file.i`.
/// The preprocessed file no longer contains C preprocessor directives.
pub fn preprocess(inputpath: &PathBuf) {
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
pub fn compile(_inputpath: &Path) {
    todo!()
}

/// Assemble and link the compiled file `path/to/file.s` and return an executable binary.
/// Deletes the original `path/to/file.s`.
pub fn assemble_and_link(inputpath: &PathBuf) {
    let mut outputpath = inputpath.clone();
    outputpath.set_extension("");
    Command::new("gcc").arg(inputpath).arg("-o").arg(outputpath).output().expect("foo");
}
