use rust_extensions::lazy::LazyVec;

use super::{HttpInputParameter, HttpParameterInputSource};

pub struct HttpParameters {
    params: Option<Vec<HttpInputParameter>>,
}

impl HttpParameters {
    pub fn new(params: Option<Vec<HttpInputParameter>>) -> Self {
        Self { params }
    }

    pub fn get_all(&self) -> &Option<Vec<HttpInputParameter>> {
        &self.params
    }

    pub fn get_filtered<TFilter: Fn(&HttpInputParameter) -> bool>(
        &self,
        filter: TFilter,
    ) -> Option<Vec<&HttpInputParameter>> {
        let params = self.params.as_ref()?;

        let mut result = LazyVec::new();

        for param in params {
            if filter(param) {
                result.add(param);
            }
        }

        result.get_result()
    }

    pub fn is_single_body_parameter(&self) -> Option<&HttpInputParameter> {
        let params = self.params.as_ref()?;
        if params.len() != 1 {
            return None;
        }

        let param = params.get(0).unwrap();

        match &param.source {
            HttpParameterInputSource::Body => Some(param),
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
