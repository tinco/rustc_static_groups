#![feature(plugin)]
#![plugin(static_groups)]

extern crate static_groups;

type TestSignature = fn () -> bool;

#[static_group="my_statics"]
fn my_static() -> bool {
	return true;
}

#[test]
fn it_works() {
	match static_groups::get::<TestSignature>("my_statics") {
		Some (my_fun) => assert!(my_fun()),
		None => assert!(false),
	};
}
