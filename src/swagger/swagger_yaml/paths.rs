use std::collections::BTreeMap;

use my_http_server::RequestCredentials;

use crate::controllers::documentation::HttpActionDescription;

#[cfg(feature = "with-authorization")]
use crate::controllers::ControllersMiddleware;

use super::yaml_writer::YamlWriter;

pub fn build<TRequestCredentials: RequestCredentials + Send + Sync + 'static>(
    yaml_writer: &mut YamlWriter,

    actions: &BTreeMap<String, BTreeMap<String, HttpActionDescription>>,
    #[cfg(feature = "with-authorization")] controllers: &ControllersMiddleware<TRequestCredentials>,
) {
    yaml_writer.reset_level();
    yaml_writer.write_empty("paths");

    yaml_writer.increase_level();

    for (path, actions) in actions {
        yaml_writer.write_empty(path);
        yaml_writer.increase_level();

        for (verb, action_description) in actions {
            super::verb_description::build(
                yaml_writer,
                verb,
                action_description,
                #[cfg(feature = "with-authorization")]
                controllers,
            )
        }

        yaml_writer.decrease_level();
    }

    yaml_writer.decrease_level();
}
