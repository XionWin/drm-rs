use crate::{util, Drm};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DrmModeProperty {
    pub(crate) prop_id: libc::c_uint,
    pub(crate) flags: libc::c_uint,
    pub(crate) name: String,
    pub(crate) count_values: libc::c_int,
	pub(crate) values: Vec<libc::c_uint>,
	pub(crate) count_enums: libc::c_int, 
    pub(crate) enums: Vec<DrmModePropertyEnum>
}

impl Into<DrmModeProperty> for *const crate::DrmModePropertyPtr {
    fn into(self) -> DrmModeProperty {
        let v = unsafe { *self };
        DrmModeProperty {
            prop_id: v.prop_id,
            flags: v.flags,
            name: util::get_string_from_ptr(v.name.as_ptr()).unwrap_or_default(),
            count_values: v.count_values,
            values: Vec::new(),
            count_enums: v.count_enums,
            enums: Vec::new()
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DrmModePropertyEnum {
	pub(crate) value: libc::c_uint,
    pub(crate) name: String,
}

impl Into<DrmModePropertyEnum> for *const crate::DrmModePropertyEnumPtr {
    fn into(self) -> DrmModePropertyEnum {
        todo!()
    }
}

impl Drm {
    pub fn get_property(&self, property_id: libc::c_uint) -> DrmModeProperty {
        unsafe {
            crate::drmModeGetProperty(self.fd, property_id)
        }.into()
    }
}