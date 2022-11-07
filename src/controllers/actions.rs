use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, RequestCredentials};

use super::{
    documentation::{HttpActionDescription, ShouldBeAuthorized},
    ControllersAuthorization, HttpRoute,
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
    type TRequestCredentials: RequestCredentials + Send + Sync + 'static;
    async fn handle_request(
        &self,
        http_route: &HttpRoute,
        ctx: &mut HttpContext<Self::TRequestCredentials>,
    ) -> Result<HttpOkResult, HttpFailResult>;
}

pub trait GetDescription {
    fn get_description(&self) -> Option<HttpActionDescription>;
}

pub struct HttpAction<TRequestCredentials: RequestCredentials + Send + Sync + 'static> {
    pub handler: Arc<
        dyn HandleHttpRequest<TRequestCredentials = TRequestCredentials> + Send + Sync + 'static,
    >,
    pub http_route: HttpRoute,
    pub description: Arc<dyn GetDescription + Send + Sync + 'static>,
    pub should_be_authorized: ShouldBeAuthorized,
}

pub struct HttpActions<TRequestCredentials: RequestCredentials + Send + Sync + 'static> {
    actions: Vec<HttpAction<TRequestCredentials>>,
}

impl<TRequestCredentials: RequestCredentials + Send + Sync + 'static>
    HttpActions<TRequestCredentials>
{
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn register(&mut self, action: HttpAction<TRequestCredentials>) {
        self.actions.push(action);
    }

    pub async fn handle_request(
        &self,
        ctx: &mut HttpContext<TRequestCredentials>,
        global_authorization: &Option<ControllersAuthorization>,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        for action in &self.actions {
            if action.http_route.is_my_path(&ctx.request.http_path) {
                // Credentials are set - goot to go
                if ctx.credentials.is_some() {
                    return Some(action.handler.handle_request(&action.http_route, ctx).await);
                }

                match &action.should_be_authorized {
                    ShouldBeAuthorized::Yes => {
                        if ctx.credentials.is_none() {
                            return Some(Err(HttpFailResult::as_unauthorized(None)));
                        } else {
                            return Some(
                                action.handler.handle_request(&action.http_route, ctx).await,
                            );
                        }
                    }
                    ShouldBeAuthorized::YesWithClaims(claims) => {
                        if ctx.credentials.is_none() {
                            return Some(Err(HttpFailResult::as_unauthorized(None)));
                        } else {
                            return self.execute_with_claims(action, ctx, claims).await;
                        }
                    }
                    ShouldBeAuthorized::No => {
                        return Some(action.handler.handle_request(&action.http_route, ctx).await);
                    }
                    ShouldBeAuthorized::UseGlobal => {
                        if let Some(global_auth) = global_authorization {
                            return self
                                .execute_with_claims(action, ctx, global_auth.get_global_claims())
                                .await;
                        } else {
                            return Some(
                                action.handler.handle_request(&action.http_route, ctx).await,
                            );
                        }
                    }
                }
            }
        }

        None
    }

    pub fn get_actions(&self) -> &Vec<HttpAction<TRequestCredentials>> {
        &self.actions
    }

    async fn execute_with_claims(
        &self,
        action: &HttpAction<TRequestCredentials>,
        ctx: &mut HttpContext<TRequestCredentials>,
        claims: &[String],
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        for claim_id in claims {
            if let Some(credential) = &ctx.credentials {
                if credential.get_claim(&ctx.request, claim_id).is_none() {
                    return Some(Err(HttpFailResult::as_unauthorized(None)));
                }
            }
        }
        return Some(action.handler.handle_request(&action.http_route, ctx).await);
    }
}
