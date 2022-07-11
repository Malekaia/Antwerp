mod article;
mod lib;
mod render;
use crate::article::{Article, ArticleSort};
use crate::render::{Render, CopyDetails};
use tera::Context;

impl ArticleSort for Article {
  fn sort_list(article_list: Vec<Article>) -> Vec<Article> {
    // sort into vec[vec[tutorial_articles], vec[project_articles], vec[opinion_articles], vec[misc_articles], vec[guide_articles]]
    let mut articles: Vec<Vec<Article>> = vec![vec![], vec![], vec![], vec![], vec![]];
    for article in article_list {
      match &*article.genre {
        "Tutorial" => articles[0].push(article),
        "Project" => articles[1].push(article),
        "Opinion" => articles[2].push(article),
        "Misc" => articles[3].push(article),
        "Guide" => articles[4].push(article),
        unknown_genre @ _ => panic!("Ignore (unknown genre): {} in {}", unknown_genre, article.template_path)
      }
    }
    // vec[...tutorial_articles, ...project_articles, ...opinion_articles, ...misc_articles, ...guide_articles]
    for genre in &mut articles {
      genre.sort_by_key(| article | article.content.len());
      genre.reverse();
    }
    articles.into_iter().flatten().collect::<Vec<Article>>()
  }
}

fn main() {
  let mut render: Render = Render {
    verbose: true,
    empty_dist: true,
    tera: Render::tera_instance("public/**/*.tera"),
    empty_context: Context::new(),
    copy_dirs: vec![
      CopyDetails("./public/images/**/*", r"\.(png|jpg)$", false),
      CopyDetails("./public/scripts/vendor/*.js", r"\.js$", false),
      CopyDetails("./public/scripts/*.js", r"\.js$", true),
      CopyDetails("./public/styles/vendor/**/*", r"\.(css|woff|woff2)$", true)
    ],
    copy_files: vec![
      CopyDetails("./public/sitemap.xml", "./dist/sitemap.xml", false)
    ],
    assets_301: vec![
      ["./dist/articles/Bootstrap/how_to_change_the_default_font_in_bootstrap/index.html", "/articles/CSS/how-to-change-the-default-font-in-bootstrap.html"],
      ["./dist/articles/Git/how_to_avoid_overusing_the_git_keyword/index.html", "/articles/Git/how-to-avoid-retyping-the-git-keyword.html"],
      ["./dist/articles/Go/globbing_in_go/index.html", "/articles/Go Lang/globbing-in-go.html"],
      ["./dist/articles/HTML/how_to_open_html_links_in_new_tabs/index.html", "/articles/HTML/how-to-open-html-links-in-new-tabs.html"],
      ["./dist/articles/Node.js/environment_detection_in_javascript/index.html", "/articles/JavaScript/environment-detection-in-javascript.html"],
      ["./dist/articles/Perl/how_to_unzip_an_archive_using_perl/index.html", "/articles/Perl/how-to-call-a-subprocess-in-perl.html"],
      ["./dist/articles/Pip/an_introduction_to_the_pip_package_manager/index.html", "/articles/Python/an-introduction-to-the-pip-package-manager.html"],
      ["./dist/articles/SASS/how_to_extend_parent_selectors_in_sass/index.html", "/articles/CSS/how-to-extend-parent-selectors-in-sass.html"],
      ["./dist/articles/Web_Development/how_to_create_a_development_server_using_http_server/index.html", "/articles/Python/how-to-create-a-development-server-using-http-server.html"]
    ],
    assets_410: vec![
      "./dist/about/index.html",
      "./dist/contact/index.html",
      "./dist/img/index.html",
      "./dist/info/index.html",
      "./dist/articles/jQuery/how_to_modify_the_jquery_global_without_modifying_jquery/index.html",
      "./dist/articles/jQuery/how_to_scroll_to_an_element_on_click/index.html",
      "./dist/articles/Node.js/brace_expansion_in_shelljs/index.html",
      "./dist/articles/Web_Development/how_to_setup_a_development_server_using_express/index.html",
      "./dist/articles/Web_Development/how_to_setup_a_development_server_using_flask/index.html"
    ],
    scss_assets: vec![
      ["./public/styles/app.scss", "./dist/styles/app.css"],
      ["./public/styles/http.scss", "./dist/styles/http.css"]
    ]
  };

  render.empty_dist();
  render.copy_static();
  render.compile_scss();
  render.assets_301("301.tera");
  render.assets_410("410.tera");
  render.template("404.tera", "./dist/404.html", &render.empty_context);

  // create the article list
  let article_list: Vec<Article> = Article::sort_list(Article::list());

  // render article index
  render.template("articles/index.tera", "./dist/articles/index.html", &render.empty_context);

  // render the index
  let mut index_context: Context = Context::new();
  index_context.insert("articles", &article_list);
  index_context.insert("page_name", "index");
  index_context.insert("image", "manuel-cosentino:n--CMLApjfI-unsplash.jpg");
  index_context.insert("artwork_credit", "Manuel Cosentino");
  render.template("index.tera", "./dist/index.html", &index_context);

  // render articles
  let mut article_context: Context = Context::new();
  article_context.insert("articles", &article_list);
  for mut article in article_list {
    article.content = render.template_string(&article.content, &article_context);
    article_context.insert("content", &article.content);
    article_context.insert("article", &article);
    article_context.insert("page_name", "article");
    article_context.insert("artwork_credit", &article.artwork_credit);
    article_context.insert("image", &article.image);
    render.template("articles/article.tera", &article.render_path, &article_context);
  }
}
