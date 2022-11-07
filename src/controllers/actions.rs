use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, RequestCredentials};

use super::{documentation::HttpActionDescription, ControllersAuthorization, HttpRoute};

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
    pub should_be_authorized: Option<bool>,
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

                if let Some(authorization_setup) = global_authorization {
                    if authorization_setup.is_global_authorization() {
                        //Global Authorization is enbaled

                        if let Some(should_be_authorized) = action.should_be_authorized {
                            if should_be_authorized {
                                if ctx.credentials.is_none() {
                                    return Some(Err(HttpFailResult::as_unauthorized(None)));
                                }
                            } else {
                                //Should be authourized is false - means we pass it though
                                return Some(
                                    action.handler.handle_request(&action.http_route, ctx).await,
                                );
                            }
                        } else {
                            //Should be authourized is not set - global authorization says - it must go with authorization
                            return Some(Err(HttpFailResult::as_unauthorized(None)));
                        }
                    } else {
                        if let Some(should_be_authorized) = action.should_be_authorized {
                            if should_be_authorized {
                                if ctx.credentials.is_none() {
                                    return Some(Err(HttpFailResult::as_unauthorized(None)));
                                }
                            } else {
                                //Should be authourized is false - means we pass it though
                                return Some(
                                    action.handler.handle_request(&action.http_route, ctx).await,
                                );
                            }
                        } else {
                            //Should be authourized is not set - global authorization says - it can go without authorization
                            return Some(
                                action.handler.handle_request(&action.http_route, ctx).await,
                            );
                        }
                    }
                } else {
                    if let Some(should_be_authorized) = action.should_be_authorized {
                        if should_be_authorized {
                            if ctx.credentials.is_none() {
                                return Some(Err(HttpFailResult::as_unauthorized(None)));
                            }
                        } else {
                            //Should be authourized is false - means we pass it though
                            return Some(
                                action.handler.handle_request(&action.http_route, ctx).await,
                            );
                        }
                    } else {
                        //Action should_be_authourized is not set - global authorization is not set - it can go though
                        return Some(action.handler.handle_request(&action.http_route, ctx).await);
                    }
                }
            }
        }

        None
    }

    pub fn get_actions(&self) -> &Vec<HttpAction<TRequestCredentials>> {
        &self.actions
    }
}
