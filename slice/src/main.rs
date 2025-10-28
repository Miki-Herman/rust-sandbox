fn main() {
    let s = String::from("one two three");
    let index = first_word(&s);

    println!("{}", (&s[0..index]));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    };

    return s.len();
} 
