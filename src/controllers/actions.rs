use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};

use super::{documentation::HttpActionDescription, HttpRoute};

pub trait GetAction {
    fn get_route(&self) -> &str;
    fn get_description(&self) -> Option<HttpActionDescription>;
}

pub trait PostAction {
    fn get_route(&self) -> &str;
    fn get_description(&self) -> Option<HttpActionDescription>;
}

pub trait PutAction {
    fn get_route(&self) -> &str;
    fn get_description(&self) -> Option<HttpActionDescription>;
}

pub trait DeleteAction {
    fn get_route(&self) -> &str;
    fn get_description(&self) -> Option<HttpActionDescription>;
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

pub struct HttpAction {
    pub handler: Arc<dyn HandleHttpRequest + Send + Sync + 'static>,
    pub http_route: HttpRoute,
    pub description: Arc<dyn GetDescription + Send + Sync + 'static>,
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
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        for action in &self.actions {
            if action.http_route.is_my_path(&ctx.request.http_path) {
                return Some(action.handler.handle_request(&action.http_route, ctx).await);
            }
        }

        None
    }

    pub fn get_actions(&self) -> &Vec<HttpAction> {
        &self.actions
    }
}
