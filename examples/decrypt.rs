// This example uses segments from https://bitmovin-a.akamaihd.net/content/art-of-motion_drm/mpds/11331.mpd

use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut input = include_bytes!("decrypt-sample/init.mp4").to_vec();
    input.extend(include_bytes!("decrypt-sample/segment_0.m4s"));

    let mut keys = HashMap::new();
    keys.insert(
        "eb676abbcb345e96bbcf616630f1a3da".to_owned(),
        "100b6c20940f779a4589152b57d2dacb".to_owned(),
    );

    let decrypted_data = mp4decrypt::mp4decrypt(&input, keys, None).unwrap();

    std::fs::File::create("decrypted.mp4")
        .unwrap()
        .write_all(&decrypted_data)
        .unwrap();
}
