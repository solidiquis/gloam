#[macro_export]
macro_rules! try_into {
    ($i:expr) => {
        $i.try_into().unwrap()
    };
}
pub use try_into;

#[macro_export]
macro_rules! try_convert {
    ($i:expr) => {
        $i.try_into().map_err(crate::Error::boxed)?
    };
}
pub use try_convert;

pub fn as_gl_bool(val: bool) -> gl::types::GLboolean {
    if val {
        gl::TRUE
    } else {
        gl::FALSE
    }
}
