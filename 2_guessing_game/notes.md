# Misc. Notes and Blurbs

_This is simply where I jot down notes for quick reference as I go through The Book._

## Conventions, Names, & Tools

- `module` is a file with import tree structure like in JS

- `crate` is Rust's package system. a `binary crate` is ready to execute, whereas a `library crate` is meant to be imported into other code, and isn't runnable on its own.

- `crates.io` is a public place to see other Rust crates!

- `rustc` is the compiler

- `cargo` is the package tool with running features

- underscoring seems to be a big convention (coming from JavaScript's land of camelCase, it's a welcome change.)

## Types

`macro`

- A function but more specific with exlclamation mark

`variable` and `reference`

- I have learned them to be conceptually the same thing, but they might be different in Rust?

- Need to research. (Infering from the docs -- saying that "references are like variables in that they are immutable".)

`Enumeration` or `Enum`

- a type that can be one of multiple possible states. Each possible state is a `variant`.

## Syntax

`fn`

- Example: `fn myFunction() { }`
- `fn` is the function keyword. For basic usecases this essentially the same syntax as JS.

`let`

- Example: `let apples = 5`
- Key word to assign a variable
- Variables are immutable buy default

`mut`

- Example: `let mut apples = 5`
- `mut` keyword defines the following variable as mutable

## Operators

## Symbols

`::`

- Example: `use std::io`
- Layman's defenition: This is a separater for module and its parts. In the example "std" is the module, and "io" is the part of it that we want to use.
- At a glance, this seems like a prettier way to import than in JS, where you conceptually import the entire module and then deconstruct it. A JS analog might look like `import { io } from 'std'`, or `const io = require('std.io')`
- After considering this, I actually like it a lot more than how JS does it!

`&`

- **Reference Data**
- Example: `io::stdin().read_line(&mut my_variable)`
- In the example, the method `read_line()` takes an argument of `&mut my_variable`. This argument can be sequentially translated to: "A reference (`&`) to a mutatable (`mut`) variable named "my_variable".
- In higher-level languages like JS a function/method does not define its signature, so anything can be passed in and you just hope that it works. 🤞
- In typed languages like TS, you will define the type of data for the signature of a function/method, so in this instance it would be safer than JS in that it could enforce that the argument is of type `String`.
- In Rust, there is even more safety in that this method needs to recieve an argument, it must know the type of data, and it must know that the data is a reference and that the data is mutable. So secure! Oh my!
- Note: What would non-reference be? In JS, you are able to pass in data "in-line", meaning that you can just create data and it not have a reference to be used as a pointer. Think about how you can just pass in a new string with no variable name in this example: `console.log("This string has no variable name")` Another example of non-reference data might be an anonymous function (oftentimes an anonymous arrow funtion) that is passed in to array methods. If a new piece of data is written inline, that means that it has no variable name to refference it, and it simply disappears when is not of use anymore.

## Common Tools & Mechanisms

`Result`

- is a type with two variants: `Ok` and `Err`. Results should be given a value to `expect()` so that there is a message in the event of an error. If Result turns out to be `Ok` then the resulting value is returned.

```
let description = String.new("cool")
println!("Temperate literals are {description}")
```

You can do temporate literals easily in double quotes using just the curly-braces. No JS backticks necessary!

```
let description_one = String.new("cool")
let description_two = String.new("awesome")
println!("Temperate literals are {}", description_one, description_two)
```
