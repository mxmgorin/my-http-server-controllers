use super::{in_parameters::HttpInputParameter, out_results::HttpResult};

pub struct HttpActionDescription<'s> {
    pub controller_name: &'s str,
    pub summary: &'s str,
    pub description: &'s str,
    pub input_params: Option<Vec<HttpInputParameter>>,
    pub results: Vec<HttpResult>,
    pub authorized: Option<bool>,
}

pub trait HttpActionDescriptionProvider {
    fn get_description(&self) -> Option<HttpActionDescription>;
}
