#[test]
fn ui_tests() {
  let t = trybuild::TestCases::new();

  t.pass("tests/trybuild/duration_pass.rs");
  t.compile_fail("tests/trybuild/duration_fail.rs");

  t.pass("tests/trybuild/bytes_pass.rs");
  t.compile_fail("tests/trybuild/bytes_fail.rs");
}
