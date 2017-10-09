use gdnative_sys::*;
use godot::*;
use string::GDString;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::cmp::{PartialEq, Eq, Ordering};

#[repr(C)]
pub struct GDVector2 {
    _vector: godot_vector2
}

fn new_vector2() -> godot_vector2 {
    godot_vector2 { _dont_touch_that: [0; 8usize] }
}

impl GDVector2 {
    pub fn new() -> GDVector2 {
        unsafe {
            let mut new_vector = new_vector2();
            godot_vector2_new(&mut new_vector, 0.0, 0.0);
            GDVector2 { _vector: new_vector }
        }
    }

    pub fn normalized(&self) -> GDVector2 {
        unsafe {
            let normalized = godot_vector2_normalized(&self._vector);
            GDVector2 { _vector: normalized }
        }
    }

    pub fn length(&self) -> f32 {
        unsafe {
            godot_vector2_length(&self._vector)
        }
    }

    pub fn angle(&self) -> f32 {
        unsafe {
            godot_vector2_angle(&self._vector)
        }
    }

    pub fn length_squared(&self) -> f32 {
        unsafe {
            godot_vector2_length_squared(&self._vector)
        }
    }

    pub fn is_normalized(&self) -> bool {
        unsafe {
            godot_vector2_is_normalized(&self._vector)
        }
    }

    pub fn distance_to(&self, to: &GDVector2) -> f32 {
        unsafe {
            godot_vector2_distance_to(&self._vector, &to._vector)
        }
    }

    pub fn distance_to_squared(&self, to: &GDVector2) -> f32 {
        unsafe {
            godot_vector2_distance_squared_to(&self._vector, &to._vector)
        }
    }

    pub fn angle_to(&self, to: &GDVector2) -> f32 {
        unsafe {
            godot_vector2_angle_to(&self._vector, &to._vector)
        }
    }

    pub fn angle_to_point(&self, to: &GDVector2) -> f32 {
        unsafe {
            godot_vector2_angle_to_point(&self._vector, &to._vector)
        }
    }

    pub fn lerp(&self, to: &GDVector2, alpha: f32) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_linear_interpolate(&self._vector, &to._vector, alpha);
            GDVector2 { _vector: vector }
        }
    }

    pub fn cubic(&self, b: &GDVector2, pre_a: &GDVector2, pre_b: &GDVector2, alpha: f32) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_cubic_interpolate(&self._vector, &b._vector, &pre_a._vector, &pre_b._vector, alpha);
            GDVector2 { _vector: vector }
        }
    }

    pub fn rotated(&self, phi: f32) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_rotated(&self._vector, phi);
            GDVector2 { _vector: vector }
        }
    }

    pub fn tangent(&self) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_tangent(&self._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn floor(&self) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_floor(&self._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn snapped(&self, snap_by: &GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_snapped(&self._vector, &snap_by._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn aspect(&self) -> f32 {
        unsafe {
            godot_vector2_aspect(&self._vector)
        }
    }

    pub fn dot_product(&self, with: &GDVector2) -> f32 {
        unsafe {
            let dot = godot_vector2_dot(&self._vector, &with._vector);
            dot
        }
    }

    pub fn slide(&self, with: &GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_slide(&self._vector, &with._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn bounce(&self, with: &GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_bounce(&self._vector, &with._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn reflect(&self, with: &GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_reflect(&self._vector, &with._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn abs(&self) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_abs(&self._vector);
            GDVector2 { _vector: vector }
        }
    }

    pub fn clamped(&self, length: f32) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_clamped(&self._vector, length);
            GDVector2 { _vector: vector }
        }
    }

    pub fn set_x(&mut self, new_x: f32) {
        unsafe {
            godot_vector2_set_x(&mut self._vector, new_x);
        }
    }

    pub fn set_y(&mut self, new_y: f32) {
        unsafe {
            godot_vector2_set_y(&mut self._vector, new_y);
        }
    }

    pub fn set(&mut self, xy: (f32, f32)) {
        unsafe {
            godot_vector2_set_x(&mut self._vector, xy.0);
            godot_vector2_set_y(&mut self._vector, xy.1);
        }
    }

    pub fn get_x(&self) -> f32 {
        unsafe {
            godot_vector2_get_x(&self._vector)
        }
    }

    pub fn get_y(&self) -> f32 {
        unsafe {
            godot_vector2_get_y(&self._vector)
        }
    }

    pub fn get_xy(&self) -> (f32, f32) {
        unsafe {
            let x = godot_vector2_get_x(&self._vector);
            let y = godot_vector2_get_y(&self._vector);
            (x, y)
        }
    }
}

impl Add for GDVector2 {
    type Output = GDVector2;

    fn add(self, other: GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_add(&self._vector, &other._vector);
            GDVector2{ _vector: vector }
        }
    }
}

impl AddAssign for GDVector2 {
    fn add_assign(&mut self, other: GDVector2) {
        unsafe {
            let vector = godot_vector2_operator_add(&self._vector, &other._vector);
            *self = GDVector2 { _vector: vector }
        }
    }
}

impl Sub for GDVector2 {
    type Output = GDVector2;

    fn sub(self, other: GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_substract(&self._vector, &other._vector);
            GDVector2 { _vector: vector }
        }
    }
}

impl SubAssign for GDVector2 {
    fn sub_assign(&mut self, other: GDVector2) {
        unsafe {
            let vector = godot_vector2_operator_substract(&self._vector, &other._vector);
            *self = GDVector2 { _vector: vector }
        }
    }
}

impl Mul<f32> for GDVector2 {
    type Output = GDVector2;

    fn mul(self, scalar: f32) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_multiply_scalar(&self._vector, scalar);
            GDVector2 { _vector: vector }
        }
    }
}

impl Mul<GDVector2> for GDVector2 {
    type Output = GDVector2;

    fn mul(self, vector: GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_multiply_vector(&self._vector, &vector._vector);
            GDVector2 { _vector: vector }
        }
    }
}

impl MulAssign<f32> for GDVector2 {
    fn mul_assign(&mut self, scalar: f32) {
        unsafe {
            let vector = godot_vector2_operator_multiply_scalar(&self._vector, scalar);
            *self = GDVector2 { _vector: vector }
        }
    }
}

impl MulAssign<GDVector2> for GDVector2 {
    fn mul_assign(&mut self, vector: GDVector2) {
        unsafe {
            let vector = godot_vector2_operator_multiply_vector(&self._vector, &vector._vector);
            *self = GDVector2 {_vector: vector }
        }
    }
}

impl Div<f32> for GDVector2 {
    type Output = GDVector2;

    fn div(self, scalar: f32) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_divide_scalar(&self._vector, scalar);
            GDVector2 { _vector: vector }
        }
    }
}

impl Div<GDVector2> for GDVector2 {
    type Output = GDVector2;

    fn div(self, vector: GDVector2) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_divide_vector(&self._vector, &vector._vector);
            GDVector2 { _vector: vector }
        }
    }
}

impl DivAssign<f32> for GDVector2 {
    fn div_assign(&mut self, scalar: f32) {
        unsafe {
            let vector = godot_vector2_operator_divide_scalar(&self._vector, scalar);
            *self = GDVector2 { _vector: vector }
        }
    }
}

impl DivAssign<GDVector2> for GDVector2 {
    fn div_assign(&mut self, vector: GDVector2) {
        unsafe {
            let vector = godot_vector2_operator_divide_vector(&self._vector, &vector._vector);
            *self = GDVector2 { _vector: vector }
        }
    }
}

impl PartialEq for GDVector2 {
    fn eq(&self, other: &GDVector2) -> bool {
        unsafe {
            godot_vector2_operator_equal(&self._vector, &other._vector)
        }
    }

    fn ne(&self, other: &GDVector2) -> bool {
        !(self != other)
    }
}

impl Eq for GDVector2 {}

impl PartialOrd for GDVector2 {
    fn partial_cmp(&self, other: &GDVector2) -> Option<Ordering> {
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

    fn lt(&self, other: &GDVector2) -> bool {
        unsafe {
            godot_vector2_operator_less(&self._vector, &other._vector)
        }
    }

    fn le(&self, other: &GDVector2) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &GDVector2) -> bool {
        !(self <= other)
    }

    fn ge(&self, other: &GDVector2) -> bool {
        !(self < other)
    }
}

impl Ord for GDVector2 {
    fn cmp(&self, other: &GDVector2) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Neg for GDVector2 {
    type Output = GDVector2;

    fn neg(self) -> GDVector2 {
        unsafe {
            let vector = godot_vector2_operator_neg(&self._vector);
            GDVector2 { _vector: vector }
        }
    }
}

impl Into<GDString> for GDVector2 {
    fn into(self) -> GDString {
        unsafe {
            let gd_string = godot_vector2_as_string(&self._vector);
            GDString { _string: gd_string }
        }
    }
}

impl ToString for GDVector2 {
    fn to_string(&self) -> String {
        unsafe {
            let gd_string = godot_vector2_as_string(&self._vector);
            GDString { _string: gd_string }.to_string()
        }
    }
}