mod support;

use crate::{Antwerp, Lib};
use serde::Serialize;
use std::{path::{Path, PathBuf}, ffi::OsStr};
use support::{header_data, header_defaults, table_of_contents, estimated_read_time, metadata};

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
  /// Extracts user-defined data from a template to generate and populate a new build of Post
  fn properties(build: &Antwerp, file_path: &String, template_roots: &Vec<&str>) -> Post {
    // Create a new build of Post, where all fields are instantiated to empty strings
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
    let file_content: String = Lib::read_file(&file_path);
    let mut content: String = file_content.to_owned();
    // Extract header data
    header_data(&build, &mut post, &file_path, &file_content, &mut content);
    // Set options to defaults if not exists
    header_defaults(&build, &mut post);
    // Generate the table of contents
    post.table_of_contents = table_of_contents(&file_content, &mut content);
    // Generate the estimated read time for the post
    post.estimated_read_time = estimated_read_time(&content);
    // Generate HTML metadata for the post
    post.metadata = metadata(&post);
    // Add the rendered content to the post
    post.template_raw = content;
    // Create a slug string for the post title
    post.slug = Lib::string_to_slug(&post.title);
    // Generate a url for the post
    post.url = build.path_render.replace("%url_root", &build.url_root);
    post.url = post.url.replace("%category", &post.category);
    post.url = post.url.replace("%slug", &post.slug);
    // Create the render path string
    let mut render_path: String = build.path_render.replace("%category", &post.category);
    render_path = render_path.replace("%slug", &post.slug);
    post.render_path = Lib::path_join(&build.dir_output, &render_path);
    // Generate the template path
    let fn_root = | s: &&OsStr | !template_roots.contains(&s.to_str().unwrap());
    let template_path: PathBuf = Path::new(file_path).iter().skip_while(fn_root).collect();
    post.template_path = template_path.into_os_string().into_string().unwrap();
    // Return the posts object
    post
  }

  /// Create a Vector containing Post objects containing information for each post in a specified directory. Use with `PostConfig`
  pub fn list(build: &Antwerp, sorter: fn(Vec<Post>) -> Vec<Post>) -> Vec<Post> {
    // Create a list of template roots & sort template roots
    let template_names = build.tera.as_ref().unwrap().get_template_names();
    let mut template_roots: Vec<&str> = template_names.filter_map(
      | path: &str | -> Option<&str> { if path.contains("/") { Some(path.split("/").collect::<Vec<&str>>()[0]) } else { None } }
    ).collect::<Vec<&str>>();
    template_roots.sort_unstable();
    template_roots.dedup();
    // Walk the given directory & generate the properties for each post
    let fn_map = | file_path: &String | Post::properties(build, file_path, &template_roots);
    let post_list: Vec<Post> = Lib::walk_dir(&build.dir_posts).iter().map(fn_map).collect::<Vec<Post>>();
    // Sort the post list
    sorter(post_list)
  }
}