use crate::antwerp::{Antwerp, Config};

pub fn init(config: &Config) {
  if config.clean == true {
    Antwerp::clean_build(&config);
  }

  Antwerp::assets(&config);

  for route in &config.routes {
    Antwerp::route(&config, route.template, route.output, &route.context);
  }

  for routes in &config.route_groups {
    Antwerp::route_group(&config, routes.template, &routes.routes);
  }
}