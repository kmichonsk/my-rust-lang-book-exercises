#![allow(dead_code, unused_variables)]

mod ip;
mod message;
mod rectangle;
mod user;

use ip::*;
use message::*;
use rectangle::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::ops::{Div, Rem};
use std::str::SplitWhitespace;

fn main() {
    exercise17();
}

fn exercise17() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    println!();
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "See which string is the longest!",
    );
    println!("The longest string is {}!", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Cool:
// // 33 |         y
// //    |         ^ lifetime `'a` required
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// // If we have just one argument this introduces the lifetime
// // specifier by default. This compiles fine just like the
// // function "first_word" in the bottom from chapter 4
// fn longest(x: &str) -> &str {
//     x
// }
// // this will also compile, the borrow checker sees no problem with it
// fn longest(x: &str) -> &str {
//     let y: String = String::new();
//     &x
// }
//
// // this won't
// fn longest(x: &str) -> &str {
//     let y: String = String::new();
//     &y
// }

fn exercise16() {
    println!(
        "largest:\n\t(1,5,3,4,2): {}\n\t(1.0,2.1,4.3,4.012): {}\n\t(1): {}\n\t(a): {}",
        largest(&[1, 5, 3, 4, 2]),
        largest(&[1.0, 2.1, 4.3, 4.012]),
        largest(&[1]),
        largest(&['a']),
    );
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn exercise15() {
    let f: File = File::open("./target/tmptest").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("./target/tmptest").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn exercise_add_employees() {
    let mut departments_employees: HashMap<String, Vec<String>> = HashMap::new();

    println!(
        "\
Usage
  add department name
    Add \"name\" to \"department\"
  list [department]
    List all entries or if provided list all entries in the \"department\" 
   "
    );

    let mut input = String::with_capacity(512);
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("stdin read error");

        let mut input_iter = input.split_whitespace();
        match input_iter.next() {
            Some(option) => match option {
                "add" => handle_add(&mut departments_employees, input_iter),
                "list" => handle_list(&mut departments_employees, input_iter),
                _ => {}
            },
            None => {}
        }
    }
}

fn handle_add(
    departments_employees: &mut HashMap<String, Vec<String>>,
    mut input: SplitWhitespace,
) {
    let department = input.next().expect("Invalid department input");
    let name = input.next().expect("Invalid name input");

    let department_employees = departments_employees
        .entry(String::from(department))
        .or_insert(vec![]);
    department_employees.push(String::from(name));
}

fn handle_list(
    departments_employees: &mut HashMap<String, Vec<String>>,
    mut input: SplitWhitespace,
) {
    match input.next() {
        None => {
            dbg!(&departments_employees);
        }
        Some(department) => {
            dbg!(&departments_employees.get(department).unwrap_or(&vec![]));
        }
    };
}

fn exercise_pig_latin() {
    println!(
        "pig_latin:\n\t(apple,hay): {}\n\t(apple): {}\n\t(): {}\n\t(ap,,hay): {}",
        pig_latinize(&vec!["apple", "hay"]),
        pig_latinize(&vec!["apple"]),
        pig_latinize(&vec![]),
        pig_latinize(&vec!["ap", "", "hay"]),
    );
}

fn pig_latinize(words: &Vec<&str>) -> String {
    let mut result = String::new();
    for (idx, word) in words.iter().enumerate() {
        result.push_str(word);
        if idx != words.len() - 1 {
            result.push('-');
        }
    }
    result
}

fn exercise_median() {
    // collection's exercises
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    let some_numbers: Vec<i32> = vec![4, 2, 1, 4];

    println!(
        "median:\n\t(1,2,3,4,5): {}\n\t(1,2,4,4): {}\n\t(): {}\n\t(1): {}\n\t(0): {}",
        median(&vec![1, 5, 3, 4, 2]).unwrap(),
        median(&some_numbers).unwrap(),
        match median(&vec![]) {
            None => "No median",
            Some(_) => panic!(),
        },
        median(&vec![1]).unwrap(),
        median(&vec![0]).unwrap(),
    );
}

fn exercise_mean() {
    println!(
        "mean:\n\t(1,4): {}\n\t(1,1): {}\n\t(0): {}\n\t(): {}",
        mean(&vec![1, 4]).unwrap(),
        mean(&vec![1, 1]).unwrap(),
        mean(&vec![0]).unwrap(),
        match mean(&vec![]) {
            None => "No mean",
            Some(_) => panic!(),
        },
    );
}

fn median(numbers: &Vec<i32>) -> Option<f64> {
    if numbers.len() == 0 {
        return None;
    }

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let len = sorted_numbers.len();
    match len.rem(2).cmp(&0) {
        Ordering::Equal => Some(
            mean(&vec![
                *sorted_numbers.get((len / 2) - 1).unwrap(),
                *sorted_numbers.get(len / 2).unwrap(),
            ])
            .unwrap(),
        ),
        _ => Some(*sorted_numbers.get(len / 2).unwrap() as f64),
    }
}

fn mean(numbers: &Vec<i32>) -> Option<f64> {
    if numbers.len() > 0 {
        return Some(
            numbers
                .iter()
                .map(|x| *x as f64)
                .sum::<f64>()
                .div(numbers.len() as f64),
        );
    }
    None
}

fn exercise14() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
    println!();

    scores.insert("Blue".to_string(), 20);
    println!("{:?}", scores);
    println!();

    scores.entry("Blue".to_string()).or_insert(999);
    scores.entry("Red".to_string()).or_insert(60);
    println!("{:?}", scores);
    println!();

    let orange_score = scores.entry("Orange".to_string()).or_insert(1);
    *orange_score += 99;
    println!("{:?}", scores);

    match scores.get("Pink") {
        None => println!("Pinks didn't participate."),
        Some(pinks_score) => println!("Pinks score is: {}", pinks_score),
    }
    println!("{:?}", scores);
}

fn exercise13() {
    let s = "initial content";
    let mut s: String = s.to_string();
    s.push(' ');
    s.push_str("foo");
    s += &" ".to_owned();
    let s_bar = String::from("bar");
    s += &s_bar;
    println!("{}", s);

    for c in s.chars() {
        print!("{} ", c);
    }
}

fn exercise12() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(element) => println!("The third element is {}", element),
        None => println!("No third element!"),
    }
    if let Some(element) = v.get(2) {
        println!("The third element is {}", element);
    }

    for i in &v {
        print!("{} ", i);
    }
    println!();

    let mut mut_vec: Vec<u32> = vec![101, 102, 103];
    for i in &mut mut_vec {
        *i += 1;
        print!("{} ", i);
    }
    println!();

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

mod a {
    pub mod a_b {
        pub fn hello() {
            println!("Hi!");
        }
    }
}

pub use a::a_b;

pub fn exercise11() {
    a_b::hello();
}

fn exercise10() {
    let some_value = Some("");
}

fn exercise9() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn exercise8() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn exercise7() {
    enum SimpleIpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn route(ip_kind: IpAddrKind) {}

fn exercise6() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
}

fn exercise5() {
    let user = build_user(String::from("email"), String::from("Username"));

    // use debug trait
    println!("{:?}", user);
    println!("{:#?}", user);
    dbg!(&user); // IMPORTANT: takes ownership so use refs

    #[derive(Debug)]
    struct D;
    // compiles cuz it evaluates to struct literal:
    dbg!(D);
    // so this is basically equal to:
    let d = D;
    dbg!(d);

    // doesn't compile cuz to evaluate to struct literal it would
    // need values of it's fields
    // dbg!(user::User);
}

fn exercise4() {
    struct AlwaysEqual;
    let strategy = AlwaysEqual;
    let str = strategy;
}

fn exercise3() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn exercise2() {
    let user = user::User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let built_user = build_user(String::from("a@a.com"), String::from("a"));
    println!("build_user: {}", built_user.username);

    let b_user = user::User {
        email: String::from("b@b.com"),
        ..user
    };

    // after all this why does this compile:
    // println!("{}", user.active);

    // but this not?
    // println!("{}", user.username);
    // wouldn't it make more sense to make the whole user ref
    // invalid if we already moved some of its fields?

    // so according to the documentation, everything that implements
    // copy will be copied, all the rest is moved making the
    // old reference invalid
}

fn build_user(email: String, username: String) -> user::User {
    user::User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn exercise1() {
    println!("{}", first_word(&"Hello owning!"));
    println!("{}", first_word(&"Owning!"));

    println!("{}", first_word(&"Hello slice!"));
    println!("{}", first_word(&"Slice!"));
}

fn first_word(s: &str) -> &str {
    for (i, char) in s.chars().enumerate() {
        if char == ' ' {
            return &s[..i];
        }
    }

    &s
}

fn exercise0() {
    println!("Hello fellow... ?");
    println!("{}", {
        let mut name = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Couldn't read line");

        format!("Ah! {}, was it?", name.trim())
    });
}
