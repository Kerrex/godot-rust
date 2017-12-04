use gdnative_sys::*;
use godot::*;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use std::cmp::{Eq, Ordering, PartialEq};

fn new_rect3() -> godot_rect3 {
    godot_rect3 {
        _dont_touch_that: [0; 24usize],
    }
}

impl GDRect3 {
    pub fn new(position: &GDVector3, size: &GDVector3) -> GDRect3 {
        unsafe {
            let mut new_rect = new_rect3();
            godot_rect3_new(&mut new_rect, &position._vector, &size._vector);
            GDRect3 { _rect: new_rect }
        }
    }

    pub fn get_position(&self) -> GDVector3 {
        unsafe {
            let position = godot_rect3_get_position(&self._rect);
            GDVector3 { _vector: position }
        }
    }

    pub fn set_position(&self, new_position: &GDVector3) {
        unsafe {
            godot_rect3_set_position(&self._rect, &new_position._vector);
        }
    }

    pub fn get_size(&self) -> GDVector3 {
        unsafe {
            let size = godot_rect3_get_size(&self._rect);
            GDVector3 { _vector: size }
        }
    }

    pub fn set_size(&self, new_size: &GDVector3) {
        unsafe {
            godot_rect3_set_size(&self._rect, &new_size._vector);
        }
    }

    pub fn get_area(&self) -> f32 {
        unsafe { godot_rect3_get_area(&self._rect) }
    }

    pub fn has_no_area(&self) -> bool {
        unsafe { godot_rect3_has_no_area(&self._rect) }
    }

    pub fn has_no_surface(&self) -> bool {
        unsafe { godot_rect3_has_no_surface(&self._rect) }
    }

    pub fn intersects(&self, other: &GDRect3) -> bool {
        unsafe { godot_rect3_intersects(&self._rect, &other._rect) }
    }

    pub fn encloses(&self, other: &GDRect3) -> bool {
        unsafe { godot_rect3_encloses(&self._rect, &other._rect) }
    }

    pub fn merged(&self, other: &GDRect3) -> GDRect3 {
        unsafe {
            let merged = godot_rect3_merge(&self._rect, &other._rect);
            GDRect3 { _rect: merged }
        }
    }

    pub fn insersection(&self, other: &GDRect3) -> GDRect3 {
        unsafe {
            let insersected = godot_rect3_intersection(&self._rect, &other._rect);
            GDRect3 { _rect: insersected }
        }
    }

    pub fn intersects_plane(&self, plane: &GDPlane) -> bool {
        unsafe { godot_rect3_intersects_plane(&self._rect, &plane._plane) }
    }

    pub fn intersects_segment(&self, from: &GDVector3, to: &GDVector3) -> bool {
        unsafe { godot_rect3_intersects_segment(&self._rect, &from._vector, &to._vector) }
    }

    pub fn has_point(&self, point: &GDVector3) -> bool {
        unsafe { godot_rect3_has_point(&self._rect, &point._vector) }
    }

    pub fn get_support(&self, direction: &GDVector3) -> GDVector3 {
        unsafe {
            let support = godot_rect3_get_support(&self._rect, &direction._vector);
            GDVector3 { _vector: support }
        }
    }

    pub fn get_longest_axis(&self) -> GDVector3 {
        unsafe {
            let vector = godot_rect3_get_longest_axis(&self._rect);
            GDVector3 { _vector: vector }
        }
    }

    pub fn get_longest_axis_index(&self) -> i32 {
        unsafe { godot_rect3_get_longest_axis_index(&self._rect) }
    }

    pub fn get_longest_axis_size(&self) -> f32 {
        unsafe { godot_rect3_get_longest_axis_size(&self._rect) }
    }

    pub fn get_shortest_axis(&self) -> GDVector3 {
        unsafe {
            let axis = godot_rect3_get_shortest_axis(&self._rect);
            GDVector3 { _vector: axis }
        }
    }

    pub fn get_shortest_axis_index(&self) -> i32 {
        unsafe { godot_rect3_get_shortest_axis_index(&self._rect) }
    }

    pub fn get_shortest_axis_size(&self) -> f32 {
        unsafe { godot_rect3_get_shortest_axis_size(&self._rect) }
    }

    pub fn expanded(&self, to_point: &GDVector3) -> GDRect3 {
        unsafe {
            let expanded = godot_rect3_expand(&self._rect, &to_point._vector);
            GDRect3 { _rect: expanded }
        }
    }

    pub fn growed(&self, by: f32) -> GDRect3 {
        unsafe {
            let growed = godot_rect3_grow(&self._rect, by);
            GDRect3 { _rect: growed }
        }
    }

    pub fn get_endpoint(&self, index: i32) -> GDVector3 {
        unsafe {
            let endpoint = godot_rect3_get_endpoint(&self._rect, index);
            GDVector3 { _vector: endpoint }
        }
    }
}

impl PartialEq for GDRect3 {
    fn eq(&self, other: &GDRect3) -> bool {
        unsafe {
            let other_gd_dict = other._rect;
            let this_gd_dict = self._rect;
            godot_rect3_operator_equal(&this_gd_dict, &other_gd_dict)
        }
    }

    fn ne(&self, other: &GDRect3) -> bool {
        !(self != other)
    }
}

impl Eq for GDRect3 {}
