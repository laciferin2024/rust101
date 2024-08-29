use std::mem::ManuallyDrop;

fn main() {
    let mut x: i32 = 42;
    let r: *const i32 = &22;
    let blunder: *mut i32 = &mut x;

    unsafe {
        println!("The value of r is: {}", *r);
        *blunder += 1;
        println!("The value of x is: {}, x is supposed to be {x}", *r);

        // dereference a raw pointer
    }
    unsafe_arr();
    manual_drop();
}

fn unsafe_arr() {
    let mut arr = [1, 2, 3];
    let ptr = arr.as_mut_ptr();

    unsafe {
        for i in 0..4 {
            *ptr.add(i) = i as i32;
        }
    }

    println!("{:?}", arr);
}

fn manual_drop() {
    let mut x = ManuallyDrop::new(Box::new(42));

    unsafe {
        ManuallyDrop::drop(&mut x); // Manually drop the Box
                                    // Double drop occurs here when `x` goes out of scope
    }
    println!("{}", x.to_string());
}
