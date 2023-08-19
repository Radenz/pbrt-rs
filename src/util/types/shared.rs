use std::sync::{Arc, Mutex};

pub type Shared<T> = Arc<Mutex<T>>;

pub fn shared<T>(value: T) -> Shared<T> {
    Arc::new(Mutex::new(value))
}

#[macro_export]
macro_rules! atomic {
    ($name:ident from $lock:expr, $expr:expr) => {
        match $lock.lock() {
            Err(_) => panic!("Lock error"),
            Ok($name) => $expr,
        }
    };
    ($name:ident from $lock:expr, $ok_expr:expr, $err_expr:expr) => {
        match $lock.lock() {
            Ok($name) => $expr,
            Err(_) => $err_expr,
        }
    };
    (mut $name:ident from $lock:expr, $expr:expr) => {
        match $lock.lock() {
            Err(_) => panic!("Lock error"),
            Ok(mut $name) => $expr,
        }
    };
    (mut $name:ident from $lock:expr, $ok_expr:expr, $err_expr:expr) => {
        match $lock.lock() {
            Ok(mut $name) => $ok_expr,
            Err(_) => $err_expr,
        }
    };
}

pub use atomic;
