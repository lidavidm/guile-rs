use guile_sys;

use std::ffi;
use std::str;

use scm::{self, Scm, TypedScm};

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
    fn check_type(scm: &Scm) -> Option<DecodeError>;
    fn cast(scm: Scm) -> DecodeResult<TypedScm<Self>>;
    unsafe fn decode(scm: &Scm) -> Self;
}

pub trait Encodable {
    fn encode(&self) -> Result<Scm, EncodeError>;
}

impl<T: Sized + Decodable> TypedScm<T> {
    pub fn convert_value(&self) -> DecodeResult<T>  {
        if let Some(err) = T::check_type(self.to_raw()) {
            Err(err)
        }
        else {
            unsafe {
                Ok(T::decode(self.to_raw()))
            }
        }
    }
}

impl Decodable for i32 {
    fn cast(scm: Scm) -> DecodeResult<TypedScm<i32>> {
        if !scm.is_exact_integer() {
            return Err(DecodeError::TypeError);
        }
        if !scm.is_signed_integer(i32::min_value() as i64,
                                  i32::max_value() as i64) {
            return Err(DecodeError::RangeError);
        }

        return Ok(scm::new_typed_scm(scm));
    }

    fn check_type(scm: &Scm) -> Option<DecodeError> {
        if !scm.is_exact_integer() {
            return Some(DecodeError::TypeError);
        }
        if !scm.is_signed_integer(i32::min_value() as i64,
                                  i32::max_value() as i64) {
            return Some(DecodeError::RangeError);
        }
        None
    }

    unsafe fn decode(scm: &Scm) -> i32 {
        guile_sys::scm_to_int32(scm.to_raw())
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
    fn cast(scm: Scm) -> DecodeResult<TypedScm<bool>> {
        Ok(scm::new_typed_scm(scm))
    }

    fn check_type(scm: &Scm) -> Option<DecodeError> {
        None
    }

    unsafe fn decode(scm: &Scm) -> bool {
        match guile_sys::scm_to_bool(scm.to_raw()) {
            0 => false,
            1 => true,
            v => panic!("scm_to_bool returned invalid value: {}", v),
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
    fn cast(scm: Scm) -> DecodeResult<TypedScm<String>> {
        Ok(scm::new_typed_scm(scm))
    }

    fn check_type(scm: &Scm) -> Option<DecodeError> {
        None
    }

    unsafe fn decode(scm: &Scm) -> String {
        let raw_str = guile_sys::scm_to_utf8_string(scm.to_raw());
        let cstr = ffi::CStr::from_ptr(raw_str);
        str::from_utf8(cstr.to_bytes()).unwrap().to_string()
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
