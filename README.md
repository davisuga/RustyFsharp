# Requirements
.NET 6.0+
Rust Compiler
Cargo
Fable

# Compiling
## Generating Rust code
fable --lang Rust RustyFsharp.fsx; mv RustyFsharp.fs.rs src/lib.rs

## Compiling the Rust Code
cargo build