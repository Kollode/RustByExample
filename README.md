# About

Here are the different **[rust by example](https://rustbyexample.com)** tasks. Each chapter has its own directory. Every directory has it own Cargo.toml and can be run with `cargo run`.

Sometimes my implementation differs from the example. That could happen because I wanted to test some language features or I was bored and needed an excuse to stroke my ego :)

To make everything runnable with cargo, the examples are named **main.rs** and not for example **hello.rs**. I think it is possible to use **hello.rs** as the default runnable. But I did not dive deep enough in cargo yet.

# Setup 

## Base

* Install rustup: https://www.rust-lang.org/en-US/install.html
* Do a `rustup update` to make sure the current stable version is installed. The Windows version for example was 1.17, even if 1.22 was available.
* On Linux I needed to add the cargo path env to my .zshrc, because the path was only added to the .bash_profile.

## Editor (VS Code)

* For now **Rust (rls)** worked best for me. It will install everything needed with rustup by it self. Yes, it will ask for permission.
* I also installed **Better TOML** to get syntax highlighting in the Cargo.toml files

**Note:** The rust extension needs the **Rust Language Server (RLS)** to work correctly. This works only if the project is a cargo project.

# Useful and interesting links

* "The Rust Programming Language" book: https://doc.rust-lang.org/book/
* "Rust by Example": https://rustbyexample.com
* Interesting list of rust links: https://github.com/rust-unofficial/awesome-rust

# Thinks I want to take a look at

* Cargo features:
  * workspaces: Sound a bit like lerna, mono repo?
  * examples: Demo code than can be executed?