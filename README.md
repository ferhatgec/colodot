# Colo[dot]rs
Easy-to-use, powerful Rust implementation of Colorized library.

```rs
colodot!(DotTypes::Bold, DotColors::Red,   "Hello, red!\n",   true);
colodot!(DotTypes::Light,DotColors::Green, "Hello, green!\n", true);
colodot!(DotTypes::Blink,DotColors::Blue,  "Hello, blue!\n",  true);

red()  ; println!("{}", "Hello, red!"  );
green(); println!("{}", "Hello, green!");
blue() ; println!("{}", "Hello, blue!" );
```

### TODO:
  * Formatter  support.
  * PowerShell support. (Windows)
  * TrueColor  support.
  * More tests.

### Colo[dot]rs licensed under the terms of MIT License.
