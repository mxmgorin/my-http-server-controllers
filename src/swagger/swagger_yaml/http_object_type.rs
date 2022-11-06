use crate::controllers::documentation::data_types::HttpObjectStructure;

use super::yaml_writer::YamlWriter;

pub fn build(result: &mut YamlWriter, http_object: &HttpObjectStructure) {
    result.increase_level();
    result.write_empty(http_object.struct_id.as_str());
    result.increase_level();
    result.write("type", "object");

    result.write_array(
        "required",
        http_object
            .fields
            .iter()
            .filter(|itm| itm.required)
            .map(|itm| itm.name.as_str()),
    );

    write_properties(result, http_object);

    result.decrease_level();
    result.decrease_level();
    //todo!("Delete this line");
    //result.write_object("properties", compile_properties(http_object));
}

fn write_properties(yaml_writer: &mut YamlWriter, src: &HttpObjectStructure) {
    yaml_writer.write_empty("properties");

    yaml_writer.increase_level();

    for field in &src.fields {
        super::http_data_type::build(yaml_writer, field.name.as_str(), &field.data_type);
    }

    yaml_writer.decrease_level();
}
