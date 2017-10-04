use gdnative_sys::*;
use std::ops::{Index, IndexMut};
use godot::*;
use variant::GDVariant;
use pool_arrays::*;
use std::mem;

pub struct GDArray {
    _array: godot_array,
}

impl GDArray {
    pub fn new() -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new(&mut array); }
        GDArray { _array: array }
    }

    pub fn from_pool_byte_array(pool_array: &GDPoolByteArray) -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new_pool_byte_array(&mut array, &pool_array._array) }
        GDArray { _array: array }
    }

    pub fn from_pool_int_array(pool_array: &GDPoolIntArray) -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new_pool_int_array(&mut array, &pool_array._array) }
        GDArray { _array: array }
    }

    pub fn from_pool_real_array(pool_array: &GDPoolRealArray) -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new_pool_real_array(&mut array, &pool_array._array) }
        GDArray { _array: array }
    }

    pub fn from_pool_string_array(pool_array: &GDPoolStringArray) -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new_pool_string_array(&mut array, &pool_array._array) }
        GDArray { _array: array }
    }

    pub fn from_pool_vector2_array(pool_array: &GDPoolVector2Array) -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new_pool_vector2_array(&mut array, &pool_array._array) }
        GDArray { _array: array }
    }

    pub fn from_pool_vector3_array(pool_array: &GDPoolVector3Array) -> GDArray {
        let mut array = godot_array { _dont_touch_that: [0; 8usize] };
        unsafe { godot_array_new_pool_vector3_array(&mut array, &pool_array._array) }
        GDArray { _array: array }
    }

    pub fn append(&mut self, variant: &GDVariant) {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(variant);
            godot_array_append(&mut self._array, gd_variant);
        }
    }

    pub fn clear(&mut self) {
        unsafe { godot_array_clear(&mut self._array) }
    }

    pub fn count(&mut self, variant: &GDVariant) -> i32 {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(variant);
            godot_array_count(&mut self._array, gd_variant)
        }
    }

    pub fn empty(&mut self) -> bool {
        unsafe { godot_array_empty(&mut self._array) }
    }

    pub fn erase(&mut self, variant: &GDVariant) {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(variant);
            godot_array_erase(&mut self._array, gd_variant)
        }
    }

    pub fn front(&mut self) -> GDVariant {
        unsafe {
            let gd_variant = godot_array_front(&mut self._array);
            let variant = (mem::transmute::<&godot_variant, &GDVariant>(&gd_variant));
            *variant
        }
    }

    pub fn back(&mut self) -> GDVariant {
        unsafe {
            let gd_variant = godot_array_back(&mut self._array);
            let variant = (mem::transmute::<&godot_variant, &GDVariant>(&gd_variant));
            *variant
        }
    }

    pub fn find(&mut self, what: &GDVariant, from: i32) -> i32 {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&what);
            godot_array_find(&mut self._array, gd_variant, from)
        }
    }

    pub fn find_last(&mut self, what: &GDVariant) -> i32 {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&what);
            godot_array_find_last(&mut self._array, gd_variant)
        }
    }

    pub fn has(&mut self, what: &GDVariant) -> bool {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&what);
            godot_array_has(&mut self._array, gd_variant)
        }
    }

    pub fn hash(&mut self) -> i32 {
        unsafe {
            godot_array_hash(&mut self._array)
        }
    }

    pub fn insert(&mut self, position: i32, value: &GDVariant) {
        unsafe {
            let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&value);
            godot_array_insert(&mut self._array, position, gd_variant)
        }
    }

    pub fn invert(&mut self) {
        unsafe {
            godot_array_invert(&mut self._array)
        }
    }

    pub fn pop_back(&mut self) -> GDVariant {
        unsafe {
            let gd_variant = godot_array_pop_back(&mut self._array);
            let variant = mem::transmute::<&godot_variant, &GDVariant>(&gd_variant);
            *variant
        }
    }

    pub fn pop_front(&mut self) -> GDVariant {
        unsafe {
            let gd_variant = godot_array_pop_front(&mut self._array);
            let variant = mem::transmute::<&godot_variant, &GDVariant>(&gd_variant);
            *variant
        }
    }

    pub fn push_back(&mut self, variant: &GDVariant) {
        unsafe {
            let gd_variant = GDVariant::into_godot_variant(variant);
            godot_array_push_back(&mut self._array, gd_variant)
        }
    }

    pub fn push_front(&mut self, variant: &GDVariant) {
        unsafe {
            let gd_variant = GDVariant::into_godot_variant(variant);
            godot_array_push_front(&mut self._array, gd_variant)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_array_remove(&mut self._array, index)
        }
    }

    pub fn size(&mut self) -> i32 {
        unsafe {
            godot_array_size(&mut self._array)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_array_resize(&mut self._array, new_size)
        }
    }

    pub fn rfind(&mut self, what: &GDVariant, from: i32) -> i32 {
        unsafe {
            let gd_variant = GDVariant::into_godot_variant(what);
            godot_array_rfind(&mut self._array, gd_variant, from)
        }
    }

    fn sort(&mut self) {
        unsafe {
            godot_array_sort(&mut self._array)
        }
    }

    fn sort_custom(&mut self, object: &mut GDObject, function: &GDString) {
        unsafe {
            let gd_object = mem::transmute::<&mut GDObject, &mut godot_object>(object);
            let gd_string = mem::transmute::<&GDString, &godot_string>(function);
            godot_array_sort_custom(&mut self._array, gd_object, gd_string)
        }
    }
}

impl Drop for GDArray {
    fn drop(&mut self) {
        unsafe {
            godot_array_destroy(&mut self._array)
        }
    }
}

impl Index<i32> for GDArray {
    type Output = GDVariant;

    fn index(&self, index: i32) -> &GDVariant {
        let mut array = self._array;
        unsafe {
            let mut v = godot_array_operator_index(&mut array, index);
            mem::transmute::<*mut godot_variant, &GDVariant>(v)
        }
    }
}

impl IndexMut<i32> for GDArray {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        let mut array = self._array;
        unsafe {
            let mut v = godot_array_operator_index(&mut array, index);
            mem::transmute::<*mut godot_variant, &mut GDVariant>(v)
        }
    }
}
