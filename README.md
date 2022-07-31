# Antwerp
## Overview:
Antwerp was a closed-source build program for [logicalbranch.github.io](https://logicalbranch.github.io). It was ported from [Node.js](https://nodejs.org/en/) & [Pug](https://pugjs.org/api/getting-started.html) to [Rust](https://www.rust-lang.org/) & [Tera](https://tera.netlify.app/) and is now an open-source framework for building static websites.

It takes resources specified in a config object and copies assets & directories, compiles SCSS stylesheets, and renders Tera templates to generate a static site in a user-defined folder. Antwerp also supports multiple builds using seperate config instances. For an example of a build config, see [example/main.rs](https://github.com/Malekaia/Antwerp/blob/main/example/main.rs).

## Crate:
This project is available on crates.io as [Antwerp](https://crates.io/crates/antwerp).

## License:
The source code included in this repository is distributed for free, under the [MIT Licence](https://choosealicense.com/licenses/mit/). For the full license, see [LICENSE.md](https://github.com/Malekaia/Antwerp/blob/master/LICENSE.md).

## Footnotes:
Antwerp [0.1.0](https://crates.io/crates/antwerp/0.1.0) and Antwerp [0.1.1](https://crates.io/crates/antwerp/0.1.1) are broken, if starting a new project use Antwerp [0.1.2](https://crates.io/crates/antwerp/0.1.2) onwards.

<!-- ## References:
**Crates**: [Colored](https://crates.io/crates/colored), [Chrono](https://crates.io/crates/chrono), [FS Extra](https://crates.io/crates/fs_extra), [Glob](https://crates.io/crates/glob), [Grass](https://crates.io/crates/grass), [Regex](https://crates.io/crates/regex), [Serde](https://crates.io/crates/serde), [SWC](https://crates.io/crates/swc), [Tera](https://crates.io/crates/tera), [Titlecase](https://crates.io/crates/titlecase)

**Language**:
* [The Rust Cheatsheet (by programming-idioms.org)](https://programming-idioms.org/cheatsheet/Rust)
* [The Rust Reference: Linkage](https://doc.rust-lang.org/reference/linkage.html)
* [The Cargo Book, Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
* [Rust Analyzer](https://rust-analyzer.github.io/)
* [Observing variable changes](https://users.rust-lang.org/t/observe-changes-of-variable/59069/8)

**Other**:
* [Known Outstanding Issues (Grass Crate)](https://github.com/connorskees/grass/issues/19)
* [StackOverflow: Why are Rust executables so huge?](https://stackoverflow.com/a/29008355/10415695) -->
<!-- * [Sitemap generator](https://www.xml-sitemaps.com/) -->
<!-- * [Google search console](https://search.google.com/search-console/) -->
<!-- * [Google search console (inspect)](https://search.google.com/search-console/welcome?action=inspect) -->
<!-- * [Google Trends](https://trends.google.com/trends/?geo=GB) -->