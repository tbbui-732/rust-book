fn main() {
    let s = String::from("hello");
    take_ownership(s); // s moves to function
    println!("{s}"); // should produce an error
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope and 'drop' is called
