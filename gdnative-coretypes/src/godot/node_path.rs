use gdnative_sys::*;
use godot::*;
use std::mem::transmute;

fn new_node_path() -> godot_node_path {
    godot_node_path {_dont_touch_that: [0; 8usize]}
}

impl GDNodePath {
    pub fn new() -> GDNodePath {
        unsafe {
            let mut new_node_path = new_node_path();
            let new_gd_string = GDString::from_string("".to_string());
            godot_node_path_new(&mut new_node_path, &new_gd_string._string);
            GDNodePath {_node_path: new_node_path}
        }
    }

    pub fn from_node_path(other: &GDNodePath) -> GDNodePath {
        unsafe {
            let mut new_node_path = new_node_path();
            let mut from = &other._node_path;
            let from_in_string = transmute::<&godot_node_path, &godot_string>(from);
            godot_node_path_new(&mut new_node_path, from_in_string);
            GDNodePath{_node_path: new_node_path}
        }
    }

    pub fn get_name(&self, index: i32) -> GDString {
        unsafe {
            let name = godot_node_path_get_name(&self._node_path, index);
            GDString{ _string: name }
        }
    }

    pub fn get_name_count(&self) -> i32 {
        unsafe {
            godot_node_path_get_name_count(&self._node_path)
        }
    }

    pub fn get_property(&self) -> GDString {
        unsafe {
            let gd_string = godot_node_path_get_property(&self._node_path);
            GDString::from(gd_string)
        }
    }

    pub fn get_subname(&self, index: i32) -> GDString {
        unsafe {
            let subname = godot_node_path_get_subname(&self._node_path, index);
            GDString::from(subname)
        }
    }

    pub fn get_subname_count(&self) -> i32 {
        unsafe {
            godot_node_path_get_subname_count(&self._node_path)
        }
    }

    pub fn is_absolute(&self) -> bool {
        unsafe {
            godot_node_path_is_absolute(&self._node_path)
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            godot_node_path_is_empty(&self._node_path)
        }
    }
}

impl Into<GDString> for GDNodePath {
    fn into(self) -> GDString {
        unsafe {
            let node_as_string = godot_node_path_as_string(&self._node_path);
            GDString::from(node_as_string)
        }
    }
}

impl From<GDString> for GDNodePath {
    fn from(from: GDString) -> GDNodePath {
        unsafe {
            let mut new_node = new_node_path();
            let gd_string = from._string;
            godot_node_path_new(&mut new_node, &gd_string);
            GDNodePath {_node_path: new_node}
        }
    }
}

impl Drop for GDNodePath {
    fn drop(&mut self) {
        unsafe {
            godot_node_path_destroy(&mut self._node_path)
        }
    }
}