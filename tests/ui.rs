#[test]
fn ui_tests() {
  let t = trybuild::TestCases::new();

  #[cfg(feature = "humantime")]
  {
    t.pass("tests/trybuild/duration_pass.rs");
    t.compile_fail("tests/trybuild/duration_fail.rs");

    t.pass("tests/trybuild/datetime_pass.rs");
    t.compile_fail("tests/trybuild/datetime_fail.rs");
  }

  #[cfg(feature = "bytesize")]
  {
    t.pass("tests/trybuild/bytes_pass.rs");
    t.compile_fail("tests/trybuild/bytes_fail.rs");
  }
}
