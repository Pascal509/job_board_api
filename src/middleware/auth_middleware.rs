use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, Transform};
use actix_web::http::header::AUTHORIZATION;
use actix_web::HttpResponse;
use futures_util::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::{ready, Ready as StdReady};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}

pub fn is_admin(claims: &Claims) -> bool {
    claims.role == "admin"
}

#[derive(Clone)]
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(AUTHORIZATION).cloned();
        let srv = self.service.clone();

        Box::pin(async move {
            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        let secret = env::var("JWT_SECRET").unwrap();
                        let validation = Validation::default();
                        let decoded = decode::<Claims>(
                            token,
                            &DecodingKey::from_secret(secret.as_bytes()),
                            &validation,
                        );

                        if let Ok(token_data) = decoded {
                            req.extensions_mut().insert(token_data.claims);
                            let res = srv.call(req).await?;
                            return Ok(res.map_into_left_body());
                        }
                    }
                }
            }

            let response = HttpResponse::Unauthorized()
                .body("Unauthorized: Missing or invalid token")
                .map_into_right_body();
            Ok(req.into_response(response))
        })
    }
}
