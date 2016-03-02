use std::marker;
use std::ops;

use guile_sys;

pub struct Untyped;

pub struct Scm<T> {
    scm: guile_sys::SCM,
    value: marker::PhantomData<*const T>
}

pub struct Exact;

impl<T> Scm<T> {
    // pub fn is_true(&self) -> bool {
    //     unsafe {
    //         match guile_sys::scm_is_true(self.to_raw()) {
    //             0 => false,
    //             1 => true,
    //             v => panic!("scm_is_true returned invalid value: {}", v),
    //         }
    //     }
    // }

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

    pub fn as_untyped(self) -> Scm<Untyped> {
        unsafe {
            self.force_cast::<Untyped>()
        }
    }

    pub unsafe fn from_raw(scm: guile_sys::SCM) -> Scm<T> {
        Scm {
            scm: scm,
            value: marker::PhantomData,
        }
    }

    pub unsafe fn force_cast<U>(self) -> Scm<U> {
         Scm {
            scm: self.scm,
            value: marker::PhantomData,
        }
    }

    pub unsafe fn to_raw(&self) -> guile_sys::SCM {
        self.scm
    }
}

impl ops::Add for Scm<Exact> {
    type Output = Scm<Exact>;

    fn add(self, rhs: Scm<Exact>) -> Scm<Exact> {
        unsafe {
            Scm::<Exact>::from_raw(guile_sys::scm_sum(self.scm, rhs.scm))
        }
    }
}
