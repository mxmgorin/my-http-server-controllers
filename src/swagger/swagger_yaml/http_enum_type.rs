use crate::controllers::documentation::data_types::{EnumType, HttpEnumStructure};

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, enum_structure: &HttpEnumStructure) {
    yaml_writer.increase_level();
    match enum_structure.enum_type {
        EnumType::Integer => {
            yaml_writer.write("type", "integer");
        }
        EnumType::String => {
            yaml_writer.write("type", "string");
        }
    }

    yaml_writer.write_array("enum", enum_structure.cases.iter().map(|case| case.value));

    yaml_writer.write("description", "|");

    write_enum_description(yaml_writer, enum_structure);

    yaml_writer.decrease_level();
}

fn write_enum_description(yaml_writer: &mut YamlWriter, enum_structure: &HttpEnumStructure) {
    yaml_writer.increase_level();

    for case in &enum_structure.cases {
        yaml_writer.write_empty(format!("* {} [{}]", case.id, case.description).as_str());
    }

    yaml_writer.decrease_level();
}
