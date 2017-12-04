use gdnative_sys::*;
use std::cmp::Ordering;
use godot::*;

fn new_color() -> godot_color {
    godot_color {
        _dont_touch_that: [0; 16usize],
    }
}

impl GDColor {
    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> GDColor {
        unsafe {
            let mut new_color = new_color();
            godot_color_new_rgba(&mut new_color, r, g, b, a);
            GDColor { _color: new_color }
        }
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> GDColor {
        unsafe {
            let mut new_color = new_color();
            godot_color_new_rgb(&mut new_color, r, g, b);
            GDColor { _color: new_color }
        }
    }

    pub fn get_r(&self) -> f32 {
        unsafe { godot_color_get_r(&self._color) }
    }

    pub fn set_r(&mut self, new_r: f32) {
        unsafe { godot_color_set_r(&mut self._color, new_r) }
    }

    pub fn get_g(&self) -> f32 {
        unsafe { godot_color_get_g(&self._color) }
    }

    pub fn set_g(&mut self, new_g: f32) {
        unsafe { godot_color_set_g(&mut self._color, new_g) }
    }

    pub fn get_b(&self) -> f32 {
        unsafe { godot_color_get_b(&self._color) }
    }

    pub fn set_b(&mut self, new_b: f32) {
        unsafe {
            godot_color_set_b(&mut self._color, new_b);
        }
    }

    pub fn get_h(&self) -> f32 {
        unsafe { godot_color_get_h(&self._color) }
    }

    pub fn get_s(&self) -> f32 {
        unsafe { godot_color_get_s(&self._color) }
    }

    pub fn get_v(&self) -> f32 {
        unsafe { godot_color_get_v(&self._color) }
    }

    pub fn into_32(&self) -> i32 {
        unsafe { godot_color_to_32(&self._color) }
    }

    pub fn into_argb32(&self) -> i32 {
        unsafe { godot_color_to_ARGB32(&self._color) }
    }

    pub fn gray(&self) -> f32 {
        unsafe {
            godot_color_gray(&self._color)
        }
    }

    pub fn inverted(&self) -> GDColor {
        unsafe {
            let gd_color = godot_color_inverted(&self._color);
            GDColor { _color: gd_color }
        }
    }

    pub fn contrasted(&self) -> GDColor {
        unsafe {
            let gd_color = godot_color_contrasted(&self._color);
            GDColor { _color: gd_color }
        }
    }

    pub fn lerp(&self, to: &GDColor, alpha: f32) -> GDColor {
        unsafe {
            let gd_color = godot_color_linear_interpolate(&self._color, &to._color, alpha);
            GDColor { _color: gd_color }
        }
    }

    pub fn blend(&self, over: &GDColor) -> GDColor {
        unsafe {
            let gd_color = godot_color_blend(&self._color, &over._color);
            GDColor { _color: gd_color }
        }
    }

    pub fn to_html(&self, with_alpha: bool) -> GDString {
        unsafe {
            let in_string = godot_color_to_html(&self._color, with_alpha);
            GDString { _string: in_string }
        }
    }
}

impl PartialEq for GDColor {
    fn eq(&self, other: &GDColor) -> bool {
        unsafe {
            let other_gd_color = other._color;
            let this_gd_color = self._color;
            godot_color_operator_equal(&this_gd_color, &other_gd_color)
        }
    }

    fn ne(&self, other: &GDColor) -> bool {
        !(self != other)
    }
}

impl Eq for GDColor {}

impl PartialOrd for GDColor {
    fn partial_cmp(&self, other: &GDColor) -> Option<Ordering> {
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

    fn lt(&self, other: &GDColor) -> bool {
        unsafe {
            let this_gd_color = self._color;
            let other_gd_color = other._color;
            godot_color_operator_less(&this_gd_color, &other_gd_color)
        }
    }

    fn le(&self, other: &GDColor) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &GDColor) -> bool {
        !(self <= other)
    }

    fn ge(&self, other: &GDColor) -> bool {
        !(self < other)
    }
}

impl Ord for GDColor {
    fn cmp(&self, other: &GDColor) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
