# rust-quickie
just pipe a rust file to the CLI tool and it'll run it in a temporary directory

# dependencies
to add dependencies we use "//+ {dependency}" syntax
so for example

```rs
//+ display_plus = "*"
//+ colourful = { git = "https://github.com/rookieCookies/colourful

fn main() { .. }
```
