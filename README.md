# C Like Markup Language
Simple, C-like language to store data | Parser written in Rust (`v1.0`)

## How to use

#### Setting up:
1. **Clone** the repository
1. **Move** `markup.rs` file into the **`src`** folder of your project.
1. **Declare** `mod markup;` inside your code.
1. **Parse** `markup::parse("file.clml")` to load the file into a **`HashMap`**
1. **Inline Comments** are supported.

#### Code example:

Markup (`config.clml`):
```
// Basic Settings
theme = dark;
font = arial;
font size = 20;
```
Code (`main.rs`):
```Rust
mod markup;

let data = markup::parse("config.clml").expect("could not find file");

for (k, v) in data {
    println!("{:?} : {:?}", k, v)
}
```
Output:
```
theme : dark
font : arial
font_size = 20
```

## Keep in mind

* `C-Like-Markdown-Language` is still in very early development.
* Currently, every piece of data is considered a `string`.
* Spaces are converted into '_' in parsing

## Future Roadmap
* Add data types like `string` or `int`.
* Add lists `List<int>` or `List<string>`.
* Add quotated strings `"hello world"` instead of `hello world`.
* Add database names like `scores` or `names`.