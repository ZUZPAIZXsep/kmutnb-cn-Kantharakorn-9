use std::collections::HashMap;
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}
struct HttpRequest {
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}
impl HttpRequest {
    fn new(method: Method, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }

    fn send(&self) -> Result<String, String> {
        // ส่ง request ไปยัง url และ method ที่ระบุ
        Ok(String::from("Success"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request() {
        let mut req = HttpRequest::new(Method::GET, "https://www.google.com");
        req.add_header("Content-Type", "application/json");
        req.set_body(r#"{"name": "PAIZ Kantharakorn", "age": 20}"#);

        let res = req.send();
        assert_eq!(res, Ok(String::from("Success")));
    }
}
