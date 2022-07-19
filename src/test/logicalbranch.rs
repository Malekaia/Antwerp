//! This is an entry point that tests Antwerp
//!
//! **Behaviour**:
//! * Use `Antwerp::Config` to define resources and declare build behaviour
//! * Trigger `Antwerp::init` to test the build program using resources for [logicalbranch.github.io](https://logicalbranch.github.io/)
use crate::antwerp::{Antwerp, Asset, Config, Post, Route, Template};
use tera::Context;

pub fn build() {
  let mut config: Config = Config::new("public/**/*.tera");

  config.uri = "https://logicalbranch.github.io";

  config.uri_post = "%uri/articles/%category/%slug.html";

  config.dir_dist = "./dist";

  config.dir_templates = "./public/articles/*/*.tera";

  config.path_render = "%dir_dist/articles/%category/%slug.html";

  config.clean = true;

  config.verbose = false;

  config.post_list = Post::list_sort(&config, | posts_unsorted: Vec<Post> | {
    // sort into vec[vec[tutorial], vec[project], vec[opinion], vec[misc], vec[guide]]
    let mut sorted: Vec<Vec<Post>> = vec![vec![], vec![], vec![], vec![], vec![]];
    for article in posts_unsorted {
      match &*article.genre {
        "Tutorial" => sorted[0].push(article),
        "Project" => sorted[1].push(article),
        "Opinion" => sorted[2].push(article),
        "Misc" => sorted[3].push(article),
        "Guide" => sorted[4].push(article),
        unknown_genre @ _ => panic!("Ignore (unknown genre): {} in {}", unknown_genre, article.template_path)
      }
    }
    // vec[...tutorial, ...project, ...opinion, ...misc, ...guide]
    for genre in &mut sorted {
      genre.sort_by_key(| article | article.template_raw.len());
      genre.reverse();
    }
    sorted.into_iter().flatten().collect::<Vec<Post>>()
  });

  config.assets = vec![
    Asset::Folder("./public/images/**/*", "./dist", r"\.(png|jpg)$", false),
    Asset::Folder("./public/scripts/vendor/*.js", "./dist", r"\.js$", false),
    Asset::Folder("./public/scripts/*.js", "./dist", r"\.js$", true),
    Asset::Folder("./public/styles/vendor/**/*", "./dist", r"\.(css|woff|woff2)$", false),
    Asset::File("./public/sitemap.xml", "./dist/sitemap.xml", false),
    Asset::Scss("./public/styles/app.scss", "./dist/styles/app.css"),
    Asset::Scss("./public/styles/http.scss", "./dist/styles/http.css"),
  ];

  config.routes = vec![
    Route::Page("404.tera", "./dist/404.html", Context::new()),
    Route::Page("index.tera", "./dist/index.html", {
      let mut context: Context = Context::new();
      context.insert("articles", &config.post_list);
      context.insert("page_name", "index");
      context.insert("image", "manuel-cosentino:n--CMLApjfI-unsplash.jpg");
      context.insert("artwork_credit", "Manuel Cosentino");
      context
    }),
    Route::Page("articles/index.tera", "./dist/articles/index.html", Context::new()),
    Route::Page("404.tera", "./dist/404.html", Context::new()),
    Route::Page("404.tera", "./dist/404.html", Context::new()),
  ];

  config.route_groups = vec![
    Route::Group("301.tera", {
      fn context_301(redirect: &str) -> Context {
        let mut context: Context = Context::new();
        context.insert("redirect", redirect);
        context
      }
      vec![
        Template { output: "./dist/articles/Bootstrap/how_to_change_the_default_font_in_bootstrap/index.html", context: context_301("/articles/CSS/how-to-change-the-default-font-in-bootstrap.html") },
        Template { output: "./dist/articles/Git/how_to_avoid_overusing_the_git_keyword/index.html", context: context_301("/articles/Git/how-to-avoid-retyping-the-git-keyword.html") },
        Template { output: "./dist/articles/Go/globbing_in_go/index.html", context: context_301("/articles/Go Lang/globbing-in-go.html") },
        Template { output: "./dist/articles/HTML/how_to_open_html_links_in_new_tabs/index.html", context: context_301("/articles/HTML/how-to-open-html-links-in-new-tabs.html") },
        Template { output: "./dist/articles/Node.js/environment_detection_in_javascript/index.html", context: context_301("/articles/JavaScript/environment-detection-in-javascript.html") },
        Template { output: "./dist/articles/Perl/how_to_unzip_an_archive_using_perl/index.html", context: context_301("/articles/Perl/how-to-call-a-subprocess-in-perl.html") },
        Template { output: "./dist/articles/Pip/an_introduction_to_the_pip_package_manager/index.html", context: context_301("/articles/Python/an-introduction-to-the-pip-package-manager.html") },
        Template { output: "./dist/articles/SASS/how_to_extend_parent_selectors_in_sass/index.html", context: context_301("/articles/CSS/how-to-extend-parent-selectors-in-sass.html") },
        Template { output: "./dist/articles/Web_Development/how_to_create_a_development_server_using_http_server/index.html", context: context_301("/articles/Python/how-to-create-a-development-server-using-http-server.html") }
      ]
    }),
    Route::Group("410.tera", vec![
      Template { output: "./dist/about/index.html", context: Context::new() },
      Template { output: "./dist/contact/index.html", context: Context::new() },
      Template { output: "./dist/img/index.html", context: Context::new() },
      Template { output: "./dist/info/index.html", context: Context::new() },
      Template { output: "./dist/articles/jQuery/how_to_modify_the_jquery_global_without_modifying_jquery/index.html", context: Context::new() },
      Template { output: "./dist/articles/jQuery/how_to_scroll_to_an_element_on_click/index.html", context: Context::new() },
      Template { output: "./dist/articles/Node.js/brace_expansion_in_shelljs/index.html", context: Context::new() },
      Template { output: "./dist/articles/Web_Development/how_to_setup_a_development_server_using_express/index.html", context: Context::new() },
      Template { output: "./dist/articles/Web_Development/how_to_setup_a_development_server_using_flask/index.html", context: Context::new() },
    ]),
    Route::Group("articles/article.tera", {
      let mut posts_group: Vec<Template> = vec![];
      for article in &config.post_list {
        posts_group.push(Template {
          output: &article.render_path,
          context: {
            let mut context: Context = Context::new();
            context.insert("articles", &config.post_list);
            context.insert("article", &article);
            context.insert("template_rendered", &Antwerp::render_string(&mut config.tera, &article.template_raw, Context::new()));
            context.insert("page_name", "article");
            context
          }
        });
      }
      posts_group
    })
  ];

  Antwerp::init(&config);
}