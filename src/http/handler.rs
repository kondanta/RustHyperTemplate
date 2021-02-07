use {
    super::{
        HttpRequest,
        HttpResponse,
    },
    crate::logic::factorial::*,
    anyhow::Result,
    futures_util::StreamExt,
    hyper::{
        Body,
        Request,
        Response,
    },
};

/// HTTP Handler for /calculate end point.
pub(super) async fn calculate_factorial(
    mut req: Request<Body>
) -> Result<Response<Body>> {
    let mut request_body = Vec::new();
    while let Some(chunk) = StreamExt::next(&mut req.body_mut()).await {
        request_body.extend_from_slice(&chunk?);
    }

    let deserialized_body: HttpRequest = serde_json::from_slice(&request_body)?;
    let f = Factorial::factorial(deserialized_body.number)?;
    let response_body = HttpResponse { factorial: f };
    let response = serde_json::to_string(&response_body)?;

    Ok(Response::new(Body::from(response)))
}

pub(super) async fn healthz() -> Result<Response<Body>>{
    Ok(Response::new(Body::from("ok")))
}
