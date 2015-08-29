
#[macro_export]
macro_rules! main_try {
    ($expression:expr) => (match $expression {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            println!("{:?}", err);
            return
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
