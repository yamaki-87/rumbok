#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01_ok_setter.rs");
    t.pass("tests/ui/03_ok_generic_setter.rs");
    t.pass("tests/ui/04_ok_getteer.rs");
    t.pass("tests/ui/06_ok_generic_getter.rs");
    t.pass("tests/ui/07_ok_getter_setter.rs");
    t.pass("tests/ui/08_ok_data.rs");
    t.pass("tests/ui/012_ok_getter_clone_option.rs");
    t.pass("tests/ui/013_ok_data_clone_option.rs");
    t.compile_fail("tests/ui/02_fail_tuple_struct_setter.rs");
    t.compile_fail("tests/ui/05_fail_tuple_struct_getter.rs");
    t.compile_fail("tests/ui/09_fail_setter_skip_option.rs");
    t.compile_fail("tests/ui/010_fail_getter_skip_option.rs");
    t.compile_fail("tests/ui/011_fail_data_skip_option.rs");
    t.compile_fail("tests/ui/014_fail_getter_skip_clone_option.rs");
    t.compile_fail("tests/ui/015_fail_data_skip_clone_option.rs");
    t.compile_fail("tests/ui/016_fail_getter_no_trait_clone_option.rs");
    t.compile_fail("tests/ui/017_fail_data_no_trait_clone_option copy.rs");
}
