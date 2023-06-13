use crate::controllers::documentation::HttpActionDescription;

use super::{in_param_as_body, in_param_as_from_data, yaml_writer::YamlWriter};

pub fn build(yaml_writer: &mut YamlWriter, action_description: &HttpActionDescription) {
    if let Some(body_param) = action_description.input_params.is_single_body_parameter() {
        yaml_writer.write_empty("requestBody");
        yaml_writer.increase_level();
        yaml_writer.write("description", body_param.description.as_str());
        yaml_writer.write_bool("required", true);
        yaml_writer.write_empty("content");
        yaml_writer.increase_level();
        yaml_writer.write_empty("application/json");
        yaml_writer.increase_level();
        super::http_data_type::build(yaml_writer, "schema", &body_param.field.data_type);
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        return;
    }

    if let Some(non_body_params) = action_description.input_params.get_non_body_params() {
        yaml_writer.write_empty("parameters");
        for param in non_body_params {
            yaml_writer.increase_level();
            yaml_writer.write_upper_level("- in", |upper_level| {
                super::query_params::write_query_input_param(upper_level, param);
            });

            yaml_writer.decrease_level();
        }
    }

    if let Some(body_params) = action_description.input_params.get_body_params() {
        yaml_writer.write_empty("requestBody");
        yaml_writer.increase_level();
        yaml_writer.write_bool("required", true);
        yaml_writer.write_empty("content");
        yaml_writer.increase_level();
        yaml_writer.write_empty("application/json");
        yaml_writer.increase_level();
        yaml_writer.write_empty("schema");
        yaml_writer.increase_level();
        yaml_writer.write("type", "object");
        yaml_writer.write_empty("properties");
        yaml_writer.increase_level();

        for param in body_params {
            in_param_as_body::write(yaml_writer, &param.field);
        }
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
    }

    if let Some(form_data_params) = action_description.input_params.get_form_data_params() {
        yaml_writer.write_empty("requestBody");
        yaml_writer.increase_level();
        yaml_writer.write_empty("content");
        yaml_writer.increase_level();
        yaml_writer.write_empty("multipart/form-data");
        yaml_writer.increase_level();
        yaml_writer.write_empty("schema");
        yaml_writer.increase_level();
        yaml_writer.write("type", "object");
        yaml_writer.write_empty("properties");

        let mut objects = Vec::new();
        yaml_writer.increase_level();

        for param in form_data_params {
            in_param_as_from_data::write(yaml_writer, &param.field);

            if param.field.data_type.is_object() {
                objects.push(param);
            }
        }
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();

        if objects.len() > 0 {
            yaml_writer.write_empty("encoding");
            yaml_writer.increase_level();
            for obj in objects {
                yaml_writer.write_empty(obj.field.name.as_str());
                yaml_writer.increase_level();
                yaml_writer.write("contentType", "application/json");
                yaml_writer.decrease_level();
            }

            yaml_writer.decrease_level();
        }

        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
    }
}
