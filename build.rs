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

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    const INCLUDE_DIR: &'static str = "include";

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
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
    
}
