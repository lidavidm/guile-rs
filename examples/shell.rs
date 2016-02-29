extern crate guile;

fn main() {
    guile::init(|vm| {
        let mut args = Vec::new();
        args.push("Test".to_string());
        vm.shell(args);
    });
}
