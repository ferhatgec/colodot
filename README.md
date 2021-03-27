# Colo[dot]rs
Easy-to-use, powerful Rust implementation of Colorized library.

```rust
colodot!(DotTypes::Bold, DotColors::Red,   "Hello, red!\n",   true);
colodot!(DotTypes::Light,DotColors::Green, "Hello, green!\n", true);
colodot!(DotTypes::Blink,DotColors::Blue,  "Hello, blue!\n",  true);

red()  ; println!("{}", "Hello, red!"  );
green(); println!("{}", "Hello, green!");
blue() ; println!("{}", "Hello, blue!" );

colodot!(colodot::dot::DotTrueColor{r: 255, g: 255, b: 255}, "Hello, world!\n");
```

### How to use?
Add this code-block to your ``Cargo.toml``

```toml
[dependencies]
colodot = "0.1.2"
```

and *Do not forget* to add this to your ``main.rs`` or ``lib.rs``

```rust
#[macro_use]
extern crate colodot;

use colodot::dot::*;

fn main() {
    colodot!(colodot::dot::DotTrueColor{r: 255, g: 255, b: 255}, "Huhuuu ^-^!\n");
}
```


### TODO:
  * Formatter  support.
  * PowerShell support. (Windows)
  * TrueColor  support.
  * More tests.

### Colo[dot]rs licensed under the terms of MIT License.
