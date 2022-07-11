fn main() {
    let s = String::from("hello");

    let mut mutable_string = String::from("hello");

    mutable_string.push_str(", world!");

    println("{}", s);
}
