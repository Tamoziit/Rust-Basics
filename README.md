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