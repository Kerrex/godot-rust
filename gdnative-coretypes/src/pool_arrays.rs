use gdnative_sys::*;

#[repr(C)]
pub struct GDPoolByteArray {
    pub(crate) _array: godot_pool_byte_array
}

#[repr(C)]
pub struct GDPoolIntArray {
    pub(crate) _array: godot_pool_int_array
}

#[repr(C)]
pub struct GDPoolRealArray {
    pub(crate) _array: godot_pool_real_array
}

#[repr(C)]
pub struct GDPoolVector2Array {
    pub(crate) _array: godot_pool_vector2_array
}

#[repr(C)]
pub struct GDPoolVector3Array {
    pub(crate) _array: godot_pool_vector3_array
}

#[repr(C)]
pub struct GDPoolColorArray {
    pub(crate) _array: godot_pool_color_array
}

#[repr(C)]
pub struct GDPoolStringArray {
    pub(crate) _array: godot_pool_string_array
}