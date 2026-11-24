# rregex

[![License](https://shields.io)](LICENSE)

A zero-dependency Rust library to recognize regular expressions defined inductively. 

Regular expressions may be defined inductively over an alphabet using atoms and operations. Atoms are the smallest form of a regular expression, and may be the empty set, the set containing the empty string, or the set containing some single symbol from the alphabet. Operations create new regular expressions by combining smaller regular expressions. Operations include the union of two regular expressions (R | S), the concatonation of two regular expressions (R . S), and the Kleene star of a single regular expression (R*). We may call this process inductive construction because each next regular expression builds on the previous ones. This also resembles the base case and recursive case found in recursion.

A given regular expression recognizes a given string if and only if that string is included in the set representing the language of the regular expression, which is built by combining atoms and operations. These 3 atoms and 3 operations are sufficient to build a regular expression to recognize any regular language.

Since regular expressions hold the same expressive power as finite automaton (finite automaton can recognize any regular language, just like regular expressions can, but no more), any regular expression can be mapped to a finite automaton which represents the same language, and vice versa. The graph structure and flexibility of a non-deterministic finite automaton (NFA) is ideal for representing regular expressions built inductively and allows for easy traversal to determine whether a given string is recognized, so this is how an `Rregex` struct is internally represented.

> **Note:** This library has not yet been published to `crates.io`.

## Installation

Add `rregex` directly from GitHub using Cargo:

```bash
cargo add --git https://github.com/reichlerba/Rregex
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
rregex = { git = "https://github.com/reichlerba/Rregex" }

# Optional: pin to a specific branch or commit
# rregex = { git = "https://github.com/reichlerba/Rregex", branch = "main" }
```

## Usage

Import the library into your Rust project.

Create a new atom by calling one of the following constructors:
- `Rregex::get_empty_set()`
    - To retrieve the regular expression which recognizes nothing.
- `Rregex::get_epsilon()`
    - To retrieve the regular expression which recognizes only the empty string.
- `Rregex::get_symbol(c: char)`
    - To retrieve the regular expression which recognizes only the length-1 string `c.to_string()`.

Combine regular expressions to create a new `Rregex` using one of the following operations:
- `Rregex::union(r: &Rregex, s: &Rregex)`
    - To retrieve the regular expression which recognizes all strings recognized by `r` and all strings recognizes by `s`.
- `Rregex::concat(r: &Rregex, s: &Rregex)`
    - To retrieve the regular expression which recognizes all strings found by concatonating a string recognized by `s` to the end of a string recognized by `r`.
- `Rregex::star(r: &Rregex)`
    - To retrieve the regular expression which recognizes the Kleene star of `r`, which includes strings made up of any number of strings recognized by `r` concatonated together, including the empty string.

Use `Rregex.recognizes(s: &str) -> bool` to determine whether a given `Rregex` recognizes `s`.

```rust
use rregex::Rregex;

fn main() {

    let mut digit = Rregex::get_empty_set();
    for c in '0'..='9' {
        let c = Rregex::get_symbol(c);
        digit = Rregex::union(&digit, &c);
    }
    // num recognizes a string of any single digit, from 0 to 9.

    let mut nonzero_digit = Rregex::get_empty_set();
    for c in '1'..='9' {
        let c = Rregex::get_symbol(c);
        nonzero_digit = Rregex::union(&nonzero_digit, &c);
    }
    // nonzero_digit recognizes a string of any single positive digit, from 1 to 9.

    let zero = Rregex::get_symbol('0');
    // zero only recognizes the string "0".

    let digit_star = Rregex::star(&digit);
    // digit_star recognizes strings like: "", "00000700", "12", "1337", etc.

    let natural_num = Rregex::concat(&nonzero_digit, &digit_star);
    // natural_num recognizes a string of any natural number (no decimal point and greater than or equal to 1) without leading 0s.

    let whole_num = Rregex::union(&zero, &natural_num);
    // whole_num recognizes a string of any whole number (no decimal point and greater than or equal to 0) without leading 0s (except for "0").

    let integer = Rregex::concat(
        &Rregex::union(
            &Rregex::get_symbol('-'),
            &Rregex::get_epsilon()
        ),
        &whole_num
    );
    // integer recognizes a string of any integer number (no decimal point) without leading 0s (after the '-' if there is one, and not including "0" and "-0").

    assert!(integer.recognizes("1"));
    assert!(integer.recognizes("1234"));
    assert!(integer.recognizes("-9999999"));
    assert!(integer.recognizes("0"));
    assert!(integer.recognizes("-0"));

    assert!(!integer.recognizes("+1"));
    assert!(!integer.recognizes("0x7"));
    assert!(!integer.recognizes("015"));
    assert!(!integer.recognizes("2^10"));

}
```

## Documentation

Since this crate isn't on `docs.rs` yet, you can build and view the full API documentation locally:

```bash
git clone https://github.com/reichlerba/Rregex
cd rregex
cargo doc --open
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT))
