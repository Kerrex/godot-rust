use gdnative_sys::*;
use vector3::GDVector3;
use vector2::GDVector2;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use std::cmp::{Eq, Ordering, PartialEq};

#[repr(C)]
#[derive(Clone)]
pub struct GDRect2 {
    pub(crate) _rect: godot_rect2,
}

fn new_rect2() -> godot_rect2 {
    godot_rect2 {
        _dont_touch_that: [0; 16usize],
    }
}

impl GDRect2 {
    pub fn new_from_vectors2(position: &GDVector2, size: &GDVector2) -> GDRect2 {
        unsafe {
            let mut new_rect = new_rect2();
            godot_rect2_new_with_position_and_size(&mut new_rect, &position._vector, &size._vector);
            GDRect2 { _rect: new_rect }
        }
    }

    pub fn new_from_f32(x: f32, y: f32, width: f32, height: f32) -> GDRect2 {
        unsafe {
            let mut new_rect = new_rect2();
            godot_rect2_new(&mut new_rect, x, y, width, height);
            GDRect2 { _rect: new_rect }
        }
    }

    pub fn get_area(&self) -> f32 {
        unsafe {
            godot_rect2_get_area(&self._rect)
        }
    }

    pub fn intersects(&self, other: &GDRect2) -> bool {
        unsafe {
            godot_rect2_intersects(&self._rect, &other._rect)
        }
    }

    pub fn encloses(&self, other: &GDRect2) -> bool {
        unsafe {
            godot_rect2_encloses(&self._rect, &other._rect)
        }
    }

    pub fn has_no_area(&self) -> bool {
        unsafe {
            godot_rect2_has_no_area(&self._rect)
        }
    }

    pub fn clipped(&self, other: &GDRect2) -> GDRect2 {
        unsafe {
            let gd_rect = godot_rect2_clip(&self._rect, &other._rect);
            GDRect2 { _rect: gd_rect }
        }
    }

    pub fn merged(&self, other: &GDRect2) -> GDRect2 {
        unsafe {
            let gd_rect = godot_rect2_merge(&self._rect, &other._rect);
            GDRect2 { _rect: gd_rect }
        }
    }

    pub fn has_point(&self, vec: &GDVector2) -> bool {
        unsafe {
            godot_rect2_has_point(&self._rect, &vec._vector)
        }
    }

    pub fn growed(&self, by: f32) -> GDRect2 {
        unsafe {
            let growed = godot_rect2_grow(&self._rect, by);
            GDRect2 { _rect: growed }
        }
    }

    pub fn expanded(&self, to: &GDVector2) -> GDRect2 {
        unsafe {
            let expanded = godot_rect2_expand(&self._rect, &to._vector);
            GDRect2 { _rect: expanded }
        }
    }

    pub fn get_position(&self) -> GDVector2 {
        unsafe {
            let position = godot_rect2_get_position(&self._rect);
            GDVector2 { _vector: position }
        }
    }

    pub fn get_size(&self) -> GDVector2 {
        unsafe {
            let size = godot_rect2_get_size(&self._rect);
            GDVector2 { _vector: size }
        }
    }

    pub fn set_position(&mut self, new_position: &GDVector2) {
        unsafe {
            godot_rect2_set_position(&mut self._rect, &new_position._vector)
        }
    }

    pub fn set_size(&mut self, new_size: &GDVector2) {
        unsafe {
            godot_rect2_set_size(&mut self._rect, &new_size._vector)
        }
    }
}

impl PartialEq for GDRect2 {
    fn eq(&self, other: &GDRect2) -> bool {
        unsafe {
            let other_gd_dict = other._rect;
            let this_gd_dict = self._rect;
            godot_rect2_operator_equal(&this_gd_dict, &other_gd_dict)
        }
    }

    fn ne(&self, other: &GDRect2) -> bool {
        !(self != other)
    }
}

impl Eq for GDRect2 {}