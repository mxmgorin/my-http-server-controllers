use crate::controllers::ControllersAuthorization;

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, auth: &ControllersAuthorization) {
    yaml_writer.write_empty("securitySchemes");

    match auth {
        ControllersAuthorization::BasicAuthentication {
            global: _,
            global_claims: _,
        } => {
            yaml_writer.increase_level();
            yaml_writer.write_empty("BasicAuth");
            yaml_writer.increase_level();
            yaml_writer.write("type", "http");
            yaml_writer.write("scheme", "basic");
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
        ControllersAuthorization::ApiKeys {
            global: _,
            global_claims: _,
        } => {
            yaml_writer.increase_level();
            yaml_writer.write_empty("ApiKeyAuth");
            yaml_writer.increase_level();
            yaml_writer.write("type", "apiKey");
            yaml_writer.write("in", "header");
            yaml_writer.write("name", "X-API-Key");
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
        ControllersAuthorization::BearerAuthentication {
            global: _,
            global_claims: _,
        } => {
            yaml_writer.increase_level();
            yaml_writer.write_empty("BearerAuth");
            yaml_writer.increase_level();
            yaml_writer.write("type", "http");
            yaml_writer.write("scheme", "bearer");
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
    }
}
