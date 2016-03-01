extern crate guile;

use guile::repr::*;
use guile::Scm;

fn test(x: Scm) -> Scm {
    x.sum(&(&2).encode().unwrap()).unwrap()
}

fn main() {
    guile::init(|vm| {
        let mut args = Vec::new();
        args.push("Test".to_string());
        vm.define_subr1("add_two", test);
        vm.define("two", (&2).encode().unwrap());
        vm.define("help", "help".encode().unwrap());

        // if !bool::decode(vm.is_defined("three", None)).unwrap() {
        //     vm.define("two", (&3).encode().unwrap());
        // }

        vm.shell(args);
    });
}
