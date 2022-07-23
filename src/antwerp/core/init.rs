use crate::antwerp::{Antwerp, Config, Lib};

/// Call build methods to handle resources defined in a `Config` object
pub fn init(config: &Config) {
  // Create dir_output and dir_resources folders if not exists (Issue: #29)
  Lib::ensure_dir(&config.dir_resources);
  Lib::ensure_dir(&config.dir_output);

  // Clean (empty) the build folder
  if config.clean == true {
    Antwerp::clean_build(&config);
  }

  // Pass build assets to the asset handler function
  Antwerp::assets(&config);

  // Render templates
  for route in &config.routes {
    Antwerp::route(&config, route.template, route.output, &route.context);
  }

  // Render route groups
  for routes in &config.route_groups {
    Antwerp::route_group(&config, routes.template, &routes.routes);
  }
}