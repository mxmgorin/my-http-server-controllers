use crate::controllers::documentation::data_types::{EnumType, HttpEnumStructure};

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, enum_structure: &HttpEnumStructure) {
    yaml_writer.increase_level();
    match enum_structure.enum_type {
        EnumType::Integer => {
            yaml_writer.write("type", "integer");

            yaml_writer.write_array_with_strings(
                "enum",
                enum_structure
                    .cases
                    .iter()
                    .map(|case| format!("{} [{}]", case.id, case.value.as_str())),
            );
        }
        EnumType::String => {
            yaml_writer.write("type", "string");

            yaml_writer.write_array(
                "enum",
                enum_structure.cases.iter().map(|case| case.value.as_str()),
            );
        }
    }

    // result.write_string_value("description", compile_description(enum_structure).as_str());

    yaml_writer.decrease_level();
}

/*
fn compile_description(enum_structure: &HttpEnumStructure) -> String {
    let mut result = String::new();

    let mut first = true;

    for case in &enum_structure.cases {
        if first {
            first = false;
        } else {
            result.push_str("\\n");
        }

        result.push_str(format!("{} = {}", case.id, case.description).as_str());
    }

    result
}
 */
