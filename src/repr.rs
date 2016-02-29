use guile_sys;

use scm::Scm;

#[derive(Debug)]
pub enum DecodeError {

}

#[derive(Debug)]
pub enum EncodeError {

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
