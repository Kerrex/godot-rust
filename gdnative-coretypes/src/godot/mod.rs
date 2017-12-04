use gdnative_sys::*;

pub mod array;
pub mod variant;
pub mod pool_arrays;
pub mod string;
pub mod node_path;
pub mod basis;
pub mod vector2;
pub mod vector3;
pub mod color;
pub mod dictionary;
pub mod quat;
pub mod plane;
pub mod rect2;
pub mod rect3;
pub mod transform2d;
pub mod transform;
pub mod rid;

#[repr(C)]
pub struct GDObject {
    pub(crate) _object: godot_object,
}

#[repr(C)]
pub struct GDArray {
    pub(crate) _array: godot_array,
}

#[repr(C)]
#[derive(Clone)]
pub struct GDPoolByteArray {
    pub(crate) _array: godot_pool_byte_array,
}

#[repr(C)]
pub struct GDPoolIntArray {
    pub(crate) _array: godot_pool_int_array,
}

#[repr(C)]
pub struct GDPoolRealArray {
    pub(crate) _array: godot_pool_real_array,
}

#[repr(C)]
pub struct GDPoolVector2Array {
    pub(crate) _array: godot_pool_vector2_array,
}

#[repr(C)]
pub struct GDPoolVector3Array {
    pub(crate) _array: godot_pool_vector3_array,
}

#[repr(C)]
pub struct GDPoolColorArray {
    pub(crate) _array: godot_pool_color_array,
}

#[repr(C)]
pub struct GDPoolStringArray {
    pub(crate) _array: godot_pool_string_array,
}

#[repr(C)]
pub struct GDBasis {
    pub(crate) _basis: godot_basis,
}

#[repr(C)]
pub struct GDColor {
    pub(crate) _color: godot_color,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDVariant {
    pub(crate) _variant: godot_variant,
}

#[repr(C)]
#[derive(Clone)]
pub struct GDDictionary {
    _dictionary: godot_dictionary,
}

#[derive(Clone)]
#[repr(C)]
pub struct GDNodePath {
    _node_path: godot_node_path,
}

#[repr(C)]
pub struct GDPlane {
    pub(crate) _plane: godot_plane,
}

#[repr(C)]
#[derive(Clone)]
pub struct GDQuat {
    pub(crate) _quat: godot_quat,
}

#[repr(C)]
#[derive(Clone)]
pub struct GDRect2 {
    pub(crate) _rect: godot_rect2,
}

#[repr(C)]
#[derive(Clone)]
pub struct GDRect3 {
    pub(crate) _rect: godot_rect3,
}

#[repr(C)]
pub struct GDRid {
    pub(crate) _rid: godot_rid,
}

#[derive(Clone)]
#[repr(C)]
pub struct GDString {
    pub(crate) _string: godot_string,
}

#[repr(C)]
pub struct GDTransform {
    pub(crate) _transform: godot_transform,
}

#[repr(C)]
pub struct GDTransform2D {
    pub(crate) _transform: godot_transform2d,
}

#[repr(C)]
pub struct GDVector2 {
    pub(crate) _vector: godot_vector2,
}

#[repr(C)]
pub struct GDVector3 {
    pub(crate) _vector: godot_vector3,
}

pub type GDError = godot_error;
