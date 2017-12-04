use gdnative_sys::*;
use godot::*;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use std::cmp::{Eq, Ordering, PartialEq};

fn new_transform() -> godot_transform {
    godot_transform {
        _dont_touch_that: [0; 48usize],
    }
}

impl GDTransform {
    pub fn new_with_axis_origin(
        x: &GDVector3,
        y: &GDVector3,
        z: &GDVector3,
        origin: &GDVector3,
    ) -> GDTransform {
        unsafe {
            let mut new = new_transform();
            godot_transform_new_with_axis_origin(
                &mut new,
                &x._vector,
                &y._vector,
                &z._vector,
                &origin._vector,
            );
            GDTransform { _transform: new }
        }
    }

    pub fn new(basis: &GDBasis, origin: &GDVector3) -> GDTransform {
        unsafe {
            let mut new = new_transform();
            godot_transform_new(&mut new, &basis._basis, &origin._vector);
            GDTransform { _transform: new }
        }
    }

    pub fn new_identity() -> GDTransform {
        unsafe {
            let mut new = new_transform();
            godot_transform_new_identity(&mut new);
            GDTransform { _transform: new }
        }
    }

    pub fn get_basis(&self) -> GDBasis {
        unsafe {
            let basis = godot_transform_get_basis(&self._transform);
            GDBasis { _basis: basis }
        }
    }

    //TODO Seems like a error to me. Check in newest version of godot api
    pub fn set_basis(&mut self, new_basis: &mut GDBasis) {
        unsafe {
            godot_transform_set_basis(&mut self._transform, &mut new_basis._basis);
        }
    }

    pub fn get_origin(&self) -> GDVector3 {
        unsafe {
            let origin = godot_transform_get_origin(&self._transform);
            GDVector3 { _vector: origin }
        }
    }

    //TODO Seems like a error to me. Check in newest version of godot api
    pub fn set_origin(&mut self, new_origin: &mut GDVector3) {
        unsafe { godot_transform_set_origin(&mut self._transform, &mut new_origin._vector) }
    }

    pub fn inversed(&self) -> GDTransform {
        unsafe {
            let inv = godot_transform_inverse(&self._transform);
            GDTransform { _transform: inv }
        }
    }

    pub fn affine_inversed(&self) -> GDTransform {
        unsafe {
            let inv = godot_transform_affine_inverse(&self._transform);
            GDTransform { _transform: inv }
        }
    }

    pub fn orthonormalized(&self) -> GDTransform {
        unsafe {
            let orthonormalized = godot_transform_orthonormalized(&self._transform);
            GDTransform {
                _transform: orthonormalized,
            }
        }
    }

    pub fn rotated(&self, axis: &GDVector3, phi: f32) -> GDTransform {
        unsafe {
            let rotated = godot_transform_rotated(&self._transform, &axis._vector, phi);
            GDTransform {
                _transform: rotated,
            }
        }
    }

    pub fn scaled(&self, scale: &GDVector3) -> GDTransform {
        unsafe {
            let scaled = godot_transform_scaled(&self._transform, &scale._vector);
            GDTransform { _transform: scaled }
        }
    }

    pub fn translated(&self, offset: &GDVector3) -> GDTransform {
        unsafe {
            let translated = godot_transform_translated(&self._transform, &offset._vector);
            GDTransform {
                _transform: translated,
            }
        }
    }

    pub fn looking_at(&self, target: &GDVector3, up: &GDVector3) -> GDTransform {
        unsafe {
            let looking =
                godot_transform_looking_at(&self._transform, &target._vector, &up._vector);
            GDTransform {
                _transform: looking,
            }
        }
    }

    pub fn xform_plane(&self, plane: &GDPlane) -> GDPlane {
        unsafe {
            let plane = godot_transform_xform_plane(&self._transform, &plane._plane);
            GDPlane { _plane: plane }
        }
    }

    pub fn xform_inverted_plane(&self, plane: &GDPlane) -> GDPlane {
        unsafe {
            let plane = godot_transform_xform_inv_plane(&self._transform, &plane._plane);
            GDPlane { _plane: plane }
        }
    }

    pub fn xform_vector(&self, vector: &GDVector3) -> GDVector3 {
        unsafe {
            let vec = godot_transform_xform_vector3(&self._transform, &vector._vector);
            GDVector3 { _vector: vec }
        }
    }

    pub fn xform_inverted_vector(&self, vector: &GDVector3) -> GDVector3 {
        unsafe {
            let vec = godot_transform_xform_inv_vector3(&self._transform, &vector._vector);
            GDVector3 { _vector: vec }
        }
    }

    pub fn xform_rect(&self, rect: &GDRect3) -> GDRect3 {
        unsafe {
            let rec = godot_transform_xform_rect3(&self._transform, &rect._rect);
            GDRect3 { _rect: rec }
        }
    }

    pub fn xform_inverted_rect(&self, rect: &GDRect3) -> GDRect3 {
        unsafe {
            let rec = godot_transform_xform_inv_rect3(&self._transform, &rect._rect);
            GDRect3 { _rect: rec }
        }
    }
}

impl PartialEq for GDTransform {
    fn eq(&self, other: &GDTransform) -> bool {
        unsafe {
            let other_gd_dict = &other._transform;
            let this_gd_dict = &self._transform;
            godot_transform_operator_equal(this_gd_dict, other_gd_dict)
        }
    }

    fn ne(&self, other: &GDTransform) -> bool {
        !(self != other)
    }
}

impl Eq for GDTransform {}

impl Mul<GDTransform> for GDTransform {
    type Output = GDTransform;

    fn mul(self, transform: GDTransform) -> Self {
        unsafe {
            let transform =
                godot_transform_operator_multiply(&self._transform, &transform._transform);
            GDTransform {
                _transform: transform,
            }
        }
    }
}
