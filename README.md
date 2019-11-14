### Rust book notes by Alan

#### Pre-chapters

I'm using

https://atom.io/packages/ide-rust

I like it a lot. Coming from python, I know that a lot of the tips
it gives wouldn't even be theoretically possible with it.
So right away, the statically typed dividend is obvious.

When I first tried this I ran into a lot of Atom crashing issues,
but with updated versions I'm feeling more confident these days.

It still doesn't seem to have an option to run stuff.

#### Chapter 1

The weird thing going on here is that I made a workspace
where I am combining the sub-projects. So instead of

```
rustc main.rs
```

run

```
cargo run hello-rust/src/main.rs
```

This seems to work, but leaves open a lot of questions about
what it means to compile and what it means to run.

```
cargo test
```

also works as of this point in time.

#### Chapter 2

current stopping point

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

how to invoke:

```
$ cargo run --bin hello-rust
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/hello-rust`
----------------------------
| Hello follow rusticians  |
----------------------------
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
$ cargo run --bin guessing_game
   Compiling guessing_game v0.1.0 (/Users/alancoding/Documents/repos/rust/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Hello, world!
```

It will error now if you run with just `cargo run`.

I should probably figure out sometime how to set a "main main".
