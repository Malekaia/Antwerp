use crate::antwerp::{Lib, Antwerp::PostsConfig};
use regex::Regex;
use serde::Serialize;
use titlecase::titlecase;

/// A struct that simplifies post management. Encapsulates post data and contains methods to extract data from and generate data for posts and collect Vectors containing Post objects for each post.
///
/// **Static data**:
///
/// pub title: String,
///
/// pub description: String,
///
/// pub category: String,
///
/// pub subcategory: String,
///
/// pub genre: String,
///
/// pub keywords: String,
///
/// pub tags: String,
///
/// pub published: String,
///
/// pub image: String,
///
/// pub author: String,
///
/// **Dynamic data**:
///
/// pub slug: String,
///
/// pub artwork_credit: String,
///
/// pub estimated_read_time: String,
///
/// pub metadata: String,
///
/// pub table_of_contents: String,
///
/// **File data**:
///
/// pub content: String,
///
/// **Source data**:
///
/// pub template_path: String,
///
/// pub render_path: String,
///
/// pub url: String,
#[derive(Serialize)]
pub struct Post {
  // static data
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
  // dynamic data
  pub slug: String,
  pub artwork_credit: String,
  pub estimated_read_time: String,
  pub metadata: String,
  pub table_of_contents: String,
  // file data
  pub content: String,
  // source data
  pub template_path: String,
  pub render_path: String,
  pub url: String,
}


impl Post {
  pub fn new() -> Post {
    Post {
      // User defined data
      title: "".to_string(),
      description: "".to_string(),
      category: "".to_string(),
      subcategory: "".to_string(),
      genre: "".to_string(),
      keywords: "".to_string(),
      tags: "".to_string(),
      published: "".to_string(),
      image: "".to_string(),
      author: "".to_string(),

      // Generated data
      slug: "".to_string(),
      artwork_credit: "".to_string(),
      estimated_read_time: "".to_string(),
      metadata: "".to_string(),
      table_of_contents: "".to_string(),

      // Template information
      template_path: "".to_string(),
      render_path: "".to_string(),
      url: "".to_string(),

      // Rendered template
      content: "".to_string()
    }
  }


  pub fn properties(config: &PostsConfig, file_path: &String) -> Post {
    // Create a new Post
    let mut post: Post = Post::new();
    // Read and create a mutable clone the file content
    let file_content: String = Lib::read_file(&file_path);
    let mut content: String = file_content.clone();


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
        unknown_key @ _ => panic!("Ignore: define \"{}\" in \"{}\"", unknown_key, file_path)
      }
    }


    // Create a string for the table of contents
    let mut table_of_contents: String = "".to_string();
    // Regex to extract header tags
    let re_toc: Regex = Regex::new("<h(3|5)(.*?c)lass=[\"\']text-title[\"\'](.*?>|>)(.*?)</h(3|5)>").unwrap();
    // Regex to extract non alpha-numeric header ends
    let re_toc_end: Regex = Regex::new(r"[^a-zA-Z0-9]$").unwrap();
    // Regex to add an ID to headers
    let re_toc_addition: Regex = Regex::new(r"<h(3|5) ").unwrap();
    // Iterate captures of "re_toc"
    for capture in re_toc.captures_iter(&file_content) {
      // Extract the header's content
      let header: String = re_toc_end.replace_all(&capture[4], "").to_string();
      // Create the slug ID for the header
      let id_slug: String = Lib::string_to_slug(&header);
      // Insert the slug ID into the header
      let html_header: String = re_toc_addition.replace(&capture[0], &format!("<h$1 id=\"{}\" ", &id_slug)).to_string();
      // Replace the header with a version that includes an ID
      content = content.replace(&capture[0], &html_header);
      // Add the current header to the table of contents
      table_of_contents.push_str(&format!("<a href=\"#{}\" level=\"{}\">{}</a>", &id_slug, &capture[1], &header));
    }
    // Add the wrappers to the table of contents
    post.table_of_contents = format!("<div class=\"table-of-contents\">{}</div>", &table_of_contents);


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
    post.metadata = format!("
        <meta name=\"keywords\" content=\"{keywords}\" />
        <meta name=\"category\" content=\"{category}\" />
        <meta name=\"topic\" content=\"{subcategory}\" />
        <meta name=\"revised\" content=\"{published}\" />
        <meta name=\"date\" content=\"{published}\" />
        <meta name=\"pagename\" content=\"{title}\" />
        <meta name=\"description\" content=\"{description}\" />
        <meta name=\"abstract\" content=\"{description}\" />
        <meta name=\"summary\" content=\"{description}\" />
        <meta name=\"subtitle\" content=\"{description}\" />
        <meta name=\"syndication-source\" content=\"{url}\" />
        <meta name=\"original-source\" content=\"{url}\" />
        <meta name=\"og:type\" content=\"{genre}\" />
        <meta name=\"og:title\" content=\"{title}\" />
        <meta name=\"og:description\" content=\"{description}\" />
        <meta name=\"og:url\" content=\"{url}\" />
        <meta name=\"og:image\" content=\"/images/{image}\" />
        <link rel=\"bookmark\" title=\"{title}\" href=\"{url}\" />
        <link rel=\"self\" type=\"application/atom+xml\" href=\"{url}\" />
        <link rel=\"canonical\" href=\"{url}\" />
      ",
      // sanitise input strings
      title = &Lib::escape_html(&post.title),
      description = &Lib::escape_html(&post.description),
      category = &Lib::escape_html(&post.category),
      subcategory = &Lib::escape_html(&post.subcategory),
      genre = &Lib::escape_html(&post.genre),
      keywords = &Lib::escape_html(&post.keywords),
      published = &Lib::escape_html(&post.published),
      image = &Lib::escape_html(&post.image),
      url = &Lib::escape_html(&post.url)
    // minify
    ).replace("\n        ", "");


    // Insert the template path
    post.template_path = file_path.to_string();
    // Add the rendered content to the post
    post.content = content;
    // Create a slug string for the post title
    post.slug = Lib::string_to_slug(&post.title);
    // Extract the artwork credits
    post.artwork_credit = titlecase(&post.image[0..post.image.find(":").unwrap()].replace("-", " "));
    // Generate the output path for the rendered template
    let root_dist: &str = if config.dir_dist.ends_with("/") { &config.dir_dist[0..(config.dir_dist.len() - 1)] } else { config.dir_dist };
    // Create the render path string
    post.render_path = config.path_render.replace("%dir_dist", root_dist)
                                            .replace("%category", &post.category)
                                            .replace("%slug", &post.slug);
    // Generate a url for the post
    post.url = config.path_render.replace("%uri", &config.uri)
                                    .replace("%category", &post.category)
                                    .replace("%slug", &post.slug);

    // Return the post
    post
  }


  /// Create a Vector containing Post objects containing information for each post in a specified directory. Use with `PostConfig`
  ///
  /// **Behaviour**:
  /// * Extract all templates in the config's `dir_tem`
  pub fn list(config: &PostsConfig) -> Vec<Post> {
    // Walk the given directory
    Lib::walk_dir(config.dir_templates)
        // Convert into an Iter
        .iter()
        // Generate the properties for each post
        .map(| file_path: &String | Post::properties(config, &file_path))
        // Collect the Iter as a Vector
        .collect::<Vec<Post>>()
  }
}