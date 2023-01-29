use crate::controllers::documentation::{
    data_types::HttpDataType, in_parameters::HttpInputParameter, HttpActionDescription,
    HttpEnumStructure, HttpSimpleType,
};

use super::{in_param_as_body, yaml_writer::YamlWriter};

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
            in_param_as_body::write(yaml_writer, &param.field);
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
            yaml_writer.write("description", input_param.description.as_str());
            write_simple_type(yaml_writer, simple_type);
            yaml_writer.decrease_level();
        }
        HttpDataType::Object(_) => {
            panic!("Object type is not supported for non body parameter")
        }
        HttpDataType::ArrayOf(array_el) => match array_el {
            crate::controllers::documentation::ArrayElement::SimpleType(simple_type) => {
                yaml_writer.increase_level();
                yaml_writer.write(
                    "name",
                    format!("{}[]", input_param.field.name.as_str()).as_str(),
                );
                yaml_writer.write("description", input_param.description.as_str());
                write_array_input_paramt(yaml_writer, simple_type);
                yaml_writer.decrease_level();
            }
            crate::controllers::documentation::ArrayElement::Object(_) => {
                panic!("Array of object type is not supported for non body parameter")
            }
            crate::controllers::documentation::ArrayElement::Enum(enum_case) => {
                yaml_writer.increase_level();
                yaml_writer.write(
                    "name",
                    format!("{}[]", input_param.field.name.as_str()).as_str(),
                );
                yaml_writer.write("description", input_param.description.as_str());
                write_array_enum_case(yaml_writer, enum_case);
                yaml_writer.decrease_level();
            }
        },
        HttpDataType::DictionaryOf(_) => {
            panic!("Dictionary can not be used as a non body parameter")
        }
        HttpDataType::DictionaryOfArray(_) => {
            panic!("Dictionary of array can not be used as a non body parameter")
        }
        HttpDataType::Enum(enum_data) => {
            yaml_writer.increase_level();
            yaml_writer.write("name", input_param.field.name.as_str());
            yaml_writer.write("description", input_param.description.as_str());
            write_enum_case(yaml_writer, enum_data);
            yaml_writer.decrease_level();
        }
        HttpDataType::None => {
            panic!("Somehow we have non parameter")
        }
    }
}

fn write_simple_type(yaml_writer: &mut YamlWriter, simple_type: &HttpSimpleType) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", simple_type.as_swagger_type());
    yaml_writer.write("format", simple_type.as_format());
    yaml_writer.decrease_level();
    yaml_writer.write("required", "true");
}

fn write_array_input_paramt(yaml_writer: &mut YamlWriter, simple_type: &HttpSimpleType) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "array");
    yaml_writer.write_empty("items");
    yaml_writer.increase_level();
    yaml_writer.write("type", simple_type.as_swagger_type());
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

fn write_enum_case(yaml_writer: &mut YamlWriter, enum_data: &HttpEnumStructure) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "string");
    yaml_writer.write_array("enum", enum_data.cases.iter().map(|itm| itm.value));
    yaml_writer.decrease_level();
}

fn write_array_enum_case(yaml_writer: &mut YamlWriter, enum_data: &HttpEnumStructure) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "array");
    yaml_writer.write_empty("items");
    yaml_writer.increase_level();
    yaml_writer.write(
        "$ref",
        format!("'#/components/schemas/{}'", enum_data.struct_id).as_str(),
    );
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}
