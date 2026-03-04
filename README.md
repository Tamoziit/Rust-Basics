# Rust-Basics
Rust Yaayyyyyyyyy

## Theory
- Rust is an **ahead-of-time** compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. (Unlike py, js, etc.)
- **Cargo**: Rust’s build system and package manager. Cargo handles a lot of tasks, such as building the code, downloading the libraries which the code depends on, and building those libraries. [Analogous: npm]
- **Cargo.toml**: Package config --> like package.json
- **cargo CMDs**:

```bash
cargo build  # Builds a debug build for development
```
```bash
cargo build --release # For production release/build
```
```bash
cargo check # Compiles the Rust code, but doesn't build an executable --> faster to check for a safe build in dev, rather than building the executable at each step
```

- **stdin**: The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for our terminal.
- **read_line**: The full job of read_line is to take whatever the user types into standard input and *append* that into a string (*without overwriting* its contents). eg, if <br />
String s = "guess"
--> read_line(guess) --> for i/p = "game"<br />
--> final s = "guessgame"<br />
- Variables in Rust are by default **immutable**.
- They can be made mutable with the keyword **mut**.
- The `::` syntax in the `::new` line indicates that **new is an associated function of the String type**. An associated function is a function that’s **implemented on a type**, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
- Strings in Rust are **growable & UTF-8 encoded**.
- read_line() return a **`Result`** value. Result is an **enumeration**, often called an **enum**, which is a type that can be in one of multiple possible states. We call each possible state a **variant**.
- Result’s variants are **Ok** and **Err**. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
- A **crate** is a collection of Rust source code files. Normal Rust project [generated through cargo new ...] is a **binary crate**, which is an executable.
- The **rand crate** [for random numbers] is a **library crate**, which contains code that is intended to be used in other programs and can’t be executed on its own.

## Datatypes in Rust
- Rust is **statically-typed at runtime**; their `<DType>` cannot be change at runtime.
- **Unsigned integers**: Only +ve integers
- **Signed integers**: Both +ve & -ve integers (in 2's complement representation).
- **usize, isize**: variables assumes unsigned/signed integers of bits compatible with the system arch (x8, x16, x32, x64...).
- `let a = 254u8; // a gets u8 type`
- `let a = 10_00_000; // value remains 1000000, but better visualized as 10,00,000`
- **0x...** - Hex Representation; **0o...** - Octal, **0b...** - Binary, **b'...'** - u8 Byte.
- All **Floating point** nos. are **signed**.

## Control Flow
- In Rust `if` condition needs **bool** condition --> **truthy/falsy  values not valid**.
eg: `if x == 0 { ... } // valid`
<br />but,<br />
`let x = 3;
if x { ... } // Not valid`

## Ownership in Rust
**Ownership** is a **set of rules** that govern how a **Rust program manages memory**.<br />
All programs have to manage the way they use a computer’s memory while running.
- Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs (Java);
- in other languages, the programmer must explicitly allocate and free the memory (C/C++).

Rust uses a third approach: **Memory is managed through a system of ownership with a set of rules that the compiler checks**. *If any of the rules are violated, the program won’t compile*. None of the features of ownership will slow down your program while it’s running.

**Ownership Rules in Rust**:
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

**Slice Type**: Slices let us **reference a contiguous sequence of elements** in a collection. A slice is a kind of reference, so it *does not have ownership*.