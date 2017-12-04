use gdnative_sys::*;
use godot::*;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use std::cmp::{Eq, Ordering, PartialEq};

fn new_rid() -> godot_rid {
    godot_rid {_dont_touch_that: [0; 8]}
}

impl GDRid {
    pub fn new() -> GDRid {
        unsafe {
            let mut new_rid = new_rid();
            godot_rid_new(&mut new_rid);
            GDRid { _rid: new_rid }
        }
    }

    pub fn new_with_resource(from: &GDObject) -> GDRid {
        unsafe {
            let mut new_rid = new_rid();
            godot_rid_new_with_resource(&mut new_rid, &from._object);
            GDRid { _rid: new_rid }
        }
    }

    pub fn get_id(&self) -> i32 {
        unsafe {
            godot_rid_get_id(&self._rid)
        }
    }
}

impl PartialEq for GDRid {
    fn eq(&self, other: &GDRid) -> bool {
        unsafe {
            let other_gd_dict = other._rid;
            let this_gd_dict = self._rid;
            godot_rid_operator_equal(&this_gd_dict, &other_gd_dict)
        }
    }

    fn ne(&self, other: &GDRid) -> bool {
        !(self != other)
    }
}

impl Eq for GDRid {}

impl PartialOrd for GDRid {
    fn partial_cmp(&self, other: &GDRid) -> Option<Ordering> {
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

    fn lt(&self, other: &GDRid) -> bool {
        unsafe {
            let this_gd_rid = self._rid;
            let other_gd_rid = other._rid;
            godot_rid_operator_less(&this_gd_rid, &other_gd_rid)
        }
    }

    fn le(&self, other: &GDRid) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &GDRid) -> bool {
        !(self <= other)
    }

    fn ge(&self, other: &GDRid) -> bool {
        !(self < other)
    }
}

impl Ord for GDRid {
    fn cmp(&self, other: &GDRid) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
