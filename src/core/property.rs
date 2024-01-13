use crate::{util, Drm};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Property {
    pub(crate) handle: *const crate::DrmModeProperty,
    pub(crate) prop_id: libc::c_uint,
    pub(crate) flags: libc::c_uint,
    pub(crate) name: String,
    pub(crate) count_values: libc::c_int,
	pub(crate) values: Vec<libc::c_uint>,
	pub(crate) count_enums: libc::c_int, 
    pub(crate) enums: Vec<PropertyEnum>
}

impl Property {
    pub(crate) fn free (&self) {
        unsafe {
            crate::drmModeFreeProperty(self.handle)
        }
    }
}

impl Drop for Property {
    fn drop(&mut self) {
        self.free()
    }
}

impl Into<Property> for *const crate::DrmModeProperty {
    fn into(self) -> Property {
        let v = unsafe { *self };
        Property {
            handle: self,
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
pub struct PropertyEnum {
	pub(crate) value: libc::c_uint,
    pub(crate) name: String,
}

impl Into<PropertyEnum> for *const crate::DrmModePropertyEnum {
    fn into(self) -> PropertyEnum {
        todo!()
    }
}

impl Drm {
    pub fn get_property(&self, property_id: libc::c_uint) -> Property {
        unsafe {
            crate::drmModeGetProperty(self.fd, property_id)
        }.into()
    }
}