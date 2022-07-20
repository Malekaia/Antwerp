# Antwerp
## About:
Antwerp was a closed-source build program for [logicalbranch.github.io](https://logicalbranch.github.io) that was ported from [Node.js](https://nodejs.org/en/) & [Pug](https://pugjs.org/api/getting-started.html) to [Rust](https://www.rust-lang.org/) & [Tera](https://tera.netlify.app/), it's now an open-source framework for building static websites.

It was ported to Rust improve the performance and speed of the build program - resulting in an optimised binary where the average build time (real / total elapsed) is 97.68% faster than its Node.js counterpart - decreasing average build speeds from 2.8 seconds to 0.065 seconds.

## Description:
The Antwerp build program takes given resources and copies (static files), compiles (SCSS stylesheets), and renders (Tera templates) to generate a static website in a user defined folder (or `./dist` for testing purposes) in the current working directory. See a working example of a build config [src/test/logicalbranch.rs](https://github.com/LogicalBranch/Antwerp/blob/master/src/test/logicalbranch.rs).

## Information:
This project is experimental and has not been tested in production, please be cautious. To see open issues and scheduled updates [click here](https://github.com/LogicalBranch/Antwerp/issues), for all issues and updates [click here](https://github.com/LogicalBranch/Antwerp/issues?q=is%3Aissue).

## License:
The source code included in this repository is freely distributed under the [MIT Licence](https://choosealicense.com/licenses/mit/), for the full licensing document see [LICENSE.md](https://github.com/LogicalBranch/Antwerp/blob/master/LICENSE.md).

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