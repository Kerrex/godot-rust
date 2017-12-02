use gdnative_sys::*;
use godot::*;
use std::mem;
use std::ops::{Add, AddAssign, Index, IndexMut};
use std::ptr;
use std::ffi::CString;
use std::cmp::Ordering;
use node_path::*;
use basis::GDBasis;
use color::GDColor;
use quat::GDQuat;
use plane::GDPlane;
use rect2::GDRect2;
use rect3::GDRect3;

#[derive(Clone)]
#[repr(C)]
pub struct GDString {
    pub(crate) _string: godot_string,
}

fn new_gd_string() -> godot_string {
    godot_string {
        _dont_touch_that: [0; 8usize],
    }
}

impl GDString {
    pub fn new() -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            godot_string_new(&mut new_string);
            GDString {
                _string: new_string,
            }
        }
    }

    pub fn from_char_array(chars: &[char]) -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            let pointer = chars.as_ptr() as *const i8;
            godot_string_new_data(&mut new_string, pointer, chars.len() as i32);
            GDString {
                _string: new_string,
            }
        }
    }

    pub fn from_string(variable: String) -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            let chars = variable.as_ptr() as *const i8;
            godot_string_new_data(&mut new_string, chars, variable.len() as i32);
            GDString {
                _string: new_string,
            }
        }
    }

    pub fn from_gdstring(variable: GDString) -> GDString {
        unsafe {
            let mut new_string = new_gd_string();
            let existing_string = variable._string;
            godot_string_new_copy(&mut new_string, &existing_string);
            GDString {
                _string: new_string,
            }
        }
    }

    pub fn len(&mut self) -> u32 {
        unsafe {
            let mut len = 0;
            godot_string_get_data(&mut self._string, ptr::null_mut(), &mut len);
            len as u32
        }
    }
}

impl Drop for GDString {
    fn drop(&mut self) {
        unsafe { godot_string_destroy(&mut self._string) }
    }
}


impl Index<i32> for GDString {
    type Output = char;

    fn index(&self, index: i32) -> &char {
        let mut string = self._string;
        unsafe {
            let v = godot_string_operator_index(&mut string, index);
            let d = v as *mut char;
            mem::transmute::<*mut char, &char>(d)
        }
    }
}

impl IndexMut<i32> for GDString {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        let mut string = self._string;
        unsafe {
            let v = godot_string_operator_index(&mut string, index);
            let d = v as *mut char;
            &mut *d
        }
    }
}

impl ToString for GDString {
    fn to_string(&self) -> String {
        unsafe {
            let str_in_u8 = godot_string_c_str(&self._string);
            let mut str_mutable = str_in_u8 as *mut i8;
            let result = CString::from_raw(str_mutable).into_string().unwrap();
            result
        }
    }
}

impl PartialEq for GDString {
    fn eq(&self, other: &GDString) -> bool {
        unsafe {
            let other_gd_string = other._string;
            let this_gd_string = self._string;
            godot_string_operator_equal(&this_gd_string, &other_gd_string)
        }
    }

    fn ne(&self, other: &GDString) -> bool {
        !(self != other)
    }
}

impl Eq for GDString {}

impl PartialOrd for GDString {
    fn partial_cmp(&self, other: &GDString) -> Option<Ordering> {
        let is_less = self < other;
        if is_less {
            return Some(Ordering::Less);
        }
        let is_greater = self > other;
        if is_greater {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &GDString) -> bool {
        unsafe {
            let this_gd_string = self._string;
            let other_gd_string = other._string;
            godot_string_operator_less(&this_gd_string, &other_gd_string)
        }
    }

    fn le(&self, other: &GDString) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &GDString) -> bool {
        !(self <= other)
    }

    fn ge(&self, other: &GDString) -> bool {
        !(self < other)
    }
}

impl Ord for GDString {
    fn cmp(&self, other: &GDString) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for GDString {
    type Output = GDString;

    fn add(self, string_to_add: GDString) -> Self::Output {
        unsafe {
            GDString {
                _string: godot_string_operator_plus(&self._string, &string_to_add._string),
            }
        }
    }
}

impl AddAssign for GDString {
    fn add_assign(&mut self, string_to_add: GDString) {
        unsafe {
            self._string = godot_string_operator_plus(&self._string, &string_to_add._string);
        }
    }
}

impl From<godot_string> for GDString {
    fn from(string: godot_string) -> Self {
        GDString { _string: string }
    }
}

impl<'a> From<&'a GDBasis> for GDString {
    fn from(basis: &GDBasis) -> Self {
        unsafe {
            let gd_string = godot_basis_as_string(&basis._basis);
            GDString { _string: gd_string }
        }
    }
}

impl From<GDColor> for GDString {
    fn from(color: GDColor) -> Self {
        unsafe {
            let gd_string = godot_color_as_string(&color._color);
            GDString { _string: gd_string }
        }
    }
}

impl From<GDQuat> for GDString {
    fn from(quat: GDQuat) -> Self {
        unsafe {
            let gd_string = godot_quat_as_string(&quat._quat);
            GDString { _string: gd_string }
        }
    }
}

impl From<GDPlane> for GDString {
    fn from(plane: GDPlane) -> Self {
        unsafe {
            let gd_string = godot_plane_as_string(&plane._plane);
            GDString { _string: gd_string }
        }
    }
}

impl From<GDRect2> for GDString {
    fn from(rect2: GDRect2) -> Self {
        unsafe {
            let gd_string = godot_rect2_as_string(&rect2._rect);
            GDString { _string: gd_string }
        }
    }
}

impl From<GDRect3> for GDString {
    fn from(rect3: GDRect3) -> Self {
        unsafe {
            let gd_string = godot_rect3_as_string(&rect3._rect);
            GDString { _string: gd_string }
        }
    }
}

//TODO Finish String
