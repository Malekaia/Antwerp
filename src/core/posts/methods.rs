use crate::{Antwerp, Lib, Post};
use regex::Regex;
use std::{path::{Path, PathBuf}, ffi::OsStr};
use titlecase::titlecase;

pub fn header_data(build: &Antwerp, post: &mut Post, file_path: &str, file_content: &mut String) {
  // Regular expression used to extract static data embedded in the template as HTML comments
  let re_define: Regex = Regex::new(r"<!-- define (.*?): (.*?) -->\n").unwrap();
  // Iterate the captures of "re_define"
  for capture in re_define.captures_iter(&file_content.to_owned()) {
    // Remove the definition from the final content
    *file_content = file_content.replace(&capture[0].to_string(), "");
    // Assign the properties to their matching field in the post object
    let property_value: String = capture[2].to_string();
    match &*capture[1].to_lowercase() {
      "title" => post.title = property_value,
      "description" => post.description = property_value,
      "category" => post.category = property_value,
      "subcategory" => post.subcategory = property_value,
      "genre" => post.genre = property_value,
      "keywords" => post.keywords = property_value,
      "tags" => post.tags = property_value,
      "published" => post.published = property_value,
      "slug" => post.slug = property_value,
      "estimated_read_time" => post.estimated_read_time = property_value,
      "metadata" => post.metadata = property_value,
      "table_of_contents" => post.table_of_contents = property_value,
      "url" => post.url = property_value,
      "path_render" => post.path_render = property_value,
      "path_template" => post.path_template = property_value,
      "template" => post.template = property_value,
      "header.image" => post.header.image = property_value,
      "header.credits" => post.header.credits = property_value,
      "author.name" => post.author.name = property_value,
      "author.image" => post.author.image = property_value,
      "author.github_url" => post.author.github_url = property_value,
      "author.github_username" => post.author.github_username = property_value,
      unknown_key @ _ => {
        Lib::log(build.verbose, "yellow", "Ignore", "unknown key", &format!("\"{unknown_key}\" in \"{file_path}\""))
      }
    }
  }
}

pub fn header_defaults(build: &Antwerp, post: &mut Post) {
  // Set options to defaults if not exists
  if post.path_render.len() < 1 {
    post.path_render = build.config.path_render();
  }
  if post.header.image.len() < 1 {
    post.header.image = build.config.header.image();
  }
  if post.header.credits.len() < 1 {
    let b_c: String = build.config.header.credits();
    // Try to extract header credits from image name and defaults or leave an empty string
    if post.header.image.contains(":") {
      post.header.credits = titlecase(&post.header.image[0..post.header.image.find(":").unwrap()].replace("-", " "));
    } else if b_c.len() > 0 {
      post.header.credits = build.config.header.credits();
    } else {
      post.header.credits = String::new();
    }
  }
  if post.author.name.len() < 1 {
    post.author.name = build.config.author.name();
  }
  if post.author.image.len() < 1 {
    post.author.image = build.config.author.image();
  }
  if post.author.github_url.len() < 1 {
    post.author.github_url = build.config.author.github_url();
  }
  if post.author.github_username.len() < 1 {
    post.author.github_username = build.config.author.github_username();
  }
}

// FIXME: convert into data structure
pub fn table_of_contents(file_content: &mut String) -> String {
  // Create a string for the table of contents
  let mut table_of_contents: String = String::new();
  // Regex to extract header tags
  let re_toc: Regex = Regex::new("<h([1-6]{1})(.*?c)lass=[\"\']text-title[\"\'](.*?>|>)(.*?)</h([1-6]{1})>").unwrap();
  // Regex to extract non alpha-numeric header ends
  let re_toc_end: Regex = Regex::new(r"[^a-zA-Z0-9]$").unwrap();
  // Regex to add an ID to headers
  let re_toc_addition: Regex = Regex::new(r"<h([1-6]{1}) ").unwrap();
  // Iterate captures of "re_toc"
  for capture in re_toc.captures_iter(&file_content.to_owned()) {
    // Extract the header's content
    let header: String = re_toc_end.replace_all(&capture[4], "").to_string();
    // Create the slug ID for the header
    let id_slug: String = Lib::string_to_slug(&header);
    // Insert the slug ID into the header
    let html_header: String = re_toc_addition.replace(&capture[0], &format!("<h$1 id=\"{}\" ", id_slug)).to_string();
    // Replace the header with a version that includes an ID
    *file_content = file_content.replace(&capture[0], &html_header);
    // Add the current header to the table of contents
    table_of_contents.push_str(&format!("<a href=\"#{}\" data-level=\"{}\">{}</a>", id_slug, &capture[1], header));
  }
  // Add the wrappers to the table of contents
  format!("<section class=\"table-of-contents\">{}</section>", table_of_contents)
}

pub fn estimated_read_time(content: &String) -> String {
  // Generate the estimated read time for the post
  let word_count: f32 = content.split(" ").collect::<Vec<&str>>().len() as f32;
  let words_per_minute: f32 = 160f32;
  // Unicode Character “𝑥” (https://www.compart.com/en/unicode/U+1D465)
  let humanised_wpm: String = if word_count > 0f32 {
    (word_count / words_per_minute).ceil().to_string()
  } else {
    String::from("&#119909;")
  };
  // Return the formatted string
  format!("{} minute read", humanised_wpm)
}

pub fn metadata(post: &Post) -> String {
  format!(
    "<meta name=\"keywords\" content=\"{keywords}\" /><meta name=\"category\" content=\"{category}\" />
     <meta name=\"topic\" content=\"{subcategory}\" /><meta name=\"revised\" content=\"{published}\" />
     <meta name=\"date\" content=\"{published}\" /><meta name=\"pagename\" content=\"{title}\" />
     <meta name=\"title\" content=\"{title}\" /><meta name=\"description\" content=\"{description}\" />
     <meta name=\"abstract\" content=\"{description}\" /><meta name=\"summary\" content=\"{description}\" />
     <meta name=\"subtitle\" content=\"{description}\" /><meta name=\"syndication-source\" content=\"{url}\" />
     <meta name=\"original-source\" content=\"{url}\" /><meta name=\"og:type\" content=\"{genre}\" />
     <meta name=\"og:title\" content=\"{title}\" /><meta name=\"og:description\" content=\"{description}\" />
     <meta name=\"og:url\" content=\"{url}\" /><meta name=\"og:image\" content=\"/images/{image}\" />
     <link rel=\"bookmark\" title=\"{title}\" href=\"{url}\" /><link rel=\"canonical\" href=\"{url}\" />
     <link rel=\"self\" type=\"application/atom+xml\" href=\"{url}\" />",
    // sanitise HTML input strings
    title = Lib::escape_html(&post.title),
    description = Lib::escape_html(&post.description),
    category = Lib::escape_html(&post.category),
    subcategory = Lib::escape_html(&post.subcategory),
    genre = Lib::escape_html(&post.genre),
    keywords = Lib::escape_html(&post.keywords),
    published = Lib::escape_html(&post.published),
    image = Lib::escape_html(&post.header.image),
    url = Lib::escape_html(&post.url)
  )
}

pub fn post_url(build: &Antwerp, post: &mut Post) -> String {
  build.path_render.replace("%url_root", &build.url_root)
    .replace("%category", &post.category)
    .replace("%slug", &post.slug)
}

pub fn path_render(build: &Antwerp, post: &mut Post) -> String {
  let mut path_render: String = build.path_render.replace("%category", &post.category);
  path_render = path_render.replace("%slug", &post.slug);
  Lib::path_join(&build.dir_output, &path_render)
}

pub fn path_template(build: &Antwerp, file_path: &String) -> String {
  // Create the tera template usable
  let fn_skip_while = | s: &&OsStr | !build.tera_root_dirs.contains(&s.to_str().unwrap().to_string());
  let path_template: PathBuf = Path::new(file_path).iter().skip_while(fn_skip_while).collect();
  path_template.into_os_string().into_string().unwrap()
}
