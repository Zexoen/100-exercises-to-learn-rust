// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}
impl From<i32> for WrappingU32 {
    fn from(a: i32) -> Self {
        WrappingU32 {
            value: a as u32,
        }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
