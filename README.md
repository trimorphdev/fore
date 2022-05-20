# Fore
*A full stack web framework for Rust.*

## Features
- **Server-side rendering**: for improved loading speeds and better SEO.
- **Component based**: modular components makes designing web pages easier.
- **RSX** _**(Requires Nightly)**_: use inline HTML as Fore components in Rust.  Made extremely fast with the use of Rust's procedural macro system.

## Example
```rust
use fore::prelude::*;

fn main() {
    println!("{}", rsx! {
        <Div>
            Hello, world!
        </Div>
    }.render()); // => <div>Hello, world!</div>
}
```

## Contributing
Use the right [Gitmoji](https://gitmoji.dev) for your commits.  Please [create an issue](https://github.com/trimorphdev/fore/issues/new) if you have any questions, thanks for contributing! :heart: