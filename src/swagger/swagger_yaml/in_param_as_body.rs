use crate::controllers::documentation::{
    in_parameters::HttpInputParameter, ArrayElement, HttpDataType, HttpField, HttpObjectStructure,
    HttpSimpleType,
};

use super::yaml_writer::YamlWriter;

pub fn write(yaml_writer: &mut YamlWriter, field: &HttpField) {
    match &field.data_type {
        HttpDataType::SimpleType(simple_type) => {
            yaml_writer.write_empty(field.name.as_str());
            write_body_simple_type(yaml_writer, simple_type);
        }
        HttpDataType::Object(object) => {
            yaml_writer.write_empty(field.name.as_str());
            write_body_object_type(yaml_writer, object);
        }
        HttpDataType::ArrayOf(array_el) => {
            yaml_writer.write_empty(field.name.as_str());
            write_body_array_type(yaml_writer, array_el);
        }
        HttpDataType::DictionaryOf(array_el) => match array_el {
            crate::controllers::documentation::ArrayElement::SimpleType(simple_type) => {
                yaml_writer.write_empty(field.name.as_str());
                yaml_writer.increase_level();
                yaml_writer.write("type", "object");
                yaml_writer.write_empty("additionalProperties");
                yaml_writer.increase_level();
                write_body_simple_type(yaml_writer, simple_type);
                yaml_writer.decrease_level();
                yaml_writer.decrease_level();
            }
            crate::controllers::documentation::ArrayElement::Object(obj) => {
                yaml_writer.write_empty(field.name.as_str());
                yaml_writer.increase_level();
                yaml_writer.write("type", "object");
                yaml_writer.write_empty("additionalProperties");
                yaml_writer.increase_level();
                yaml_writer.write_empty("items");
                yaml_writer.increase_level();
                write_body_object_type(yaml_writer, obj);
                yaml_writer.decrease_level();
                yaml_writer.decrease_level();
                yaml_writer.decrease_level();
            }
        },
        HttpDataType::DictionaryOfArray(array_el) => {
            yaml_writer.write_empty(field.name.as_str());
            yaml_writer.increase_level();
            yaml_writer.write("type", "object");
            yaml_writer.write_empty("additionalProperties");
            yaml_writer.increase_level();
            write_body_array_type(yaml_writer, array_el);
            yaml_writer.decrease_level();
            yaml_writer.decrease_level();
        }
        HttpDataType::Enum(_) => {}
        HttpDataType::None => {}
    }
}

fn write_body_simple_type(yaml_writer: &mut YamlWriter, simple_type: &HttpSimpleType) {
    yaml_writer.increase_level();
    yaml_writer.write("type", simple_type.as_swagger_type());

    yaml_writer.decrease_level();
}

fn write_body_object_type(yaml_writer: &mut YamlWriter, object: &HttpObjectStructure) {
    yaml_writer.increase_level();
    yaml_writer.write("type", "object");
    yaml_writer.write_empty("properties");
    yaml_writer.increase_level();
    for obj_field in &object.fields {
        write(yaml_writer, obj_field);
    }

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

fn write_body_array_type(yaml_writer: &mut YamlWriter, array_el: &ArrayElement) {
    match array_el {
        crate::controllers::documentation::ArrayElement::SimpleType(simple_type) => {
            yaml_writer.increase_level();
            yaml_writer.write("type", "array");
            yaml_writer.write_empty("items");
            write_body_simple_type(yaml_writer, simple_type);
            yaml_writer.decrease_level();
        }
        crate::controllers::documentation::ArrayElement::Object(obj) => {
            yaml_writer.increase_level();

            yaml_writer.write("type", "array");
            yaml_writer.write_empty("items");
            write_body_object_type(yaml_writer, obj);
            yaml_writer.decrease_level();
        }
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
            write(yaml_writer, &param.field);
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
            write(yaml_writer, &param.field);
        }
    }

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}
