fn main() {
    let decrypted_data = mp4decrypt::mp4split(include_bytes!("split-sample/sample.mp4")).unwrap();

    println!("init.mp4 + {} splitted.mp4", decrypted_data.len() - 1);

    assert_eq!(&decrypted_data[0], include_bytes!("split-sample/init.mp4"));
    assert_eq!(
        &decrypted_data[190],
        include_bytes!("split-sample/segment-1.0190.m4s")
    );
}
