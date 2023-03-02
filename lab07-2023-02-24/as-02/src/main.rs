#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, headers: Vec<String>, body: String) -> Self {
        HttpResponse {
            status_code,
            headers,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_http_response() {
        let status_code = 200;
        let headers = vec!["Content-Type: text/html".to_string()];
        let body = "<html><body><h1> Kantharakorn </h1></body></html>".to_string();
        let response = HttpResponse::new(status_code, headers.clone(), body.clone());

        assert_eq!(response.status_code, status_code);
        assert_eq!(response.headers, headers);
        assert_eq!(response.body, body);
    }
}
