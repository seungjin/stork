pub mod bindings;
pub mod request;
pub mod s3;

pub struct Component;

bindings::export!(Component with_types_in bindings);
