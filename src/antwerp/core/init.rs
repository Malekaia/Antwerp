use crate::antwerp::{Antwerp, Config, Route};

pub fn init(config: &Config) {
  if config.clean == true {
    Antwerp::empty_root(&config);
  }

  Antwerp::assets(&config);

  for route in &config.routes {
    match route {
      Route::Page(template_name, output, context) => {
        Antwerp::route(&config, template_name, output, context);
      },
      Route::Group(_, _) => panic!("Error: put route groups in your config's \"route_groups\" field")
    }
  }

  for route in &config.route_groups {
    match route {
      Route::Page(_, _, _) => panic!("Error: put page routes in your config's \"route\" field"),
      Route::Group(template_name, route_group) => {
        Antwerp::route_group(&config, template_name, route_group);
      }
    }
  }
}