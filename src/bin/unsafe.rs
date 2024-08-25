fn main() {
    let x: i32 = 42;
    let r: *const i32 = &x;

    unsafe {
        println!("The value of x is: {}", *r);
    // dereference a raw pointer
    }
}
