//! src/generics/question_operator.rs
use crate::print_title;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn parse_i32_bytes(b: &[u8]) -> GenericResult<i32> {
    Ok(std::str::from_utf8(b)?.parse::<i32>()?)
}

pub fn index(show: bool) {
    if show {
        print_title("Question Operator");

        let ip_local = [127]; //[u8::from(127)];

        let result = parse_i32_bytes(&ip_local);

        println!("result: {:?}", result);
    }
}
