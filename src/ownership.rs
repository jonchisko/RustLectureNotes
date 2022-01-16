use std::{rc::Rc};



pub fn ownership_example1() {
    let names = vec!["Miha".to_string(), "Tilka".to_string(), "Robi".to_string()];
    for name in &names { //names is moved (consumed) here, cannot be used anymore
        println!("Name {}", name);
    }
    // uncomment to get compiler error
    println!("{:?}", names);
    // OR if we take a reference in the loop, names is dropped here
}

pub fn ownership_example2() {
    let boxxed_value = Box::new((10, 20));
    println!("{}, {}", boxxed_value.0, boxxed_value.1);
} // boxxed_value is dropped here, freeing the tuple stored on the heap

pub fn ownership_example3() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    //let v2 = v1;

    // borrow of moved value, we want to use v1 after it was MOVED to v2
    println!("{:?}", v1);
    //println!("{:?}", v2);
}

pub fn ownership_example4() {
    let g = (1, 2);
    let a = 109;
    doesnt_take_ownership(a); // a is copied
    let b = a + a; // a can still be used
    println!("{}", b);
    test(g);
    println!("{}", g.0);
}

fn test(value: (i32, i32)) {

}

fn doesnt_take_ownership(value: i32) {
    println!("Inside of doesnt_take_ownership {}", value);
} // value dropped

#[derive(Debug)]
struct Author {
    name: String,
    age: u32,
}

pub fn ownership_example5() {
    let mut _name = "Marcus".to_string();
    _name = "Victus".to_string(); // value marcus is dropped

    let mut name = "Marcus".to_string();
    let t = name;
    name = "Edvard".to_string(); // value Marcus is not dropped here, it was moved to t

    println!("{}", t);
    println!("{}", name);

    let mut authors = vec![Author {name: "Mikhail".to_string(), age: 54},
                                  Author {name: "Jane".to_string(), age: 20},];
    //let author = authors.pop();

    println!("{:?}", authors);
}

#[derive(Debug)]
struct Author2 {
    name: Option<String>,
    age: u32,
}

pub fn ownership_example6() {

    let mut v = vec![Author2 {name: Some("Zen".to_string()), age: 19},
                            Author2 {name: Some("Zan".to_string()), age: 25},
                            Author2 {name: Some("Jen".to_string()), age: 55},
                            ];

    //let second_ele = v[1];
    //let second_ele = std::mem::replace(&mut v[1].name, None);
    //println!("{:?}", v);

    //let second_ele = v[1].name.take();
    //println!("{:?}", v);
}

pub fn ownership_example7() {
    let v = vec![Box::new("Dan".to_string()), Box::new("Hera".to_string()), Box::new("Zoe".to_string())];
}

pub fn ownership_example8() {
    // Rc example
    let rc_string = Rc::new("David".to_string());
    let c = rc_string.clone();
    let _d = rc_string.clone();

    println!("{}", c);
    println!("{}", rc_string);
}



