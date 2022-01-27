fn main() {
    let str: &str = "Hello world";
    let mut string: String = String::from("Hello world");

    let slice = &string[..6];
    slice.len();

    string.push('1');
    string.push_str("! Bob");
    string = string.replace("Hello", "Bye");
    println!("{}", string);
}
