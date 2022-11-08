use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};

use super::{
    documentation::{HttpActionDescription, ShouldBeAuthorized},
    AuthorizationMap, HttpRoute,
};

pub trait GetAction {
    fn get_route(&self) -> &str;
}

pub trait PostAction {
    fn get_route(&self) -> &str;
}

pub trait PutAction {
    fn get_route(&self) -> &str;
}

pub trait DeleteAction {
    fn get_route(&self) -> &str;
}

#[async_trait::async_trait]
pub trait HandleHttpRequest {
    async fn handle_request(
        &self,
        http_route: &HttpRoute,
        ctx: &mut HttpContext,
    ) -> Result<HttpOkResult, HttpFailResult>;
}

pub trait GetDescription {
    fn get_description(&self) -> Option<HttpActionDescription>;
}

pub trait GetShouldBeAuthoriazed {
    fn get_should_be_authorized(&self) -> &ShouldBeAuthorized;
}

pub struct HttpAction {
    pub handler: Arc<dyn HandleHttpRequest + Send + Sync + 'static>,
    pub http_route: HttpRoute,
    pub description: Arc<dyn GetDescription + Send + Sync + 'static>,
    pub should_be_authorized: ShouldBeAuthorized,
}

impl GetShouldBeAuthoriazed for HttpAction {
    fn get_should_be_authorized(&self) -> &ShouldBeAuthorized {
        &self.should_be_authorized
    }
}

pub struct HttpActions {
    actions: Vec<HttpAction>,
}

impl HttpActions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn register(&mut self, action: HttpAction) {
        self.actions.push(action);
    }

    pub async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        authorization_map: &AuthorizationMap,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        for action in &self.actions {
            if action.http_route.is_my_path(&ctx.request.http_path) {
                match authorization_map.is_authorized(
                    action,
                    &ctx.credentials,
                    ctx.request.get_ip().get_real_ip(),
                ) {
                    super::AuthorizationResult::Allowed => {
                        return Some(action.handler.handle_request(&action.http_route, ctx).await);
                    }
                    super::AuthorizationResult::NotAuthenticated => {
                        return Some(Err(HttpFailResult::as_unauthorized(Some(
                            "No session credentials are found".to_string(),
                        ))));
                    }
                    super::AuthorizationResult::NotAuthorized => {
                        return Some(Err(HttpFailResult::as_unauthorized(None)));
                    }
                }
            }
        }

        None
    }

    pub fn get_actions(&self) -> &Vec<HttpAction> {
        &self.actions
    }
}
