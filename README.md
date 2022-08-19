rlife is a small naive implementation of Conway's Game of Life written in
[Rust](https://www.rust-lang.org). The project was made to be simple, something one could write in
a day, but also make use of a wide variety of features of the Rust programming language and it's
standard library. Because of this goal the program uses a minimal amount of dependencies, only the
bare minimum to get a window up and running on multiple platforms. The features used in this
project include: Modules, Traits, Generics, const Generics and many more.

## Build instructions

### Installing Rust
Install rust by following the instructions [here](https://www.rust-lang.org/tools/install)

### Dependencies
rlife uses minifb to create a simple window, on linux minifb needs a few dependencies:
```
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```
Command may vary depending on the linux distribution

### Building
```
cargo build --release
```

### Running
```
cargo run --release
```
