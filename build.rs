fn main() {
    println!("cargo:rerun-if-changed=src/mp4decrypt.cpp");
    println!("cargo:rerun-if-changed=src/mp4decrypt.h");
    // let bento4 = std::path::Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("Bento4");
    // println!("cargo:rustc-link-search=.");
    // println!("cargo:rustc-link-lib=static=ap4");

    let mut builder = cc::Build::new();
    let mut builder = builder
        .cpp(true)
        .warnings(false)
        .static_crt(true)
        // .cpp_link_stdlib("c++_static")
        // .cpp_link_stdlib("stdc++")
        .include("Bento4/Source/C++/Core")
        .include("Bento4/Source/C++/Codecs")
        .include("Bento4/Source/C++/Crypto")
        .include("Bento4/Source/C++/MetaData")
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
        .files(
            glob::glob("Bento4/Source/C++/System/StdC/*.cpp")
                .unwrap()
                .map(|x| x.unwrap()),
        )
        .file("src/mp4decrypt.cpp");

    if builder.get_compiler().is_like_msvc() {
        builder = builder
            .define("_CRT_SECURE_NO_WARNINGS", None)
            .file("Bento4/Source/C++/System/Win32/Ap4Win32Random.cpp");
    } else {
        builder = builder.file("Bento4/Source/C++/System/Posix/Ap4PosixRandom.cpp");
    }
    builder.compile("ap4decrypt");

    // let bindings = bindgen::Builder::default()
    //     .header("src/mp4decrypt.h")
    //     .generate()
    //     .unwrap()
    //     .write_to_file("bindings.rs")
    //     .unwrap();
}
