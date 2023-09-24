// #[deprecated...]
// cargo test
#[test]
#[should_panic]
fn tests_a_thing() {
    assert_eq!(8, 9);
}

#[test]
fn tests_another_thing() {

}

#[repr(C)]      // C언어와 같게 처리해준다.
struct SomeRustStruct {
    one: u8,
    two: u16
}

fn main() {

}
