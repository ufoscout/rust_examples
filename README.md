# rust_examples
Having fun with the Rust programming language

## Rust runtime dependencies
Rust depends on glibc at runtime (ouch!!). This is weird, for example, to run docker executables in docker, glibc has to be installed; it is installed by default in Ubuntu but not present in smaller images like Alpine.

Luckily, Rust code can be compiled with statically linked musl insted of gclib. This makes the executables extremely portable; they can even run with the docker scratch base!

## Compile using musl
It is required to use rustup (https://www.rustup.rs/) and to add a specific target:
- Install musl specific target:
  $> rustup target install x86_64-unknown-linux-musl
- then build using the musl target:
  cargo build --release --target=x86_64-unknown-linux-musl

## Strip the executable to reduce its size
To strip unused library methods and further reduce the size of the binary file, the "strip" utility can be used:
strip target/x86_64-unknown-linux-musl/release/executable_name
