// use hyper::service::{make_service_fn, service_fn};
// use hyper::{Body, Request, Response, Server};
// use std::convert::Infallible;
// use std::fs::File;
// use std::io::Read;
// use std::path::{Path, PathBuf};
// use tokio::runtime::Runtime;

// async fn serve_file(req: Request<Body>, base_path: &str) -> Result<Response<Body>, Infallible> {
//     let path = req.uri().path().trim_start_matches('/');
//     let path = if path.is_empty() { "index.html" } else { path };

//     let file_path = Path::new(base_path).join(path);

//     if file_path.is_file() {
//         let mut file = File::open(&file_path).expect("file not found");
//         let mut contents = Vec::new();
//         file.read_to_end(&mut contents).expect("unable to read file");

//         Ok(Response::new(Body::from(contents)))
//     } else {
//         let not_found = "404 Not Found";
//         Ok(Response::builder()
//             .status(404)
//             .body(Body::from(not_found))
//             .unwrap())
//     }
// }

// pub async fn run_file_server(addr: &str, base_path: &str) {
//     let addr = addr.parse().expect("Unable to parse socket address");

//     let make_svc = make_service_fn(move |_conn| {
//         let base_path = base_path.to_string();
//         async move {
//             Ok::<_, Infallible>(service_fn(move |req| {
//                 serve_file(req, &base_path)
//             }))
//         }
//     });

//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Listening on http://{}", addr);

//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
// }

