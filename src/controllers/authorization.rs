pub enum ControllersAuthorization {
    BasicAuthentication { global: bool },
    ApiKeys { global: bool },
    BearerAuthentication { global: bool },
}

impl ControllersAuthorization {
    pub fn is_global_authorization(&self) -> bool {
        match self {
            ControllersAuthorization::BasicAuthentication { global } => *global,
            ControllersAuthorization::ApiKeys { global } => *global,
            ControllersAuthorization::BearerAuthentication { global } => *global,
        }
    }

    pub fn as_openid_str(&self) -> &str {
        match self {
            ControllersAuthorization::BasicAuthentication { global: _ } => "BasicAuth",
            ControllersAuthorization::ApiKeys { global: _ } => "ApiKeyAuth",
            ControllersAuthorization::BearerAuthentication { global: _ } => "BearerAuth",
        }
    }
}
