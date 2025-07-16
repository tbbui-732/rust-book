fn main() {
    /*
     * UNCOMMENT TO TEST
    let s1: String = String::from("hello");
    take_ownership(s1); // s moves to function
    println!("{s}"); // should produce an error
    */  

    let mut s2: String = String::from("goodbye");
    s2 = take_and_give_back_ownership(s2);
    println!("{s2}");
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope and 'drop' is called

fn take_and_give_back_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}
