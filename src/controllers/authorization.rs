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
}
