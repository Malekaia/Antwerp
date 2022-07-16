use crate::lib;
use regex::Regex;
use serde::Serialize;
use titlecase::titlecase;

#[derive(Serialize)]
pub struct Article {
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

impl Article {
  pub fn new() -> Article {
    Article {
      // static data
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
      // dynamic data
      slug: "".to_string(),
      artwork_credit: "".to_string(),
      estimated_read_time: "".to_string(),
      metadata: "".to_string(),
      table_of_contents: "".to_string(),
      // file data
      content: "".to_string(),
      // source data
      template_path: "".to_string(),
      render_path: "".to_string(),
      url: "".to_string()
    }
  }

  fn generate_properties(file_path: &String, dist_root: &str, root_url: &str) -> Article {
    // get the file content and create the article object
    let file_content: String = lib::read_file(&file_path);
    let mut content: String = file_content.clone();
    let mut article: Article = Article::new();

    // extract static data
    let re_define: Regex = Regex::new(r"<!-- define (.*?): (.*?) -->\n").unwrap();
    for capture in re_define.captures_iter(&file_content) {
      // remove declaration from the file content
      content = content.replace(&capture[0].to_string(), "");
      // assign the properties to the article object
      let property_value: String = capture[2].to_string();
      match &*capture[1].to_lowercase() {
        "title" => article.title = property_value,
        "description" => article.description = property_value,
        "category" => article.category = property_value,
        "subcategory" => article.subcategory = property_value,
        "genre" => article.genre = property_value,
        "keywords" => article.keywords = property_value,
        "tags" => article.tags = property_value,
        "published" => article.published = property_value,
        "image" => article.image = property_value,
        "author" => article.author = property_value,
        unknown_key @ _ => {
          let article_info: &str = &file_path.replace("public/articles/", "");
          panic!("Ignore: define \"{}\" in \"{}\"", unknown_key, article_info);
        }
      }
    }

    // dynamic article properties
    article.slug = lib::string_to_slug(&article.title);
    article.artwork_credit = titlecase(&article.image[0..article.image.find(":").unwrap()].replace("-", " "));
    article.template_path = format!("./{}", &file_path);
    let root_dist: &str = if dist_root.ends_with("/") { &dist_root[0..(dist_root.len() - 1)] } else { dist_root };
    article.render_path = format!("{}/articles/{}/{}.html", root_dist, &article.category, &article.slug);
    article.url = format!("{}/articles/{}/{}.html", &root_url, &article.category, &article.slug);

    // generate the article's estimated read time
    let zero: f32 = 0 as f32;
    let word_count: f32 = content.split(" ").collect::<Vec<&str>>().len() as f32;
    let words_per_minute: f32 = 160 as f32;
    article.estimated_read_time = if word_count > zero {
      format!("{} minute read", (word_count / words_per_minute).round())
    } else {
      // Unicode Character “𝑥” (https://www.compart.com/en/unicode/U+1D465)
      "&#119909; minute read".to_string()
    };

    // generate the table of contents
    let mut table_of_contents: String = "".to_string();
    let re_toc: Regex = Regex::new("<h(3|5)(.*?c)lass=[\"\']text-title[\"\'](.*?>|>)(.*?)</h(3|5)>").unwrap();
    let re_toc_end: Regex = Regex::new(r"[^a-zA-Z0-9]$").unwrap();
    let re_toc_addition: Regex = Regex::new(r"<h(3|5) ").unwrap();
    for capture in re_toc.captures_iter(&file_content) {
      let header: String = re_toc_end.replace_all(&capture[4], "").to_string();
      let id_slug: String = lib::string_to_slug(&header);
      let html_header: String = re_toc_addition.replace(&capture[0], &format!("<h$1 id=\"{}\" ", &id_slug)).to_string();
      content = content.replace(&capture[0], &html_header);
      table_of_contents.push_str(&format!("<a href=\"#{}\" level=\"{}\">{}</a>", &id_slug, &capture[1], &header));
    }
    article.table_of_contents = format!("<div class=\"table-of-contents\">{}</div>", &table_of_contents);

    // generated metadata
    article.metadata = format!("
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
      // format variables
      title = &lib::escape_html(&article.title),
      description = &lib::escape_html(&article.description),
      category = &lib::escape_html(&article.category),
      subcategory = &lib::escape_html(&article.subcategory),
      genre = &lib::escape_html(&article.genre),
      keywords = &lib::escape_html(&article.keywords),
      published = &lib::escape_html(&article.published),
      image = &lib::escape_html(&article.image),
      url = &lib::escape_html(&article.url)
    // minify
    ).replace("\n        ", "");

    // assign the article content
    article.content = content;
    article
  }

  pub fn list(dist_root: &str, root_url: &str) -> Vec<Article> {
    // Walk the given articles directory
    lib::walk_dir("./public/articles/*/*.tera")
        // Convert into an Iter
        .iter()
        // Generate the properties for each article
        .map(| file_path: &String | Article::generate_properties(&file_path, dist_root, root_url))
        // Collect the Iter as a Vector
        .collect::<Vec<Article>>()
  }
}