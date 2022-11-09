use crate::controllers::documentation::{
    data_types::{EnumType, HttpDataType},
    in_parameters::HttpInputParameter,
    HttpActionDescription,
};

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, action_description: &HttpActionDescription) {
    if let Some(in_params) = &action_description.input_params {
        let mut has_form_data = false;

        let mut parameters_is_set = false;

        for param in in_params {
            if param.source.is_form_data() {
                has_form_data = true;
            } else {
                if !parameters_is_set {
                    yaml_writer.write_empty("parameters");
                    parameters_is_set = true;
                }
                yaml_writer.write("- in", param.source.as_str());
                build_parameter(yaml_writer, param);
            }
        }

        if has_form_data {
            yaml_writer.write_empty("requestBody");
            yaml_writer.increase_level();
            yaml_writer.write_empty("content");
            yaml_writer.increase_level();

            yaml_writer.write_empty("application/x-www-form-urlencoded");
            yaml_writer.increase_level();
            yaml_writer.write_empty("schema");
            yaml_writer.increase_level();
            yaml_writer.write("type", "object");
            yaml_writer.write_empty("properties");
            yaml_writer.increase_level();
            for param in in_params {
                if param.source.is_form_data() {
                    yaml_writer.write_empty(&param.field.name);
                    yaml_writer.increase_level();
                    if let Some(param_type) = get_param_type(&param.field.data_type) {
                        yaml_writer.write("type", param_type);
                        yaml_writer.write("required", "true");
                    }
                    yaml_writer.decrease_level();
                }
            }

            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
    }
}

fn build_parameter(yaml_writer: &mut YamlWriter, param: &HttpInputParameter) {
    yaml_writer.increase_level();
    yaml_writer.increase_level();
    yaml_writer.write("description", param.description.as_str());

    if param.source.is_query() {
        if param.field.data_type.is_array() {
            yaml_writer.write("name", format!("{}[]", param.field.name.as_str()).as_str());
        } else {
            yaml_writer.write("name", param.field.name.as_str());
        }
    } else {
        yaml_writer.write("name", param.field.name.as_str());
    }

    if param.field.required && param.field.default_value.is_none() {
        yaml_writer.write_bool("required", true);
    }

    if let Some(param_type) = get_param_type(&param.field.data_type) {
        yaml_writer.write("type", param_type);
    }

    if let Some(default_value) = &param.field.default_value {
        let line_to_add = format!("{}. Default value: {}", param.description, default_value);
        yaml_writer.write("description", line_to_add.as_str());
    }

    super::http_data_type::build(yaml_writer, "schema", &param.field.data_type);

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

fn get_param_type(data_type: &HttpDataType) -> Option<&str> {
    match data_type {
        HttpDataType::SimpleType(param_type) => Some(param_type.as_swagger_type()),
        HttpDataType::ObjectId { struct_id: _ } => None,
        HttpDataType::None => None,
        HttpDataType::ArrayOf(_) => None,
        HttpDataType::Object(_) => None,
        HttpDataType::Enum(data) => match &data.enum_type {
            EnumType::Integer => Some("integer"),
            EnumType::String => Some("string"),
        },
    }
}
