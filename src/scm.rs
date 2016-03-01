use guile_sys;

pub struct Scm(guile_sys::SCM);

impl Scm {
    // pub fn is_true(&self) -> bool {
    //     unsafe {
    //         match guile_sys::scm_is_true(self.to_raw()) {
    //             0 => false,
    //             1 => true,
    //             v => panic!("scm_is_true returned invalid value: {}", v),
    //         }
    //     }
    // }

    pub fn sum(&self, rhs: &Scm) -> Option<Scm> {
        if !self.is_number() || !rhs.is_number() {
            return None;
        }
        unsafe {
            Some(Scm::from_raw(guile_sys::scm_sum(self.to_raw(),
                                                  rhs.to_raw())))
        }
    }

    pub fn is_number(&self) -> bool {
        unsafe {
            match guile_sys::scm_is_number(self.to_raw()) {
                0 => false,
                1 => true,
                v => panic!("scm_is_number returned invalid value: {}", v),
            }
        }
    }

    pub fn is_exact_integer(&self) -> bool {
        unsafe {
            match guile_sys::scm_is_exact_integer(self.to_raw()) {
                0 => false,
                1 => true,
                v => panic!("scm_is_exact_integer returned invalid value: {}", v),
            }
        }
    }

    pub fn is_signed_integer(&self, min: i64, max: i64) -> bool {
        let min = min as guile_sys::intmax_t;
        let max = max as guile_sys::intmax_t;
        unsafe {
            match guile_sys::scm_is_signed_integer(self.to_raw(), min, max) {
                0 => false,
                1 => true,
                v => panic!("scm_is_signed_integer returned invalid value: {}", v),
            }
        }
    }

    pub fn is_unsigned_integer(&self, min: u64, max: u64) -> bool {
        let min = min as guile_sys::uintmax_t;
        let max = max as guile_sys::uintmax_t;
        unsafe {
            match guile_sys::scm_is_unsigned_integer(self.to_raw(), min, max) {
                0 => false,
                1 => true,
                v => panic!("scm_is_unsigned_integer returned invalid value: {}", v),
            }
        }
    }

    pub unsafe fn from_raw(scm: guile_sys::SCM) -> Scm {
        Scm(scm)
    }

    pub unsafe fn to_raw(&self) -> guile_sys::SCM {
        self.0
    }
}

use std::marker;

pub struct TypedScm<T> {
    scm: Scm,
    value: marker::PhantomData<T>,
}

impl<T> TypedScm<T> {
    pub fn to_raw(&self) -> &Scm {
        &self.scm
    }
}

pub fn new_typed_scm<T>(scm: Scm) -> TypedScm<T> {
    TypedScm {
        scm: scm,
        value: marker::PhantomData,
    }
}
