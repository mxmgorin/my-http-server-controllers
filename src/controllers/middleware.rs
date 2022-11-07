use async_trait::async_trait;
use hyper::Method;
use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
    RequestCredentials,
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

pub struct ControllersMiddleware<TRequestCredentials: RequestCredentials + Send + Sync + 'static> {
    pub get: HttpActions<TRequestCredentials>,
    pub post: HttpActions<TRequestCredentials>,
    pub put: HttpActions<TRequestCredentials>,
    pub delete: HttpActions<TRequestCredentials>,
    pub http_objects: Vec<HttpObjectStructure>,

    #[cfg(feature = "with-authorization")]
    pub authorization: Option<ControllersAuthorization>,
}

impl<TRequestCredentials: RequestCredentials + Send + Sync + 'static>
    ControllersMiddleware<TRequestCredentials>
{
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
        TGetAction: GetAction
            + HandleHttpRequest<TRequestCredentials = TRequestCredentials>
            + GetDescription
            + Send
            + Sync
            + 'static,
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
        TPostAction: PostAction
            + HandleHttpRequest<TRequestCredentials = TRequestCredentials>
            + GetDescription
            + Send
            + Sync
            + 'static,
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
        TPutAction: PostAction
            + HandleHttpRequest<TRequestCredentials = TRequestCredentials>
            + GetDescription
            + Send
            + Sync
            + 'static,
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
        TDeleteAction: DeleteAction
            + HandleHttpRequest<TRequestCredentials = TRequestCredentials>
            + GetDescription
            + Send
            + Sync
            + 'static,
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

    pub fn list_of_get_route_actions(&self) -> &Vec<HttpAction<TRequestCredentials>> {
        self.get.get_actions()
    }

    pub fn list_of_post_route_actions(&self) -> &Vec<HttpAction<TRequestCredentials>> {
        self.post.get_actions()
    }

    pub fn list_of_put_route_actions(&self) -> &Vec<HttpAction<TRequestCredentials>> {
        self.put.get_actions()
    }

    pub fn list_of_delete_route_actions<'s>(&self) -> &Vec<HttpAction<TRequestCredentials>> {
        self.delete.get_actions()
    }
}

#[async_trait]
impl<TRequestCredentials: RequestCredentials + Send + Sync + 'static> HttpServerMiddleware
    for ControllersMiddleware<TRequestCredentials>
{
    type TRequestCredentials = TRequestCredentials;
    async fn handle_request(
        &self,
        ctx: &mut HttpContext<TRequestCredentials>,
        get_next: &mut HttpServerRequestFlow<TRequestCredentials>,
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
