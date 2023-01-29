use crate::controllers::documentation::{
    data_types::{EnumType, HttpDataType},
    in_parameters::HttpInputParameter,
    HttpActionDescription,
};

use super::yaml_writer::YamlWriter;

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
            yaml_writer.write("- in", param.source.as_str());
            build_parameter(yaml_writer, param);
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
            build_parameter(yaml_writer, param);
        }
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
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
        HttpDataType::DictionaryOf(_) => None,

        HttpDataType::None => None,
        HttpDataType::ArrayOf(_) => None,
        HttpDataType::Object(_) => None,
        HttpDataType::Enum(data) => match &data.enum_type {
            EnumType::Integer => Some("integer"),
            EnumType::String => Some("string"),
        },
        HttpDataType::DictionaryOfArray(_) => None,
    }
}

fn build_req_body_as_file_to_upload(yaml_writer: &mut YamlWriter) {
    yaml_writer.write_empty("requestBody");
    yaml_writer.increase_level();
    yaml_writer.write_empty("content");
    yaml_writer.increase_level();
    yaml_writer.write_empty("application/octet-stream");
    yaml_writer.increase_level();
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "string");
    yaml_writer.write("format", "binary");
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

fn build_req_body_model_reader(
    yaml_writer: &mut YamlWriter,
    in_params: &Vec<HttpInputParameter>,
    encoding_type: &str,
) {
    yaml_writer.write_empty("requestBody");
    yaml_writer.increase_level();
    yaml_writer.write_empty("content");
    yaml_writer.increase_level();

    yaml_writer.write_empty(encoding_type);
    yaml_writer.increase_level();
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "object");
    yaml_writer.write_empty("properties");
    yaml_writer.increase_level();
    for param in in_params {
        if param.is_body_reader() {
            yaml_writer.write_empty(&param.field.name);
            yaml_writer.increase_level();
            if let Some(param_type) = get_param_type(&param.field.data_type) {
                yaml_writer.write("type", param_type);
                yaml_writer.write("required", "true");

                if let HttpDataType::SimpleType(simple_type) = &param.field.data_type {
                    yaml_writer.write("format", simple_type.as_format());
                }
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

fn build_req_body_form_data(yaml_writer: &mut YamlWriter, in_params: &Vec<HttpInputParameter>) {
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
    yaml_writer.increase_level();
    for param in in_params {
        if param.is_form_data() {
            yaml_writer.write_empty(&param.field.name);
            yaml_writer.increase_level();
            if let Some(param_type) = get_param_type(&param.field.data_type) {
                yaml_writer.write("type", param_type);
                yaml_writer.write("required", "true");

                if let HttpDataType::SimpleType(simple_type) = &param.field.data_type {
                    yaml_writer.write("format", simple_type.as_format());
                }
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
