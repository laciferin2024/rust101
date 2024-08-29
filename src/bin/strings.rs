use std::sync::Arc;
use std::thread;

// static multi_txt: String = String::from("Read by multiple threads");

fn main() {
    // let multi_txt = String::from("Read by multiple threads");
    // let multi_slice = &multi_txt[..]; //doesn't have static lifetime
    //
    let multi_txt = Arc::new("Read by multiple threads");
    let multi_slice = &multi_txt[..]; //doesn't have static lifetime
    //
    // let multi_slice = move || -> &str{
    //     &multi_txt[..]
    // }();


    println!("multi_slice {multi_slice}");

    let shared_txt: Arc<&str> = Arc::from(multi_slice);
    let mut handles = vec![];

    for _ in 0..3 {
        let text_ref = Arc::clone(&shared_txt);
        let handle = thread::spawn(move || {
            println!("Thread is reading {text_ref}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}