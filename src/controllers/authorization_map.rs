use my_http_server::HttpContext;

use super::{actions::HttpAction, documentation::ShouldBeAuthorized, ControllersAuthorization};

pub enum AuthorizationResult {
    Authorized,
    Unauthenticated,
    Unauthorized,
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
    pub fn is_authorized(&self, action: &HttpAction, ctx: &HttpContext) -> AuthorizationResult {
        if ctx.credentials.is_some() {
            return AuthorizationResult::Authorized;
        }

        match &action.should_be_authorized {
            ShouldBeAuthorized::Yes => {
                if ctx.credentials.is_some() {
                    return AuthorizationResult::Authorized;
                } else {
                    return AuthorizationResult::Unauthenticated;
                }
            }
            ShouldBeAuthorized::YesWithClaims(action_claims) => {
                if ctx.credentials.is_none() {
                    return AuthorizationResult::Unauthenticated;
                } else {
                    return self.execute_with_claims(ctx, action_claims);
                }
            }
            ShouldBeAuthorized::No => return AuthorizationResult::Authorized,
            ShouldBeAuthorized::UseGlobal => {
                if let Some(global_auth) = self.global_authorization.as_ref() {
                    if global_auth.is_global_authorization_enabled() {
                        if let Some(action_claims) = global_auth.get_global_claims() {
                            return self.execute_with_claims(ctx, action_claims);
                        } else {
                            if ctx.credentials.is_some() {
                                return AuthorizationResult::Authorized;
                            } else {
                                return AuthorizationResult::Unauthenticated;
                            }
                        }
                    } else {
                        //TODO - make unit tests
                        return AuthorizationResult::Authorized;
                    }
                } else {
                    return AuthorizationResult::Authorized;
                }
            }
        }
    }

    fn execute_with_claims(
        &self,
        ctx: &HttpContext,
        action_claims: &Vec<String>,
    ) -> AuthorizationResult {
        if let Some(credential) = &ctx.credentials {
            for claim_id in action_claims {
                if credential.get_claim(&ctx.request, claim_id).is_some() {
                    continue;
                }
            }

            return AuthorizationResult::Authorized;
        }

        return AuthorizationResult::Unauthorized;
    }
}
