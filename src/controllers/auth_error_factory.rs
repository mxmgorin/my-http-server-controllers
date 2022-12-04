use my_http_server::HttpFailResult;

pub trait AuthErrorFactory {
    fn get_not_authenticated(&self) -> HttpFailResult;
    fn get_not_authorized(&self, claim_name: &str) -> HttpFailResult;
}
