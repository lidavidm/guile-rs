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
    fn decode(scm: &Scm) -> Result<i32, DecodeError> {
        unsafe {
            Ok(guile_sys::scm_to_int32(scm.0))
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
