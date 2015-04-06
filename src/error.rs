//! Error handling utilities.

use cl::cl_int;
use cl::CLStatus::CL_SUCCESS;

fn error_str(status_code: cl_int) -> String {
    // FIXME -- after FromPrimitive removed, we need a better way to handle
    // this
    match status_code {
        0 => format!("CL_SUCCESS"),
        -1 => format!("DEVICE_NOT_FOUND"),
        _ => format!("Unknown Error: {}", status_code)
    }
}

pub fn check(status: cl_int, message: &str) {
    if status != CL_SUCCESS as cl_int {
        panic!("{} ({})", message, error_str(status))
    }
}
