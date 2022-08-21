mod methods;

use crate::{Antwerp, Lib};
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
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
  pub author_image: String,
  pub author_github_url: String,
  pub author_github_username: String,
  pub slug: String,
  pub artwork_credit: String,
  pub estimated_read_time: String,
  pub metadata: String,
  pub table_of_contents: String,
  pub url: String,
  pub render_path: String,
  pub template_path: String,
  pub template_raw: String
}

impl Post {
  /// Extracts user-defined data from a template to generate and populate a new instance of Post
  fn properties(build: &Antwerp, file_path: &String) -> Post {
    // Create a new instance of Post, where all fields are instantiated to empty strings
    let mut post: Post = Post {
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
      author_image: String::new(),
      author_github_url: String::new(),
      author_github_username: String::new(),
      slug: String::new(),
      artwork_credit: String::new(),
      estimated_read_time: String::new(),
      metadata: String::new(),
      table_of_contents: String::new(),
      url: String::new(),
      render_path: String::new(),
      template_path: String::new(),
      template_raw: String::new()
    };
    // Read and create a mutable copy the file content
    let mut file_content: String = Lib::read_file(&file_path);
    // Extract header data
    methods::header_data(&build, &mut post, &file_path, &mut file_content);
    // Set options to defaults if not exists
    methods::header_defaults(&build, &mut post);
    // Generate the table of contents
    post.table_of_contents = methods::table_of_contents(&mut file_content);
    // Generate the estimated read time for the post
    post.estimated_read_time = methods::estimated_read_time(&mut file_content);
    // Add the rendered content to the post
    post.template_raw = file_content;
    // Generate HTML metadata for the post
    post.metadata = methods::metadata(&post);
    // Create a slug string for the post title
    post.slug = Lib::string_to_slug(&post.title);
    // Generate a url for the post
    post.url = methods::post_url(&build, &mut post);
    // Create the render path string
    post.render_path = methods::render_path(&build, &mut post);
    // Generate the template path
    post.template_path = methods::template_path(&build, &file_path);
    // Return the posts object
    post
  }

  /// Create a Vector containing Post objects containing information for each post in a specified directory
  pub fn list(build: &Antwerp, sorter: fn(Vec<Post>) -> Vec<Post>) -> Vec<Post> {
    // Walk the given directory & generate a sorted list containing properties for each post
    let fn_map = | file_path: &String | Post::properties(&build, file_path);
    sorter(Lib::walk_dir(&build.config.dir_posts()).iter().map(fn_map).collect::<Vec<Post>>())
  }
}