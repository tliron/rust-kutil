/// If the expression is [Err], give the error and optionally use a default expression.
#[macro_export]
macro_rules! unwrap_or_give {
    ( $result:expr, $errors:expr $(,)? ) => {
        if let ::std::result::Result::Err(error) = $result {
            $errors.give(error)?;
        }
    };

    ( $result:expr, $errors:expr, $default:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                $errors.give(error)?;
                $default
            }
        }
    };
}

/// If the expression is [Err], give the error return an expression.
///
/// Meant to be used similarly to the `?` operator.
#[macro_export]
macro_rules! unwrap_or_give_and_return {
    ( $result:expr, $errors:expr, $return:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                $errors.give(error)?;
                return $return;
            }
        }
    };
}
