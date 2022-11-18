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
