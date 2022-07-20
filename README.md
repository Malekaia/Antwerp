# Antwerp
## About:
Antwerp was a closed-source build program for [logicalbranch.github.io](https://logicalbranch.github.io). It was ported from [Node.js](https://nodejs.org/en/) & [Pug](https://pugjs.org/api/getting-started.html) to [Rust](https://www.rust-lang.org/) & [Tera](https://tera.netlify.app/) and is now an open-source framework for building static websites.

## Description:
The Antwerp build program takes given resources and copies (static files), compiles (SCSS stylesheets), and renders (Tera templates) to generate a static website in a user-defined folder in the current working directory (or `./dist` for testing).

For a sample build config, see [src/test/logicalbranch.rs](https://github.com/LogicalBranch/Antwerp/blob/master/src/test/logicalbranch.rs).

## Information:
This project is experimental and not production ready or tested, please be cautious. For open issues and scheduled updates, [click here](https://github.com/LogicalBranch/Antwerp/issues). For all issues and updates, [click here](https://github.com/LogicalBranch/Antwerp/issues?q=is%3Aissue).

## Why Rust?
Build speed was the deciding factor in the decision to go from JavaScript (Node.js) to Rust. During testing, the Node.js version logged average build speeds of ~2.8s to ~3.0s, while the Rust version clocked in at ~0.07s on average, roughly ~97.58% faster for the same test cases.

## License:
The source code included in this repository is distributed, for free, under the [MIT Licence](https://choosealicense.com/licenses/mit/), for the full license, see [LICENSE.md](https://github.com/LogicalBranch/Antwerp/blob/master/LICENSE.md).

## References:
**Crates**: [Colored](https://crates.io/crates/colored), [Glob](https://crates.io/crates/glob), [Grass](https://crates.io/crates/grass), [Regex](https://crates.io/crates/regex), [Serde](https://crates.io/crates/serde), [SWC](https://crates.io/crates/swc), [Tera](https://crates.io/crates/tera), [Titlecase](https://crates.io/crates/titlecase)

**Language**:
* [The Rust Cheatsheet (by programming-idioms.org)](https://programming-idioms.org/cheatsheet/Rust)
* [The Rust Reference: Linkage](https://doc.rust-lang.org/reference/linkage.html)
* [The Cargo Book, Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
* [Rust Analyzer](https://rust-analyzer.github.io/)
* [Observing variable changes](https://users.rust-lang.org/t/observe-changes-of-variable/59069/8)

**Other**:
* [Known Outstanding Issues (Grass Crate)](https://github.com/connorskees/grass/issues/19)
* [StackOverflow: Why are Rust executables so huge?](https://stackoverflow.com/a/29008355/10415695)
<!-- * [Sitemap generator](https://www.xml-sitemaps.com/) -->
<!-- * [Google search console](https://search.google.com/search-console/) -->
<!-- * [Google search console (inspect)](https://search.google.com/search-console/welcome?action=inspect) -->
<!-- * [Google Trends](https://trends.google.com/trends/?geo=GB) -->