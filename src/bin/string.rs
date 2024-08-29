use std::rc::Rc;

fn get_greeting(hour: u8) -> &'static str {
    if hour < 12 {
        "Good Morning!"
    } else if hour < 18 {
        "Good Afternoon"
    } else {
        "Good Evening"
    }
}

fn main() {
    let greet = get_greeting(12);
    println!("greet = {greet}");

    let large_text: &'static str = "Hey..... this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`";
    let rc_str = Rc::from(&large_text[4..24]);
}
