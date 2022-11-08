use my_http_server::RequestCredentials;

use super::{
    actions::GetShouldBeAuthoriazed, documentation::ShouldBeAuthorized, ControllersAuthorization,
};

pub enum AuthorizationResult {
    Allowed,
    NotAuthenticated,
    NotAuthorized,
}

impl AuthorizationResult {
    pub fn is_allowed(&self) -> bool {
        match self {
            AuthorizationResult::Allowed => true,
            AuthorizationResult::NotAuthenticated => false,
            AuthorizationResult::NotAuthorized => false,
        }
    }

    pub fn not_authenticated(&self) -> bool {
        match self {
            AuthorizationResult::Allowed => false,
            AuthorizationResult::NotAuthenticated => true,
            AuthorizationResult::NotAuthorized => false,
        }
    }

    pub fn not_authorized(&self) -> bool {
        match self {
            AuthorizationResult::Allowed => false,
            AuthorizationResult::NotAuthenticated => false,
            AuthorizationResult::NotAuthorized => true,
        }
    }
}

pub struct AuthorizationMap {
    pub global_authorization: Option<ControllersAuthorization>,
}

impl AuthorizationMap {
    pub fn new(global_authorization: Option<ControllersAuthorization>) -> Self {
        Self {
            global_authorization,
        }
    }

    pub fn authorizatio_is_enabled(&self) -> bool {
        self.global_authorization.is_some()
    }
    pub fn is_authorized<TGetShouldBeAuthoriazed: GetShouldBeAuthoriazed>(
        &self,
        action: &TGetShouldBeAuthoriazed,
        request_credentials: &Option<Box<dyn RequestCredentials + Send + Sync + 'static>>,
        ip: &str,
    ) -> AuthorizationResult {
        match action.get_should_be_authorized() {
            ShouldBeAuthorized::Yes => {
                if request_credentials.is_some() {
                    return AuthorizationResult::Allowed;
                } else {
                    return AuthorizationResult::NotAuthenticated;
                }
            }
            ShouldBeAuthorized::YesWithClaims(action_claims) => {
                if let Some(req_credentials) = request_credentials {
                    if action_claims.authorized_by_claims(ip, req_credentials.get_claims()) {
                        return AuthorizationResult::Allowed;
                    } else {
                        return AuthorizationResult::NotAuthorized;
                    }
                } else {
                    return AuthorizationResult::NotAuthenticated;
                }
            }
            ShouldBeAuthorized::No => return AuthorizationResult::Allowed,
            ShouldBeAuthorized::UseGlobal => {
                if let Some(global_auth) = self.global_authorization.as_ref() {
                    if global_auth.is_global_authorization_enabled() {
                        if let Some(req_credentials) = request_credentials {
                            let global_claims = global_auth.get_global_claims();

                            if global_claims.authorized_by_claims(ip, req_credentials.get_claims())
                            {
                                return AuthorizationResult::Allowed;
                            } else {
                                return AuthorizationResult::NotAuthorized;
                            }
                        } else {
                            return AuthorizationResult::NotAuthenticated;
                        }
                    } else {
                        return AuthorizationResult::Allowed;
                    }
                } else {
                    return AuthorizationResult::Allowed;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use my_http_server::RequestClaim;

    use crate::controllers::RequiredClaims;

    use super::*;

    pub struct HttpActionMock {
        value: ShouldBeAuthorized,
    }

    impl GetShouldBeAuthoriazed for HttpActionMock {
        fn get_should_be_authorized(&self) -> &ShouldBeAuthorized {
            &self.value
        }
    }

    pub struct RequestCredentialsMock {
        value: Option<Vec<RequestClaim>>,
    }

    impl RequestCredentials for RequestCredentialsMock {
        fn get_id(&self) -> &str {
            "test"
        }

        fn get_claims(&self) -> Option<&[RequestClaim]> {
            let result = self.value.as_ref()?;
            Some(result.as_slice())
        }
    }

    #[test]
    fn test_global_auth_is_disabled_request_has_no_credentials() {
        let auth_map = AuthorizationMap::new(None);

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::No,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> = None;

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.is_allowed());
    }

    #[test]
    fn test_global_auth_is_disabled_request_has_no_credentials_by_action_should_be_authorized_with_no_claims(
    ) {
        let auth_map = AuthorizationMap::new(None);

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::Yes,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> = None;

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.not_authenticated());
    }

    #[test]
    fn test_global_auth_is_disabled_request_credentials_with_no_claims() {
        let auth_map = AuthorizationMap::new(None);

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::No,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> =
            Some(Box::new(RequestCredentialsMock { value: None }));

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.is_allowed());
    }

    #[test]
    fn test_global_auth_is_disabled_by_setup_with_no_claims_request_has_no_credentials() {
        let auth_map = AuthorizationMap::new(
            ControllersAuthorization::BearerAuthentication {
                global: false,
                global_claims: RequiredClaims::from_vec(vec![]),
            }
            .into(),
        );

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::No,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> = None;

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.is_allowed());
    }

    #[test]
    fn test_global_enabled_with_no_global_claims_and_not_request_credentials_action_is_setup_to_global_scheme(
    ) {
        let auth_map = AuthorizationMap::new(
            ControllersAuthorization::BearerAuthentication {
                global: true,
                global_claims: RequiredClaims::from_vec(vec![]),
            }
            .into(),
        );

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::UseGlobal,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> = None;

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.not_authenticated());
    }

    #[test]
    fn test_global_enabled_with_no_global_claims_action_is_setup_to_global_scheme_request_has_creds_with_no_claims(
    ) {
        let auth_map = AuthorizationMap::new(
            ControllersAuthorization::BearerAuthentication {
                global: true,
                global_claims: RequiredClaims::from_vec(vec![]),
            }
            .into(),
        );

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::UseGlobal,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> = None;

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.not_authenticated());
    }

    #[test]
    fn test_global_disabled_action_should_be_authorized_with_no_claims() {
        let auth_map = AuthorizationMap::new(None);

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::Yes,
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> = None;

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.not_authenticated());
    }

    #[test]
    fn test_global_disabled_action_should_be_authorized_with_test_claim() {
        let auth_map = AuthorizationMap::new(None);

        let action: HttpActionMock = HttpActionMock {
            value: ShouldBeAuthorized::YesWithClaims(RequiredClaims::from_vec(vec![
                "test".to_string()
            ])),
        };

        let client_credentials: Option<Box<dyn RequestCredentials + Send + Sync + 'static>> =
            Some(Box::new(RequestCredentialsMock { value: None }));

        let result = auth_map.is_authorized(&action, &client_credentials, "127.0.0.1");

        assert!(result.not_authorized());
    }
}
