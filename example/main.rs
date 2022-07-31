use antwerp::{Antwerp, Asset, Config, Lib, Post, Route, Routes, Template};
use tera::Context;

fn main() {
  // Create a new Config object
  let template_directory: String = Lib::path::from_cwd("./public/**/*.tera");
  let mut config: Config = Config::new(template_directory);

  // Define basic build config (boolean options, urls, directories)
  config.clean = true;
  config.safe_clean = false;
  config.verbose = false;
  config.url_root = "https://logicalbranch.github.io";
  config.url_post = "%url_root/articles/%category/%slug.html";
  config.dir_resources = Lib::path::absolute("./public/");
  Lib::ensure_dir("./dist/");
  config.dir_output = Lib::path::absolute("./dist/");
  config.dir_posts = Lib::path::from_cwd("./public/articles/*/*.tera");
  config.path_render = "articles/%category/%slug.html";

  // Create a sorted post list
  config.post_list = Post::list_sort(&config, | posts_unsorted: Vec<Post> | {
    // sort into vec[vec[tutorial], vec[project], vec[opinion], vec[misc], vec[guide]]
    let mut sorted: Vec<Vec<Post>> = vec![vec![], vec![], vec![], vec![], vec![]];
    for post in posts_unsorted {
      match &*post.genre {
        "Tutorial" => sorted[0].push(post),
        "Project" => sorted[1].push(post),
        "Opinion" => sorted[2].push(post),
        "Misc" => sorted[3].push(post),
        "Guide" => sorted[4].push(post),
        unknown_genre @ _ => panic!("Ignore (unknown genre): {} in {}", unknown_genre, post.template_path)
      }
    }
    // vec[...tutorial, ...project, ...opinion, ...misc, ...guide]
    for genre in &mut sorted {
      genre.sort_by_key(| post | post.template_raw.len());
      genre.reverse();
    }
    sorted.into_iter().flatten().collect::<Vec<Post>>()
  });

  // Define build assets
  config.assets = vec![
    // Define a list of folders to copy
    Asset::Folder("images/**/*", r"\.(png|jpg)$", false),
    Asset::Folder("scripts/vendor/*.js", r"\.js$", false),
    Asset::Folder("scripts/*.js", r"\.js$", true),
    Asset::Folder("styles/vendor/**/*", r"\.(css|woff|woff2)$", false),
    // Define a list of files to copy
    Asset::File("sitemap.xml", "sitemap.xml", false),
    // Define SCSS assets
    Asset::Scss("styles/app.scss", "styles/app.css"),
    Asset::Scss("styles/http.scss", "styles/http.css")
  ];

  // Define routes (/404.html, /articles/index.html, /index.html)
  config.routes = vec![
    // Define the route for "/404.html"
    Route {
      template: "404.tera",
      output: "404.html",
      context: Context::new()
    },

    // Define the route for "/articles/index.html"
    Route {
      template: "articles/index.tera",
      output: "articles/index.html",
      context: Context::new()
    },

    // Define the route for ".index.html"
    Route {
      template: "index.tera",
      output: "index.html",
      context: {
        let mut context: Context = Context::new();
        context.insert("articles", &config.post_list);
        context.insert("page_name", "index");
        context.insert("image", "manuel-cosentino:n--CMLApjfI-unsplash.jpg");
        context.insert("artwork_credit", "Manuel Cosentino");
        context
      }
    }
  ];

  // Define route groups (for 410 gone, 301 moved, and posts)
  config.route_groups = vec![
    // Define a route group for (410) gone/deleted routes
    Routes {
      template: "410.tera",
      routes: vec![
        Template { output: "about/index.html", context: Context::new() },
        Template { output: "contact/index.html", context: Context::new() },
        Template { output: "img/index.html", context: Context::new() },
        Template { output: "info/index.html", context: Context::new() },
        Template { output: "articles/jQuery/how_to_modify_the_jquery_global_without_modifying_jquery/index.html", context: Context::new() },
        Template { output: "articles/jQuery/how_to_scroll_to_an_element_on_click/index.html", context: Context::new() },
        Template { output: "articles/Node.js/brace_expansion_in_shelljs/index.html", context: Context::new() },
        Template { output: "articles/Web_Development/how_to_setup_a_development_server_using_express/index.html", context: Context::new() },
        Template { output: "articles/Web_Development/how_to_setup_a_development_server_using_flask/index.html", context: Context::new() },
      ]
    },

    // Define a route group for (301) moved routes
    Routes {
      template: "301.tera",
      routes: {
        fn c301(redirect: &str) -> Context {
          let mut context: Context = Context::new();
          context.insert("redirect", redirect);
          context
        }
        vec![
          Template { output: "articles/Bootstrap/how_to_change_the_default_font_in_bootstrap/index.html", context: c301("/articles/CSS/how-to-change-the-default-font-in-bootstrap.html") },
          Template { output: "articles/Git/how_to_avoid_overusing_the_git_keyword/index.html", context: c301("/articles/Git/how-to-avoid-retyping-the-git-keyword.html") },
          Template { output: "articles/Go/globbing_in_go/index.html", context: c301("/articles/Go Lang/globbing-in-go.html") },
          Template { output: "articles/HTML/how_to_open_html_links_in_new_tabs/index.html", context: c301("/articles/HTML/how-to-open-html-links-in-new-tabs.html") },
          Template { output: "articles/Node.js/environment_detection_in_javascript/index.html", context: c301("/articles/JavaScript/environment-detection-in-javascript.html") },
          Template { output: "articles/Perl/how_to_unzip_an_archive_using_perl/index.html", context: c301("/articles/Perl/how-to-call-a-subprocess-in-perl.html") },
          Template { output: "articles/Pip/an_introduction_to_the_pip_package_manager/index.html", context: c301("/articles/Python/an-introduction-to-the-pip-package-manager.html") },
          Template { output: "articles/SASS/how_to_extend_parent_selectors_in_sass/index.html", context: c301("/articles/CSS/how-to-extend-parent-selectors-in-sass.html") },
          Template { output: "articles/Web_Development/how_to_create_a_development_server_using_http_server/index.html", context: c301("/articles/Python/how-to-create-a-development-server-using-http-server.html") }
        ]
      }
    },

    // Define a route group for posts
    Routes {
      template: "articles/article.tera",
      routes: config.post_list.iter().map(| post: &Post | {
        Template {
          output: &post.render_path,
          context: {
            let mut context: Context = Context::new();
            context.insert("articles", &config.post_list);
            context.insert("article", &post);
            context.insert("template_rendered", &Antwerp::render_string(&mut config.tera, &post.template_raw, Context::new()));
            context.insert("page_name", "article");
            context
          }
        }
      }).collect::<Vec<Template>>()
    }
  ];

  // Initiate the build
  Antwerp::build(&config);
}
