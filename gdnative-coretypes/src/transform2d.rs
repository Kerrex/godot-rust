use gdnative_sys::*;
use vector3::GDVector3;
use vector2::GDVector2;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use std::cmp::{Eq, Ordering, PartialEq};
use rect2::GDRect2;

#[repr(C)]
pub struct GDTransform2D {
    pub(crate) _transform: godot_transform2d,
}

fn new_transform() -> godot_transform2d {
    godot_transform2d {
        _dont_touch_that: [0; 24usize],
    }
}

impl GDTransform2D {
    pub fn new_by_rotation_and_position(rotation: f32, position: &GDVector2) -> GDTransform2D {
        unsafe {
            let mut new = new_transform();
            godot_transform2d_new(&mut new, rotation, &position._vector);
            GDTransform2D { _transform: new }
        }
    }

    pub fn new() -> GDTransform2D {
        unsafe {
            let mut new = new_transform();
            godot_transform2d_new_identity(&mut new);
            GDTransform2D { _transform: new }
        }
    }

    pub fn new_axis_origin(x: &GDVector2, y: &GDVector2, origin: &GDVector2) -> GDTransform2D {
        unsafe {
            let mut new = new_transform();
            godot_transform2d_new_axis_origin(&mut new, &x._vector, &y._vector, &origin._vector);
            GDTransform2D { _transform: new }
        }
    }

    pub fn inversed(&self) -> GDTransform2D {
        unsafe {
            let inversed = godot_transform2d_inverse(&self._transform);
            GDTransform2D {
                _transform: inversed,
            }
        }
    }

    pub fn affine_inversed(&self) -> GDTransform2D {
        unsafe {
            let inversed = godot_transform2d_affine_inverse(&self._transform);
            GDTransform2D {
                _transform: inversed,
            }
        }
    }

    pub fn get_rotation(&self) -> f32 {
        unsafe { godot_transform2d_get_rotation(&self._transform) }
    }

    pub fn get_origin(&self) -> GDVector2 {
        unsafe {
            let vec = godot_transform2d_get_origin(&self._transform);
            GDVector2 { _vector: vec }
        }
    }

    pub fn get_scale(&self) -> GDVector2 {
        unsafe {
            let vec = godot_transform2d_get_scale(&self._transform);
            GDVector2 { _vector: vec }
        }
    }

    pub fn orthonormalized(&self) -> GDTransform2D {
        unsafe {
            let trans = godot_transform2d_orthonormalized(&self._transform);
            GDTransform2D { _transform: trans }
        }
    }

    pub fn rotated(&self, phi: f32) -> GDTransform2D {
        unsafe {
            let trans = godot_transform2d_rotated(&self._transform, phi);
            GDTransform2D { _transform: trans }
        }
    }

    pub fn scaled(&self, scale: &GDVector2) -> GDTransform2D {
        unsafe {
            let scaled = godot_transform2d_scaled(&self._transform, &scale._vector);
            GDTransform2D { _transform: scaled }
        }
    }

    pub fn translated(&self, offset: &GDVector2) -> GDTransform2D {
        unsafe {
            let translated = godot_transform2d_translated(&self._transform, &offset._vector);
            GDTransform2D {
                _transform: translated,
            }
        }
    }

    pub fn xform_vector2(&self, vector: &GDVector2) -> GDVector2 {
        unsafe {
            let vec = godot_transform2d_xform_vector2(&self._transform, &vector._vector);
            GDVector2 { _vector: vec }
        }
    }

    pub fn xform_inv_vector2(&self, vector: &GDVector2) -> GDVector2 {
        unsafe {
            let vec = godot_transform2d_xform_inv_vector2(&self._transform, &vector._vector);
            GDVector2 { _vector: vec }
        }
    }

    pub fn basis_xform_vector2(&self, vector: &GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_transform2d_basis_xform_vector2(&self._transform, &vector._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn basis_xform_inv_vector2(&self, vector: &GDVector2) -> GDVector2 {
        unsafe {
            let vector =
                godot_transform2d_basis_xform_inv_vector2(&self._transform, &vector._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn interpolated(&self, transform: &GDTransform2D, phi: f32) -> GDTransform2D {
        unsafe {
            let interpolated =
                godot_transform2d_interpolate_with(&self._transform, &transform._transform, phi);
            GDTransform2D {
                _transform: interpolated,
            }
        }
    }

    pub fn xform_rect(&self, rect: &GDRect2) -> GDRect2 {
        unsafe {
            let rect = godot_transform2d_xform_rect2(&self._transform, &rect._rect);
            GDRect2 { _rect: rect }
        }
    }

    pub fn xform_inv_rect(&self, rect: &GDRect2) -> GDRect2 {
        unsafe {
            let rect = godot_transform2d_xform_inv_rect2(&self._transform, &rect._rect);
            GDRect2 { _rect: rect }
        }
    }
}

impl PartialEq for GDTransform2D {
    fn eq(&self, other: &GDTransform2D) -> bool {
        unsafe {
            let other_gd_dict = other._transform;
            let this_gd_dict = self._transform;
            godot_transform2d_operator_equal(&this_gd_dict, &other_gd_dict)
        }
    }

    fn ne(&self, other: &GDTransform2D) -> bool {
        !(self != other)
    }
}

impl Eq for GDTransform2D {}

impl Mul<GDTransform2D> for GDTransform2D {
    type Output = GDTransform2D;

    fn mul(self, transform: GDTransform2D) -> Self {
        unsafe {
            let transform =
                godot_transform2d_operator_multiply(&self._transform, &transform._transform);
            GDTransform2D {
                _transform: transform,
            }
        }
    }
}
