use gdnative_sys::*;
use godot::*;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign, Mul, MulAssign, Div, DivAssign,
               Neg};
use std::cmp::{PartialEq, Eq, Ordering};

fn new_quat() -> godot_quat {
    godot_quat { _dont_touch_that: [0; 16usize] }
}

impl GDQuat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> GDQuat {
        unsafe {
            let mut new_quat = new_quat();
            godot_quat_new(&mut new_quat, x, y, z, w);
            GDQuat { _quat: new_quat }
        }
    }

    pub fn new_with_axis_angle(axis: GDVector3, angle: f32) -> GDQuat {
        unsafe {
            let mut new_quat = new_quat();
            godot_quat_new_with_axis_angle(&mut new_quat, &axis._vector, angle);
            GDQuat { _quat: new_quat }
        }
    }

    pub fn get_x(&self) -> f32 {
        unsafe { godot_quat_get_x(&self._quat) }
    }

    pub fn set_x(&mut self, new_x: f32) {
        unsafe {
            godot_quat_set_x(&mut self._quat, new_x);
        }
    }

    pub fn get_y(&self) -> f32 {
        unsafe { godot_quat_get_y(&self._quat) }
    }

    pub fn set_y(&mut self, new_y: f32) {
        unsafe {
            godot_quat_set_y(&mut self._quat, new_y);
        }
    }

    pub fn get_z(&self) -> f32 {
        unsafe { godot_quat_get_z(&self._quat) }
    }

    pub fn set_z(&mut self, new_z: f32) {
        unsafe {
            godot_quat_set_z(&mut self._quat, new_z);
        }
    }

    pub fn get_w(&self) -> f32 {
        unsafe { godot_quat_get_w(&self._quat) }
    }

    pub fn set_w(&mut self, new_w: f32) {
        unsafe {
            godot_quat_set_w(&mut self._quat, new_w);
        }
    }

    pub fn length(&self) -> f32 {
        unsafe { godot_quat_length(&self._quat) }
    }

    pub fn length_squared(&self) -> f32 {
        unsafe { godot_quat_length_squared(&self._quat) }
    }

    pub fn normalized(&self) -> Self {
        unsafe {
            let normalized = godot_quat_normalized(&self._quat);
            GDQuat { _quat: normalized }
        }
    }

    pub fn is_normalized(&self) -> bool {
        unsafe { godot_quat_is_normalized(&self._quat) }
    }

    pub fn inversed(&self) -> Self {
        unsafe {
            let inversed = godot_quat_inverse(&self._quat);
            GDQuat { _quat: inversed }
        }
    }

    pub fn dot_product(&self, other: &GDQuat) -> f32 {
        unsafe { godot_quat_dot(&self._quat, &other._quat) }
    }

    /* Don't-know-what's-that section start */
    pub fn xform(&self, vector: &GDVector3) -> GDVector3 {
        unsafe {
            let xform = godot_quat_xform(&self._quat, &vector._vector);
            GDVector3 { _vector: xform }
        }
    }

    pub fn slerp(&self, other: &GDQuat, alpha: f32) -> GDQuat {
        unsafe {
            let slerp = godot_quat_slerp(&self._quat, &other._quat, alpha);
            GDQuat { _quat: slerp }
        }
    }

    pub fn slerpni(&self, other: &GDQuat, alpha: f32) -> GDQuat {
        unsafe {
            let slerpni = godot_quat_slerpni(&self._quat, &other._quat, alpha);
            GDQuat { _quat: slerpni }
        }
    }

    pub fn cubic_slerp(&self, b: &GDQuat, pre_a: &GDQuat, post_b: &GDQuat, alpha: f32) -> GDQuat {
        unsafe {
            let cubic =
                godot_quat_cubic_slerp(&self._quat, &b._quat, &pre_a._quat, &post_b._quat, alpha);
            GDQuat { _quat: cubic }
        }
    }
}

impl Add for GDQuat {
    type Output = GDQuat;

    fn add(self, other: GDQuat) -> GDQuat {
        unsafe {
            let quat = godot_quat_operator_add(&self._quat, &other._quat);
            GDQuat { _quat: quat }
        }
    }
}

impl AddAssign for GDQuat {
    fn add_assign(&mut self, other: GDQuat) {
        unsafe {
            let quat = godot_quat_operator_add(&self._quat, &other._quat);
            *self = GDQuat { _quat: quat }
        }
    }
}

impl Sub for GDQuat {
    type Output = GDQuat;

    fn sub(self, other: GDQuat) -> GDQuat {
        unsafe {
            let quat = godot_quat_operator_substract(&self._quat, &other._quat);
            GDQuat { _quat: quat }
        }
    }
}

impl SubAssign for GDQuat {
    fn sub_assign(&mut self, other: GDQuat) {
        unsafe {
            let quat = godot_quat_operator_substract(&self._quat, &other._quat);
            *self = GDQuat { _quat: quat }
        }
    }
}

impl Mul<f32> for GDQuat {
    type Output = GDQuat;

    fn mul(self, scalar: f32) -> GDQuat {
        unsafe {
            let quat = godot_quat_operator_multiply(&self._quat, scalar);
            GDQuat { _quat: quat }
        }
    }
}

impl MulAssign<f32> for GDQuat {
    fn mul_assign(&mut self, scalar: f32) {
        unsafe {
            let quat = godot_quat_operator_multiply(&self._quat, scalar);
            *self = GDQuat { _quat: quat }
        }
    }
}

impl Div<f32> for GDQuat {
    type Output = GDQuat;

    fn div(self, scalar: f32) -> GDQuat {
        unsafe {
            let quat = godot_quat_operator_divide(&self._quat, scalar);
            GDQuat { _quat: quat }
        }
    }
}

impl DivAssign<f32> for GDQuat {
    fn div_assign(&mut self, f: f32) {
        unsafe {
            let q = godot_quat_operator_divide(&self._quat, f);
            *self = GDQuat { _quat: q }
        }
    }
}

impl PartialEq for GDQuat {
    fn eq(&self, other: &GDQuat) -> bool {
        unsafe { godot_quat_operator_equal(&self._quat, &other._quat) }
    }

    fn ne(&self, other: &GDQuat) -> bool {
        !(self != other)
    }
}

impl Eq for GDQuat {}

impl Neg for GDQuat {
    type Output = GDQuat;

    fn neg(self) -> GDQuat {
        unsafe {
            let quat = godot_quat_operator_neg(&self._quat);
            GDQuat { _quat: quat }
        }
    }
}