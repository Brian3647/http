# s-http

A (very) simple library for parsing http requests and generating responses.

## Usage

Add this to your `Cargo.toml` dependencies:

```
http = { git = "https://github.com/Squioole/http" }
```

Example code:

```rs
use http::request::{HttpRequest, Method, Version, Resource};
use std::collections::HashMap;

let raw_request = String::from("GET /example HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: rust\r\nAccept: */*\r\n\r\nhello world!");

let req: HttpRequest = raw_request.into();
let mut headers_expected = HashMap::new();
headers_expected.insert("Host".into(), "localhost".into());
headers_expected.insert("Accept".into(), "*/*".into());
headers_expected.insert("User-Agent".into(), "rust".into());
assert_eq!(Method::Get, req.method);
assert_eq!(Version::V1_1, req.version);
assert_eq!(Resource::Path("/example".to_string()), req.resource);
assert_eq!(headers_expected, req.headers);
assert_eq!("hello world!", req.msg_body);
```
