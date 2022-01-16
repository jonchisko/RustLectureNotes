# References and lifetimes
In this video we will examine Rust references (they are non owning pointer types) and their lifetimes. This is called borrowing a value from an owner,
remember previous video where we talked about ownership and move semantics. Borrowing is another way to help us
flex the rigid tree of ownership.

## References
We have two types of references, an immutable reference and a mutable one. As the name suggests - we cannot mutate data via the first one, 
while we can through a mutable reference. 
Information to keep in mind:

1. We can have as many immutable references as we want.
2. We can only have one mutable reference.
3. We cannot use a mutable reference, if we also have an immutable reference (to the same data).
4. The reference needs to be alive as long as the variable that uses it (ensuring the variable is not pointing to null), but it also needs to be encapsaulted by the lifetime of the data that the ref is refering to.

### Examples

Let's take a look at the following example:
```rust
type Works = Vec<String>;

struct Author {
    name: String,
    works: Works,
}

fn show(author: Author) {
    println!("Author: {}", author.name);
    for work in author.works {
        print!("{} ", work);
    }
    println!();
}

fn main() {
    let imaginary_author = Author {
        name: "Neobstojec".to_string(), 
        works: vec!["Work1".to_string(), "Work2".to_string()]
    };

    show(imaginary_author); // author is moved into function's argument author
    println!("Author's name: {}", imaginary_author.name); // uninitialized, cannot be used
}
```

The above conondrum can be solved by an immutable reference, let's take a look:
```rust
show(&imaginary_author); // add immutable ref here, so we BORROW

fn show(author: &Author) { // add immutable ref here, the fun borrows!
    println!("Author: {}", author.name);
    for work in &author.works { // add imm. ref here, otherwise we move works!
        print!("{} ", work);
    }
    println!();
}
```

Now the `show` function borrows our `Author` struct instead of taking ownership. We had to add `&` in function defenition and in function call. We also had to borrow in the for loop, since for loop implicitly calls `into_iter` on vectors, which takes ownership. This would mean, we would move our works out of struct and that would let struct be only partially initialized. Thus we borrow in the for loop.

What if we wanted to change the works of author in a function? We would take ownership and return the struct (btw, we could do that with show function as well) or we would take a mutable reference and change the works.

```rust
// code in main
    let img_author1 = Author {
        name: "Neobstojec".to_string(), 
        works: vec!["Work1".to_string(), "Work2".to_string()]
    };

    // we dont need img_author1 mutable above, because the fn takes ownership
    // the argument is declared as mutable, so we can change it in the fun
    let mut img_author1 = add_work1(img_author1, "Work3".to_string());
    show(&img_author1);

    // here we need to declare img_author1 as mut ^^^^^^ to be allowed to have a mut ref
    add_work2(&mut img_author1, "Work4".to_string());
    show(&img_author1);

    add_work3(&mut img_author1, "Work5"); // already a slice
    show(&img_author1);

    add_work3(&mut img_author1, &String::from("Work6")); // works with strings as well
    show(&img_author1);

// end code in main

fn add_work1(mut author: Author, work: String) -> Author {
    author.works.push(work);
    author
}

fn add_work2(author: &mut Author, work: String) {
    author.works.push(work);
}

fn add_work3(author: &mut Author, work: &str) {
    author.works.push(work.to_string());
}
```

Remember how we said, you can have multiple immutable refs to a certain variable? What happens if we move or change the value, while we are borrowing?

```rust
    // ex1
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    x = 2; // err
    println!("{} {}", r1, r2);

    // <----------------->
    // ex2
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    println!("{} {}", r1, r2);
    x = 2; // all good

    // <----------------->
    // ex3
        let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    let r3 = &mut x; // problematic, because immutable refs used later, their lifetimes
    // clash with mut borrow

    println!("{} {}", r1, r2);
    x = 2;

    // <----------------->
    // ex4
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;

    println!("{} {}", r1, r2);

    let r3 = &mut x; // this is alright
    //println!("{}", x); // if you uncomment this, you are immut borrowing while mut borrowed
    *r3 *= 10; 
    println!("{}", x);
```

Useful thing about references is, that they can get automatically derefed (and borrowed - `vector.sort()`) when using `.` operator (also comparison operators automatically deref, but these have to have equal number of ref levels, you can use std::ptr::eq to check if two refs have the same mem addr).

```rust
    // ex1
    let x = 10;
    let y = 20;

    let r1 = &&&x;
    let r2 = &&&x;

    assert!(r1 == r2);

    let r3 = &&x;
    assert!(r1 == r3); // problem

    // ex2
    let mut v = vec![1, 2, 3];
    let mut mutref = &mut v;
    let mutrefmutref = &mut mutref;

    mutrefmutref.push(20);
    println!("{:?}", mutrefmutref);

    // ex3 reborrow
    let mut a = (10, 20);
    let ref1 = &mut a;
    let ref2 = &mut ref1.0;

    println!("{}", ref2);
    println!("{}", ref1.0);
```

## Lifetimes
Ok let's now focus on lifetimes. These are very important since they ensure at compile time that we ain't derefing a dangling ref. It will be best to learn them by example, so I am just going to dive straight in.

### Examples

```rust
    // this is ok, ref of x must be encapsulated by x's lifetime, the compiler chooses the smallest lifetime here
    // ref of x lifetime also needs to encapsulate the lifetime of r, so that we do not end up with a dangling ref.
    let r;
    {
        let x = 10;
        r = &x; // since we dont use r later on, our ref x is valid here in this line
    }

    // ex2, not correct
    /*
    In this example, ref of x is valid on 197 and 198,
    but it should also ve valid at 200 to prevent dangling ref
    */
    let r;
    {
        let x = 10;
        r = &x;
    }
    println!("{}", *r);
```

Read through the comments in the code. In short, we have certain value, that is alive for lifetime 'a. The reference must have a lifetime of 'b, which is entirely encapsulated by that 'a. But it also must encapsulate the lifetime of 'c of the variable that holds the reference (`r` in our example above). If these requests lead to a contradiction ("paradox"), the compiler refuses to compile the code.

```rust
// ex1
static mut NAME: &str = "HERO";

fn main() {

    let val = "LoL";
    change_name(val);
}

fn change_name(value: &'static str) {
    unsafe { // because having a global mutable is not (thread) safe
        NAME = value;
    }
}

// ex2
static mut NAME: &str = "HERO";

fn main() {

    let val = String::from("LoL"); // does not compile, val doesnt hav static lifetime
    change_name(&val);
}

fn change_name(value: &'static str) {
    unsafe {
        NAME = value;
    }
}

// show this fun
fn change_name<'a>(value: &'a str) {
    unsafe {
        NAME = value; // does not work, cuz NAME has static, which outlives 'a
    }
}
```

So in the above examples we can observe lifetimes in action. Whenever we have references, we have lifetimes asigned to them, but Rust is good at infering them, so the user does not need to specify them unless he or she wants to. Ex1 is a working example. Ex2 does not work, since our value `val` does not live for the entire program's lifetime - it doesn't have a `'static` lifetime. So basically our function `change_name` can only accept values that have a `'static` lifetime. 

The last function doesn't work either, because `'a` can be any lifetime, but in this case it must be `'static`.

In the bellow example, you could change the lifetimes in S, to make them independent. The code does not work, because lifetime `'a` is extended to encapsulate `r`. Unfortunately that conflicts with `b`, which does not live long enough. If we define two lifetimes in struct S, everything is alright (or if we use r in the smaller scope).
```rust
fn main() {

    let a = 10;
    let r;
    {
        let b = 20;
        let s = S{x: &a, y: &b};
        r = &s.x;
    }
    println!("{}", r);
}

struct D<'a> {
    e: S<'a>,
}

struct S<'a> {
    x: &'a i32,
    y: &'a i32,
}
```

And the last example before we call it a day:
```rust

fn main() {
    let mut v1 = vec![1,2,3];
    let v2 = vec![2,3,4];
    extend_vec(&mut v1, &v2);
    println!("{:?}", v1);

    extend_vec(&mut v1, &v1); // borrowing as immutable while mutably borrowed
    // what if v1 would need to reallocate????
}

fn get_second(v: &[u32]) -> &u32 {
    &v[1]
}

fn get_second2<'a>(v: &'a [u32]) -> &'a u32 {
    &v[1]
}

fn get_reference<'a>(a: &'a i32, b: &'a i32, _c: &i32) -> &'a i32 {
    b
}

fn extend_vec(v: &mut Vec<u32>, ex: &[u32]) {
    v.extend(ex);
}
```