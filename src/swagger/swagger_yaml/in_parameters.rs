use crate::controllers::documentation::{
    data_types::{EnumType, HttpDataType},
    in_parameters::HttpInputParameter,
    HttpActionDescription,
};

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, action_description: &HttpActionDescription) {
    yaml_writer.write_empty("parameters");
    yaml_writer.increase_level();

    if let Some(in_params) = &action_description.input_params {
        for param in in_params {
            build_parameter(yaml_writer, param);
        }
    }

    yaml_writer.decrease_level();
}

fn build_parameter(yaml_writer: &mut YamlWriter, param: &HttpInputParameter) {
    yaml_writer.write("- in", param.source.as_str());
    yaml_writer.increase_level();
    yaml_writer.write("description", param.description.as_str());

    if let HttpDataType::Enum(enum_struct) = &param.field.data_type {
        yaml_writer.write_array(
            "enum",
            enum_struct.cases.iter().map(|case| case.value.as_str()),
        );
        super::http_data_type::build(yaml_writer, "schema", &param.field.data_type);
    } else {
        super::http_data_type::build(yaml_writer, "schema", &param.field.data_type);
    }

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

    if let Some(param_format) = get_param_format(&param.field.data_type) {
        yaml_writer.write("format", param_format);
    }

    if let Some(param_type) = get_param_type(&param.field.data_type) {
        yaml_writer.write("type", param_type);
    }

    if let Some(default_value) = &param.field.default_value {
        let line_to_add = format!("{}. Default value: {}", param.description, default_value);
        yaml_writer.write("description", line_to_add.as_str());
    }

    yaml_writer.decrease_level();
}

fn get_param_format(data_type: &HttpDataType) -> Option<&str> {
    match data_type {
        HttpDataType::SimpleType(param_type) => Some(param_type.as_str()),
        HttpDataType::ObjectId { struct_id: _ } => None,
        HttpDataType::None => None,
        HttpDataType::ArrayOf(_) => None,
        HttpDataType::Object(_) => None,
        HttpDataType::Enum(_) => None,
    }
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
