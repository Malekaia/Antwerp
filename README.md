## Antwerp <img align="right" src="https://github.com/Malekaia/Antwerp/actions/workflows/build.yaml/badge.svg">

[Antwerp](https://crates.io/crates/antwerp) is a framework for Github Pages based on the [Marcus](https://crates.io/crates/marcus) MarkDown to HTML parser. Antwerp converts MarkDown templates in `public/` into HTML and writes them to `dist/`.

### Demonstration:

`main.rs`

```rust
use antwerp;

fn main() {
  antwerp::build();
}
```

### Input:

`public/base.html`
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
    <p>This is the default body.</p>
  {% endblock body %}

  {% block footer | trim | text %}
    * Default List<br/>
      * Won't get converted into HTML
  {% endblock footer %}
</body>
</html>
```

`public/index.md`
```markdown
{% extends "base.html" %}

{% block title %}This is the title{% endblock title %}

{% block body %}
# Hello World!

This is the template, it contains a link to [a file](/section/chapter/file.html) in the first chapter of a random section.
{% endblock body %}
```

`public/section/chapter/file.md`
```markdown
{% extends "base.html" %}
{% block title %}This is the title{% endblock title %}

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
<footer>This is a custom footer</footer>
{% endblock footer %}
```

### Output:

`dist/index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title><p>This is the title</p></title>
</head>
<body>
  <h1>Hello World!</h1>

<p>This is the template, it contains a link to <a href="/section/chapter/file.html">a file</a> in the first chapter of a random section.</p>

  * Default List<br/>
      * Won't get converted into HTML
</body>
</html>
```

`dist/section/chapter/file.md`
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title><p>This is the title</p></title>
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

  <footer>This is a custom footer</footer>
</body>
</html>
```
