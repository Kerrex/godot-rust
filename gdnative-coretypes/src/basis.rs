use gdnative_sys::*;
use godot::*;
use vector3::GDVector3;
use std::iter::Map;
use std::slice::*;
use std::iter::*;
use string::GDString;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[repr(C)]
pub struct GDBasis {
    pub ( crate ) _basis: godot_basis,
}

fn new_basis() -> godot_basis {
    godot_basis { _dont_touch_that: [0; 36usize] }
}

pub enum GDBasisAxis {
    AxisX,
    AxisY,
    AxisZ
}

impl GDBasisAxis {
    fn value(&self) -> i32 {
        match *self {
            GDBasisAxis::AxisX => 0,
            GDBasisAxis::AxisY => 1,
            GDBasisAxis::AxisZ => 2
        }
    }
}

pub enum GDBasisRow {
    RowX,
    RowY,
    RowZ
}

impl GDBasisRow {
    fn value(&self) -> i32 {
        match *self {
            GDBasisRow::RowX => 0,
            GDBasisRow::RowY => 1,
            GDBasisRow::RowZ => 2
        }
    }
}

impl GDBasis {

    pub fn new() -> GDBasis {
        unsafe {
            let mut new = new_basis();
            godot_basis_new(&mut new);
            GDBasis { _basis: new }
        }
    }

    pub fn new_with_rows(x: &[GDVector3; 3], y: &[GDVector3; 3], z: &[GDVector3; 3]) -> GDBasis {
        unsafe {
            let mut new_basis = new_basis();
            let x_vectors: Vec<godot_vector3> = x.into_iter().map(|vector| vector._vector).collect();
            let y_vectors: Vec<godot_vector3> = y.into_iter().map(|vector| vector._vector).collect();
            let z_vectors: Vec<godot_vector3> = z.into_iter().map(|vector| vector._vector).collect();
            godot_basis_new_with_rows(&mut new_basis, x_vectors.as_ptr(), y_vectors.as_ptr(), z_vectors.as_ptr());
            GDBasis { _basis: new_basis }
        }
    }

    pub fn new_with_axis_and_angle(axis: &GDVector3, phi: f32) -> GDBasis {
        unsafe {
            let mut new_basis = new_basis();
            godot_basis_new_with_axis_and_angle(&mut new_basis, &axis._vector, phi);
            GDBasis { _basis: new_basis }
        }
    }

    pub fn new_with_euler(euler: &GDVector3) -> GDBasis {
        unsafe {
            let mut new_basis = new_basis();
            godot_basis_new_with_euler(&mut new_basis, &euler._vector);
            GDBasis { _basis: new_basis }
        }
    }

    //TODO: Need to create GDQuat first
    //pub fn new_with_euler_quat(quat: &GDQuat) -> GDBasis {


    pub fn inverse(&self) -> GDBasis {
        unsafe {
            let mut inversed = godot_basis_inverse(&self._basis);
            GDBasis { _basis: inversed }
        }
    }

    pub fn transposed(&self) -> GDBasis {
        unsafe {
            let mut transposed = godot_basis_transposed(&self._basis);
            GDBasis { _basis: transposed }
        }
    }

    pub fn orthonormalized(&self) -> GDBasis {
        unsafe {
            let mut orthonormalized = godot_basis_orthonormalized(&self._basis);
            GDBasis { _basis: orthonormalized }
        }
    }

    pub fn determinant(&self) -> f32 {
        unsafe {
            godot_basis_determinant(&self._basis)
        }
    }

    pub fn rotated(&self, rotation_vector: GDVector3, phi: f32) -> GDBasis {
        unsafe {
            let rotated = godot_basis_rotated(&self._basis, &rotation_vector._vector, phi);
            GDBasis { _basis: rotated }
        }
    }

    pub fn scaled(&self, scale: GDVector3) -> GDBasis {
        unsafe {
            let scaled = godot_basis_scaled(&self._basis, &scale._vector);
            GDBasis { _basis: scaled }
        }
    }

    pub fn set_scale(&mut self, scale: GDVector3) {
        unsafe {
            godot_basis_set_scale(&mut self._basis, &scale._vector)
        }
    }

    pub fn set_rotation_euler(&mut self, euler: GDVector3) {
        unsafe {
            godot_basis_set_rotation_euler(&mut self._basis, &euler._vector)
        }
    }

    pub fn set_rotation_axis_angle(&mut self, axis: GDVector3, angle: f32) {
        unsafe {
            godot_basis_set_rotation_axis_angle(&mut self._basis, &axis._vector, angle)
        }
    }

    pub fn get_scale(&self) -> GDVector3 {
        unsafe {
            let vector = godot_basis_get_scale(&self._basis);
            GDVector3 { _vector: vector }
        }
    }

    pub fn get_euler(&self) -> GDVector3 {
        unsafe {
            let euler = godot_basis_get_euler(&self._basis);
            GDVector3 { _vector: euler }
        }
    }

    /* Dont-know-what's-that section start*/
    pub fn tdotx(&self, with: GDVector3) -> f32 {
        unsafe {
            godot_basis_tdotx(&self._basis, &with._vector)
        }
    }

    pub fn tdoty(&self, with: GDVector3) -> f32 {
        unsafe {
            godot_basis_tdoty(&self._basis, &with._vector)
        }
    }

    pub fn tdotz(&self, with: GDVector3) -> f32 {
        unsafe {
            godot_basis_tdotz(&self._basis, &with._vector)
        }
    }

    pub fn xfrom(&self, vector: GDVector3) -> GDVector3 {
        unsafe {
            let gd_vector = godot_basis_xform(&self._basis, &vector._vector);
            GDVector3 { _vector: gd_vector }
        }
    }

    pub fn xform_inv(&self, vector: GDVector3) -> GDVector3 {
        unsafe {
            let gd_vector = godot_basis_xform_inv(&self._basis, &vector._vector);
            GDVector3 { _vector: gd_vector }
        }
    }

    /* Dont-know-what's-that section end */

    pub fn get_orthogonal_index(&self) -> i32 {
        unsafe {
            godot_basis_get_orthogonal_index(&self._basis)
        }
    }

    pub fn get_elements(&mut self) -> [GDVector3; 3] {
        unsafe {
            let mut elements = [godot_vector3 {_dont_touch_that: [0; 12usize]}; 3];
            godot_basis_get_elements(&mut self._basis, elements.as_mut_ptr());
            [GDVector3 { _vector: elements[0] }, GDVector3 { _vector: elements[1] }, GDVector3 { _vector: elements[2] }]
        }
    }

    pub fn get_axis(&self, axis: GDBasisAxis) -> GDVector3 {
        unsafe {
            let vector = godot_basis_get_axis(&self._basis, axis.value());
            GDVector3 { _vector: vector }
        }
    }

    pub fn set_axis(&mut self, vector: GDVector3, axis: GDBasisAxis) {
        unsafe {
            godot_basis_set_axis(&mut self._basis, axis.value(), &vector._vector)
        }
    }

    pub fn get_row(&self, row: GDBasisRow) -> GDVector3 {
        unsafe {
            let row = godot_basis_get_row(&self._basis, row.value());
            GDVector3 { _vector: row }
        }
    }

    pub fn set_row(&mut self, vector: GDVector3, row: GDBasisRow) {
        unsafe {
            godot_basis_set_row(&mut self._basis, row.value(), &vector._vector);
        }
    }

}

impl PartialEq for GDBasis {
    fn eq(&self, other: &GDBasis) -> bool {
        unsafe {
            godot_basis_operator_equal(&self._basis, &other._basis)
        }
    }

    fn ne(&self, other: &GDBasis) -> bool {
        !(self != other)
    }
}

impl Eq for GDBasis {}

impl Add for GDBasis {
    type Output = GDBasis;

    fn add(self, other: GDBasis) -> GDBasis {
        unsafe {
            let basis = godot_basis_operator_add(&self._basis, &other._basis);
            GDBasis { _basis: basis }
        }
    }
}

impl AddAssign for GDBasis {
    fn add_assign(&mut self, other: GDBasis) {
        unsafe {
            let basis = godot_basis_operator_add(&self._basis, &other._basis);
            *self = GDBasis { _basis: basis}
        }
    }
}

impl Sub for GDBasis {
    type Output = GDBasis;

    fn sub(self, other: GDBasis) -> GDBasis {
        unsafe {
            let basis = godot_basis_operator_substract(&self._basis, &other._basis);
            GDBasis { _basis: basis }
        }
    }
}

impl SubAssign for GDBasis {
    fn sub_assign(&mut self, other: GDBasis) {
        unsafe {
            let basis = godot_basis_operator_substract(&self._basis, &other._basis);
            *self = GDBasis { _basis: basis }
        }
    }
}

impl Mul<f32> for GDBasis {
    type Output = GDBasis;

    fn mul(self, scalar: f32) -> GDBasis {
        unsafe {
            let basis = godot_basis_operator_multiply_scalar(&self._basis, scalar);
            GDBasis { _basis: basis }
        }
    }
}

impl Mul<GDBasis> for GDBasis {
    type Output = GDBasis;

    fn mul(self, other: GDBasis) -> GDBasis {
        unsafe {
            let basis = godot_basis_operator_multiply_vector(&self._basis, &other._basis);
            GDBasis { _basis: basis }
        }
    }
}

impl MulAssign<f32> for GDBasis {
    fn mul_assign(&mut self, scalar: f32) {
        unsafe {
            let basis = godot_basis_operator_multiply_scalar(&self._basis, scalar);
            *self = GDBasis { _basis: basis }
        }
    }
}

impl MulAssign<GDBasis> for GDBasis {
    fn mul_assign(&mut self, other: GDBasis) {
        unsafe {
            let basis = godot_basis_operator_multiply_vector(&self._basis, &other._basis);
            *self = GDBasis { _basis: basis }
        }
    }
}

impl ToString for GDBasis {
    fn to_string(&self) -> String {
        let gd_string: GDString = self.into();
        gd_string.to_string()
    }
}

