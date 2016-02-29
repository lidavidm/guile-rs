use guile_sys;

pub struct Scm(pub guile_sys::SCM);

impl Scm {
    pub unsafe fn from_raw(scm: guile_sys::SCM) -> Scm {
        Scm(scm)
    }
}
