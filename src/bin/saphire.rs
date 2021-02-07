use {
    container::{
        config::opt::Opt,
        http,
    },
    env_logger::Env,
    hyper,
    log::info,
};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c signal handler");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let opts = Opt::args();

    let log_level = &opts.log_level.unwrap_or_default();
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level))
        .init();

    info!("Serving Project Saphire on {}", &opts.http_listen);

    let http_server = {
        let addr = opts
            .http_listen
            .parse()
            .expect("could not parse http server addr");
        let service = hyper::service::make_service_fn(move |_| async move {
            Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| {
                http::router(req)
            }))
        });
        hyper::Server::bind(&addr).serve(service)
    };

    http_server
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("failed to start http server");

    Ok(())
}
