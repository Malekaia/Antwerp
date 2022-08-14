use antwerp::{Antwerp, Post};
use tera::Context;

pub fn build() {
  // Create a new build instance
  let mut build: Antwerp = Antwerp::default();

  // Define template data for the tera instance
  build.tera("./public/**/*.tera");

  // Define config for urls, directories and post paths
  build.url_root("https://malekaia.github.io");
  build.url_post("%url_root/articles/%category/%slug.html");
  build.dir_resources("./public/");
  build.dir_output("./dist/malekaia.github.io/");
  build.dir_posts("./public/articles/*/*.tera");
  build.path_render("articles/%category/%slug.html");

  // NOTE: clean can only occur AFTER `build.dir_output` has been defined
  build.verbose(true);
  build.clean(true, true);

  // Copy directories
  build.folder("images/**/*", r"\.(png|jpg)$", false);
  build.folder("scripts/vendor/*.js", r"\.js$", false);
  build.folder("scripts/*.js", r"\.js$", true);
  build.folder("styles/vendor/**/*", r"\.(css|woff|woff2)$", false);

  // Compile SCSS assets
  build.scss("styles/app.scss", "styles/app.css");
  build.scss("styles/http.scss", "styles/http.css");

  // Generate the post list
  build.post_list(| unsorted: Vec<Post> | -> Vec<Post> {
    // sort into vec[vec[tutorial], vec[project], vec[opinion], vec[misc], vec[guide]]
    let mut sorted: Vec<Vec<Post>> = vec![vec![], vec![], vec![], vec![], vec![]];
    for post in unsorted {
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

  // Build "/404.html" template
  build.route("404.tera", "404.html", &build.empty_context);

  // Build HTTP 410 (deleted) templates
  for output in [
    "about/index.html",
    "contact/index.html",
    "img/index.html",
    "info/index.html",
    "articles/jQuery/how_to_modify_the_jquery_global_without_modifying_jquery/index.html",
    "articles/jQuery/how_to_scroll_to_an_element_on_click/index.html",
    "articles/Node.js/brace_expansion_in_shelljs/index.html",
    "articles/Web_Development/how_to_setup_a_development_server_using_express/index.html",
    "articles/Web_Development/how_to_setup_a_development_server_using_flask/index.html"
  ] {
    build.route("410.tera", output, &build.empty_context);
  }

  // Build HTTP 301 (moved) templates
  for [output, redirect] in [
    ["articles/Bootstrap/how_to_change_the_default_font_in_bootstrap/index.html", "/articles/CSS/how-to-change-the-default-font-in-bootstrap.html"],
    ["articles/Git/how_to_avoid_overusing_the_git_keyword/index.html", "/articles/Git/how-to-avoid-retyping-the-git-keyword.html"],
    ["articles/Go/globbing_in_go/index.html", "/articles/Go Lang/globbing-in-go.html"],
    ["articles/HTML/how_to_open_html_links_in_new_tabs/index.html", "/articles/HTML/how-to-open-html-links-in-new-tabs.html"],
    ["articles/Node.js/environment_detection_in_javascript/index.html", "/articles/JavaScript/environment-detection-in-javascript.html"],
    ["articles/Perl/how_to_unzip_an_archive_using_perl/index.html", "/articles/Perl/how-to-call-a-subprocess-in-perl.html"],
    ["articles/Pip/an_introduction_to_the_pip_package_manager/index.html", "/articles/Python/an-introduction-to-the-pip-package-manager.html"],
    ["articles/SASS/how_to_extend_parent_selectors_in_sass/index.html", "/articles/CSS/how-to-extend-parent-selectors-in-sass.html"],
    ["articles/Web_Development/how_to_create_a_development_server_using_http_server/index.html", "/articles/Python/how-to-create-a-development-server-using-http-server.html"]
  ] {
    build.route("301.tera", output, &{
      let mut context: Context = Context::new();
      context.insert("redirect", redirect);
      context
    });
  }

  // Build "/index.html" template
  build.route("index.tera", "index.html", &{
    let mut context: Context = Context::new();
    context.insert("articles", &build.post_list);
    context.insert("page_name", "index");
    context.insert("image", "nasa-yZygONrUBe8-unsplash.jpg");
    context.insert("artwork_credit", "Manuel Cosentino");
    context
  });

  // Build "/articles/index.html" template
  build.route("articles/index.tera", "articles/index.html", &build.empty_context);

  // Build post templates
  for post in &build.post_list {
    build.route("articles/article.tera", &post.render_path, &{
      let mut context: Context = Context::new();
      context.insert("articles", &build.post_list);
      context.insert("article", &post);
      context.insert("template_rendered", &build.render(&post.template_path, &build.empty_context));
      context.insert("page_name", "article");
      context
    });
  }
}
