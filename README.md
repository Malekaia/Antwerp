# Antwerp
## Description:
Antwerp used to be a closed-source turned open-source build program for [logicalbranch.github.io](https://logicalbranch.github.io) which is being ported from [Node.js](https://nodejs.org/en/) & [Pug](https://pugjs.org/api/getting-started.html) to [Rust](https://www.rust-lang.org/) & [Tera](https://tera.netlify.app/). It's currently in the process of being converted into a minimalist framework for building static websites.

## About:
This program takes given resources and copies (static files), compiles (SCSS stylesheets), and renders (Tera templates) to generate a static website in a folder (`./dist`) in the current working directory.

It was ported from Node.js & Pug to Rust & Tera to improve the performance and speed of the build program, resulting in an (optimised) binary where the real (total elapsed) build time is - on average - 97.678% faster than its Node.js counterpart, a decrease in average build speed from 2.8 seconds to 0.065 seconds.

**Update**: 6 days, 18 hours, 28 minutes and 13 seconds after the initial commit (11/07/2022 at 05:14:29), the first complete build was made using the Rust port of Antwerp. See the full working example at [src/test/logicalbranch.rs](https://github.com/LogicalBranch/Antwerp/blob/master/src/test/logicalbranch.rs).

## License:
The source code included in this repository is distributed under an [MIT Licence](https://choosealicense.com/licenses/mit/), for the full licensing document see [LICENSE.md](https://github.com/LogicalBranch/Antwerp/blob/master/LICENSE.md).

## Changes (Crate):
- [x] Add documentation to existing code
- [x] Remove redundant `lib.rs` methods
- [x] Allow for user defined directory structures
- [x] Standardise and integrate the `Article` module into Antwerp
- [x] Implement a `Post` module to handle all post types
- [x] Modularise code and separate concerns
- [x] Ensure the Tera inheritance chain is built **once** and **only once**
- [x] Ensure Post data is collected and generated **once** and **only once**
- [x] Implement a `Config` module to define build resources and behavior
- [x] Allow for multiple builds
- [x] Remove all `.clone()` calls and reduce use of `.to_owned()`<sup>[[2]](#footnotes)</sup>
- [x] Remove `static mut VERBOSE` and `mut static VERBOSE_CHECKED`
- [x] Reimplement unsafe blocks with safe code
- [x] Remove unnecessary references
- [ ] Fix post template content not rendering
- [ ] Add (opt-in) support for [SWC](https://swc.rs/)

## Changes (Standalone executable):
- [ ] Implement a config file (`Antwerp.toml`)
  - [ ] Only print if `verbose` is enabled
  - [ ] Move `empty_root` to a config file
    - [ ] confirm delete if option is true (?)
- [ ] Implement CLI interface
  - [ ] Call init methods for config options
  - [ ] Implement a `watch` CLI option (`--watch`)
  - [ ] Generate new projects
    - [ ] Generate directory structure
    - [ ] Generate sample files and source code

## References:
**Rust**:
* [The Rust Cheatsheet (by programming-idioms.org)](https://programming-idioms.org/cheatsheet/Rust)
* [The Rust Reference: Linkage](https://doc.rust-lang.org/reference/linkage.html)
* [The Cargo Book, Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

**Crates**:
* [Colored](https://crates.io/crates/colored)
* [Glob](https://crates.io/crates/glob)
* [Grass](https://crates.io/crates/grass)
* [Regex](https://crates.io/crates/regex)
* [Serde](https://crates.io/crates/serde)
* [Tera](https://crates.io/crates/tera)
* [Titlecase](https://crates.io/crates/titlecase)

**Other**:
* [Known Outstanding Issues (Grass Crate)](https://github.com/connorskees/grass/issues/19)
* [StackOverflow: Why are Rust executables so huge?](https://stackoverflow.com/a/29008355/10415695)

## Footnotes:
1. ~~New scopes around `Lib::print` calls in `core.rs` are placeholders for verbose checks~~
2. `Antwerp::Lib::escape_html` and `Antwerp::Post::properties`
