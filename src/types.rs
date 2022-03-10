use mpi::ffi;

pub type MPIAint = ffi::MPI_Aint;
pub type MPIFint = ffi::RSMPI_Fint;
pub type MPIOffset = ffi::MPI_Offset;
pub type MPIComm = ffi::MPI_Comm;
pub type MPICount = ffi::MPI_Count;
pub type MPIMessage = ffi::MPI_Message;
pub type MPIRequest = ffi::MPI_Request;
pub type MPIInfo = ffi::MPI_Info;
pub type MPIErrhandler = ffi::MPI_Errhandler;
pub type MPIStatus = ffi::MPI_Status;
pub type MPIDatatype = ffi::MPI_Datatype;
pub type MPIOp = ffi::MPI_Op;
pub type MPIWin = ffi::MPI_Win;
pub type MPIGroup = ffi::MPI_Group;
pub type MPIFile = ffi::MPI_File;

/// MPI 4.0 ch. 9.3.1 pg. 461 line 27 MPI_Comm_create_errhandler
pub type MPICommErrhandlerFunction = ::std::option::Option<
    unsafe extern "C" fn(comm: *mut MPIComm, error_code: *mut ::std::os::raw::c_int, ...),
>;

/// MPI 4.0 ch. 7.7.2 pg. 367 line 33 MPI_Comm_copy_attr_function
pub type MPICommCopyAttrFunction = ::std::option::Option<
    unsafe extern "C" fn(
        oldcomm: MPIComm,
        comm_keyval: ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
        attribute_val_in: *mut ::std::os::raw::c_void,
        attribute_val_out: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 7.7.2 pg. 367 line 38 MPI_Comm_delete_attr_function
pub type MPICommDeleteAttrFunction = ::std::option::Option<
    unsafe extern "C" fn(
        comm: MPIComm,
        comm_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 9.3.3 pg. 465 line 26 MPI_File_errhandler_function
pub type MPIFileErrhandlerFunction = ::std::option::Option<
    unsafe extern "C" fn(file: *mut MPIFile, error_code: *mut ::std::os::raw::c_int, ...),
>;

/// MPI 4.0 ch. 13.2 pg. 635 line 2 MPI_Grequest_query_function
pub type MPIGrequestQueryFunction = ::std::option::Option<
    unsafe extern "C" fn(
        extra_state: *mut ::std::os::raw::c_void,
        status: *mut MPIStatus,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 13.2 pg. 635 line 34 MPI_Grequest_free_function
pub type MPIGrequestFreeFunction = ::std::option::Option<
    unsafe extern "C" fn(extra_state: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 13.2 pg. 636 line 27 MPI_Grequest_cancel_function
pub type MPIGrequestCancelFunction = ::std::option::Option<
    unsafe extern "C" fn(
        extra_state: *mut ::std::os::raw::c_void,
        complete: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 6.9.5 pg. 234 line 23 MPI_User_function
pub type MPIUserFunction = ::std::option::Option<
    unsafe extern "C" fn(
        invec: *mut ::std::os::raw::c_void,
        inoutvec: *mut ::std::os::raw::c_void,
        len: *mut ::std::os::raw::c_int,
        datatype: *mut MPIDatatype,
    ),
>;

/// MPI 4.0 ch. 14.5.3 pg. 706 line 22 MPI_Datarep_conversion_function
pub type MPIDatarepConversionFunction = ::std::option::Option<
    unsafe extern "C" fn(
        userbuf: *mut ::std::os::raw::c_void,
        datatype: MPIDatatype,
        count: ::std::os::raw::c_int,
        filebuf: *mut ::std::os::raw::c_void,
        position: MPIOffset,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 14.5.3 pg. 705 line 40 MPI_Datarep_extent_function
pub type MPIDatarepExtentFunction = ::std::option::Option<
    unsafe extern "C" fn(
        datatype: MPIDatatype,
        extent: *mut MPIAint,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 7.7.4 pg. 376 line 45 MPI_Type_copy_attr_function
pub type MPITypeCopyAttrFunction = ::std::option::Option<
    unsafe extern "C" fn(
        oldtype: MPIDatatype,
        type_keyval: ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
        attribute_val_in: *mut ::std::os::raw::c_void,
        attribute_val_out: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 7.7.4 pg. 377 line 1 MPI_Type_delete_attr_function
pub type MPITypeDeleteAttrFunction = ::std::option::Option<
    unsafe extern "C" fn(
        datatype: MPIDatatype,
        type_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 9.3.2 pg. 463 line 31 MPI_Win_errhandler_function
pub type MPIWinErrhandlerFunction = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut MPIWin, arg2: *mut ::std::os::raw::c_int, ...),
>;

/// MPI 4.0 ch. 7.7.3 pg. 373 line 22 MPI_Win_copy_attr_function
pub type MPIWinCopyAttrFunction = ::std::option::Option<
    unsafe extern "C" fn(
        oldwin: MPIWin,
        win_keyval: ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
        attribute_val_in: *mut ::std::os::raw::c_void,
        attribute_val_out: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;

/// MPI 4.0 ch. 7.7.3 pg. 373 line 27 MPI_Win_delete_attr_function
pub type MPIWinDeleteAttrFunction = ::std::option::Option<
    unsafe extern "C" fn(
        win: MPIWin,
        win_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
