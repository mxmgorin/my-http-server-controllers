use crate::{
    controllers::documentation::HttpSimpleType, swagger::swagger_yaml::yaml_writer::YamlWriter,
};

pub fn write_simple_type(
    yaml_writer: &mut YamlWriter,
    simple_type: &HttpSimpleType,
    required: bool,
) {
    yaml_writer.write_empty("schema");
    yaml_writer.increase_level();
    yaml_writer.write("type", simple_type.as_swagger_type());
    yaml_writer.write("format", simple_type.as_format());
    yaml_writer.decrease_level();
    yaml_writer.write_bool("required", required);
}
