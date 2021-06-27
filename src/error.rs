use crate::ErrorKind;

fn parse_err_msg(msg: &mut String, pattern: &str) -> bool {
    if msg.starts_with(pattern) {
        let new_msg = msg[pattern.len()..].to_string();
        *msg = new_msg;
        true
    } else {
        false
    }
}

pub(crate) fn to_msg_and_kind(orig_what: &str) -> (String, ErrorKind) {
    let mut msg: String = orig_what.to_string();
    {
        // This must be kept in sync with catcher.h. As described there, we
        // parse the error message to generate the corresponding Rust error.
        let kind = if parse_err_msg(&mut msg, "Pylon::AccessException: ") {
            ErrorKind::AccessException
        } else if parse_err_msg(&mut msg, "Pylon::BadAllocException: ") {
            ErrorKind::BadAllocException
        } else if parse_err_msg(&mut msg, "Pylon::DynamicCastException: ") {
            ErrorKind::DynamicCastException
        } else if parse_err_msg(&mut msg, "Pylon::LogicalErrorException: ") {
            ErrorKind::LogicalErrorException
        } else if parse_err_msg(&mut msg, "Pylon::OutOfRangeException: ") {
            ErrorKind::OutOfRangeException
        } else if parse_err_msg(&mut msg, "Pylon::PropertyException: ") {
            ErrorKind::PropertyException
        } else if parse_err_msg(&mut msg, "Pylon::TimeoutException: ") {
            ErrorKind::TimeoutException
        } else if parse_err_msg(&mut msg, "Pylon::RuntimeException: ") {
            ErrorKind::RuntimeException
        } else if parse_err_msg(&mut msg, "Pylon::GenericException: ") {
            ErrorKind::GenericException
        } else if parse_err_msg(&mut msg, "std::exception: ") {
            ErrorKind::StdCppException
        } else {

            #[cfg(target_os="windows")]
            {
                if parse_err_msg(&mut msg, "Pylon::AviWriterFatalException: ") {
                    ErrorKind::AviWriterFatalException
                } else {
                    panic!("failed conversion of exception message: {}", msg);
                }
            }

            #[cfg(not(target_os="windows"))]
            panic!("failed conversion of exception message: {}", msg);

        };
        (msg, kind)
    }
}
