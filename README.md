# Antwerp <img align="right" src="https://github.com/Malekaia/Antwerp/actions/workflows/build.yaml/badge.svg">

[Antwerp](https://crates.io/crates/antwerp) is an open-source web framework ported from Node.js & Pug to Rust for GitHub Pages.
It outputs static web pages in `dist/` using HTML and MarkDown templates in `public/`, which are converted to HTML using the [Marcus](https://crates.io/crates/marcus) MarkDown to HTML parser.


## References & Getting Started:
- <https://crates.io/crates/antwerp>
- <https://github.com/Malekaia/Antwerp>
- <https://docs.rs/antwerp/latest/antwerp/>
- <https://crates.io/crates/marcus>
- <https://github.com/Malekaia/Marcus/>
- <https://docs.rs/marcus/latest/marcus/>
- <https://developer.mozilla.org/en-US/docs/Web/HTML>
- <https://www.markdownguide.org/>


## Demonstration:

**Cargo.toml** (`Cargo.toml`):
```toml
[package]
name = "<NAME>"
version = "<VERSION>"
edition = "2021"
description = "<DESCRIPTION>"
license = "MIT"
readme = "README.md"

[dependencies]
antwerp = "0.3.1"
```

**main.rs** (`src/main.rs`):

```rust
use antwerp;

fn main() {
  antwerp::build();
}
```


## Input:

**base.html** (`public/base.html`):

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{% block title | text | trim %}{% endblock title %}</title>
</head>
<body>
  {% block body %}
    <p>This paragraph serves as the default content for the <code>body</code> block.</p>
  {% endblock body %}

  <footer>
    {% block footer | trim | text %}
      # This is a header for the default footer (it won't get converted to HTML).
    {% endblock footer %}
  </footer>
</body>
</html>
```

**index.md** (`public/index.md`):
```markdown
{% extends "base.html" %}

{% block title %}Homepage{% endblock title %}

{% block body %}
# Hello World!

This is the template, it contains a link to [a file](/section/chapter/file.html) in the first chapter of a random section and the default footer (below).
{% endblock body %}
```

**file.md** (`public/section/chapter/file.md`):
```markdown
{% extends "base.html" %}
{% block title %}Section / Chapter / File{% endblock title %}

{% block body %}
# Hello World!

This is the template, it takes you to [the homepage](/).

This page also includes CSS styles, which are ignored by the [Marcus](https://crates.io/crates/marcus) MarkDown to HTML converter.

<style type="text/css">
  a {
    color: hotpink !important;
    text-decoration: none
  }
  h1 {
    font-family: sans-serif
  }
</style>
{% endblock body %}

{% block footer %}
  <footer>
    This is a custom footer for the `section / chapter / file` page.
  </footer>
{% endblock footer %}
```


## Output:

**index.html** (`dist/index.html`):
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title><p>Homepage</p></title>
</head>
<body>
  <h1>Hello World!</h1>

<p>This is the template, it contains a link to <a href="/section/chapter/file.html">a file</a> in the first chapter of a random section and the default footer (below).</p>

  <footer>
    # This is a header for the default footer (it won't get converted to HTML).
  </footer>
</body>
</html>
```

**file.md** (`dist/section/chapter/file.md`):
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title><p>Section / Chapter / File</p></title>
</head>
<body>
  <h1>Hello World!</h1>

<p>This is the template, it takes you to <a href="/">the homepage</a>.</p>

<p>This page also includes CSS styles, which are ignored by the <a href="https://crates.io/crates/marcus">Marcus</a> MarkDown to HTML converter.</p>

<style type="text/css">
  a {
    color: hotpink !important;
    text-decoration: none
  }
  h1 {
    font-family: sans-serif
  }
</style>

  <footer>
    <footer>

<p> This is a custom footer for the <code>section / chapter / file</code> page.</p>
  </footer>
  </footer>
</body>
</html>
```
