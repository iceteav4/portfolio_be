#[macro_export]
macro_rules! try_api_response {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return ApiResponse::from(e),
        }
    };
}
