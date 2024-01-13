const DRM_NAME_STRING_LEN: usize = 32usize;
#[repr(C)]
#[derive(Debug)]
pub struct DrmModePropertyPtr {
    prop_id: libc::c_uint,
    flags: libc::c_uint,
    name: [libc::c_char; DRM_NAME_STRING_LEN],
    count_values: libc::c_int,
	values: *const libc::c_uint,
	count_enums: libc::c_int,
    enums: *const DrmModePropertyEnum
}

#[repr(C)]
#[derive(Debug)]
pub struct DrmModePropertyEnum {
	value: libc::c_uint,
    name: [libc::c_char; DRM_NAME_STRING_LEN],
}

#[link(name = "drm")]
#[allow(improper_ctypes, dead_code)]
extern "C" {
    pub fn drmModeGetProperty(fd: libc::c_int, property_id: libc::c_uint) -> *const DrmModePropertyPtr;
    pub fn drmModeConnectorSetProperty(fd: libc::c_int, connector_id: libc::c_uint, property_id: libc::c_uint, value: libc::c_uint) -> libc::c_int;
}
