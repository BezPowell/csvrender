# csvrender
A simple utility for rendering each row of a CSV to a separate file; with templating logic similar to languages such as Jinja etc. This was initially created to convert notes stored on an old iPhone into markdown files for easier reading, but may prove useful in any situation where you have data in a CSV that you want to render as individual files (e.g. converting a dynamic website into markdown for a static site generator).

## Usage
```
csvrender --output-path <OUTPUT_PATH> --file-template <FILE_TEMPLATE> <CSV>
```

## How it Works
Rows from a CSV are rendered using the [Tera templating engine](https://keats.github.io/tera/), which is similar to other templating engines such as Jinja. Each column in the row is made available to the templating engine as a variable, which can be inserted as normal, or used for conditional logic. The same variables can also be used in the `--output-path` filename using the [Rust string formatting rules](https://doc.rust-lang.org/std/fmt/).

Look in the tests folder to see more details of how the process works.

## Example usage: Converting WordPress to Zola
Given a CSV of WordPress posts, an example usage might be.
```
csvrender --output-path "content/posts/{{post_name}}.md" --file-template "post.md" posts.csv
```

With a `posts.md` template content of
```
+++
title = "{{ post_title }}"
{% if post_excerpt -%}
description = "{{ post_excerpt }}"
{% endif -%}

date = {{ post_date }}
updated = {{ post_modified }}
+++
{{ post_content | safe }}

```

The basic Hello World! post would be rendered to `content/posts/hello-world.md` with the following content:
```
+++
title = "Hello world!"
date = 2025-11-26 10:39:38
updated = 2025-11-26 10:39:38
+++
<p>Welcome to WordPress. This is your first post. Edit or delete it, then start writing!</p>
```

## Compiling
Assuming you already have Rust and Cargo installed, compilation is simple.

```
git clone https://github.com/BezPowell/csvrender.git
cd csvrender
cargo build --release
```

The binary file will then be located in the target/release folder.

## Limitations
This is currently at an initial, proof-of-concept stage. Any feedback and suggestions are welcome. Some issues to be aware of:

- Only CSV input is supported.
- All columns are handled as strings. Any operations requiring other data types (e.g. addition) will require the variable to be first cast to the appropriate data type in-template.
