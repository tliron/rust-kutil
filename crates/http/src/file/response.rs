use {
    http::{header::*, *},
    std::path::*,
    tower::*,
    tower_http::services::fs::*,
};

/// Serve file.
pub async fn response_from_file<PathT>(path: PathT, transient: bool) -> Response<ServeFileSystemResponseBody>
where
    PathT: AsRef<Path>,
{
    let mut response = ServeFile::new(path).oneshot(Request::new(())).await.expect("infallible");
    if transient {
        let headers = response.headers_mut();
        headers.remove(LAST_MODIFIED);
    }
    response
}
