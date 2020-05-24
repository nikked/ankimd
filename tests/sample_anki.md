## [Rust, udemy] What is the problem with this code? How to fix it?

```rust

fn main() {
    println!("Hello from Rust!"); 
}
```

---

Text outside bullets
* Will throw missing lifetime specifier since Person might outlive name
* Fix with lifetime specifier
* third bullet
More text out of bullets

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

