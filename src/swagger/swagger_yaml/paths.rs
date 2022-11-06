use std::collections::BTreeMap;

use crate::{
    controllers::{documentation::HttpActionDescription, ControllersMiddleware},
    swagger::json_object_writer::JsonObjectWriter,
};

use super::yaml_writer::YamlWriter;

pub fn build(
    yaml_writer: &mut YamlWriter,
    controllers: &ControllersMiddleware,
    actions: &BTreeMap<String, BTreeMap<String, HttpActionDescription>>,
) {
    yaml_writer.reset_level();
    yaml_writer.write_empty("paths");

    yaml_writer.increase_level();

    for (path, actions) in actions {
        yaml_writer.write_empty(path);
        yaml_writer.increase_level();

        for (verb, action_description) in actions {
            super::verb_description::build(yaml_writer, verb, controllers, action_description)
        }

        yaml_writer.decrease_level();
    }

    yaml_writer.decrease_level();
}
