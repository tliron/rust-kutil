use {http::*, std::path::Path, tower::*, tower_http::services::fs::*};

/// Serve file.
pub async fn response_from_file<PathT>(path: PathT) -> Response<ServeFileSystemResponseBody>
where
    PathT: AsRef<Path>,
{
    ServeFile::new(path).oneshot(Request::new(())).await.expect("infallible")
}
