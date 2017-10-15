use gdnative_sys::*;
use godot::*;
use variant::GDVariant;
use array::GDArray;
use string::GDString;
use std::ops::{Index, IndexMut};
use std::mem::transmute;

#[repr(C)]
#[derive(Clone)]
pub struct GDDictionary {
    _dictionary: godot_dictionary,
}

fn new_dictionary() -> godot_dictionary {
    godot_dictionary {
        _dont_touch_that: [0; 8usize],
    }
}

impl GDDictionary {
    pub fn new() -> GDDictionary {
        unsafe {
            let mut new_dict = new_dictionary();
            godot_dictionary_new(&mut new_dict);
            GDDictionary {
                _dictionary: new_dict,
            }
        }
    }

    pub fn copy(&self) -> GDDictionary {
        unsafe {
            let mut new_dict = new_dictionary();
            godot_dictionary_new_copy(&mut new_dict, &self._dictionary);
            GDDictionary {
                _dictionary: new_dict,
            }
        }
    }

    pub fn size(&self) -> i32 {
        unsafe { godot_dictionary_size(&self._dictionary) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { godot_dictionary_empty(&self._dictionary) }
    }

    pub fn clear(&mut self) {
        unsafe { godot_dictionary_clear(&mut self._dictionary) }
    }

    pub fn contains_key(&self, key: &GDVariant) -> bool {
        unsafe { godot_dictionary_has(&self._dictionary, &key._variant) }
    }

    pub fn contains_all(&self, keys: &GDArray) -> bool {
        unsafe { godot_dictionary_has_all(&self._dictionary, &keys._array) }
    }

    pub fn erase(&mut self, key: &GDVariant) {
        unsafe { godot_dictionary_erase(&mut self._dictionary, &key._variant) }
    }

    pub fn hash(&self) -> i32 {
        unsafe { godot_dictionary_hash(&self._dictionary) }
    }

    pub fn keys(&self) -> GDArray {
        unsafe {
            let array = godot_dictionary_keys(&self._dictionary);
            GDArray { _array: array }
        }
    }

    pub fn values(&self) -> GDArray {
        unsafe {
            let array = godot_dictionary_values(&self._dictionary);
            GDArray { _array: array }
        }
    }

    pub fn get(&self, key: &GDVariant) -> GDVariant {
        unsafe {
            let variant = godot_dictionary_get(&self._dictionary, &key._variant);
            GDVariant { _variant: variant }
        }
    }

    pub fn put(&mut self, key: &GDVariant, value: &GDVariant) {
        unsafe {
            let key_v = transmute::<&GDVariant, &godot_variant>(&key);
            let value_v = transmute::<&GDVariant, &godot_variant>(&key);
            godot_dictionary_set(&mut self._dictionary, key_v, value_v)
        }
    }

    pub fn next(&self, key: &GDVariant) -> GDVariant {
        unsafe {
            let variant = godot_dictionary_next(&self._dictionary, &key._variant);
            *transmute::<*mut godot_variant, &GDVariant>(variant)
        }
    }

    pub fn to_json(&self) -> GDString {
        unsafe {
            let string = godot_dictionary_to_json(&self._dictionary);
            GDString { _string: string }
        }
    }
}

impl<'a> Index<&'a GDVariant> for GDDictionary {
    type Output = GDVariant;

    fn index(&self, key: &GDVariant) -> &GDVariant {
        unsafe {
            let mut dict = self._dictionary;
            let mut v = godot_dictionary_operator_index(&mut dict, &key._variant);
            transmute::<*mut godot_variant, &GDVariant>(v)
        }
    }
}

impl<'a> IndexMut<&'a GDVariant> for GDDictionary {
    fn index_mut(&mut self, key: &GDVariant) -> &mut Self::Output {
        unsafe {
            let mut v = godot_dictionary_operator_index(&mut self._dictionary, &key._variant);
            transmute::<*mut godot_variant, &mut GDVariant>(v)
        }
    }
}

impl Drop for GDDictionary {
    fn drop(&mut self) {
        unsafe { godot_dictionary_destroy(&mut self._dictionary) }
    }
}

impl PartialEq for GDDictionary {
    fn eq(&self, other: &GDDictionary) -> bool {
        unsafe {
            let other_gd_dict = other._dictionary;
            let this_gd_dict = self._dictionary;
            godot_dictionary_operator_equal(&this_gd_dict, &other_gd_dict)
        }
    }

    fn ne(&self, other: &GDDictionary) -> bool {
        !(self != other)
    }
}

impl Eq for GDDictionary {}
