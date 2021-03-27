# Colo[dot]rs
Easy-to-use, powerful Rust implementation of Colorized library.

```rust
colodot!(DotTypes::Bold, DotColors::Red,   "Hello, red!\n",   true);
colodot!(DotTypes::Light,DotColors::Green, "Hello, green!\n", true);
colodot!(DotTypes::Blink,DotColors::Blue,  "Hello, blue!\n",  true);

red()  ; println!("{}", "Hello, red!"  );
green(); println!("{}", "Hello, green!");
blue() ; println!("{}", "Hello, blue!" );

colodot!  (colodot::dot::DotTrueColor{r: 255, g: 255, b: 255}, "Hello, world!\n");
colodottc!(colodot::dot::DotTrueColor{r: 12 , g: 50, b: 40  }, "Hello, world!"  );

colodottc!(colodot::dot::DotTrueColor{r: 200, g: 100, b: 12 }, "Hello, world!", true);
        
true_color   (1  , 1 , 1 , &["Hello", " , ", "world", "!"]);
bg_true_color(230, 15, 60, &["Hello", " , ", "world", "!"]);

basic_true_color   (colodot::dot::DotTrueColor{r: 5  , g: 5 , b: 5 }, "Hello, world!");
basic_bg_true_color(colodot::dot::DotTrueColor{r: 235, g: 20, b: 65}, "Hello, world!");
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
  * More tests.

### Colo[dot]rs licensed under the terms of MIT License.
