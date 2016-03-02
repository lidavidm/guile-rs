extern crate guile;

use guile::repr::*;
use guile::{Exact, Scm, Untyped};

fn test(x: Scm<Untyped>) -> Scm<Exact> {
    let two = (&2).encode().unwrap();
    Decodable::cast(x).unwrap() + two
}

fn main() {
    guile::init(|vm| {
        let mut args = Vec::new();
        args.push("Test".to_string());
        vm.define_subr1("add_two", test);
        vm.define("two", (&2).encode().unwrap());
        // vm.define("help", "help".encode().unwrap());

        // if !bool::decode(vm.is_defined("three", None)).unwrap() {
        //     vm.define("two", (&3).encode().unwrap());
        // }

        vm.shell(args);
    });
}
