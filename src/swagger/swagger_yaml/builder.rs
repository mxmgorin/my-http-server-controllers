use std::collections::BTreeMap;

use crate::controllers::{documentation::HttpActionDescription, ControllersMiddleware};

use super::yaml_writer::YamlWriter;

pub fn build(
    controllers: &ControllersMiddleware,
    title: &str,
    version: &str,
    host: &str,
    scheme: &str,
) -> Vec<u8> {
    let mut yaml_writer = YamlWriter::new();

    yaml_writer.write("openapi", "3.0.0");

    yaml_writer.write_empty("info");

    yaml_writer.increase_level();
    yaml_writer.write("title", title);
    yaml_writer.write("version", version);

    yaml_writer.reset_level();
    yaml_writer.write_empty("servers");
    yaml_writer.increase_level();
    yaml_writer.write("- url", format!("{}://{}", scheme, host).as_str());

    let path_descriptions = build_paths_descriptions(controllers);

    super::definitions::build_and_write(&mut yaml_writer, controllers, &path_descriptions);

    super::paths::build(&mut yaml_writer, controllers, &path_descriptions);

    if let Some(authorization) = controllers.authorization.as_ref() {

        /*
        todo!("Uncomment");
        if authorization.is_global_authorization() {
            json_object_writer.write_raw("security", "[{\"Bearer\": []}]");
        }

        json_object_writer.write_object("securityDefinitions", super::security_defentions::build());
         */
    }

    yaml_writer.build()
}

fn build_paths_descriptions(
    controllers: &ControllersMiddleware,
) -> BTreeMap<String, BTreeMap<String, HttpActionDescription>> {
    let mut result = BTreeMap::new();

    for route_action in controllers.list_of_get_route_actions() {
        if let Some(description) = route_action.description.get_description() {
            if !result.contains_key(route_action.http_route.route.as_str()) {
                result.insert(route_action.http_route.route.to_string(), BTreeMap::new());
            }

            result
                .get_mut(route_action.http_route.route.as_str())
                .unwrap()
                .insert("get".to_string(), description);
        }
    }

    for route_action in controllers.list_of_post_route_actions() {
        if let Some(description) = route_action.description.get_description() {
            if !result.contains_key(route_action.http_route.route.as_str()) {
                result.insert(route_action.http_route.route.to_string(), BTreeMap::new());
            }

            result
                .get_mut(route_action.http_route.route.as_str())
                .unwrap()
                .insert("post".to_string(), description);
        }
    }

    for route_action in controllers.list_of_put_route_actions() {
        if let Some(description) = route_action.description.get_description() {
            if !result.contains_key(route_action.http_route.route.as_str()) {
                result.insert(route_action.http_route.route.to_string(), BTreeMap::new());
            }

            result
                .get_mut(route_action.http_route.route.as_str())
                .unwrap()
                .insert("put".to_string(), description);
        }
    }

    for route_action in controllers.list_of_delete_route_actions() {
        if let Some(description) = route_action.description.get_description() {
            if !result.contains_key(route_action.http_route.route.as_str()) {
                result.insert(route_action.http_route.route.to_string(), BTreeMap::new());
            }

            result
                .get_mut(route_action.http_route.route.as_str())
                .unwrap()
                .insert("delete".to_string(), description);
        }
    }

    result
}
