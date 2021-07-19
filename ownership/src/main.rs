fn main() {
    // this is stored in the stack, and very quick
    let literal = "this is";
    // this gets stored in the heap and is slower
    let mut string = String::from(literal);
    // This line would invalidate "string" and move it to string2
    //let mut string2 = string;
    let mut string2 = string.clone();

    // literal does not have the push_string function
    string.push_str(" coool ");
    string2.push_str(" cool2");
    println!("{}{}", string, string2);
}