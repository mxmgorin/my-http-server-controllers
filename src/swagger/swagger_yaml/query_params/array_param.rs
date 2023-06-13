use crate::{
    controllers::documentation::{HttpEnumStructure, HttpSimpleType},
    swagger::swagger_yaml::yaml_writer::YamlWriter,
};

pub fn write_array_input_param(yaml_writer: &mut YamlWriter, simple_type: &HttpSimpleType) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "array");
    yaml_writer.write_empty("items");
    yaml_writer.increase_level();
    yaml_writer.write("type", simple_type.as_swagger_type());
    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}

pub fn write_array_enum_case(yaml_writer: &mut YamlWriter, enum_structure: &HttpEnumStructure) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", "array");
    yaml_writer.write_empty("items");
    yaml_writer.increase_level();

    yaml_writer.write_empty("schema");

    super::super::http_enum_type::build(yaml_writer, enum_structure);
    //super::write_enum_type(yaml_writer, enum_structure);

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}
