use std::{collections::HashMap, fmt::Display};

/// Resource requested
#[derive(Debug, PartialEq, Clone)]
pub enum Resource {
	/// A path for a subpage
	Path(String)
}

/// Http Request struct.
/// ```
/// use http::request::{HttpRequest, Method, Version, Resource};
/// use std::collections::HashMap;
///
/// let raw_request = String::from("GET /example HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: rust\r\nAccept: */*\r\n\r\nhello world!");
///
/// let req: HttpRequest = raw_request.into();
/// let mut headers_expected = HashMap::new();
/// headers_expected.insert("Host".into(), "localhost".into());
/// headers_expected.insert("Accept".into(), "*/*".into());
/// headers_expected.insert("User-Agent".into(), "rust".into());
/// assert_eq!(Method::Get, req.method);
/// assert_eq!(Version::V1_1, req.version);
/// assert_eq!(Resource::Path("/example".to_string()), req.resource);
/// assert_eq!(headers_expected, req.headers);
/// assert_eq!("hello world!", req.msg_body);
/// ```
#[derive(Debug, Clone)]
pub struct HttpRequest {
	pub method: Method,
	pub version: Version,
	pub resource: Resource,
	pub headers: HashMap<String, String>,
	pub msg_body: String
}

impl From<String> for HttpRequest {
	fn from(req: String) -> Self {
		let mut parsed_method = Method::Uninitialized;
		let mut parsed_version = Version::V1_1;
		let mut parsed_resource = Resource::Path("".to_string());
		let mut parsed_headers = HashMap::new();
		let mut parsed_msg_body = "".to_string();
		let mut in_body = false;

		for line in req.lines() {
			if !in_body {
				if line.contains("HTTP") {
					let (method, resource, version) = process_req_line(line);
					parsed_method = method;
					parsed_version = version;
					parsed_resource = resource;
				} else if line.contains(':') {
					let (key, value) = process_header_line(line);
					parsed_headers.insert(key, value);
				} else if line.is_empty() {
					// Blank line. Next line will be processed as the body.
					in_body = true;
				}
			} else {
				parsed_msg_body.push_str(line);
			}
		}

		HttpRequest {
			method: parsed_method,
			version: parsed_version,
			resource: parsed_resource,
			headers: parsed_headers,
			msg_body: parsed_msg_body.trim_end_matches('\u{0}').into()
		}
	}
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
	let mut words = s.split_whitespace();
	let method = words.next().unwrap();
	let resource = words.next().unwrap();
	let version = words.next().unwrap();

	(
		method.into(),
		Resource::Path(resource.to_string()),
		version.into()
	)
}

fn process_header_line(s: &str) -> (String, String) {
	let mut header_items = s.split(':');
	let mut key = String::from("");
	let mut value = String::from("");

	if let Some(k) = header_items.next() {
		key = k.to_string();
	}

	if let Some(v) = header_items.next() {
		value = v.to_string().trim_start().to_string()
	}

	(key, value)
}

/// Http method
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
	Get,
	Post,
	Head,
	Put,
	Delete,
	Connect,
	Options,
	Trace,
	Patch,
	Uninitialized
}

impl Display for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("{:?}", self).to_uppercase())
	}
}

impl From<&str> for Method {
	fn from(s: &str) -> Method {
		match s {
			"GET" => Method::Get,
			"POST" => Method::Post,
			"HEAD" => Method::Head,
			"OPTIONS" => Method::Options,
			"TRACE" => Method::Trace,
			"PATCH" => Method::Patch,
			_ => Method::Uninitialized
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Version {
	V1_1,
	V2_0,
	Uninitialized
}

impl From<&str> for Version {
	fn from(s: &str) -> Version {
		match s {
			"HTTP/1.1" => Version::V1_1,
			"HTTP/2.0" => Version::V2_0,
			_ => Version::Uninitialized
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::request::*;

	#[test]
	fn test_method_into() {
		let m: Method = "GET".into();
		assert_eq!(m, Method::Get);
	}

	#[test]
	fn test_version_into() {
		let m: Version = "HTTP/1.1".into();
		let m2: Version = "HTTP/2.0".into();
		let m3: Version = "321dshaui".into();
		assert_eq!(m, Version::V1_1);
		assert_eq!(m2, Version::V2_0);
		assert_eq!(m3, Version::Uninitialized);
	}

	#[test]
	fn test_read_http() {
		let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\ntestbody123");
		let mut headers_expected = HashMap::new();
		headers_expected.insert("Host".into(), "localhost".into());
		headers_expected.insert("Accept".into(), "*/*".into());
		headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
		let req: HttpRequest = s.into();
		assert_eq!(Method::Get, req.method);
		assert_eq!(Version::V1_1, req.version);
		assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
		assert_eq!(headers_expected, req.headers);
		assert_eq!("testbody123", req.msg_body);
	}
}
