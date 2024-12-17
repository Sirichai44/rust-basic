#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

// use rust_basic::customer::Customer;
// use rust_basic::person::Person;
use rust_basic::{customer::Customer, person::Person, speaking::Speaking};
use std::collections::HashMap;

fn main() {
    // print!("Hello, world!");
    // variables
    let mut x: i32;
    x = 10;

    let x = 5;
    let (x, y) = (1, 2);

    const PI: f32 = 3.14;

    // tuple
    let x: (u8, f64, i32) = (1, 3.14, 1000);
    let a = x.0;
    let b = x.1;
    let c = x.2;

    // array
    let x: [i32; 5];
    x = [1, 2, 3, 4, 5];
    let x = [0; 5];

    // if
    let score = 50;
    let mut grade = "";

    if score >= 80 {
        grade = "A";
    } else if score >= 70 {
        grade = "B";
    } else if score >= 60 {
        grade = "C";
    } else if score >= 50 {
        grade = "D";
    } else {
        grade = "F";
    }

    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    };

    // result = score >= 50 ? "Pass" : "Fail";
    let result = if score >= 50 { "Pass" } else { "Fail" };

    // loop
    while true {
        break;
    }

    'label1: loop {
        'label2: loop {
            break 'label1;
            continue 'label2;
        }
    }

    // for 0-9
    for i in 0..10 {
        println!("{}", i);
    }

    // for 0-10
    for i in 0..=10 {
        println!("{}", i);
    }

    let numbers = [10, 20, 30];
    for n in numbers.iter() {
        println!("{}", n);
    }

    for n in [10, 20, 30].iter() {
        println!("{}", n);
    }

    let numbers = [(1, 2), (3, 4), (5, 6)];
    for (i, j) in numbers.iter() {
        println!("{} {}", i, j);
    }

    // String
    let s = "Hello, world!";
    let s = String::from("Hello, world!");
    let s = "Hello, ".to_string();

    // Collection
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    let y = v.pop();

    let mut v = vec![10, 20, 30];

    // HashMap
    let mut x: HashMap<&str, &str> = HashMap::new();
    x.insert("th", "Thailand");
    x.insert("jp", "Japan");
    let y = x.get("th");
    println!("{}", y.unwrap());

    // Struct
    let p = Person::new("John".to_string(), 20);
    p.hello();

    // Traits
    p.speak();

    // Enum
    let x = Color::Red;
    let mut color = "";
    match x {
        // switch
        Color::Red => {
            color = "Red";
            println!("Red");
        }
        Color::Green => println!("Green"),
        // Color::Blue => println!("Blue"),
        _ => println!("Blue"), // else or default
    }

    let color = match x {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
    };

    let x = check_grade(80);
    match x {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(v) => println!("{}", v),
    }

    let x = check_grade2(60);
    match x {
        None => println!("Invalid score"),
        Some(v) => println!("{}", v),
    }

    let x = check_grade3(40);
    match x {
        Err(e) => println!("{}", e),
        Ok(v) => println!("{}", v),
    }

    let x = check_grade3(70);
    // let y = x.unwrap(); // panic if error

    // if x.is_err() {
    //     println!("Error");
    // }

    // if let Ok(v) = x {
    //     println!("{}", v);
    // }

    let y = match x {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    // Closure or Lambda or Anonymous function
    let x = add(10, 20); // normal function

    let x = |a: i32, b: i32| a + b; // closure
    let y = x(10, 20);

    let y = cal(10, 20, x);
    let y = cal(10, 20, |a, b| a + b);
    let y = cal(10, 20, add);
}

fn cal<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f: F) -> i32 {
    f(a, b)
}

fn cal2<F>(a: i32, b: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
enum Color {
    Red,
    Green,
    Blue,
}

fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("Invalid score".to_string());
    }

    if score >= 80 {
        GradeResult::Value("A".to_string())
    } else if score >= 70 {
        GradeResult::Value("B".to_string())
    } else if score >= 60 {
        GradeResult::Value("C".to_string())
    } else if score >= 50 {
        GradeResult::Value("D".to_string())
    } else {
        GradeResult::Value("F".to_string())
    }
}

fn check_grade2(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }

    if score >= 80 {
        Some("A".to_string())
    } else if score >= 70 {
        Some("B".to_string())
    } else if score >= 60 {
        Some("C".to_string())
    } else if score >= 50 {
        Some("D".to_string())
    } else {
        Some("F".to_string())
    }
}

fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Invalid score".to_string());
    }

    if score >= 80 {
        Ok("A".to_string())
    } else if score >= 70 {
        Ok("B".to_string())
    } else if score >= 60 {
        Ok("C".to_string())
    } else if score >= 50 {
        Ok("D".to_string())
    } else {
        Ok("F".to_string())
    }
}

enum GradeResult {
    Value(String),
    Error(String),
}

fn get_number() -> i32 {
    let a = 10;
    let b = 20;

    a + b
    // return 10;
}
