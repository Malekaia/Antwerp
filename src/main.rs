mod core;
mod article;
mod lib;
use crate::core::{Antwerp, Antwerp::Template, Antwerp::Asset};
use article::{Article, ArticleSort};
use tera::{Context, Tera};

impl ArticleSort for Article {
  fn sort_list(article_list: Vec<Article>) -> Vec<Article> {
    // sort into vec[vec[tutorial], vec[project], vec[opinion], vec[misc], vec[guide]]
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
    // vec[...tutorial, ...project, ...opinion, ...misc, ...guide]
    for genre in &mut articles {
      genre.sort_by_key(| article | article.content.len());
      genre.reverse();
    }
    articles.into_iter().flatten().collect::<Vec<Article>>()
  }
}

fn main() {
  Antwerp::empty_root("./dist/");

  let mut tera: Tera = Antwerp::tera("public/**/*.tera");
  let article_list: Vec<Article> = Article::sort_list(Article::list("./dist"));

  Antwerp::assets(vec![
    Asset::Folder("./public/images/**/*", "./dist", r"\.(png|jpg)$", false),
    Asset::Folder("./public/scripts/vendor/*.js", "./dist", r"\.js$", false),
    Asset::Folder("./public/scripts/*.js", "./dist", r"\.js$", true),
    Asset::Folder("./public/styles/vendor/**/*", "./dist", r"\.(css|woff|woff2)$", false),
    Asset::File("./public/sitemap.xml", "./dist/sitemap.xml", false),
    Asset::Scss("./public/styles/app.scss", "./dist/styles/app.css"),
    Asset::Scss("./public/styles/http.scss", "./dist/styles/http.css"),
  ]);

  fn context_301(redirect: &str) -> Context {
    let mut context: Context = Context::new();
    context.insert("redirect", redirect);
    context
  }

  Antwerp::route_group(&tera,"301.tera", vec![
    Template { output: "./dist/articles/Bootstrap/how_to_change_the_default_font_in_bootstrap/index.html", context: context_301("/articles/CSS/how-to-change-the-default-font-in-bootstrap.html") },
    Template { output: "./dist/articles/Git/how_to_avoid_overusing_the_git_keyword/index.html", context: context_301("/articles/Git/how-to-avoid-retyping-the-git-keyword.html") },
    Template { output: "./dist/articles/Go/globbing_in_go/index.html", context: context_301("/articles/Go Lang/globbing-in-go.html") },
    Template { output: "./dist/articles/HTML/how_to_open_html_links_in_new_tabs/index.html", context: context_301("/articles/HTML/how-to-open-html-links-in-new-tabs.html") },
    Template { output: "./dist/articles/Node.js/environment_detection_in_javascript/index.html", context: context_301("/articles/JavaScript/environment-detection-in-javascript.html") },
    Template { output: "./dist/articles/Perl/how_to_unzip_an_archive_using_perl/index.html", context: context_301("/articles/Perl/how-to-call-a-subprocess-in-perl.html") },
    Template { output: "./dist/articles/Pip/an_introduction_to_the_pip_package_manager/index.html", context: context_301("/articles/Python/an-introduction-to-the-pip-package-manager.html") },
    Template { output: "./dist/articles/SASS/how_to_extend_parent_selectors_in_sass/index.html", context: context_301("/articles/CSS/how-to-extend-parent-selectors-in-sass.html") },
    Template { output: "./dist/articles/Web_Development/how_to_create_a_development_server_using_http_server/index.html", context: context_301("/articles/Python/how-to-create-a-development-server-using-http-server.html") }
  ]);

  Antwerp::route_group(&tera,"410.tera", vec![
    Template { output: "./dist/about/index.html", context: Context::new() },
    Template { output: "./dist/contact/index.html", context: Context::new() },
    Template { output: "./dist/img/index.html", context: Context::new() },
    Template { output: "./dist/info/index.html", context: Context::new() },
    Template { output: "./dist/articles/jQuery/how_to_modify_the_jquery_global_without_modifying_jquery/index.html", context: Context::new() },
    Template { output: "./dist/articles/jQuery/how_to_scroll_to_an_element_on_click/index.html", context: Context::new() },
    Template { output: "./dist/articles/Node.js/brace_expansion_in_shelljs/index.html", context: Context::new() },
    Template { output: "./dist/articles/Web_Development/how_to_setup_a_development_server_using_express/index.html", context: Context::new() },
    Template { output: "./dist/articles/Web_Development/how_to_setup_a_development_server_using_flask/index.html", context: Context::new() },
  ]);

  Antwerp::route(&tera, "404.tera", "./dist/404.html", Context::new());

  Antwerp::route(&tera, "index.tera", "./dist/index.html", {
    let mut context: Context = Context::new();
    context.insert("articles", &article_list);
    context.insert("page_name", "index");
    context.insert("image", "manuel-cosentino:n--CMLApjfI-unsplash.jpg");
    context.insert("artwork_credit", "Manuel Cosentino");
    context
  });

  Antwerp::route(&tera, "articles/index.tera", "./dist/articles/index.html", {
    let mut context: Context = Context::new();
    context.insert("articles", &article_list);
    context.insert("page_name", "index");
    context.insert("image", "manuel-cosentino:n--CMLApjfI-unsplash.jpg");
    context.insert("artwork_credit", "Manuel Cosentino");
    context
  });

  let mut template_group: Vec<Template> = vec![];
  for article in &article_list {
    template_group.push(Template {
      output: &article.render_path,
      context: {
        let mut context: Context = Context::new();
        context.insert("articles", &article_list);
        let content: String = Antwerp::render_string(&mut tera,&article.content, Context::new());
        context.insert("content", &content);
        context.insert("article", &article);
        context.insert("page_name", &"article");
        context.insert("artwork_credit", &article.artwork_credit);
        context.insert("image", &article.image);
        context
      }
    });
  }

  Antwerp::route_group(&tera,"articles/article.tera", template_group);
}
