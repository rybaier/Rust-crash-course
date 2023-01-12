// Inside that file, create a test function that verifies:
// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
use testing::sploosh;
use testing::splish;
#[test]
fn test_function3() {
   assert_eq!(sploosh(splish(-1, 0), splish(1,1), splish(3,2)), 4)
}