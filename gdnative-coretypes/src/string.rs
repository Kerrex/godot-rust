use gdnative_sys::*;
use godot::*;

#[derive(Copy, Clone)]
struct GDString {
    pub ( crate ) _string: godot_string
}

fn new_gd_string() -> godot_string {
    godot_string { _dont_touch_that: [0; 8usize] }
}

impl GDString {
    pub fn new() -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            godot_string_new(&mut new_string);
            GDString { _string: new_string }
        }
    }

    pub fn from_char_array(chars: &[char]) -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            let pointer = chars.as_ptr() as *const i8;
            godot_string_new_data(&mut new_string, pointer, chars.len() as i32);
            GDString { _string: new_string }
        }
    }

    pub fn from_string(variable: String) -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            let chars = variable.as_ptr() as *const i8;
            godot_string_new_data(&mut new_string, chars, variable.len() as i32);
            GDString { _string: new_string }
        }
    }

    pub fn from_gdstring(variable: GDString) -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            let existing_string = variable._string;
            godot_string_new_copy(&mut new_string, &existing_string);
            GDString { _string: new_string }
        }
    }

    //TODO Finish String
}

