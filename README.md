# ankimd: The opinionated Anki-card maker

> Write Anki cards in Markdown and import to Anki as csv file

Everyday I write notes to a file called anki.md and convert this markdown to Anki cards. This is achieving by first converting the anki.md to html and then making a Anki importable csv.

This app is highly coupled to my personal workflow. I have done over 2000 Anki cards with the apps previous reincarnation (written in Python). It saves me a lot of time when I can just use my customized markdown and not use Anki's own sluggish editor. 


## Examples


Create a file `anki.md`. You are going to write your notes on these file. Here is an example of a simple card written in markdown:

```markdown

## Card front
Card back 

## [card_tag1, card_tag2] Card front
Card back 

## [Dutch] Do you work in a restaurant?
Werk je in een restaurant?

## [Rust, udemy] How to make a multiline comment in rust?

With `/*` and `*/`:

```rust
/*
a multiline
comment
*/
\```

```


By default, the Card front is a one-liner. It starts with two hashes and is followed by a tag array. However with the separator `---` you can have multiline card fronts:

```markdown
## [card_tag1, card_tag2] Card front 

```rust

fn main() {
    println!("Hello from Rust!"); 
}
\```

Last line of card front
---

Here starts card back
* first bullet
* second 
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
\```
Last line of card back
```

## TODO



### Card types
* Add card type support
* Remove BAS, REV, CLO from tag literal in card front

### Add syntax highlighting
https://github.com/cobalt-org/cobalt.rs/blob/master/src/syntax_highlight/syntect.rs

### CLI
* Enabling adding tag vector as CLI arg (default: [anki-rust])

### Hygiene
* Investigate why 'let matched_string' is allowed twice
* Add type defs to all vars (?)

