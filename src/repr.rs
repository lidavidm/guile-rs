use guile_sys;

use std::ffi;
use std::str;

use scm::Scm;

#[derive(Debug)]
pub enum DecodeError {
    Utf8Error(str::Utf8Error),
}

#[derive(Debug)]
pub enum EncodeError {

}

impl From<str::Utf8Error> for DecodeError {
    fn from(err: str::Utf8Error) -> DecodeError {
        DecodeError::Utf8Error(err)
    }
}

pub trait Decodable: Sized {
    fn decode(scm: &Scm) -> Result<Self, DecodeError>;
}

pub trait Encodable {
    fn encode(&self) -> Result<Scm, EncodeError>;
}

impl Decodable for i32 {
    // TODO: actually check, so that Guile doesn't take over and
    // automatically error
    fn decode(scm: &Scm) -> Result<i32, DecodeError> {
        unsafe {
            Ok(guile_sys::scm_to_int32(scm.to_raw()))
        }
    }
}

impl Encodable for i32 {
    fn encode(&self) -> Result<Scm, EncodeError> {
        unsafe {
            Ok(Scm::from_raw(guile_sys::scm_from_int32(*self)))
        }
    }
}

impl Decodable for bool {
    fn decode(scm: &Scm) -> Result<bool, DecodeError> {
        unsafe {
            Ok(match guile_sys::scm_to_bool(scm.to_raw()) {
                0 => false,
                1 => true,
                v => panic!("scm_to_bool returned invalid value: {}", v),
            })
        }
    }
}

// impl Encodable for bool {
//     fn encode(&self) -> Result<Scm, EncodeError> {
//         unsafe {
//             if *self {
//                 Ok(Scm::from_raw(guile_sys::scm_from_bool(1)))
//             }
//             else {
//                 Ok(Scm::from_raw(guile_sys::scm_from_bool(0)))
//             }
//         }
//     }
// }

impl Decodable for String {
    fn decode(scm: &Scm) -> Result<String, DecodeError> {
        unsafe {
            let raw_str = guile_sys::scm_to_utf8_string(scm.to_raw());
            let cstr = ffi::CStr::from_ptr(raw_str);
            Ok(try!(str::from_utf8(cstr.to_bytes())).to_string())
        }
    }
}

impl Encodable for str {
    fn encode(&self) -> Result<Scm, EncodeError> {
        unsafe {
            let raw_str = ffi::CString::new(self).unwrap().as_ptr();
            Ok(Scm::from_raw(guile_sys::scm_from_utf8_string(raw_str)))
        }
    }
}
