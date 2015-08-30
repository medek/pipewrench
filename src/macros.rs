
#[macro_export]
macro_rules! main_try {
    ($expression:expr) => (match $expression {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            panic!("{:?}", err)
        }
    })
}

macro_rules! println_err {
    ($($arg:tt)*) => (
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), $($arg)*) {
            Ok(_) => {},
            Err(e) => panic!("Unable to write to stderr: {}", e)
        })
}

#[macro_export]
macro_rules! config_string {
    ($expr:expr) => (
            $crate::config::Value::String($expr.to_string())
        )
}

#[macro_export]
macro_rules! config_int {
    ($expr:expr) => (
        $crate::config::Value::Integer($expr as i64)
        )
}

#[macro_export]
macro_rules! config_bool {
    ($expr:expr) => (
        $crate::config::Value::Boolean($expr as bool)
        )
}
