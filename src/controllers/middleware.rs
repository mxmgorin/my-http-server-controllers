use async_trait::async_trait;
use hyper::Method;
use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
};

use super::{
    actions::{
        DeleteAction, GetAction, GetDescription, HandleHttpRequest, HttpAction, HttpActions,
        PostAction,
    },
    documentation::{data_types::HttpObjectStructure, ShouldBeAuthorized},
    HttpRoute,
};

#[cfg(feature = "with-authorization")]
use super::ControllersAuthorization;

pub struct ControllersMiddleware {
    pub get: HttpActions,
    pub post: HttpActions,
    pub put: HttpActions,
    pub delete: HttpActions,
    pub http_objects: Vec<HttpObjectStructure>,

    #[cfg(feature = "with-authorization")]
    pub authorization: Option<ControllersAuthorization>,
}

impl ControllersMiddleware {
    pub fn new(
        #[cfg(feature = "with-authorization")] authorization: Option<ControllersAuthorization>,
    ) -> Self {
        Self {
            get: HttpActions::new(),
            post: HttpActions::new(),
            put: HttpActions::new(),
            delete: HttpActions::new(),
            http_objects: Vec::new(),
            #[cfg(feature = "with-authorization")]
            authorization,
        }
    }

    pub fn register_get_action<
        TGetAction: GetAction + HandleHttpRequest + GetDescription + Send + Sync + 'static,
    >(
        &mut self,
        action: Arc<TGetAction>,
    ) {
        let http_route = HttpRoute::new(action.get_route());
        self.get.register(HttpAction {
            handler: action.clone(),
            http_route,

            should_be_authorized: if let Some(desc) = action.get_description() {
                desc.should_be_authorized
            } else {
                ShouldBeAuthorized::UseGlobal
            },
            description: action,
        });
    }

    pub fn register_post_action<
        TPostAction: PostAction + HandleHttpRequest + GetDescription + Send + Sync + 'static,
    >(
        &mut self,
        action: Arc<TPostAction>,
    ) {
        let http_route = HttpRoute::new(action.get_route());
        self.post.register(HttpAction {
            handler: action.clone(),
            http_route,

            should_be_authorized: if let Some(desc) = action.get_description() {
                desc.should_be_authorized
            } else {
                ShouldBeAuthorized::UseGlobal
            },
            description: action,
        });
    }

    pub fn register_put_action<
        TPutAction: PostAction + HandleHttpRequest + GetDescription + Send + Sync + 'static,
    >(
        &mut self,
        action: Arc<TPutAction>,
    ) {
        let http_route = HttpRoute::new(action.get_route());
        self.put.register(HttpAction {
            handler: action.clone(),
            http_route,

            should_be_authorized: if let Some(desc) = action.get_description() {
                desc.should_be_authorized
            } else {
                ShouldBeAuthorized::UseGlobal
            },
            description: action,
        });
    }

    pub fn register_delete_action<
        TDeleteAction: DeleteAction + HandleHttpRequest + GetDescription + Send + Sync + 'static,
    >(
        &mut self,
        action: Arc<TDeleteAction>,
    ) {
        let http_route = HttpRoute::new(action.get_route());
        self.delete.register(HttpAction {
            handler: action.clone(),
            http_route,

            should_be_authorized: if let Some(desc) = action.get_description() {
                desc.should_be_authorized
            } else {
                ShouldBeAuthorized::UseGlobal
            },
            description: action,
        });
    }

    pub fn list_of_get_route_actions(&self) -> &Vec<HttpAction> {
        self.get.get_actions()
    }

    pub fn list_of_post_route_actions(&self) -> &Vec<HttpAction> {
        self.post.get_actions()
    }

    pub fn list_of_put_route_actions(&self) -> &Vec<HttpAction> {
        self.put.get_actions()
    }

    pub fn list_of_delete_route_actions<'s>(&self) -> &Vec<HttpAction> {
        self.delete.get_actions()
    }
}

#[async_trait]
impl HttpServerMiddleware for ControllersMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        match ctx.request.get_method() {
            &Method::GET => {
                {
                    if let Some(result) = self.get.handle_request(ctx, &self.authorization).await {
                        return result;
                    }
                }
                return get_next.next(ctx).await;
            }
            &Method::POST => {
                if let Some(result) = self.post.handle_request(ctx, &self.authorization).await {
                    return result;
                } else {
                    return get_next.next(ctx).await;
                }
            }
            &Method::PUT => {
                if let Some(result) = self.put.handle_request(ctx, &self.authorization).await {
                    return result;
                } else {
                    return get_next.next(ctx).await;
                }
            }
            &Method::DELETE => {
                if let Some(result) = self.delete.handle_request(ctx, &self.authorization).await {
                    return result;
                } else {
                    return get_next.next(ctx).await;
                }
            }
            _ => {
                return get_next.next(ctx).await;
            }
        }
    }
}
