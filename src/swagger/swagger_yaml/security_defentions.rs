use crate::{controllers::ControllersAuthorization, swagger::json_object_writer::JsonObjectWriter};

pub fn build(auth: &ControllersAuthorization) -> JsonObjectWriter {
    let mut security_fileds = JsonObjectWriter::as_object();

    match auth {
        ControllersAuthorization::BasicAuthentication { global } => {
            security_fileds.write_string_value("type", "apiKey");
            security_fileds.write_string_value("description", "Bearer Token");
            security_fileds.write_string_value("name", "Authorization");
            security_fileds.write_string_value("in", "header");

            let mut result = JsonObjectWriter::as_object();
            result.write_object("Bearer", security_fileds);
            result
        }
        ControllersAuthorization::ApiKeys { global } => todo!(),
        ControllersAuthorization::BearerAuthentication { global } => {
            security_fileds.write_string_value("type", "apiKey");
            security_fileds.write_string_value("description", "Bearer Token");
            security_fileds.write_string_value("name", "Authorization");
            security_fileds.write_string_value("in", "header");

            let mut result = JsonObjectWriter::as_object();
            result.write_object("Bearer", security_fileds);
            result
        }
    }
}
