//! src/cow_type/definition.rs
use std::borrow::Cow;

#[derive(Debug)]
struct ErrorInfo {
    error: LocalError,
    message: String,
}

#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}

fn generate_message(message: &'static str, error_info: Option<ErrorInfo>) -> Cow<'static, str> {
    match error_info {
        None => message.into(),
        Some(info) => {
            let info_error = info.error;
            let info_message = info.message;
            format!("{message}: {info_error:?} {info_message:?}").into()
        }
    }
}

pub fn index(show: bool) {
    if show {
        println!("---Cow Definition");

        let msg1 = generate_message("Everything is fine", None);

        let msg2 = generate_message(
            "Got an error",
            Some(ErrorInfo {
                error: LocalError::TooBig,
                message: "It was too big".to_string(),
            }),
        );

        let msg3 = generate_message(
            "Got an error",
            Some(ErrorInfo {
                error: LocalError::TooSmall,
                message: "It was too small".to_string(),
            }),
        );

        for msg in [msg1, msg2, msg3] {
            match msg {
                Cow::Borrowed(msg) => {
                    println!("Borrowed, didn't need an allocation:\n {msg}")
                }
                Cow::Owned(msg) => {
                    println!("Owned, because we needed an allocation:\n {msg}")
                }
            }
        }
    }
}
