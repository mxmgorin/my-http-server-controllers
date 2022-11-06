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

    let mut result = JsonObjectWriter::as_object();
    for (path, actions) in actions {
        let mut path_object = JsonObjectWriter::as_object();
        for (verb, action_description) in actions {
            super::verb_description::build(yaml_writer, verb, controllers, action_description)
        }
        result.write_object(path.as_str(), path_object);
    }

    yaml_writer.decrease_level();
}
