use http_body_util::{BodyExt, Collected, Empty};
use hyper::{
    body::{Buf, Bytes},
    Request, StatusCode, Uri,
};
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use std::{env, error::Error, process::Command};
use std::{
    net::{SocketAddr, TcpListener},
    time::Duration,
};
use tokio::time::sleep;
use wait_timeout::ChildExt;

#[tokio::test]
async fn exporter_integration_test() -> Result<(), Box<dyn Error>> {
    let binary = env!("CARGO_BIN_EXE_openweathermap_exporter");

    let port = get_available_port();

    let mut child_process = Command::new(binary)
        .current_dir("tests")
        .env("LISTEN_ADDRESS", "127.0.0.1")
        .env("LISTEN_PORT", port.to_string())
        .env("RUST_BACKTRACE", "1")
        .env("RUST_LOG", "info")
        .args(Vec::<String>::new())
        .spawn()
        .expect("unable to start exporter");

    match child_process.wait_timeout(Duration::from_millis(1000)) {
        Ok(Some(status)) => panic!("Exporter process exited prematurely with {status}"),
        Ok(None) => (), // this is good, it's still running
        Err(e) => panic!("Error waiting to see if child process is running or has ended: {e}"),
    };

    let uri = format!("http://127.0.0.1:{port}")
        .parse::<Uri>()
        .unwrap_or_else(|e| panic!("Error parsing URI: {e:?}"));

    let attempts = 20;
    for remaining in (0..attempts).rev() {
        let (status, body) = read_from(uri.clone()).await;

        assert_eq!(status, StatusCode::OK);
        if body.contains("owm_query_success{q=\"Paris, FR,FR\"} 1") {
            println!(
                "Found expected response on attempt #{} (1-based)",
                (attempts - remaining)
            );

            assert!(body.contains(
                "# HELP owm_api_call_time_milliseconds Histogram of successful call times per location in milliseconds"
            ));
            assert!(body.contains("# TYPE owm_api_call_time_milliseconds summary"));

            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="0"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="0.5"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="0.9"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="0.95"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="0.99"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="0.999"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds{quantile="1"} "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds_sum "#));
            assert!(body.contains(r#"owm_api_call_time_milliseconds_count "#));

            break;
        }
        if remaining == 0 {
            panic!("Failed to find paris query status after {attempts} attempts.")
        }
        sleep(Duration::from_millis(1000)).await;
    }

    child_process.kill()?;

    Ok(())
}

fn get_available_port() -> u16 {
    TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .unwrap_or_else(|e| panic!("Unable to bind to an available port on localhost, {e}"))
        .local_addr()
        .expect("Unable to obtain local address from TcpListener")
        .port()
}

async fn read_from(endpoint: Uri) -> (StatusCode, String) {
    let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(HttpConnector::new());

    let req = Request::builder()
        .uri(endpoint.to_string())
        .body(Empty::<Bytes>::new())
        .unwrap_or_else(|e| panic!("Failed building request: {e:?}"));

    let response = client
        .request(req)
        .await
        .unwrap_or_else(|e| panic!("Failed requesting data from {endpoint}: {e:?}"));

    let status = response.status();
    let mut body = response
        .into_body()
        .collect()
        .await
        .map(Collected::aggregate)
        .unwrap_or_else(|e| panic!("Error reading response: {e:?}"));

    let body_string = String::from_utf8(body.copy_to_bytes(body.remaining()).to_vec())
        .unwrap_or_else(|e| panic!("Error decoding response body: {e:?}"));

    (status, body_string)
}
