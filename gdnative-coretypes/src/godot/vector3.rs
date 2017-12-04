use gdnative_sys::*;
use godot::*;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::cmp::{PartialEq, Eq, Ordering};

#[repr(C)]
pub enum GDVector3Axis {
    AxisX,
    AxisY,
    AxisZ
}

impl GDVector3Axis {
    fn value(&self) -> godot_vector3_axis {
        match *self {
            GDVector3Axis::AxisX => godot_vector3_axis::GODOT_VECTOR3_AXIS_X,
            GDVector3Axis::AxisY => godot_vector3_axis::GODOT_VECTOR3_AXIS_Y,
            GDVector3Axis::AxisZ => godot_vector3_axis::GODOT_VECTOR3_AXIS_Z
        }
    }
}

impl From<godot_vector3_axis> for GDVector3Axis {
    fn from(axis: godot_vector3_axis) -> GDVector3Axis {
        match axis {
            godot_vector3_axis::GODOT_VECTOR3_AXIS_X => GDVector3Axis::AxisX,
            godot_vector3_axis::GODOT_VECTOR3_AXIS_Y => GDVector3Axis::AxisY,
            godot_vector3_axis::GODOT_VECTOR3_AXIS_Z => GDVector3Axis::AxisZ
        }
    }
}

fn new_vector3() -> godot_vector3 {
    godot_vector3 { _dont_touch_that: [0; 12usize] }
}

impl GDVector3 {
    pub fn new() -> GDVector3 {
        unsafe {
            let mut vector = new_vector3();
            godot_vector3_new(&mut vector, 0.0, 0.0, 0.0);
            GDVector3 { _vector: vector }
        }
    }

    pub fn min_axis(&self) -> i32 {
        unsafe {
            godot_vector3_min_axis(&self._vector)
        }
    }

    pub fn max_axis(&self) -> i32 {
        unsafe {
            godot_vector3_max_axis(&self._vector)
        }
    }

    pub fn length(&self) -> f32 {
        unsafe {
            godot_vector3_length(&self._vector)
        }
    }

    pub fn length_squared(&self) -> f32 {
        unsafe {
            godot_vector3_length_squared(&self._vector)
        }
    }

    pub fn is_normalized(&self) -> bool {
        unsafe {
            godot_vector3_is_normalized(&self._vector)
        }
    }

    pub fn normalized(&self) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_normalized(&self._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn inversed(&self) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_inverse(&self._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn snapped(&self, by: &GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_snapped(&self._vector, &by._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn rotated(&self, axis: &GDVector3, phi: f32) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_rotated(&self._vector, &axis._vector, phi);
            GDVector3 { _vector: vector }
        }
    }

    pub fn lerp(&self, to: &GDVector3, alpha: f32) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_linear_interpolate(&self._vector, &to._vector, alpha);
            GDVector3 { _vector: vector }
        }
    }

    pub fn cubic(&self, b: &GDVector3, pre_a: &GDVector3, pre_b: &GDVector3, alpha: f32) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_cubic_interpolate(&self._vector, &b._vector, &pre_a._vector, &pre_b._vector, alpha);
            GDVector3 {_vector: vector }
        }
    }

    pub fn dot_product(&self, with: &GDVector3) -> f32 {
        unsafe {
            godot_vector3_dot(&self._vector, &with._vector)
        }
    }

    pub fn cross_product(&self, with: &GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_cross(&self._vector, &with._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn outer(&self, vector: &GDVector3) -> GDBasis {
        unsafe {
            let basis = godot_vector3_outer(&self._vector, &vector._vector);
            GDBasis { _basis: basis }
        }
    }

    pub fn to_diagonal_matrix(&self) -> GDBasis {
        unsafe {
            let basis = godot_vector3_to_diagonal_matrix(&self._vector);
            GDBasis { _basis: basis }
        }
    }

    pub fn abs(&self) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_abs(&self._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn floor(&self) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_floor(&self._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn ceil(&self) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_ceil(&self._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn distance_to(&self, to: &GDVector3) -> f32 {
        unsafe {
            godot_vector3_distance_to(&self._vector, &to._vector)
        }
    }

    pub fn distance_to_squared(&self, to: &GDVector3) -> f32 {
        unsafe {
            godot_vector3_distance_squared_to(&self._vector, &to._vector)
        }
    }

    pub fn angle_to(&self, to: &GDVector3) -> f32 {
        unsafe {
            godot_vector3_angle_to(&self._vector, &to._vector)
        }
    }

    pub fn slide(&self, to: &GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_slide(&self._vector, &to._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn bounce(&self, to: &GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_bounce(&self._vector, &to._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn reflect(&self, to: &GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_reflect(&self._vector, &to._vector);
            GDVector3 { _vector: vector }
        }
    }

    pub fn set_x(&mut self, x: f32) {
        unsafe {
            godot_vector3_set_axis(&mut self._vector, GDVector3Axis::AxisX.value(), x);
        }
    }

    pub fn set_y(&mut self, y: f32) {
        unsafe {
            godot_vector3_set_axis(&mut self._vector, GDVector3Axis::AxisY.value(), y);
        }
    }

    pub fn set_z(&mut self, z: f32) {
        unsafe {
            godot_vector3_set_axis(&mut self._vector, GDVector3Axis::AxisZ.value(), z);
        }
    }

    pub fn set(&mut self, xyz: (f32, f32, f32)) {
        unsafe {
            self.set_x(xyz.0);
            self.set_y(xyz.1);
            self.set_z(xyz.2);
        }
    }

    pub fn get_x(&self) -> f32 {
        unsafe {
            godot_vector3_get_axis(&self._vector, GDVector3Axis::AxisX.value())
        }
    }

    pub fn get_y(&self) -> f32 {
        unsafe {
            godot_vector3_get_axis(&self._vector, GDVector3Axis::AxisY.value())
        }
    }

    pub fn get_z(&self) -> f32 {
        unsafe {
            godot_vector3_get_axis(&self._vector, GDVector3Axis::AxisZ.value())
        }
    }

    pub fn tupled(&self) -> (f32, f32, f32) {
        unsafe {
            (self.get_x(), self.get_y(), self.get_z())
        }
    }


}


impl Add for GDVector3 {
    type Output = GDVector3;

    fn add(self, other: GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_add(&self._vector, &other._vector);
            GDVector3 { _vector: vector }
        }
    }
}

impl AddAssign for GDVector3 {
    fn add_assign(&mut self, other: GDVector3) {
        unsafe {
            let vector = godot_vector3_operator_add(&self._vector, &other._vector);
            *self = GDVector3 { _vector: vector }
        }
    }
}

impl Sub for GDVector3 {
    type Output = GDVector3;

    fn sub(self, other: GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_substract(&self._vector, &other._vector);
            GDVector3 { _vector: vector }
        }
    }
}

impl SubAssign for GDVector3 {
    fn sub_assign(&mut self, other: GDVector3) {
        unsafe {
            let vector = godot_vector3_operator_substract(&self._vector, &other._vector);
            *self = GDVector3 { _vector: vector }
        }
    }
}

impl Mul<f32> for GDVector3 {
    type Output = GDVector3;

    fn mul(self, scalar: f32) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_multiply_scalar(&self._vector, scalar);
            GDVector3 { _vector: vector }
        }
    }
}

impl Mul<GDVector3> for GDVector3 {
    type Output = GDVector3;

    fn mul(self, vector: GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_multiply_vector(&self._vector, &vector._vector);
            GDVector3 { _vector: vector }
        }
    }
}

impl MulAssign<f32> for GDVector3 {
    fn mul_assign(&mut self, scalar: f32) {
        unsafe {
            let vector = godot_vector3_operator_multiply_scalar(&self._vector, scalar);
            *self = GDVector3 { _vector: vector }
        }
    }
}

impl MulAssign<GDVector3> for GDVector3 {
    fn mul_assign(&mut self, vector: GDVector3) {
        unsafe {
            let vector = godot_vector3_operator_multiply_vector(&self._vector, &vector._vector);
            *self = GDVector3 { _vector: vector }
        }
    }
}

impl Div<f32> for GDVector3 {
    type Output = GDVector3;

    fn div(self, scalar: f32) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_divide_scalar(&self._vector, scalar);
            GDVector3 { _vector: vector }
        }
    }
}

impl Div<GDVector3> for GDVector3 {
    type Output = GDVector3;

    fn div(self, vector: GDVector3) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_divide_vector(&self._vector, &vector._vector);
            GDVector3 { _vector: vector }
        }
    }
}

impl DivAssign<f32> for GDVector3 {
    fn div_assign(&mut self, scalar: f32) {
        unsafe {
            let vector = godot_vector3_operator_divide_scalar(&self._vector, scalar);
            *self = GDVector3 { _vector: vector }
        }
    }
}

impl DivAssign<GDVector3> for GDVector3 {
    fn div_assign(&mut self, vector: GDVector3) {
        unsafe {
            let vector = godot_vector3_operator_divide_vector(&self._vector, &vector._vector);
            *self = GDVector3 { _vector: vector }
        }
    }
}

impl PartialEq for GDVector3 {
    fn eq(&self, other: &GDVector3) -> bool {
        unsafe {
            godot_vector3_operator_equal(&self._vector, &other._vector)
        }
    }

    fn ne(&self, other: &GDVector3) -> bool {
        !(self != other)
    }
}

impl Eq for GDVector3 {}

impl PartialOrd for GDVector3 {
    fn partial_cmp(&self, other: &GDVector3) -> Option<Ordering> {
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

    fn lt(&self, other: &GDVector3) -> bool {
        unsafe {
            godot_vector3_operator_less(&self._vector, &other._vector)
        }
    }

    fn le(&self, other: &GDVector3) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &GDVector3) -> bool {
        !(self <= other)
    }

    fn ge(&self, other: &GDVector3) -> bool {
        !(self < other)
    }
}

impl Ord for GDVector3 {
    fn cmp(&self, other: &GDVector3) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Neg for GDVector3 {
    type Output = GDVector3;

    fn neg(self) -> GDVector3 {
        unsafe {
            let vector = godot_vector3_operator_neg(&self._vector);
            GDVector3 { _vector: vector }
        }
    }
}

impl From<(f32, f32, f32)> for GDVector3 {
    fn from(xyz: (f32, f32, f32)) -> GDVector3 {
        unsafe {
            let mut vector = new_vector3();
            godot_vector3_new(&mut vector, xyz.0, xyz.1, xyz.2);
            GDVector3 { _vector: vector }
        }
    }
}

impl Into<GDString> for GDVector3 {
    fn into(self) -> GDString {
        unsafe {
            let gd_string = godot_vector3_as_string(&self._vector);
            GDString { _string: gd_string }
        }
    }
}

impl ToString for GDVector3 {
    fn to_string(&self) -> String {
        unsafe {
            let gd_string = godot_vector3_as_string(&self._vector);
            GDString { _string: gd_string }.to_string()
        }
    }
}