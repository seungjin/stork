use crate::bindings;
use bindings::exports::seungjin::stork::s3;

impl s3::Guest for crate::Component {
    fn foo() -> String {
        "rasarsars".to_string()
    }
}
