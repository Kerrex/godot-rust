extern crate gdnative_sys;


mod godot {
    use gdnative_sys::*;
    use std::ops::{Index, IndexMut};
    use std::mem;

    /// Taken from godot cpp_bindings, original spelling
    enum Error {
        Ok,
        Failed,
        ///< Generic fail error
        ErrUnavailable,
        ///< What is requested is unsupported/unavailable
        ErrUnconfigured,
        ///< The object being used hasnt been properly set up yet
        ErrUnauthorized,
        ///< Missing credentials for requested resource
        ErrParameterRangeError,
        ///< Parameter given out of range (5)
        ErrOutOfMemory,
        ///< Out of memory
        ErrFileNotFound,
        ErrFileBadDrive,
        ErrFileBadPath,
        ErrFileNoPermission,
        // (10)
        ErrFileAlreadyInUse,
        ErrFileCantOpen,
        ErrFileCantWrite,
        ErrFileCantRead,
        ErrFileUnrecognized,
        // (15)
        ErrFileCorrupt,
        ErrFileMissingDependencies,
        ErrFileEof,
        ErrCantOpen,
        ///< Can't open a resource/socket/file
        ErrCantCreate,
        // (20)
        ErrQueryFailed,
        ErrAlreadyInUse,
        ErrLocked,
        ///< resource is locked
        ErrTimeout,
        ErrCantConnect,
        // (25)
        ErrCantResolve,
        ErrConnectionError,
        ErrCantAquireResource,
        ErrCantFork,
        ErrInvalidData,
        ///< Data passed is invalid	(30)
        ErrInvalidParameter,
        ///< Parameter passed is invalid
        ErrAlreadyExists,
        ///< When adding, item already exists
        ErrDoesNotExist,
        ///< When retrieving/erasing, it item does not exist
        ErrDatabaseCantRead,
        ///< database is full
        ErrDatabaseCantWrite,
        ///< database is full	(35)
        ErrCompilationFailed,
        ErrMethodNotFound,
        ErrLinkFailed,
        ErrScriptFailed,
        ErrCyclicLink,
        // (40)
        ErrInvalidDeclaration,
        ErrDuplicateSymbol,
        ErrParseError,
        ErrBusy,
        ErrSkip,
        // (45)
        ErrHelp,
        ///< user requested help!!
        ErrBug,
        ///< a bug in the software certainly happened, due to a double check failing or unexpected behavior.
        ErrPrinterOnFire,
        /// the parallel port printer is engulfed in flames
        ErrOmfgThisIsVeryVeryBad //< shit happens, has never been used, though
    }

    macro_rules! cmp_epsilon {
        () => {0.00001};
    }

    macro_rules! cmp_epsilon2 {
        () => {cmp_epsilon!() * cmp_epsilon!()};
    }

    macro_rules! pi {
        () => {3.14159265358979323846};
    }

    macro_rules! _plane_eq_dot_epsilon {
        () => {0.999};
    }

    macro_rules! _plane_eq_d_epsilon {
        () => {0.0001};
    }


    struct GDPoolByteArray {
        _array: godot_pool_byte_array
    }

    struct GDPoolIntArray {
        _array: godot_pool_int_array
    }

    struct GDPoolRealArray {
        _array: godot_pool_real_array
    }

    struct GDPoolVector2Array {
        _array: godot_pool_vector2_array
    }

    struct GDPoolVector3Array {
        _array: godot_pool_vector3_array
    }

    struct GDPoolColorArray {
        _array: godot_pool_color_array
    }

    struct GDPoolStringArray {
        _array: godot_pool_string_array
    }

    struct GDArray {
        _array: godot_array,
    }

    #[derive(Copy, Clone)]
    pub struct GDVariant {
        _variant: godot_variant
    }

    // Temporary, delete after generating bindings based on *.json file
    #[derive(Copy, Clone)]
    pub struct GDObject {}

    #[derive(Copy, Clone)]
    pub struct GDString{}

    impl GDVariant {
        fn into_godot_variant(variant: &GDVariant) -> &godot_variant {
            unsafe {
                mem::transmute::<&GDVariant, &godot_variant>(&variant)
            }
        }

        fn into_variant(&gd_variant: &godot_variant) -> &GDVariant {
            unsafe {
                mem::transmute::<&godot_variant, &GDVariant>(&gd_variant)
            }
        }
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

        fn append(&mut self, variant: &GDVariant) {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(variant);
                godot_array_append(&mut self._array, gd_variant);
            }
        }

        fn clear(&mut self) {
            unsafe { godot_array_clear(&mut self._array) }
        }

        fn count(&mut self, variant: &GDVariant) -> i32 {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(variant);
                godot_array_count(&mut self._array, gd_variant)
            }
        }

        fn empty(&mut self) -> bool {
            unsafe { godot_array_empty(&mut self._array) }
        }

        fn erase(&mut self, variant: &GDVariant) {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(variant);
                godot_array_erase(&mut self._array, gd_variant)
            }
        }

        fn front(&mut self) -> GDVariant {
            unsafe {
                let gd_variant = godot_array_front(&mut self._array);
                let variant = (mem::transmute::<&godot_variant, &GDVariant>(&gd_variant));
                *variant
            }
        }

        fn back(&mut self) -> GDVariant {
            unsafe {
                let gd_variant = godot_array_back(&mut self._array);
                let variant = (mem::transmute::<&godot_variant, &GDVariant>(&gd_variant));
                *variant
            }
        }

        fn find(&mut self, what: &GDVariant, from: i32) -> i32 {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&what);
                godot_array_find(&mut self._array, gd_variant, from)
            }
        }

        fn find_last(&mut self, what: &GDVariant) -> i32 {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&what);
                godot_array_find_last(&mut self._array, gd_variant)
            }
        }

        fn has(&mut self, what: &GDVariant) -> bool {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&what);
                godot_array_has(&mut self._array, gd_variant)
            }
        }

        fn hash(&mut self) -> i32 {
            unsafe {
                godot_array_hash(&mut self._array)
            }
        }

        fn insert(&mut self, position: i32, value: &GDVariant) {
            unsafe {
                let gd_variant = mem::transmute::<&GDVariant, &godot_variant>(&value);
                godot_array_insert(&mut self._array, position, gd_variant)
            }
        }

        fn invert(&mut self) {
            unsafe {
                godot_array_invert(&mut self._array)
            }
        }

        fn pop_back(&mut self) -> GDVariant {
            unsafe {
                let gd_variant = godot_array_pop_back(&mut self._array);
                let variant = mem::transmute::<&godot_variant, &GDVariant>(&gd_variant);
                *variant
            }
        }

        fn pop_front(&mut self) -> GDVariant {
            unsafe {
                let gd_variant = godot_array_pop_front(&mut self._array);
                let variant = mem::transmute::<&godot_variant, &GDVariant>(&gd_variant);
                *variant
            }
        }

        fn push_back(&mut self, variant: &GDVariant) {
            unsafe {
                let gd_variant = GDVariant::into_godot_variant(variant);
                godot_array_push_back(&mut self._array, gd_variant)
            }
        }

        fn push_front(&mut self, variant: &GDVariant) {
            unsafe {
                let gd_variant = GDVariant::into_godot_variant(variant);
                godot_array_push_front(&mut self._array, gd_variant)
            }
        }

        fn remove(&mut self, index: i32) {
            unsafe {
                godot_array_remove(&mut self._array, index)
            }
        }

        fn size(&mut self) -> i32 {
            unsafe {
                godot_array_size(&mut self._array)
            }
        }

        fn resize(&mut self, new_size: i32) {
            unsafe {
                godot_array_resize(&mut self._array, new_size)
            }
        }

        fn rfind(&mut self, what: &GDVariant, from: i32) -> i32 {
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
                godot_array_sort_custom(&mut self._array,gd_object, gd_string)
            }
        }

        // TODO DOPISAĆ RESZTĘ ARRAY'A
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
}