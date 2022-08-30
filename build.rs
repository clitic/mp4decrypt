fn main() {
    println!("cargo:rerun-if-changed=src/mp4decrypt.cpp");
    println!("cargo:rerun-if-changed=src/mp4decrypt.h");

    cc::Build::new()
        .cpp(true)
        .file("src/mp4decrypt.cpp")
        .files(
            glob::glob("Bento4/Source/C++/Core/*.cpp")
                .unwrap()
                .map(|x| x.unwrap()),
        )
        .files(
            glob::glob("Bento4/Source/C++/Codecs/*.cpp")
                .unwrap()
                .map(|x| x.unwrap()),
        )
        .files(
            glob::glob("Bento4/Source/C++/Crypto/*.cpp")
                .unwrap()
                .map(|x| x.unwrap()),
        )
        .files(
            glob::glob("Bento4/Source/C++/MetaData/*.cpp")
                .unwrap()
                .map(|x| x.unwrap()),
        )
        // .files(glob::glob("Bento4/Source/C++/System/StdC/*.cpp").unwrap().map(|x| x.unwrap()))
        .include("Bento4/Source/C++/Core")
        .include("Bento4/Source/C++/Codecs")
        .include("Bento4/Source/C++/Crypto")
        .include("Bento4/Source/C++/MetaData")
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
