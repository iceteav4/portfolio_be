#[macro_export]
macro_rules! to_api_res {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return ApiResponse::from(e),
        }
    };
}
