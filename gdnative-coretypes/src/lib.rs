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


    struct PoolByteArray {
        _array: godot_pool_byte_array
    }

    struct PoolIntArray {
        _array: godot_pool_int_array
    }

    struct PoolRealArray {
        _array: godot_pool_real_array
    }

    struct PoolVector2Array {
        _array: godot_pool_vector2_array
    }

    struct PoolVector3Array {
        _array: godot_pool_vector3_array
    }

    struct PoolColorArray {
        _array: godot_pool_color_array
    }

    struct PoolStringArray {
        _array: godot_pool_string_array
    }

    struct Array {
        _array: godot_array,
    }

    struct Variant {
        _variant: godot_variant
    }

    impl Array {
        pub fn new() -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new(&mut array); }
            Array { _array: array }
        }

        pub fn from_pool_byte_array(pool_array: &PoolByteArray) -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new_pool_byte_array(&mut array, &pool_array._array) }
            Array { _array: array }
        }

        pub fn from_pool_int_array(pool_array: &PoolIntArray) -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new_pool_int_array(&mut array, &pool_array._array) }
            Array { _array: array }
        }

        pub fn from_pool_real_array(pool_array: &PoolRealArray) -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new_pool_real_array(&mut array, &pool_array._array) }
            Array { _array: array }
        }

        pub fn from_pool_string_array(pool_array: &PoolStringArray) -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new_pool_string_array(&mut array, &pool_array._array) }
            Array { _array: array }
        }

        pub fn from_pool_vector2_array(pool_array: &PoolVector2Array) -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new_pool_vector2_array(&mut array, &pool_array._array) }
            Array { _array: array }
        }

        pub fn from_pool_vector3_array(pool_array: &PoolVector3Array) -> Array {
            let mut array = godot_array { _dont_touch_that: [0; 8usize] };
            unsafe { godot_array_new_pool_vector3_array(&mut array, &pool_array._array) }
            Array { _array: array }
        }

        unsafe fn append(&mut self, variant: &Variant) {
            let gd_variant = mem::transmute::<&Variant, &godot_variant>(variant);
            godot_array_append(&mut self._array, gd_variant);
        }

        unsafe fn clear(&mut self) {
            godot_array_clear(&mut self._array)
        }

        unsafe fn count(&mut self, variant: &Variant) -> i32 {
            let gd_variant = mem::transmute::<&Variant, &godot_variant>(variant);
            godot_array_count(&mut self._array, gd_variant)
        }

        unsafe fn empty(&mut self) -> bool {
            godot_array_empty(&mut self._array)
        }

        unsafe fn erase(&mut self, variant: &Variant) {
            let gd_variant = mem::transmute::<&Variant, &godot_variant>(variant);
            godot_array_erase(&mut self._array, gd_variant)
        }

        // TODO DOPISAĆ RESZTĘ ARRAY'A
    }

    impl Index<i32> for Array {
        type Output = Variant;

        fn index(&self, index: i32) -> &Variant {
            let mut array = self._array;
            unsafe {
                let v = godot_array_operator_index(&mut array, index);
                mem::transmute::<*mut godot_variant, &Variant>(v)
            }
        }
    }

    impl IndexMut<i32> for Array {
        fn index_mut(&mut self, index: i32) -> &mut Self::Output {
            let mut array = self._array;
            unsafe {
                let v = godot_array_operator_index(&mut array, index);
                mem::transmute::<*mut godot_variant, &mut Variant>(v)
            }
        }
    }
}