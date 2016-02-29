extern crate guile;

use guile::repr::*;
use guile::Scm;

fn test(x: Scm) -> Scm {
    (i32::decode(&x).unwrap() + 2).encode().unwrap()
}

fn main() {
    guile::init(|vm| {
        let mut args = Vec::new();
        args.push("Test".to_string());
        vm.define_primitive_subroutine("add_two", test);
        vm.shell(args);
    });
}
