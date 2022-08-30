fn main() {
    let bento4_source_path = std::path::PathBuf::from(std::env::var("BENTO4_SOURCE_PATH")
    .expect("Download Bento4 (https://github.com/axiomatic-systems/Bento4/tags) source and store that path in BENTO4_SOURCE_PATH envoirnment variable."));
    let ap4_lib_path = std::env::var("AP4_LIB_PATH").expect(
        "Use AP4_LIB_PATH envoirnment variable to store path of ap4 library from Bento4 SDK.",
    );

    println!("cargo:rustc-link-search=native={}", ap4_lib_path);
    println!("cargo:rustc-link-lib=static=ap4");

    cc::Build::new()
        .cpp(true)
        .file("src/mp4decrypt.cpp")
        .include(bento4_source_path.join("Source/C++/Core"))
        .include(bento4_source_path.join("Source/C++/Codecs"))
        .include(bento4_source_path.join("Source/C++/Crypto"))
        .include(bento4_source_path.join("Source/C++/MetaData"))
        .compile("decrypt");

    // let bindings = bindgen::Builder::default()
    //     .header("src/mp4decrypt.h")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file("bindings.rs")
    //     .expect("Couldn't write bindings!");
}
