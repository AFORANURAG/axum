use axum::{
    async_trait,
    extract::FromRequest,
    http::Request,
};
use axum_macros::debug_handler;

struct A;

#[async_trait]
impl<S> FromRequest<S> for A
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request(_req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        unimplemented!()
    }
}

impl A {
    #[debug_handler]
    async fn handler(&mut self) {}
}

fn main() {}
