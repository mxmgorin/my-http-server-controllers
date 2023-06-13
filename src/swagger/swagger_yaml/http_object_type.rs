use crate::controllers::documentation::data_types::HttpObjectStructure;

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, http_object: &HttpObjectStructure) {
    yaml_writer.write_upper_level(http_object.struct_id, |yaml_writer| {
        yaml_writer.write("type", "object");

        yaml_writer.write_array(
            "required",
            http_object
                .fields
                .iter()
                .filter(|itm| itm.required)
                .map(|itm| itm.name.as_str().into()),
        );

        write_properties(yaml_writer, http_object);
    });
}

fn write_properties(yaml_writer: &mut YamlWriter, src: &HttpObjectStructure) {
    yaml_writer.write_upper_level("properties", |yaml_writer| {
        for field in &src.fields {
            super::http_data_type::build(yaml_writer, field.name.as_str(), &field.data_type);
        }
    });
}
