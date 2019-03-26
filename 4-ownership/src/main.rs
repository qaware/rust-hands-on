#[allow(unused)]

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);

    let i1 = 5;
    let i2 = i1;

    println!("{}", i1);

    let (s3, len) = calc_len_and_ret(s2);

    let len = calc_len_borrow(&s1);
    println!("Length of {}: {}", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    let my_string = String::from("Hello world");
    let hello = &my_string[0..5];
    let world = &my_string[6..];

    println!("{}, {}!", hello, world);
}

fn calc_len(s: String) -> usize {
    s.len()
}

fn calc_len_and_ret(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calc_len_borrow(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
