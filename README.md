# Antwerp
## Description:
Build program for [logicalbranch.github.io](https://logicalbranch.github.io) ported from [Node.js](https://nodejs.org/en/) and [Pug](https://pugjs.org/api/getting-started.html) to [Rust](https://www.rust-lang.org/) & [Tera](https://tera.netlify.app/) and is in the process of being converted into a minimalist standalone framework for generating static websites.

## About:
This program takes given resources and copies (static files), compiles (SCSS stylesheets), and renders (Tera templates) to generate a static website in a `./dist` folder.

It was ported from Node.js & Pug to Rust & Tera to improve the performance and speed of the build program, resulting in an (optimised) binary where the real (total elapsed) build time is - on average - 97.50% faster than its Node.js counterpart, a decrease in average build speed from 2.8 seconds to 0.07 seconds.

## Todo:
- [ ] Remove unnecessary `lib.rs` methods
- [ ] Enforce use of `public` and `dist` directories
- [ ] Document existing code
- [ ] Implement a config file (`Antwerp.toml`)
  - [ ] Only print if `verbose` is enabled
  - [ ] Move `empty_root` to a config file
    - [ ] confirm delete if option is true (?)
  - [ ] Add (opt-in) support for [Babel](https://babeljs.io/)
- [ ] Implement CLI interface
  - [ ] Call init methods for config options
  - [ ] Implement a `watch` CLI option (`--watch`)
  - [ ] Generate new projects
    - [ ] Generate directory structure
    - [ ] Generate sample files and source code

## References:
 * https://programming-idioms.org/cheatsheet/Rust
 * https://github.com/connorskees/grass/issues/19
 * https://doc.rust-lang.org/std/fs/struct.File.html
 * https://doc.rust-lang.org/std/fs/fn.copy.html
 * https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
 * https://stackoverflow.com/a/32384768/10415695
 * https://stackoverflow.com/a/29008355/10415695
 * https://sass-lang.com/guide#topic-1