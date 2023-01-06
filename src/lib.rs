//! This crate provides a safe function to decrypt encrypted mp4 data stream using [Bento4](https://github.com/axiomatic-systems/Bento4).
//!
//! Maximum supported stream size is around `4.29` G.B i.e. [u32::MAX](u32::MAX).

#![allow(improper_ctypes)]

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};
use std::sync::atomic::{AtomicBool, Ordering};

static CRATE_LOCKED: AtomicBool = AtomicBool::new(false);

fn acquire_crate_lock() {
    loop {
        if !CRATE_LOCKED.load(Ordering::Acquire) {
            CRATE_LOCKED.store(true, Ordering::SeqCst);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn release_crate_lock() {
    CRATE_LOCKED.store(false, Ordering::Release);
}

extern "C" {
    fn decrypt_in_memory(
        data: *const c_uchar,
        data_size: c_uint,
        keyids: *mut *const c_char,
        keys: *mut *const c_char,
        nkeys: c_int,
        decrypted_data: *mut Vec<u8>,
        callback: extern "C" fn(*mut Vec<u8>, *const c_uchar, c_uint),
    ) -> c_int;

    fn decrypt_in_memory_with_fragments_info(
        data: *const c_uchar,
        data_length: c_uint,
        keyids: *mut *const c_char,
        keys: *mut *const c_char,
        nkeys: c_int,
        decrypted_data: *mut Vec<u8>,
        callback: extern "C" fn(*mut Vec<u8>, *const c_uchar, c_uint),
        fragments_info_data: *const c_uchar,
        fragments_info_data_size: c_uint,
    ) -> c_int;

    fn basic_mp4split(
        data: *const c_uchar,
        data_size: c_uint,
        split_data: *mut Vec<Vec<u8>>,
        callback: extern "C" fn(*mut Vec<Vec<u8>>, *const c_uchar, c_uint),
    ) -> c_int;
}

extern "C" fn decrypt_callback(decrypted_stream: *mut Vec<u8>, data: *const c_uchar, size: c_uint) {
    unsafe {
        *decrypted_stream = std::slice::from_raw_parts(data, size as usize).to_vec();
    }
}

/// Decrypt encrypted mp4 data stream using given keys.
///
/// # Arguments
///
/// * `data` - Encrypted data stream.
/// * `keys` - Hashmap of keys for decrypting data stream.
///            Hashmap `key` is either a track ID in decimal or a 128-bit KID in hex.
///            Hashmap `value` is a 128-bit key in hex. <br>
///            1. For dcf files, use 1 as the track index <br>
///            2. For Marlin IPMP/ACGK, use 0 as the track ID <br>
///            3. KIDs are only applicable to some encryption methods like MPEG-CENC <br>
/// * `fragments_info` (optional) - Decrypt the fragments read from data stream, with track info read from this stream.
///
/// # Example
///
/// ```no_run
/// use std::collections::HashMap;
///
/// let mut keys = HashMap::new();
/// keys.insert(
///     "eb676abbcb345e96bbcf616630f1a3da".to_owned(),
///     "100b6c20940f779a4589152b57d2dacb".to_owned(),
/// );
///
/// let decrypted_data = mp4decrypt::mp4decrypt(&[0, 0, 0, 112], keys, None).unwrap();
/// ```
pub fn mp4decrypt(
    data: &[u8],
    keys: HashMap<String, String>,
    fragments_info: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    let mut data = data.to_vec();
    let data_size = u32::try_from(data.len()).map_err(|_| "data stream is too large".to_owned())?;

    let mut c_kids_holder = vec![];
    let mut c_keys_holder = vec![];
    let mut c_kids = vec![];
    let mut c_keys = vec![];

    for (i, (kid, key)) in keys.iter().enumerate() {
        c_kids_holder.push(CString::new(kid.to_owned()).unwrap());
        c_keys_holder.push(CString::new(key.to_owned()).unwrap());
        c_kids.push(c_kids_holder[i].as_ptr());
        c_keys.push(c_keys_holder[i].as_ptr());
    }

    let mut decrypted_data: Box<Vec<u8>> = Box::new(vec![]);

    let result = unsafe {
        if let Some(fragments_info) = fragments_info {
            let mut fragments_info_data = fragments_info.to_vec();
            let fragments_info_data_size = u32::try_from(fragments_info_data.len())
                .map_err(|_| "fragments info is too large".to_owned())?;

            acquire_crate_lock();
            let result = decrypt_in_memory_with_fragments_info(
                data.as_mut_ptr(),
                data_size,
                c_kids.as_mut_ptr(),
                c_keys.as_mut_ptr(),
                1,
                &mut *decrypted_data,
                decrypt_callback,
                fragments_info_data.as_mut_ptr(),
                fragments_info_data_size,
            );
            release_crate_lock();
            result
        } else {
            acquire_crate_lock();
            let result = decrypt_in_memory(
                data.as_mut_ptr(),
                data_size,
                c_kids.as_mut_ptr(),
                c_keys.as_mut_ptr(),
                1,
                &mut *decrypted_data,
                decrypt_callback,
            );
            release_crate_lock();
            result
        }
    };

    if result == 0 {
        Ok(*decrypted_data)
    } else {
        Err(match result {
            100 => "invalid hex format for key id".to_owned(),
            101 => "invalid key id".to_owned(),
            102 => "invalid hex format for key".to_owned(),
            x => format!("failed to decrypt data with error code {}", x),
        })
    }
}

extern "C" fn split_callback(
    decrypted_stream: *mut Vec<Vec<u8>>,
    data: *const c_uchar,
    size: c_uint,
) {
    unsafe {
        (*decrypted_stream).push(std::slice::from_raw_parts(data, size as usize).to_vec());
    }
}

/// Splits a fragmented MP4 stream into discrete streams.
///
/// # Example
///
/// ```no_run
/// let split_data = mp4decrypt::mp4split(&[0, 0, 0, 112]).unwrap();
/// let init = split_data[0].clone();
/// let segments = split_data[1..].to_vec();
/// ```
pub fn mp4split(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    let mut data = data.to_vec();
    let data_size = u32::try_from(data.len()).map_err(|_| "data stream is too large".to_owned())?;

    let mut split_data: Box<Vec<Vec<u8>>> = Box::new(vec![]);

    let result = unsafe {
        acquire_crate_lock();
        let result = basic_mp4split(
            data.as_mut_ptr(),
            data_size,
            &mut *split_data,
            split_callback,
        );
        release_crate_lock();
        result
    };

    if result == 0 {
        Ok(*split_data)
    } else {
        Err(match result {
            100 => "no movie found in data stream".to_owned(),
            101 => "cannot write ftyp segment".to_owned(),
            102 => "cannot write init segment".to_owned(),
            103 => "invalid media format".to_owned(),
            x => format!("failed to split data with error code {}", x),
        })
    }
}
