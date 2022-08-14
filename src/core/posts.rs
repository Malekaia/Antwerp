use crate::{Antwerp, Lib};
use regex::Regex;
use serde::Serialize;
use std::{path::{Path, PathBuf}, ffi::OsStr};
use titlecase::titlecase;

/// A struct that simplifies post management. Encapsulates post data and contains methods to extract data from and generate data for posts and collect Vectors containing Post objects for each post.
///
/// **Extracted (user-defined) post data**:
/// * title
/// * description
/// * category
/// * subcategory
/// * genre
/// * keywords
/// * tags
/// * published
/// * image
/// * author
///
/// **Generated post data**:
/// * slug
/// * artwork_credit
/// * estimated_read_time
/// * metadata
/// * table_of_contents
///
/// **Post template data**:
/// * template_path
/// * render_path
/// * url
/// * content
#[derive(Serialize)]
pub struct Post {
  // Extracted (user-defined) post data
  pub title: String,
  pub description: String,
  pub category: String,
  pub subcategory: String,
  pub genre: String,
  pub keywords: String,
  pub tags: String,
  pub published: String,
  pub image: String,
  pub author: String,
  // Generated post data
  pub slug: String,
  pub artwork_credit: String,
  pub estimated_read_time: String,
  pub metadata: String,
  pub table_of_contents: String,
  // Post template data
  pub url: String,
  pub render_path: String,
  pub template_path: String,
  pub template_raw: String
}

impl Post {
  /// Create a new instance of Post, where all fields are instantiated to empty strings using `String::new()`
  pub fn new() -> Post {
    Post {
      // Extracted (user-defined) post data
      title: String::new(),
      description: String::new(),
      category: String::new(),
      subcategory: String::new(),
      genre: String::new(),
      keywords: String::new(),
      tags: String::new(),
      published: String::new(),
      image: String::new(),
      author: String::new(),
      // Generated post data
      slug: String::new(),
      artwork_credit: String::new(),
      estimated_read_time: String::new(),
      metadata: String::new(),
      table_of_contents: String::new(),
      // Post template data
      url: String::new(),
      render_path: String::new(),
      template_path: String::new(),
      template_raw: String::new()
    }
  }

  /// Extracts user-defined data from a template to generate and populate a new instance of Post
  ///
  /// **Behaviour**:
  /// * Extract the post's `title`, `description`, `category`, `subcategory`, `genre`, `keywords`, `tags`, `published` (date), `image` and `author`
  ///   * Unknown fields are ignored
  /// * Remove user define statements from the post template
  /// * Generate the `table_of_contents`, `estimated_read_time`, `metadata`, `template_path`, `slug`, `artwork_credit`, `render_path` and `url` for the post
  /// * Fail safe and avoid panicking where possible
  pub fn properties(config: &Antwerp, file_path: &String, template_path_roots: &Vec<&str>) -> Post {
    // Create a new Post
    let mut post: Post = Post::new();
    // Read and create a mutable copy the file content
    let file_content: String = Lib::read_file(&file_path);
    let mut content: String = file_content.to_owned();

    // Regular expression used to extract static data embedded in the template as HTML comments
    let re_define: Regex = Regex::new(r"<!-- define (.*?): (.*?) -->\n").unwrap();
    // Iterate the captures of "re_define"
    for capture in re_define.captures_iter(&file_content) {
      // Remove the definition from the final content
      content = content.replace(&capture[0].to_string(), "");
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
        "image" => post.image = property_value,
        "author" => post.author = property_value,
        unknown_key @ _ => {
          Lib::log(config.verbose, "yellow", "Ignore", "unknown key", &format!("\"{unknown_key}\" in \"{file_path}\""))
        }
      }
    }

    // Create a string for the table of contents
    let mut table_of_contents: String = "".to_string();
    // Regex to extract header tags
    let re_toc: Regex = Regex::new("<h([1-6]{1})(.*?c)lass=[\"\']text-title[\"\'](.*?>|>)(.*?)</h([1-6]{1})>").unwrap();
    // Regex to extract non alpha-numeric header ends
    let re_toc_end: Regex = Regex::new(r"[^a-zA-Z0-9]$").unwrap();
    // Regex to add an ID to headers
    let re_toc_addition: Regex = Regex::new(r"<h([1-6]{1}) ").unwrap();
    // Iterate captures of "re_toc"
    for capture in re_toc.captures_iter(&file_content) {
      // Extract the header's content
      let header: String = re_toc_end.replace_all(&capture[4], "").to_string();
      // Create the slug ID for the header
      let id_slug: String = Lib::string_to_slug(&header);
      // Insert the slug ID into the header
      let html_header: String = re_toc_addition.replace(&capture[0], &format!("<h$1 id=\"{}\" ", id_slug)).to_string();
      // Replace the header with a version that includes an ID
      content = content.replace(&capture[0], &html_header);
      // Add the current header to the table of contents
      table_of_contents.push_str(&format!("<a href=\"#{}\" data-level=\"{}\">{}</a>", id_slug, &capture[1], header));
    }
    // Add the wrappers to the table of contents
    post.table_of_contents = format!("<section class=\"table-of-contents\">{}</section>", table_of_contents);

    // Generate the estimated read time for the post
    let word_count: f32 = content.split(" ").collect::<Vec<&str>>().len() as f32;
    let words_per_minute: f32 = 160f32;
    // Unicode Character “𝑥” (https://www.compart.com/en/unicode/U+1D465)
    let humanised_wpm: String = if word_count > 0f32 {
      (word_count / words_per_minute).ceil().to_string()
    } else {
      "&#119909;".to_string()
    };
    post.estimated_read_time = format!("{} minute read", humanised_wpm);

    // Generate HTML metadata for the post
    post.metadata = format!(
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
      image = Lib::escape_html(&post.image),
      url = Lib::escape_html(&post.url)
    );

    // Insert the template path
    let template_path: PathBuf = Path::new(file_path).iter()
                                      .skip_while(| s: &&OsStr | !template_path_roots.contains(&s.to_str().unwrap()))
                                      .collect();
    post.template_path = template_path.into_os_string().into_string().unwrap();
    // Add the rendered content to the post
    post.template_raw = content;
    // Create a slug string for the post title
    post.slug = Lib::string_to_slug(&post.title);
    // Extract the artwork credits
    post.artwork_credit = titlecase(&post.image[0..post.image.find(":").unwrap()].replace("-", " "));
    // Create the render path string
    let render_path: String = config.path_render.replace("%category", &post.category).replace("%slug", &post.slug);
    post.render_path = Lib::path_join(&config.dir_output, &render_path);
    // Generate a url for the post
    post.url = config.path_render.replace("%url_root", &config.url_root).replace("%category", &post.category).replace("%slug", &post.slug);
    post
  }

  /// Create a Vector containing Post objects containing information for each post in a specified directory. Use with `PostConfig`
  ///
  /// **Behaviour**:
  /// * Extract all templates in the config's `dir_tem`
  pub fn list(config: &Antwerp) -> Vec<Post> {
    // Create a list of template roots
    let mut template_path_roots: Vec<&str> = config.tera.as_ref().unwrap().get_template_names().filter_map(| path: &str | {
      if path.contains("/") {
        Some(path.split("/").collect::<Vec<&str>>()[0])
      } else {
        None
      }
    }).collect::<Vec<&str>>();
    template_path_roots.sort_unstable();
    template_path_roots.dedup();

    // Walk the given directory
    Lib::walk_dir(&config.dir_posts)
        // Convert into an Iter
        .iter()
        // Generate the properties for each post
        .map(| file_path: &String | Post::properties(config, file_path, &template_path_roots))
        // Collect the Iter as a Vector
        .collect::<Vec<Post>>()
  }

  pub fn list_sort(config: &Antwerp, sorter: fn(Vec<Post>) -> Vec<Post>) -> Vec<Post> {
    sorter(Post::list(&config))
  }
}