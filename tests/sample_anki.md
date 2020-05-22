## [Rust, udemy] What is the problem with this code? How to fix it?

```rust
struct Person {
    name: &str,
}

impl Person {
    fn talk(&self) {
        println!("Hi my name is {}", self.name)
    }
}

fn main() {
    let person = Person { name: "niko" };
}
```

---

* Will throw missing lifetime specifier since Person might outlive name
* Fix with lifetime specifier


```rust
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi my name is {}", self.name)
    }
}
```


## [Rust, udemy] What is reference counting? How to figure out how  many references exist? When does it not work?


> A convenient way to share around a single variable in many parts of your code without worrying about borrowing

* A way to have multiple immutable pointers to a reference type var (e.g.: string)
* Figure out with Rc::strong_count(&name)
* Only works with single thread


```rust
use std::rc::Rc;

struct Person {
    name: Rc<String>,
}
impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("I am {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("Niko".to_string());
    let person = Person::new(name.clone());

    person.greet();

    println!("{}", name);
}
fn main() {
    rc_demo();
}

```


## [Rust, udemy] How to do reference counting in multiple threads?

By using Arc instead of Rc


```rust
use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>,
}
impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("I am {}", self.name);
    }
}

fn rc_demo() {
    let name = Arc::new("Niko".to_string());

    let person = Person::new(name.clone());
    person.greet();

    println!("{}", name);

    let t = thread::spawn(move || {
        person.greet();
    });
}
fn main() {
    rc_demo();
}

```


## [Rust, udemy] What is a mutex?

> mutual exclusion


* In computer science, mutual exclusion is a property of concurrency control, which is instituted for the purpose of preventing race conditions. 
* It is the requirement that one thread of execution never enters its critical section at the same time that another concurrent thread of execution enters its own critical section, which refers to an interval of time during which a thread of execution accesses a shared resource, such as shared memory. 

* it means that threads that threads are mutually excluded to modify a var until explicitly allowed


## [Rust, udemy] What do you need to do if you want to pass a var to multiple threads an modify it?

* Use Arc and Mutex


```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}
impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person {
            name: name,
            state: state,
        }
    }

    fn greet(&self) {
        println!("I am {}", self.name);

        let mut state = self.state.lock().unwrap();

        state.clear();
        state.push_str("excited");
    }
}

fn rc_demo() {
    let name = Arc::new("Niko".to_string());
    let state = Arc::new(Mutex::new("Okay".to_string()));

    let person = Person::new(name.clone(), state.clone());
    person.greet();

    println!("{}, state: {}", name, state.lock().unwrap().as_str());

    let t = thread::spawn(move || {
        person.greet();
    });
}
fn main() {
    rc_demo();
}

```



