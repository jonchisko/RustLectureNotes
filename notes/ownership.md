# Ownership and moves

If you have been following the news, you have probably already heard of Rust numerous times, thus you know that it has this peculiar way
of dealing with memory allocation and deallocation. Let us dive in and see what all the fuzz is about.

In general we have two ways of dealing with memory allocations in our programs. One and the most convenient way is using garbage collector in languages such as Python, Java, Ruby ... The other is manual management, which C and C++ use.
Rust tries to combine both the safety approach of the GC languages and still allow the programer similar control as C and C++ provide. Thus it has some rules that are checked at compile time to ensure we have no memory related errors: dangling pointers, use after free, double free and uninitialized memory.

## Ownership

The concept of ownership is a simple one. The variable that points or has been assigned a certain value, owns that value. For instance in C++:
```c++
std::string name = "Adnan Mijovic";
```
The variable `name` owns the string value. Other variables might point to it at some time during its lifetime, but the `name` is responsible for releasing the allocated memory that the string holds. The references or pointers that refer/point to the `name` after it has already been freed should not been used and this is up to the programer to uphold. 

In Rust though, such things are checked at compile time, so programer is actually forbidden to use a reference to a dropped value!

In rust a variable (structs own their fields, arrays, tuples, vectors own their elements) owns the data and when the variables goes out of scope, it also frees (or in Rust jargon, drops) the data (buffer that it holds). For example if we have a vector of strings in Rust. That would in memory look like (show img). We could also have a vector of boxxed values of strings, which would look like this. In a sense it starts to look like an ownership tree and when the original value goes out of scope, the whole thing is dropped (freed).

There are certain additional rules in Rust, which make the whole ownership model more flexible:

1. variables can be moved to other variables,
2. we can take references instead of ownership,
3. simple types implement copy trait,
4. standard lib provides reference counted types (Rc, Arc ...).

## Moves

Let's make a quick detour and check how assigning is done in Python and C++ (just two simple cases to get a feel for two different approaches).

Python:
```python
t = ["Robi", "Miha", "Cilka"]
u = t
s = t
```

In python, t basically points to python list object, which also holds a ref counter (besides capacity and length). This python list object points to the buffer which points to specific python elements (py ascii object, which hold similar data as the py object list). Setting `u, s` to `t` is very simple and quick, it just updates the ref counter in the py list object.

C++:
```c++
std::vector<string> t = {"Robi", "Miha", "Cilka"};
std::vector<string> u = t;
std::vector<string> s = t;
``` 
Uses a copy constructor and thus creates a deep copy of the original vector of strings. Thus `t, u, s` are all their own vectors of strings and get freed after they leave scope.

Rust:
```rust
let t = vec!["Robi".to_string(), "Miha".to_string(), "Cilka".to_string()];
let u = t;
let s = t; // use of a moved value, t is now uninitialized, u owns the data

// deep copy approach
let u = t.clone();
let s = t.clone();

// not yet discussed, ref approach
let u = &t;
let s = &t;
```
We have used `to_string()` because otherwise `"Robi"` is just a string literal, which is put into read-only mem and we want to stay as close to the C++ example as possible. The last approach takes an immutable reference to the `t` value.

Anyway, the approach Rust takes, is simple and fast as Python's, but still the ownership responsibility remains clear. Variable `u` is now responsible for the allocated data. (We could also mimic the Python approach more by using the Rc pointer wrapper types.)



# Sources
Blandy, J., Orendorff, J., &amp; S., T. L. F. (2021). Programming rust: Fast, safe systems development. O'Reilly. 

[Box type, Rust (doc) book](https://doc.rust-lang.org/book/ch15-01-box.html)