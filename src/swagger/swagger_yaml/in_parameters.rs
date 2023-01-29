use crate::controllers::documentation::{
    data_types::HttpDataType, in_parameters::HttpInputParameter, HttpActionDescription,
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
            yaml_writer.increase_level();
            yaml_writer.write("- in", param.source.as_str());
            yaml_writer.increase_level();
            write_query_input_param(yaml_writer, param);
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
    }

    if let Some(body_params) = action_description.input_params.get_body_params() {
        yaml_writer.write_empty("requestBody");
        yaml_writer.increase_level();
        yaml_writer.write_bool("required", true);
        yaml_writer.write_empty("content");
        yaml_writer.increase_level();
        //yaml_writer.write_empty("application/x-www-form-urlencoded");
        yaml_writer.write_empty("application/json");
        yaml_writer.increase_level();
        yaml_writer.write_empty("schema");
        yaml_writer.increase_level();
        yaml_writer.write("type", "object");
        yaml_writer.write_empty("properties");
        yaml_writer.increase_level();

        for param in body_params {
            write_body_input_param(yaml_writer, param);
        }
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
        yaml_writer.decrease_level();
    }
}

fn write_query_input_param(yaml_writer: &mut YamlWriter, input_param: &HttpInputParameter) {
    match &input_param.field.data_type {
        HttpDataType::SimpleType(simple_type) => {
            yaml_writer.increase_level();
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.write_empty("schema");
            yaml_writer.increase_level();
            yaml_writer.write("type", simple_type.as_swagger_type());
            yaml_writer.write("format", simple_type.as_format());
            yaml_writer.decrease_level();
            yaml_writer.write("required", "true");
            yaml_writer.decrease_level();
        }
        HttpDataType::Object(object) => {
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.increase_level();
            yaml_writer.write_empty("schema");
            yaml_writer.increase_level();
            yaml_writer.write("$ref", object.struct_id);
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
        HttpDataType::ArrayOf(_) => {}
        HttpDataType::DictionaryOf(_) => {}
        HttpDataType::DictionaryOfArray(_) => {}
        HttpDataType::Enum(_) => {}
        HttpDataType::None => {}
    }
}

fn write_body_input_param(yaml_writer: &mut YamlWriter, input_param: &HttpInputParameter) {
    match &input_param.field.data_type {
        HttpDataType::SimpleType(simple_type) => {
            yaml_writer.increase_level();
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.increase_level();
            yaml_writer.write("type", simple_type.as_swagger_type());
            yaml_writer.write("format", simple_type.as_format());
            yaml_writer.write("required", "true");
            yaml_writer.decrease_level();
        }
        HttpDataType::Object(object) => {
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.increase_level();
            yaml_writer.write_empty("schema");
            yaml_writer.increase_level();
            yaml_writer.write("$ref", object.struct_id);
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
        HttpDataType::ArrayOf(_) => {}
        HttpDataType::DictionaryOf(_) => {}
        HttpDataType::DictionaryOfArray(_) => {}
        HttpDataType::Enum(_) => {}
        HttpDataType::None => {}
    }
}

/*
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
 */
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
            write_body_input_param(yaml_writer, &param);
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
            write_body_input_param(yaml_writer, &param);
        }
    }

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}
