/// Example for actix-web
#[cfg(feature = "actix")]
pub mod actix_integration {
    use super::UserAgentBlocker;
    use actix_web::{
        dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
        Error, HttpResponse,
    };
    use actix_web::http::header;
    use futures::future::{ready, LocalBoxFuture, Ready};
    use std::rc::Rc;
    use std::task::{Context, Poll};
    
    pub struct UserAgentFilter {
        blocker: Rc<UserAgentBlocker>,
    }
    
    impl UserAgentFilter {
        pub fn new(blocker: UserAgentBlocker) -> Self {
            UserAgentFilter {
                blocker: Rc::new(blocker),
            }
        }
    }
    
    impl<S, B> Transform<S, ServiceRequest> for UserAgentFilter
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Response = ServiceResponse<B>;
        type Error = Error;
        type Transform = UserAgentFilterMiddleware<S>;
        type InitError = ();
        type Future = Ready<Result<Self::Transform, Self::InitError>>;
        
        fn new_transform(&self, service: S) -> Self::Future {
            ready(Ok(UserAgentFilterMiddleware {
                service,
                blocker: self.blocker.clone(),
            }))
        }
    }
    
    pub struct UserAgentFilterMiddleware<S> {
        service: S,
        blocker: Rc<UserAgentBlocker>,
    }
    
    impl<S, B> Service<ServiceRequest> for UserAgentFilterMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Response = ServiceResponse<B>;
        type Error = Error;
        type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
        
        forward_ready!(service);
        
        fn call(&self, req: ServiceRequest) -> Self::Future {
            // Check user agent
            if let Some(user_agent) = req.headers().get(header::USER_AGENT) {
                if let Ok(ua_str) = user_agent.to_str() {
                    if let Some(pattern) = self.blocker.block_reason(ua_str) {
                        // Log blocking
                        // eprintln!("Blocked access from agent: {}", ua_str);
                        
                        // Return 403 Forbidden
                        let response = HttpResponse::Forbidden()
                            .body(format!("Access denied: User agent contains blocked pattern '{}'", pattern));
                            
                        return Box::pin(async { Ok(req.into_response(response)) });
                    }
                }
            }
            
            // Not blocked, continue with normal request processing
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        }
    }
}
