use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::web::Data;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use tracing::error;
use crate::auth::AuthConfiguration;


pub struct HandleSession(pub bool);

impl<S, B> Transform<S, ServiceRequest> for HandleSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionMiddleware {
            service: Rc::new(service),
            should_redirect: self.0,
        }))
    }
}

pub struct SessionMiddleware<S> {
    service: Rc<S>,
    should_redirect: bool,
}

impl<S, B> Service<ServiceRequest> for SessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let data = req.app_data::<Data<AuthConfiguration>>();
        if data.is_none() {
            error!("No auth configuration found");
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        let auth_configuration = data.unwrap().clone();

        let fut = self.service.call(req);
        let should_redirect = self.should_redirect;
        Box::pin(async move {
            let should_redirect = should_redirect;
            let config = auth_configuration;
            let res = fut.await?;
            // Check if the user has a session cookie.
            // If so then check if the session is valid.
            // If not if should_redirect is true then redirect to the login page. Else return a 401.
            Ok(res)
        })
    }
}
