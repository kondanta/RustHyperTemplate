use {
    anyhow::Result,
    hyper::{
        Body,
        Method,
        Request,
        Response,
        StatusCode,
    },
    serde::{
        Deserialize,
        Serialize,
    },
};

pub mod handler;

#[derive(Debug, Serialize)]
struct HttpResponse {
    factorial: i128,
}

#[derive(Debug, Deserialize)]
struct HttpRequest {
    number: i128,
}

/// HTTP Endpoint router. Routes each path to its own handler function.
pub async fn router(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/calculate") => {
            handler::calculate_factorial(req).await
        }
	(&Method::GET, "/healthz") => {
	    handler::healthz().await
	}
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
