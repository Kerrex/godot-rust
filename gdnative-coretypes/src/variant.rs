use godot::*;
use gdnative_sys::*;
use std::mem;

#[derive(Copy, Clone)]
pub struct GDVariant {
    pub ( crate ) _variant: godot_variant
}

fn new_gd_variant() -> godot_variant {
    godot_variant { _dont_touch_that: [0; 24usize] }
}

impl GDVariant {
    pub fn into_godot_variant(variant: &GDVariant) -> &godot_variant {
        unsafe {
            mem::transmute::<&GDVariant, &godot_variant>(&variant)
        }
    }

    pub fn into_variant(&gd_variant: &godot_variant) -> &GDVariant {
        unsafe {
            mem::transmute::<&godot_variant, &GDVariant>(&gd_variant)
        }
    }


    pub fn new() -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_nil(&mut new_variant);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_variant(variant: &GDVariant) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            let gd_variant = variant._variant;
            godot_variant_new_copy(&mut new_variant, &gd_variant);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_bool(variable: bool) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_bool(&mut new_variant, variable);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_i64(variable: i64) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_int(&mut new_variant, variable);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_u64(variable: u64) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_uint(&mut new_variant, variable);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_i32(variable: i32) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_int(&mut new_variant, variable as i64);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_u32(variable: u32) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_uint(&mut new_variant, variable as u64);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_char(variable: char) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_int(&mut new_variant, variable as i64);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_f32(variable: f32) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_real(&mut new_variant, variable as f64);
            GDVariant { _variant: new_variant }
        }
    }

    pub fn from_f64(variable: f64) -> GDVariant {
        unsafe {
            let mut new_variant = new_gd_variant();
            godot_variant_new_real(&mut new_variant, variable);
            GDVariant { _variant: new_variant }
        }
    }

    // TODO FROM OTHER TYPES
}