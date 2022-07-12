# Antwerp
## Description:
Build program for [logicalbranch.github.io](https://logicalbranch.github.io) ported from [Node.js](https://nodejs.org/en/) and [Pug](https://pugjs.org/api/getting-started.html).

## About:
This program takes given resources and copies (static files), compiles (SCSS stylesheets), and renders (Tera templates) to generate a static website (usually in `./dist`).

It was ported from Node.js & Pug to [Rust](https://www.rust-lang.org/) & [Tera](https://tera.netlify.app/) to improve the performance and speed of the program. Resulting in an (optimised) binary where the real (total elapsed) build time is (on average) 97.50% faster than its Node.js counterpart, a decrease in (average) build speed from 2.8 seconds to 0.07 seconds.

## TODO:
* Move render single calls to init method
* Use "Render::defaults()" with "Render"
* Add option to generate new projects:
  * Generate directory structure
  * Generate associated files and source code
* Add optional support for [Babel](https://babeljs.io/)