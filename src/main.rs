use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use tokio::fs;


#[derive(Serialize, Deserialize)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}


async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = match req.uri().path() {
        "/" => {
            let json_response = json!({
                "message": "Hello, World!"
            });
            Response::builder()
                .header("Content-Type", "application/json")
                .body(Body::from(json_response.to_string()))
                .unwrap()
        }
        "/hello" => {
            let msg = json!({
                "message": "Hello from /hello endpoint!"
            });
            Response::builder()
                .header("Content-Type", "application/json")
                .body(Body::from(msg.to_string()))
                .unwrap()
        }
        "/products" => {
            match get_products().await {
                Ok(products_json) => Response::builder()
                    .header("Content-Type", "application/json")
                    .body(Body::from(products_json))
                    .unwrap(),
                Err(_) => Response::builder()
                    .status(500)
                    .header("Content-Type", "application/json")
                    .body(Body::from("{\"error\": \"Could not read products\"}"))
                    .unwrap(),
            }
        }
        _ => Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(Body::from("{\"error\": \"Not Found\"}"))
            .unwrap(),
    };
    Ok(response)
}


async fn get_products() -> Result<String, std::io::Error> {
    let file_content = fs::read_to_string("products.json").await?;
    Ok(file_content)
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();


    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(router)) }
    });


    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);


    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
