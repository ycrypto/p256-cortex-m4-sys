use std::cfg;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET")?;

    // Cortex-M33 is compatible with Cortex-M4 and its DSP extension instruction UMAAL.
    let cortex_m4 = target.starts_with("thumbv7em") || target.starts_with("thumbv8m.main");

    assert!(cortex_m4);

    let mut builder = cc::Build::new();

    let mut builder = builder
        .flag("-std=c11")
        .file("P256-Cortex_M4/p256-cortex-m4.c")
        .file("P256-Cortex_M4/p256-cortex-m4-asm-gcc.S")
        .flag("-march=armv7e-m")
    ;

    builder.compile("micro-ecc-sys");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("P256-Cortex_M4/p256-cortex-m4.h")
        .clang_arg(format!("--target={}", target))
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)

        .generate()
        .expect("Unable to generate bindings");

    let out_file = out_dir.join("bindings.rs");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");

    Ok(())
}