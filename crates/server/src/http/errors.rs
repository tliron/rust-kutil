use {http::StatusCode, std::error::Error, std::fmt};

//
// BoxedError
//

/// Boxed error.
pub type BoxedError = Box<dyn Error + Send + Sync>;

//
// MapErrorStatusCode
//

/// Map result error to a status code.
pub trait MapErrorStatusCode<OkT> {
    /// Map result error to a status code.
    fn map_err_status_code(self, status: StatusCode, message: &str) -> Result<OkT, StatusCode>;

    /// Map result error to [StatusCode::INTERNAL_SERVER_ERROR].
    fn map_err_internal_server(self, message: &str) -> Result<OkT, StatusCode>;
}

impl<OkT, FromErrorT> MapErrorStatusCode<OkT> for Result<OkT, FromErrorT>
where
    FromErrorT: fmt::Display,
{
    fn map_err_status_code(self, status: StatusCode, message: &str) -> Result<OkT, StatusCode> {
        self.map_err(|error| {
            tracing::error!("{}: {}", message, error);
            status
        })
    }

    fn map_err_internal_server(self, message: &str) -> Result<OkT, StatusCode> {
        self.map_err_status_code(StatusCode::INTERNAL_SERVER_ERROR, message)
    }
}
