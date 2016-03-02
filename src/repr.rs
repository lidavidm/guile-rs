use guile_sys;

use std::ffi;
use std::str;

use scm::{self, Scm, Untyped};

#[derive(Debug)]
pub enum DecodeError {
    Utf8Error(str::Utf8Error),
    TypeError,
    RangeError,
}

#[derive(Debug)]
pub enum EncodeError {

}

pub type DecodeResult<T> = Result<T, DecodeError>;

impl From<str::Utf8Error> for DecodeError {
    fn from(err: str::Utf8Error) -> DecodeError {
        DecodeError::Utf8Error(err)
    }
}

pub trait Decodable: Sized {
    fn check_type(scm: &Scm<Untyped>) -> Option<DecodeError>;
    fn cast(scm: Scm<Untyped>) -> DecodeResult<Scm<Self>> {
        match Self::check_type(&scm) {
            Some(err) => Err(err),
            None => unsafe {
                Ok(scm.force_cast::<Self>())
            }
        }
    }
    fn decode(scm: &Scm<Self>) -> Self;
}

pub trait Encodable {
    type Output;
    fn encode(&self) -> Result<Scm<Self::Output>, EncodeError>;
}

impl Scm<Untyped> {
    pub fn check_type<T: Decodable>(&self) -> Option<DecodeError> {
        return T::check_type(self);
    }

    pub fn cast<T: Decodable>(self) -> DecodeResult<Scm<T>> {
        return T::cast(self);
    }
}

impl Decodable for scm::Exact {
    fn check_type(scm: &Scm<Untyped>) -> Option<DecodeError> {
        unsafe {
            match guile_sys::scm_is_exact_integer(scm.to_raw()) {
                0 => Some(DecodeError::TypeError),
                1 => None,
                v => panic!("scm_is_exact_integer returned invalid value: {}", v),
            }
        }
    }

    fn decode(_: &Scm<scm::Exact>) -> scm::Exact {
        scm::Exact {}
    }
}

impl Decodable for i32 {
    fn check_type(scm: &Scm<Untyped>) -> Option<DecodeError> {
        if !scm.is_exact_integer() {
            return Some(DecodeError::TypeError);
        }
        if !scm.is_signed_integer(i32::min_value() as i64,
                                  i32::max_value() as i64) {
            return Some(DecodeError::RangeError);
        }
        None
    }

    fn decode(scm: &Scm<i32>) -> i32 {
        unsafe { guile_sys::scm_to_int32(scm.to_raw()) }
    }
}

impl Encodable for i32 {
    type Output = scm::Exact;

    fn encode(&self) -> Result<Scm<scm::Exact>, EncodeError> {
        unsafe {
            Ok(Scm::<scm::Exact>::from_raw(guile_sys::scm_from_int32(*self)))
        }
    }
}

impl Decodable for bool {
    fn check_type(scm: &Scm<Untyped>) -> Option<DecodeError> {
        // TODO:
        None
    }

    fn decode(scm: &Scm<bool>) -> bool {
        unsafe {
            match guile_sys::scm_to_bool(scm.to_raw()) {
                0 => false,
                1 => true,
                v => panic!("scm_to_bool returned invalid value: {}", v),
            }
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
    fn check_type(scm: &Scm<Untyped>) -> Option<DecodeError> {
        unsafe {
            match guile_sys::scm_is_string(scm.to_raw()) {
                0 => Some(DecodeError::TypeError),
                1 => None,
                v => panic!("scm_is_string returned invalid value {}", v),
            }
        }
    }

    fn decode(scm: &Scm<String>) -> String {
        unsafe {
            let raw_str = guile_sys::scm_to_utf8_string(scm.to_raw());
            let cstr = ffi::CStr::from_ptr(raw_str);
            str::from_utf8(cstr.to_bytes()).unwrap().to_string()
        }
    }
}

impl Encodable for String {
    type Output = String;

    fn encode(&self) -> Result<Scm<String>, EncodeError> {
        unsafe {
            let raw_str = ffi::CString::new(self.as_bytes()).unwrap().as_ptr();
            Ok(Scm::from_raw(guile_sys::scm_from_utf8_string(raw_str)))
        }
    }
}

impl Encodable for str {
    type Output = String;

    fn encode(&self) -> Result<Scm<String>, EncodeError> {
        unsafe {
            let raw_str = ffi::CString::new(self.as_bytes()).unwrap().as_ptr();
            Ok(Scm::from_raw(guile_sys::scm_from_utf8_string(raw_str)))
        }
    }
}
