fn main() {
    let mut x: i32 = 42;
    let r: *const i32 = &22;
    let  blunder: *mut i32 = &mut x;

    unsafe {
        println!("The value of x is: {}", *r);
        *blunder+=1;
        *blunder;
    // dereference a raw pointer
    }
}
