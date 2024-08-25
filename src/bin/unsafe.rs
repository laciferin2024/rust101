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
    unsafe_arr()
}


fn unsafe_arr(){
     let mut arr = [1, 2, 3];
    let ptr = arr.as_mut_ptr();

    unsafe {
        for i in 0..4 {
            *ptr.add(i) = i as i32;
        }
    }

    println!("{:?}", arr);
}