fn main() {
    let s = String::from("hello world");
    // let word = first_word(&s);
    let word = first_word(&s);
    s.clear();
    println!("{}", word);

}

fn first_word(s: &str) -> &str {
    let b = s.bytes();
    let mut idx: Option<usize> = None;
    for (i, byte) in b.enumerate() {
        if  byte == b' '{
            idx = Some(i);
        }
    }
    match idx {
        Some(idx) => &s[0..idx],
        None => &s,
    }
}
