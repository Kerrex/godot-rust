extern crate gdnative_sys;
extern crate cgmath;

pub mod array;
pub mod variant;
pub mod pool_arrays;
pub mod string;
pub mod node_path;
pub mod basis;
pub mod vector2;
pub mod vector3;

mod godot {
    use gdnative_sys::*;
    use cgmath::{Vector2, Vector3};
    use std::mem;
    use array::GDArray;
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



    // Temporary, delete after generating bindings based on *.json file
    #[derive(Copy, Clone)]
    pub struct GDObject {}

    pub type GDVector3 = Vector3<f32>;
}