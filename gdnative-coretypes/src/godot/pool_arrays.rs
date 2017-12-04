use gdnative_sys::*;
use godot::*;

fn new_byte_array() -> godot_pool_byte_array {
    godot_pool_byte_array {_dont_touch_that: [0; 8usize] }
}

//byte array

impl GDPoolByteArray {
    pub fn new() -> GDPoolByteArray {
        unsafe {
            let mut new_array = new_byte_array();
            godot_pool_byte_array_new(&mut new_array);
            GDPoolByteArray { _array: new_array }
        }
    }

    pub fn new_copy(byte_array: &GDPoolByteArray) -> GDPoolByteArray {
        unsafe {
            let mut new_array = new_byte_array();
            godot_pool_byte_array_new_copy(&mut new_array, &byte_array._array);
            GDPoolByteArray { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: u8) -> GDError {
        unsafe {
            godot_pool_byte_array_insert(&mut self._array, index, data)
        }
    }

    pub fn inverted(&self) -> GDPoolByteArray {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_byte_array_invert(&mut cloned);
            GDPoolByteArray { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: u8) {
        unsafe {
            godot_pool_byte_array_push_back(&mut self._array, data)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_byte_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_byte_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: u8) {
        unsafe {
            godot_pool_byte_array_set(&mut self._array, index, data)
        }
    }

    pub fn get(&self, index: i32) -> u8 {
        unsafe {
            godot_pool_byte_array_get(&self._array, index)
        }
    }

    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_byte_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolByteArray {
    fn drop(&mut self) {
        unsafe { godot_pool_byte_array_destroy(&mut self._array) }
    }
}

impl Appendable<u8> for GDPoolByteArray {
    fn append(&mut self, byte: u8) {
        unsafe {
            godot_pool_byte_array_append(&mut self._array, byte);
        }
    }
}

impl Appendable<GDPoolByteArray> for GDPoolByteArray {
    fn append(&mut self, array: GDPoolByteArray) {
        unsafe {
            godot_pool_byte_array_append_array(&mut self._array, &array._array);
        }
    }
}

trait Appendable<T> {
    fn append(&mut self, _: T);
}

impl From<GDArray> for GDPoolByteArray {
    fn from(array: GDArray) -> GDPoolByteArray {
        unsafe {
            let mut new_array = new_byte_array();
            godot_pool_byte_array_new_with_array(&mut new_array, &array._array);
            GDPoolByteArray { _array: new_array }
        }

    }
}

//int array
fn new_int_array() -> godot_pool_int_array {
    godot_pool_int_array { _dont_touch_that: [0; 8usize] }
}

impl GDPoolIntArray {
    pub fn new() -> GDPoolIntArray {
        unsafe {
            let mut new_array = new_int_array();
            godot_pool_int_array_new(&mut new_array);
            GDPoolIntArray { _array: new_array }
        }
    }

    pub fn new_copy(int_array: &GDPoolIntArray) -> GDPoolIntArray {
        unsafe {
            let mut new_array = new_int_array();
            godot_pool_int_array_new_copy(&mut new_array, &int_array._array);
            GDPoolIntArray { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: i32) -> GDError {
        unsafe {
            godot_pool_int_array_insert(&mut self._array, index, data)
        }
    }

    pub fn inverted(&self) -> GDPoolIntArray {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_int_array_invert(&mut cloned);
            GDPoolIntArray { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: i32) {
        unsafe {
            godot_pool_int_array_push_back(&mut self._array, data)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_int_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_int_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: i32) {
        unsafe {
            godot_pool_int_array_set(&mut self._array, index, data)
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        unsafe {
            godot_pool_int_array_get(&self._array, index)
        }
    }

    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_int_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolIntArray {
    fn drop(&mut self) {
        unsafe { godot_pool_int_array_destroy(&mut self._array) }
    }
}

impl Appendable<i32> for GDPoolIntArray {
    fn append(&mut self, data: i32) {
        unsafe {
            godot_pool_int_array_append(&mut self._array, data);
        }
    }
}

impl Appendable<GDPoolIntArray> for GDPoolIntArray {
    fn append(&mut self, array: GDPoolIntArray) {
        unsafe {
            godot_pool_int_array_append_array(&mut self._array, &array._array);
        }
    }
}


impl From<GDArray> for GDPoolIntArray {
    fn from(array: GDArray) -> GDPoolIntArray {
        unsafe {
            let mut new_array = new_int_array();
            godot_pool_int_array_new_with_array(&mut new_array, &array._array);
            GDPoolIntArray { _array: new_array }
        }
    }
}

// real array

fn new_real_array() -> godot_pool_real_array {
    godot_pool_real_array { _dont_touch_that: [0; 8usize] }
}

impl GDPoolRealArray {
    pub fn new() -> GDPoolRealArray {
        unsafe {
            let mut new_array = new_real_array();
            godot_pool_real_array_new(&mut new_array);
            GDPoolRealArray { _array: new_array }
        }
    }

    pub fn new_copy(real_array: &GDPoolRealArray) -> GDPoolRealArray {
        unsafe {
            let mut new_array = new_real_array();
            godot_pool_real_array_new_copy(&mut new_array, &real_array._array);
            GDPoolRealArray { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: f32) -> GDError {
        unsafe {
            godot_pool_real_array_insert(&mut self._array, index, data)
        }
    }

    pub fn inverted(&self) -> GDPoolRealArray {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_real_array_invert(&mut cloned);
            GDPoolRealArray { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: f32) {
        unsafe {
            godot_pool_real_array_push_back(&mut self._array, data)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_real_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_real_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: f32) {
        unsafe {
            godot_pool_real_array_set(&mut self._array, index, data)
        }
    }

    pub fn get(&self, index: i32) -> f32 {
        unsafe {
            godot_pool_real_array_get(&self._array, index)
        }
    }

    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_real_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolRealArray {
    fn drop(&mut self) {
        unsafe { godot_pool_real_array_destroy(&mut self._array) }
    }
}

impl Appendable<f32> for GDPoolRealArray {
    fn append(&mut self, data: f32) {
        unsafe {
            godot_pool_real_array_append(&mut self._array, data);
        }
    }
}

impl Appendable<GDPoolRealArray> for GDPoolRealArray {
    fn append(&mut self, array: GDPoolRealArray) {
        unsafe {
            godot_pool_real_array_append_array(&mut self._array, &array._array);
        }
    }
}

impl From<GDArray> for GDPoolRealArray {
    fn from(array: GDArray) -> GDPoolRealArray {
        unsafe {
            let mut new_array = new_real_array();
            godot_pool_real_array_new_with_array(&mut new_array, &array._array);
            GDPoolRealArray { _array: new_array }
        }
    }
}

//string array 
fn new_string_array() -> godot_pool_string_array {
    godot_pool_string_array { _dont_touch_that: [0; 8usize] }
}

impl GDPoolStringArray {
    pub fn new() -> GDPoolStringArray {
        unsafe {
            let mut new_array = new_string_array();
            godot_pool_string_array_new(&mut new_array);
            GDPoolStringArray { _array: new_array }
        }
    }

    pub fn new_copy(string_array: &GDPoolStringArray) -> GDPoolStringArray {
        unsafe {
            let mut new_array = new_string_array();
            godot_pool_string_array_new_copy(&mut new_array, &string_array._array);
            GDPoolStringArray { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: &GDString) -> GDError {
        unsafe {
            godot_pool_string_array_insert(&mut self._array, index, &data._string)
        }
    }

    pub fn inverted(&self) -> GDPoolStringArray {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_string_array_invert(&mut cloned);
            GDPoolStringArray { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: &GDString) {
        unsafe {
            godot_pool_string_array_push_back(&mut self._array, &data._string)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_string_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_string_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: &GDString) {
        unsafe {
            godot_pool_string_array_set(&mut self._array, index, &data._string)
        }
    }

    pub fn get(&self, index: i32) -> GDString {
        unsafe {
            let string = godot_pool_string_array_get(&self._array, index);
            GDString { _string: string }
        }
    }
    
    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_string_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolStringArray {
    fn drop(&mut self) {
        unsafe { godot_pool_string_array_destroy(&mut self._array) }
    }
}

impl Appendable<GDString> for GDPoolStringArray {
    fn append(&mut self, data: GDString) {
        unsafe {
            godot_pool_string_array_append(&mut self._array, &data._string);
        }
    }
}

impl Appendable<GDPoolStringArray> for GDPoolStringArray {
    fn append(&mut self, array: GDPoolStringArray) {
        unsafe {
            godot_pool_string_array_append_array(&mut self._array, &array._array);
        }
    }
}

impl From<GDArray> for GDPoolStringArray {
    fn from(array: GDArray) -> GDPoolStringArray {
        unsafe {
            let mut new_array = new_string_array();
            godot_pool_string_array_new_with_array(&mut new_array, &array._array);
            GDPoolStringArray { _array: new_array }
        }
    }
}

//vector2 array 
fn new_vector2_array() -> godot_pool_vector2_array {
    godot_pool_vector2_array { _dont_touch_that: [0; 8usize] }
}

impl GDPoolVector2Array {
    pub fn new() -> GDPoolVector2Array {
        unsafe {
            let mut new_array = new_vector2_array();
            godot_pool_vector2_array_new(&mut new_array);
            GDPoolVector2Array { _array: new_array }
        }
    }

    pub fn new_copy(vector2_array: &GDPoolVector2Array) -> GDPoolVector2Array {
        unsafe {
            let mut new_array = new_vector2_array();
            godot_pool_vector2_array_new_copy(&mut new_array, &vector2_array._array);
            GDPoolVector2Array { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: &GDVector2) -> GDError {
        unsafe {
            godot_pool_vector2_array_insert(&mut self._array, index, &data._vector)
        }
    }

    pub fn inverted(&self) -> GDPoolVector2Array {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_vector2_array_invert(&mut cloned);
            GDPoolVector2Array { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: &GDVector2) {
        unsafe {
            godot_pool_vector2_array_push_back(&mut self._array, &data._vector)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_vector2_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_vector2_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: &GDVector2) {
        unsafe {
            godot_pool_vector2_array_set(&mut self._array, index, &data._vector)
        }
    }

    pub fn get(&self, index: i32) -> GDVector2 {
        unsafe {
            let vector = godot_pool_vector2_array_get(&self._array, index);
            GDVector2 { _vector: vector }
        }
    }
    
    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_vector2_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolVector2Array {
    fn drop(&mut self) {
        unsafe { godot_pool_vector2_array_destroy(&mut self._array) }
    }
}

impl Appendable<GDVector2> for GDPoolVector2Array {
    fn append(&mut self, data: GDVector2) {
        unsafe {
            godot_pool_vector2_array_append(&mut self._array, &data._vector);
        }
    }
}

impl Appendable<GDPoolVector2Array> for GDPoolVector2Array {
    fn append(&mut self, array: GDPoolVector2Array) {
        unsafe {
            godot_pool_vector2_array_append_array(&mut self._array, &array._array);
        }
    }
}

impl From<GDArray> for GDPoolVector2Array {
    fn from(array: GDArray) -> GDPoolVector2Array {
        unsafe {
            let mut new_array = new_vector2_array();
            godot_pool_vector2_array_new_with_array(&mut new_array, &array._array);
            GDPoolVector2Array { _array: new_array }
        }
    }
}

//vector3 array 
fn new_vector3_array() -> godot_pool_vector3_array {
    godot_pool_vector3_array { _dont_touch_that: [0; 8usize] }
}

impl GDPoolVector3Array {
    pub fn new() -> GDPoolVector3Array {
        unsafe {
            let mut new_array = new_vector3_array();
            godot_pool_vector3_array_new(&mut new_array);
            GDPoolVector3Array { _array: new_array }
        }
    }

    pub fn new_copy(vector3_array: &GDPoolVector3Array) -> GDPoolVector3Array {
        unsafe {
            let mut new_array = new_vector3_array();
            godot_pool_vector3_array_new_copy(&mut new_array, &vector3_array._array);
            GDPoolVector3Array { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: &GDVector3) -> GDError {
        unsafe {
            godot_pool_vector3_array_insert(&mut self._array, index, &data._vector)
        }
    }

    pub fn inverted(&self) -> GDPoolVector3Array {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_vector3_array_invert(&mut cloned);
            GDPoolVector3Array { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: &GDVector3) {
        unsafe {
            godot_pool_vector3_array_push_back(&mut self._array, &data._vector)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_vector3_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_vector3_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: &GDVector3) {
        unsafe {
            godot_pool_vector3_array_set(&mut self._array, index, &data._vector)
        }
    }

    pub fn get(&self, index: i32) -> GDVector3 {
        unsafe {
            let vector = godot_pool_vector3_array_get(&self._array, index);
            GDVector3 { _vector: vector }
        }
    }
    
    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_vector3_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolVector3Array {
    fn drop(&mut self) {
        unsafe { godot_pool_vector3_array_destroy(&mut self._array) }
    }
}

impl Appendable<GDVector3> for GDPoolVector3Array {
    fn append(&mut self, data: GDVector3) {
        unsafe {
            godot_pool_vector3_array_append(&mut self._array, &data._vector);
        }
    }
}

impl Appendable<GDPoolVector3Array> for GDPoolVector3Array {
    fn append(&mut self, array: GDPoolVector3Array) {
        unsafe {
            godot_pool_vector3_array_append_array(&mut self._array, &array._array);
        }
    }
}

impl From<GDArray> for GDPoolVector3Array {
    fn from(array: GDArray) -> GDPoolVector3Array {
        unsafe {
            let mut new_array = new_vector3_array();
            godot_pool_vector3_array_new_with_array(&mut new_array, &array._array);
            GDPoolVector3Array { _array: new_array }
        }
    }
}

//color array 
fn new_color_array() -> godot_pool_color_array {
    godot_pool_color_array { _dont_touch_that: [0; 8usize] }
}

impl GDPoolColorArray {
    pub fn new() -> GDPoolColorArray {
        unsafe {
            let mut new_array = new_color_array();
            godot_pool_color_array_new(&mut new_array);
            GDPoolColorArray { _array: new_array }
        }
    }

    pub fn new_copy(color_array: &GDPoolColorArray) -> GDPoolColorArray {
        unsafe {
            let mut new_array = new_color_array();
            godot_pool_color_array_new_copy(&mut new_array, &color_array._array);
            GDPoolColorArray { _array: new_array }
        }
    }

    pub fn insert(&mut self, index: i32, data: &GDColor) -> GDError {
        unsafe {
            godot_pool_color_array_insert(&mut self._array, index, &data._color)
        }
    }

    pub fn inverted(&self) -> GDPoolColorArray {
        unsafe {
            let mut cloned = self._array.clone();
            godot_pool_color_array_invert(&mut cloned);
            GDPoolColorArray { _array: cloned }
        }
    }

    pub fn push_back(&mut self, data: &GDColor) {
        unsafe {
            godot_pool_color_array_push_back(&mut self._array, &data._color)
        }
    }

    pub fn remove(&mut self, index: i32) {
        unsafe {
            godot_pool_color_array_remove(&mut self._array, index)
        }
    }

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            godot_pool_color_array_resize(&mut self._array, new_size)
        }
    }

    pub fn set(&mut self, index: i32, data: &GDColor) {
        unsafe {
            godot_pool_color_array_set(&mut self._array, index, &data._color)
        }
    }

    pub fn get(&self, index: i32) -> GDColor {
        unsafe {
            let color = godot_pool_color_array_get(&self._array, index);
            GDColor { _color: color }
        }
    }
    
    pub fn size(&self) -> i32 {
        unsafe {
            godot_pool_color_array_size(&self._array)
        }
    }

}

impl Drop for GDPoolColorArray {
    fn drop(&mut self) {
        unsafe { godot_pool_color_array_destroy(&mut self._array) }
    }
}

impl Appendable<GDColor> for GDPoolColorArray {
    fn append(&mut self, data: GDColor) {
        unsafe {
            godot_pool_color_array_append(&mut self._array, &data._color);
        }
    }
}

impl Appendable<GDPoolColorArray> for GDPoolColorArray {
    fn append(&mut self, array: GDPoolColorArray) {
        unsafe {
            godot_pool_color_array_append_array(&mut self._array, &array._array);
        }
    }
}

impl From<GDArray> for GDPoolColorArray {
    fn from(array: GDArray) -> GDPoolColorArray {
        unsafe {
            let mut new_array = new_color_array();
            godot_pool_color_array_new_with_array(&mut new_array, &array._array);
            GDPoolColorArray { _array: new_array }
        }
    }
}