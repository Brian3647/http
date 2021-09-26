use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
	version: &'a str,
	status_code: &'a str,
	status_text: &'a str,
	pub headers: Option<HashMap<&'a str, String>>,
	body: Option<String>
}

impl<'a> Default for HttpResponse<'a> {
	fn default() -> Self {
		Self {
			version: "HTTP/1.1",
			status_code: "200",
			status_text: "OK",
			headers: None,
			body: None
		}
	}
}

impl<'a> HttpResponse<'a> {
	/// Deprecated. Use `HttpResponse::ok(...)`, `HttpResponse::bad_request(...)`, etc instead.
	#[deprecated]
	pub fn new(
		status_code: &'a str,
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> HttpResponse<'a> {
		let mut response: HttpResponse<'a> = HttpResponse::default();

		if status_code != "200" {
			response.status_code = status_code;
		};

		response.headers = match &headers {
			Some(_h) => headers,
			None => {
				let mut h: HashMap<&str, String> = HashMap::new();
				h.insert("Content-Type", "text/plain".to_string());
				Some(h)
			}
		};

		response.status_text = match response.status_code {
			"200" => "OK",
			"400" => "Bad Request",
			"404" => "Not Found",
			"500" => "Internal Server Error",
			_ => ""
		};

		response.body = body;
		response
	}

	pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
		let res = self.clone();
		let response_string = String::from(res);
		let _ = write!(write_stream, "{}", response_string);
		Ok(())
	}
}

impl<'a> HttpResponse<'a> {
	fn version(&self) -> &str {
		self.version
	}

	fn status_code(&self) -> &str {
		self.status_code
	}

	fn status_text(&self) -> &str {
		self.status_text
	}

	fn headers(&self) -> String {
		let map = self.headers.clone().unwrap();
		let mut header_string: String = "".into();
		for (k, v) in map.iter() {
			header_string = format!("{}{}:{}\r\n", header_string, k, v);
		}
		header_string
	}

	pub fn body(&self) -> &str {
		match &self.body {
			Some(b) => b.as_str(),
			None => ""
		}
	}
}

impl<'a> From<HttpResponse<'a>> for String {
	fn from(res: HttpResponse) -> String {
		format!(
			"{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
			&res.clone().version(),
			&res.clone().status_code(),
			&res.clone().status_text(),
			&res.clone().headers(),
			&res.clone().body.unwrap_or_else(|| "".into()).len(),
			&res.body()
		)
	}
}

#[cfg(test)]
mod tests {
	use crate::response::*;

	#[test]
	fn test_response_struct_creation_200() {
		let response_actual = HttpResponse::new(
			"200",
			None,
			Some("Item was shipped on 21st Dec 2020".into())
		);

		let response_expected = HttpResponse {
			version: "HTTP/1.1",
			status_code: "200",
			status_text: "OK",
			headers: {
				let mut h = HashMap::new();
				h.insert("Content-Type", "text/plain".to_string());
				Some(h)
			},
			body: Some("Item was shipped on 21st Dec 2020".into())
		};

		assert_eq!(response_actual, response_expected);
	}

	#[test]
	fn test_response_struct_creation_404() {
		let response_actual = HttpResponse::new(
			"404",
			None,
			Some("Item was shipped on 21st Dec 2020".into())
		);

		let response_expected = HttpResponse {
			version: "HTTP/1.1",
			status_code: "404",
			status_text: "Not Found",
			headers: {
				let mut h = HashMap::new();
				h.insert("Content-Type", "text/plain".to_string());
				Some(h)
			},
			body: Some("Item was shipped on 21st Dec 2020".into())
		};

		assert_eq!(response_actual, response_expected);
	}

	#[test]
	fn test_http_response_creation() {
		let response_expected = HttpResponse {
			version: "HTTP/1.1",
			status_code: "404",
			status_text: "Not Found",
			headers: {
				let mut h = HashMap::new();
				h.insert("Content-Type", "text/html".to_string());
				Some(h)
			},
			body: Some("Item was shipped on 21st Dec 2020".into())
		};

		let http_string: String = response_expected.into();
		let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";

		assert_eq!(http_string, response_actual);
	}
}

impl<'a> HttpResponse<'a> {
	pub fn new_from_status(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>,
		status_code: &'a str,
		status_text: &'a str
	) -> Self {
		let mut response: HttpResponse<'a> = HttpResponse::default();

		if status_code != "200" {
			response.status_code = status_code;
		};

		response.headers = match &headers {
			Some(_) => headers,
			None => {
				let mut h: HashMap<&str, String> = HashMap::new();
				h.insert("Content-Type", "text/plain".to_string());
				Some(h)
			}
		};

		response.status_text = status_text;

		response.body = body;
		response
	}

	pub fn _continue(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "100", "Continue")
	}

	pub fn switching_protocol(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> Self {
		Self::new_from_status(headers, body, "101", "Switching Protocol")
	}

	pub fn early_hints(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "103", "Early Hints")
	}

	pub fn ok(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "200", "OK")
	}

	pub fn created(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "201", "Created")
	}

	pub fn accepted(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "202", "Accepted")
	}

	pub fn non_authoritative_information(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> Self {
		Self::new_from_status(headers, body, "203", "Non-Authoritative Information")
	}

	pub fn no_content(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "204", "No Content")
	}

	pub fn reset_content(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "205", "Reset Content")
	}

	pub fn partial_content(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "206", "Partial Content")
	}

	pub fn found(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "302", "Found")
	}

	pub fn see_other(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "303", "See Other")
	}

	pub fn not_modified(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "304", "Not Modified")
	}

	pub fn temporary_redirect(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> Self {
		Self::new_from_status(headers, body, "307", "Temporary Redirect")
	}

	pub fn permanent_redirect(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> Self {
		Self::new_from_status(headers, body, "308", "Permanent Redirect")
	}

	pub fn bad_request(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "400", "Bad Request")
	}

	pub fn unauthorized(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "401", "Unauthorized")
	}

	pub fn forbidden(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "403", "Forbidden")
	}

	pub fn not_found(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "404", "Not Found")
	}

	pub fn method_not_allowed(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> Self {
		Self::new_from_status(headers, body, "405", "Method Not Allowed")
	}

	pub fn request_timeout(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "408", "Request Timeout")
	}

	pub fn gone(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "410", "Gone")
	}

	pub fn im_a_teapot(headers: Option<HashMap<&'a str, String>>, body: Option<String>) -> Self {
		Self::new_from_status(headers, body, "418", "I'm a teapot")
	}

	pub fn internal_server_error(
		headers: Option<HashMap<&'a str, String>>,
		body: Option<String>
	) -> Self {
		Self::new_from_status(headers, body, "500", "Internal Server Error")
	}
}
