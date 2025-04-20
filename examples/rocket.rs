/// Example for rocket
#[cfg(feature = "rocket")]
pub mod rocket_integration {
    use super::UserAgentBlocker;
    use rocket::fairing::{Fairing, Info, Kind};
    use rocket::http::Status;
    use rocket::{Data, Request, Response};
    use rocket::request::{FromRequest, Outcome};
    use std::sync::Arc;
    
    pub struct UserAgentGuard {
        blocker: Arc<UserAgentBlocker>,
    }
    
    impl UserAgentGuard {
        pub fn new(blocker: UserAgentBlocker) -> Self {
            UserAgentGuard {
                blocker: Arc::new(blocker),
            }
        }
    }
    
    #[rocket::async_trait]
    impl Fairing for UserAgentGuard {
        fn info(&self) -> Info {
            Info {
                name: "User Agent Filter",
                kind: Kind::Request,
            }
        }
        
        async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
            if let Some(user_agent) = request.headers().get_one("User-Agent") {
                if self.blocker.should_block(user_agent) {
                    request.local_cache(|| Status::Forbidden);
                }
            }
        }
    }
    
    // Request guard
    pub struct BlockedUserAgent {
        pub pattern: String,
    }
    
    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for BlockedUserAgent {
        type Error = ();
        
        async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
            let blocker = request.rocket().state::<Arc<UserAgentBlocker>>()
                .expect("UserAgentBlocker not registered as state");
                
            if let Some(user_agent) = request.headers().get_one("User-Agent") {
                if let Some(pattern) = blocker.block_reason(user_agent) {
                    return Outcome::Failure((Status::Forbidden, ()));
                }
            }
            
            Outcome::Forward(())
        }
    }
}
