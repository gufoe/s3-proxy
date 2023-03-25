mod config;
mod s3;

#[macro_use]
extern crate serde_derive;

use std::{path::PathBuf, time::Instant};
use structopt::StructOpt;

use actix_web::{
    get,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "S3 Proxy", about = "A simple S3 proxy with streaming support.")]
struct CliOptions {
    #[structopt(
        parse(from_os_str),
        short = "c",
        long = "config-file",
        help = "The server configuration file"
    )]
    config: PathBuf,
}

#[derive(Clone)]
struct State {
    config: config::Config,
    client: reqwest::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    setup_logs();
    let opts = CliOptions::from_args();

    let config = config::read_config(&opts.config);

    log::info!("Hosting content from bucket '{}' ", config.s3_bucket);

    let workers = config.workers;
    let addr = format!("{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                config: config.clone(),
                client: reqwest::Client::new(),
            }))
            .service(handler)
    })
    .workers(workers.unwrap_or_else(|| num_cpus::get()))
    .bind(addr)?
    .run()
    .await
    .unwrap();

    Ok(())
}

#[get("/{path:.*}")]
async fn handler(
    data: web::Data<State>,
    path: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let url = s3::get_s3_file_url(&data.config, &path);
    let mut http_req = data.client.get(url);

    let forward_headers = ["range"];
    for h in forward_headers {
        let value: Option<String> = req
            .headers()
            .get(h.to_string())
            .and_then(|r| r.to_str().ok())
            .map(From::from);
        if let Some(value) = value {
            println!("forward header: {}: {}", h, value);
            http_req = http_req.header(h.to_string(), value);
        }
    }

    let t_2 = Instant::now();
    let s3_response = http_req.send().await.unwrap();
    let t_3 = Instant::now();

    log::info!(
        "[{}] {: >4} ms {: >7.02} MB /{}",
        s3_response.status().as_u16(),
        t_2.elapsed().as_millis() - t_3.elapsed().as_millis(),
        s3_response.content_length().unwrap_or(0) as f32 / 1024.0 / 1024.0,
        path,
    );

    if s3_response.status().as_u16() < 200 || s3_response.status().as_u16() >= 300 {
        return HttpResponse::NotFound().body("File not found");
    }
    let mut reply = HttpResponse::Ok();
    reply.keep_alive();

    if let Some(mime) = s3_response.headers().get("content-type") {
        // println!("set mime to {:?}", &mime);
        reply.content_type(mime.as_bytes().to_ascii_lowercase());
    }

    reply.streaming(s3_response.bytes_stream())
}

fn setup_logs() {
    use chrono::Local;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::io::Write;

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
