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

    pub unsafe fn from_raw(scm: guile_sys::SCM) -> Scm {
        Scm(scm)
    }

    pub unsafe fn to_raw(&self) -> guile_sys::SCM {
        self.0
    }
}
