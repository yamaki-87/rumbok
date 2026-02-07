#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01_ok_setter.rs");
    t.pass("tests/ui/03_ok_generic_setter.rs");
    t.pass("tests/ui/04_ok_getteer.rs");
    t.pass("tests/ui/06_ok_generic_getter.rs");
    t.pass("tests/ui/07_ok_getter_setter.rs");
    t.pass("tests/ui/08_ok_data.rs");
    t.compile_fail("tests/ui/02_fail_tuple_struct_setter.rs");
    t.compile_fail("tests/ui/05_fail_tuple_struct_getter.rs");
}
