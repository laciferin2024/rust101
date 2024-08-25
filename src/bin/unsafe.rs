fn main() {
    let mut x: i32 = 42;
    let r: *const i32 = &22;
    let  blunder: *mut i32 = &mut x;

    unsafe {
        println!("The value of r is: {}", *r);
        *blunder+=1;
        println!("The value of x is: {}, x is supposed to be {x}", *r);

    // dereference a raw pointer
    }
}
