use std::error::Error;

pub type CommonResult<T> = Result<T, Box<dyn Error>>;
