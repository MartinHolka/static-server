use std::fs;
use std::path::Path;

pub fn handle_get(path: &str, root_dir: &str) -> Vec<u8> {
    if path.contains("..") {
        let status_line = "HTTP/1.1 400 Bad Request\r\n";
        return status_line.as_bytes().to_vec();
    };

    let mut full_path = Path::new(root_dir).join(&path[1..]); // remove leading "/"
    if path.ends_with("/") {
        full_path = full_path.join("index.html");
    } else if full_path.extension().is_none() {
        full_path = full_path.with_extension("html")
    }

    let (status_line, body) = match fs::read(&full_path) {
        Ok(contents) => ("HTTP/1.1 200 OK\r\n", contents),
        Err(_) => ("HTTP/1.1 404 Not Found\r\n", b"404 Not Found".to_vec()),
    };

    let content_type = match full_path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    };

    let headers = format!(
        "Content-Length: {}\r\nContent-Type: {}\r\n\r\n",
        body.len(),
        content_type
    );

    [status_line.as_bytes(), headers.as_bytes(), &body].concat()
}
