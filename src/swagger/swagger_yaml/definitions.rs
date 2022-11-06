use std::collections::{BTreeMap, HashMap};

use crate::controllers::{
    documentation::{
        data_types::{ArrayElement, HttpDataType, HttpObjectStructure},
        HttpActionDescription,
    },
    ControllersMiddleware,
};

use super::yaml_writer::YamlWriter;

pub fn build_and_write(
    yaml_writer: &mut YamlWriter,
    controllers: &ControllersMiddleware,
    path_descriptions: &BTreeMap<String, BTreeMap<String, HttpActionDescription>>,
) {
    let mut definitions = HashMap::new();

    yaml_writer.write_empty("schemas");
    yaml_writer.increase_level();

    for http_object in &controllers.http_objects {
        if !definitions.contains_key(http_object.struct_id.as_str()) {
            super::http_object_type::build(yaml_writer, http_object);

            println!("Populating from http_object: {}", http_object.struct_id);

            definitions.insert(http_object.struct_id.to_string(), ());
        }
    }

    for (_, action_descriptions) in path_descriptions {
        for (_, action_description) in action_descriptions {
            for result in &action_description.results {
                populate_object_type(yaml_writer, &mut definitions, &result.data_type);
            }

            if let Some(input_parameters) = &action_description.input_params {
                for in_param in input_parameters {
                    populate_object_type(yaml_writer, &mut definitions, &in_param.field.data_type);
                }
            }
        }
    }

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

fn populate_object_type(
    yaml_writer: &mut YamlWriter,
    definitions: &mut HashMap<String, ()>,
    data_type: &HttpDataType,
) {
    match data_type {
        HttpDataType::SimpleType(_) => {}
        HttpDataType::Object(object_type) => {
            write_object_type(yaml_writer, definitions, object_type);
        }
        HttpDataType::ObjectId { struct_id: _ } => {}
        HttpDataType::ArrayOf(array_element) => {
            populate_array_type(yaml_writer, definitions, array_element);
        }
        HttpDataType::Enum(enum_structure) => {
            write_enum_type(yaml_writer, definitions, enum_structure);
        }
        HttpDataType::None => {}
    }
}

fn populate_array_type(
    yaml_writer: &mut YamlWriter,
    definitions: &mut HashMap<String, ()>,
    array_element: &ArrayElement,
) {
    match array_element {
        ArrayElement::SimpleType(_) => {}
        ArrayElement::ObjectId { struct_id: _ } => {}
        ArrayElement::Object(object_type) => {
            write_object_type(yaml_writer, definitions, object_type)
        }
    }
}

fn write_object_type(
    yaml_writer: &mut YamlWriter,
    definitions: &mut HashMap<String, ()>,
    object_type: &HttpObjectStructure,
) {
    if !definitions.contains_key(object_type.struct_id.as_str()) {
        super::http_object_type::build(yaml_writer, object_type);
        definitions.insert(object_type.struct_id.to_string(), ());
    }

    for field in &object_type.fields {
        populate_object_type(yaml_writer, definitions, &field.data_type);
    }
}

fn write_enum_type(
    yaml_writer: &mut YamlWriter,
    definitions: &mut HashMap<String, ()>,
    enum_structure: &crate::controllers::documentation::data_types::HttpEnumStructure,
) {
    if definitions.contains_key(enum_structure.struct_id.as_str()) {
        return;
    };

    yaml_writer.write_empty(enum_structure.struct_id.as_ref());

    super::http_enum_type::build(yaml_writer, enum_structure);

    definitions.insert(enum_structure.struct_id.to_string(), ());
}
