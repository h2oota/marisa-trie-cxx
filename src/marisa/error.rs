use crate::ffi::{
    exception_record,
    exception_name,
    exception_message,
};

#[derive(Debug)]
pub struct MarisaError {
    source: String,
    message: String,
}

impl MarisaError {
    pub fn new(s: &str, m: &str) -> Self {
	Self {
	    source: String::from(s),
	    message: String::from(m),
	}
    }
}

impl std::fmt::Display for MarisaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", &self.source, &self.message)
    }
}

impl std::error::Error for MarisaError {}

pub trait MarisaException {
    fn from_exception(err_record: *const exception_record) -> Self;
}

impl MarisaException for MarisaError {
    fn from_exception(err_record: *const exception_record) -> Self
    {
	unsafe {
	    Self {
		source: std::ffi::CStr::from_ptr(exception_name(err_record)).to_string_lossy().into_owned(),
		message: std::ffi::CStr::from_ptr(exception_message(err_record)).to_string_lossy().into_owned()
	    }
	}
    }
}
