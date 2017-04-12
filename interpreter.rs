#![crate_type = "dylib"]

use std::ffi::CStr;
use std::os::raw::c_char;

pub struct AstIdentifier {
    ident: String,
}

#[no_mangle]
pub extern fn create_identifier(ident: *const c_char) -> AstIdentifier {
    let ident_string = unsafe {
        assert!(!ident.is_null());
        let ident_str = CStr::from_ptr(ident).to_str()
            .expect("Not a valid string.");

        String::from(ident_str).clone()
    };
    
    AstIdentifier {
        ident: ident_string
    }
}

#[no_mangle]
pub extern fn print_identifier(ident: AstIdentifier) {
    println!("{}", ident.ident);
}

#[no_mangle]
pub extern fn execute_rust() {
    println!("Rust is executing");
}

