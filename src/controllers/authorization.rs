pub enum ControllersAuthorization {
    BasicAuthentication {
        global: bool,
        global_claims: Option<Vec<String>>,
    },
    ApiKeys {
        global: bool,
        global_claims: Option<Vec<String>>,
    },
    BearerAuthentication {
        global: bool,
        global_claims: Option<Vec<String>>,
    },
}

impl ControllersAuthorization {
    pub fn is_global_authorization(&self) -> bool {
        match self {
            ControllersAuthorization::BasicAuthentication {
                global,
                global_claims: _,
            } => *global,
            ControllersAuthorization::ApiKeys {
                global,
                global_claims: _,
            } => *global,
            ControllersAuthorization::BearerAuthentication {
                global,
                global_claims: _,
            } => *global,
        }
    }

    pub fn get_global_claims(&self) -> &Option<Vec<String>> {
        match self {
            ControllersAuthorization::BasicAuthentication {
                global: _,
                global_claims,
            } => global_claims,
            ControllersAuthorization::ApiKeys {
                global: _,
                global_claims,
            } => global_claims,
            ControllersAuthorization::BearerAuthentication {
                global: _,
                global_claims,
            } => global_claims,
        }
    }

    pub fn as_openid_str(&self) -> &str {
        match self {
            ControllersAuthorization::BasicAuthentication {
                global: _,
                global_claims: _,
            } => "BasicAuth",
            ControllersAuthorization::ApiKeys {
                global: _,
                global_claims: _,
            } => "ApiKeyAuth",
            ControllersAuthorization::BearerAuthentication {
                global: _,
                global_claims: _,
            } => "BearerAuth",
        }
    }
}
