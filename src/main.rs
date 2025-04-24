use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn handle_request(
    req: Request<Body>,
    base_path: Arc<PathBuf>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().trim_start_matches('/');
    let path = base_path.join(path);

    if path.exists() && path.is_file() {
        let mut file = File::open(&path).await.unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await.unwrap();

        let mime_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("mpd") => "application/dash+xml",
            Some("html") | Some("htm") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            _ => "application/octet-stream",
        };

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", mime_type)
            .body(Body::from(contents))
            .unwrap();

        Ok(response)
    } else {
        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap();

        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let base_path = Arc::new(PathBuf::from("./DashOut")); // Change this to your static files directory

    let make_svc = make_service_fn(move |_conn| {
        let base_path = Arc::clone(&base_path);
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, Arc::clone(&base_path))
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

