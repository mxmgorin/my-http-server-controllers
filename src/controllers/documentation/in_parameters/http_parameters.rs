use rust_extensions::lazy::LazyVec;

use super::{HttpInputParameter, HttpParameterInputSource};

#[derive(Debug)]
pub struct HttpParameters {
    non_body_params: Option<Vec<HttpInputParameter>>,
    body_params: Option<Vec<HttpInputParameter>>,
}

impl HttpParameters {
    pub fn new(params: Option<Vec<HttpInputParameter>>) -> Self {
        println!("----");
        println!("Params: {:#?}", params);
        println!("----");
        if params.is_none() {
            return Self {
                non_body_params: None,
                body_params: None,
            };
        }

        let params = params.unwrap();

        let mut non_body_params = LazyVec::new();
        let mut body_params = LazyVec::new();

        for param in params {
            if param.source.is_body() {
                body_params.add(param);
            } else {
                non_body_params.add(param);
            }
        }

        let result = Self {
            body_params: body_params.get_result(),
            non_body_params: non_body_params.get_result(),
        };

        println!("Params: {:?}", result);
        println!("--------");
        result
    }

    pub fn get_non_body_params(&self) -> Option<&Vec<HttpInputParameter>> {
        self.non_body_params.as_ref()
    }

    pub fn get_body_params(&self) -> Option<&Vec<HttpInputParameter>> {
        self.body_params.as_ref()
    }

    pub fn is_single_body_parameter(&self) -> Option<&HttpInputParameter> {
        let params = self.body_params.as_ref()?;

        if params.len() != 1 {
            return None;
        }

        let param = params.get(0).unwrap();

        match &param.source {
            HttpParameterInputSource::BodyModel => Some(param),
            _ => None,
        }
    }
}

impl From<Vec<HttpInputParameter>> for HttpParameters {
    fn from(src: Vec<HttpInputParameter>) -> Self {
        if src.len() == 0 {
            return Self::new(None);
        } else {
            Self::new(Some(src))
        }
    }
}
