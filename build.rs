use std::{
    //ffi::OsStr,
    env,
    path::{PathBuf, Path},
    convert::AsRef,
    iter::IntoIterator
};

fn join<I>(it: I) ->  PathBuf
where
    I: IntoIterator,
    I::Item: AsRef<Path>,
{
    it.into_iter().collect()
}


#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("error while retreiving environment variable {0}")]
    Env(#[from] env::VarError),
    #[error("error running binding generation {0}")]
    Bindgen(#[from] cbindgen::Error),
}

static INCLUDE_DIR: &'static str = "include";

fn run() -> Result<(), Error> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR")?;
    
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()?
        .write_to_file(join(&[INCLUDE_DIR, "chelp.h"]));
    
    /*
    cc::Build::new()
        .file(join(&["test", "test.c"]))
        .warnings(true)
        .extra_warnings(true)
        .warnings_into_errors(true)
        .include(include_dir)
        .out_dir("build")
        .compile("test");
    */
    
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {},
        Err(err) => eprint!("Error: {}\n", err),
    }
}
