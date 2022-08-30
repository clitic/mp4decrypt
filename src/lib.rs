//! This crate provides a safe function to decrypt encrypted mp4 data stream using [Bento4 SDK](https://github.com/axiomatic-systems/Bento4).
//! To build this crate it require these envoirnment variables to be defined.
//!
//! * `BENTO4_SOURCE_PATH` - The path of [Bento4](https://github.com/axiomatic-systems/Bento4/tags) source code.
//! * `AP4_LIB_PATH` - The path of ap4 library from Bento4 SDK.

#![allow(improper_ctypes)]

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};

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
}

extern "C" fn callback(store: *mut Vec<u8>, data: *const c_uchar, size: c_uint) {
    unsafe {
        *store = std::slice::from_raw_parts(data, size as usize).to_vec();
    }
}

/// Decrypt encrypted mp4 data stream using given keys.
/// Maximum supported stream size is around `4.29` G.B.
///
/// ## Arguments
///
/// * `data` - Encrypted data stream.
/// * `keys` - Keys hashmap with key id mapping to decryption key.
/// * `fragments_info` (optional) - Create decryption processor from this stream not `data` stream.
///
/// ## Example
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
/// let decrypted_data = mp4decrypt::decrypt(&[0, 0, 0, 112], keys, None).unwrap();
/// ```
pub fn decrypt(
    data: &[u8],
    keys: HashMap<String, String>,
    fragments_info: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    let mut data = data.to_vec();
    let data_size = u32::try_from(data.len()).map_err(|_| format!("data stream is too large"))?;

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
                .map_err(|_| format!("fragments info is too large"))?;

            decrypt_in_memory_with_fragments_info(
                data.as_mut_ptr(),
                data_size,
                c_kids.as_mut_ptr(),
                c_keys.as_mut_ptr(),
                1,
                &mut *decrypted_data,
                callback,
                fragments_info_data.as_mut_ptr(),
                fragments_info_data_size,
            )
        } else {
            decrypt_in_memory(
                data.as_mut_ptr(),
                data_size,
                c_kids.as_mut_ptr(),
                c_keys.as_mut_ptr(),
                1,
                &mut *decrypted_data,
                callback,
            )
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
