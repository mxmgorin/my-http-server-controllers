use my_http_server::WebContentType;

use crate::controllers::documentation::{
    data_types::HttpDataType, out_results::HttpResult, HttpActionDescription,
};

use crate::controllers::ControllersMiddleware;

use super::yaml_writer::YamlWriter;

pub fn build(
    yaml_writer: &mut YamlWriter,
    verb: &str,
    action_description: &HttpActionDescription,
    controllers: &ControllersMiddleware,
) {
    yaml_writer.write_empty(verb);

    yaml_writer.increase_level();

    if let Some(authorization) = &controllers.authorization_map.global_authorization {
        let mut should_be_authorized = authorization.is_global_authorization_enabled();

        match &action_description.should_be_authorized {
            crate::controllers::documentation::ShouldBeAuthorized::Yes => {
                should_be_authorized = true;
            }
            crate::controllers::documentation::ShouldBeAuthorized::YesWithClaims(_) => {
                should_be_authorized = true;
            }
            crate::controllers::documentation::ShouldBeAuthorized::No => {
                should_be_authorized = false;
            }
            crate::controllers::documentation::ShouldBeAuthorized::UseGlobal => {}
        }

        if should_be_authorized {
            yaml_writer.write_empty("security");

            yaml_writer.write(
                format!(" - {}", authorization.as_openid_str()).as_str(),
                "[]",
            );
        }
    }

    yaml_writer.write_array(
        "tags",
        [action_description.controller_name]
            .into_iter()
            .map(|itm| itm.into()),
    );

    yaml_writer.write("summary", action_description.summary);

    yaml_writer.write("description", action_description.description);

    compile_produces(yaml_writer, action_description);

    super::in_parameters::build(yaml_writer, &action_description);

    compile_responses(yaml_writer, &action_description.results);

    yaml_writer.decrease_level();
}

fn compile_produces(yaml_writer: &mut YamlWriter, action_description: &HttpActionDescription) {
    let mut produces = Vec::new();

    for http_result in &action_description.results {
        let produce_type = match http_result.data_type {
            HttpDataType::SimpleType(_) => Some(WebContentType::Text.as_str()),
            HttpDataType::DictionaryOf(_) => Some(WebContentType::Json.as_str()),
            HttpDataType::Object(_) => Some(WebContentType::Json.as_str()),
            HttpDataType::DictionaryOfArray(_) => Some(WebContentType::Json.as_str()),
            HttpDataType::None => None,
            HttpDataType::ArrayOf(_) => None,
            HttpDataType::Enum(_) => None,
        };

        if let Some(produce_type) = produce_type {
            if !produces.iter().any(|itm| itm == produce_type) {
                produces.push(produce_type.to_string());
            }
        }
    }

    yaml_writer.write_array("produces", produces.iter().map(|itm| itm.as_str().into()));
}

fn compile_responses(yaml_writer: &mut YamlWriter, results: &[HttpResult]) {
    yaml_writer.write_empty("responses");
    yaml_writer.increase_level();

    for http_result in results {
        yaml_writer.write_empty(format!("{}", http_result.http_code).as_str());
        yaml_writer.increase_level();
        compile_response(yaml_writer, http_result);
        yaml_writer.decrease_level();
    }

    yaml_writer.decrease_level();
}

fn compile_response(yaml_writer: &mut YamlWriter, src: &HttpResult) {
    yaml_writer.write("description", src.description.as_str());

    if src.data_type.is_none() {
        return;
    }

    yaml_writer.write_empty("content");
    yaml_writer.increase_level();
    yaml_writer.write_empty("application/json");
    yaml_writer.increase_level();
    super::http_data_type::build(yaml_writer, "schema", &src.data_type);

    yaml_writer.decrease_level();
    yaml_writer.decrease_level();
}
