use std::num::ParseIntError;

use wiremock::{http::HeaderValue, Match, Request};

struct WellFormedJson;

// Function to extract and parse the Content-Length header
fn get_content_length(request: &Request) -> Result<usize, ParseIntError> {
    // Try to get the Content-Length header and convert it into a string
    // The ? operator will automatically return early if the result is None or Err
    let content_length_str = request
        .headers
        .get("Content-Length")
        .ok_or_else(|| {
            "Content-Length header missing"
                .parse::<usize>()
                .unwrap_err()
        }) // Create a ParseIntError if header is missing
        .and_then(|hv| {
            hv.to_str()
                .map_err(|_e| "Invalid header value".parse::<usize>().unwrap_err())
        })?; // Convert HeaderValue to str, handle potential error

    // Try to parse the string into a usize
    content_length_str.parse::<usize>()
}

impl Match for WellFormedJson {
    fn matches(&self, request: &Request) -> bool {
        /*
        - The method is `POST`
        - The `Content-Type` header is present and set to `application/json`
        - The request body is a valid JSON object
        - The `Content-Length` header is set and its value matches the length of the request body (in bytes)
        */
        let ok_method = request.method == "POST";
        let ok_content_type = match request.headers.get("Content-Type") {
            Some(content_type) => content_type.to_str().unwrap() == "application/json",
            None => false,
        };
        let content_length_usize = get_content_length(request);
        let ok_content_length = match content_length_usize {
            Ok(content_length) => content_length == request.body.len(),
            Err(_) => false,
        };
        let ok_body = serde_json::from_slice::<serde_json::Value>(&request.body).is_ok();
        ok_method && ok_content_type && ok_content_length && ok_body
        /*


        */
    }
}

#[cfg(test)]
mod tests {
    use crate::WellFormedJson;
    use googletest::assert_that;
    use googletest::matchers::eq;
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn test_server() -> MockServer {
        let server = MockServer::start().await;
        server
            .register(Mock::given(WellFormedJson).respond_with(ResponseTemplate::new(200)))
            .await;
        server
    }

    #[googletest::test]
    #[tokio::test]
    async fn errors_on_invalid_json() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        // Trailing comma is not valid in JSON
        let body = r#"{"hi": 2,"#;
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .header("Content-Type", "application/json")
            .body(r#"{"hi": 2,"#)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::test]
    #[tokio::test]
    async fn errors_on_missing_content_type() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&json!({"hi": 2})).unwrap();
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::test]
    #[tokio::test]
    async fn errors_on_invalid_content_length() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&json!({"hi": 2})).unwrap();
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::test]
    #[tokio::test]
    async fn errors_on_non_post() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = json!({"hi": 2});

        let outcome = client
            .patch(&server.uri())
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::test]
    #[tokio::test]
    async fn happy_path() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = json!({"hi": 2});

        let outcome = client.post(&server.uri()).json(&body).send().await.unwrap();
        assert_that!(outcome.status().as_u16(), eq(200));
    }
}
