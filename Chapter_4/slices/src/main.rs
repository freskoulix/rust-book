fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("s: {}", s);
    println!("word: {}", word);

    s.clear(); // this empties the String, making it equal to ""

    println!("s: {}", s);
    println!("word: {}", word);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}/{}", hello, world);

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_2(&my_string[0..6]);
    println!("{}", word);
    let word = first_word_2(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which is equivalent
    // to a slice of the whole `String`
    let word = first_word_2(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_2(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word_2(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_2(my_string_literal);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
