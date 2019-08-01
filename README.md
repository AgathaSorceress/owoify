# owoify
A simple rust library for string owoification.  
Inspired by:
* [owo text generator](https://honk.moe/tools/owo.html)
* [OWO Chrome extension](https://chrome.google.com/webstore/detail/owo/jolaggjkdhhgcdhcjjhfkkbllefoggob?hl=en)

# Credits
* [MashAllPotatoes](https://twitter.com/MashNewGamePlus) - Regular expressions and some of the owo faces

# Usage
Add this to ``Cargo.toml``:

```
[dependencies]
owoify = "0.1.1"
```
``example.rs``:  
```
use owoify::OwOifiable;

fn main() {
    let text = String::from("euthanize me senpai!!");
    println!("{}", text.owoify());
}
```

# Documentation  
* [Docs.rs](https://docs.rs/owoify)
