use openexr_sys as sys;

use crate::{
    core::{
        channel_list::{ChannelList, ChannelListRef, ChannelListRefMut},
        cppstd::{
            CppString, CppVectorFloat, CppVectorFloatRef, CppVectorFloatRefMut,
            CppVectorString, CppVectorStringRef, CppVectorStringRefMut,
        },
        preview_image::{PreviewImage, PreviewImageRef, PreviewImageRefMut},
        refptr::{OpaquePtr, Ref, RefMut},
        tile_description::TileDescription,
        Chromaticities, Compression, Envmap, LineOrder,
    },
    deep::DeepImageState,
};

use imath_traits::{Bound2, Matrix33, Matrix44, Vec2, Vec3};

use std::ffi::CStr;

#[repr(transparent)]
pub struct Attribute(pub(crate) *mut sys::Imf_Attribute_t);

pub trait TypedAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t;
}

// ----------------------------------------------------------------------------
// Box2iAttribute
#[repr(transparent)]
pub struct Box2iAttribute(pub(crate) *mut sys::Imf_Box2iAttribute_t);

unsafe impl OpaquePtr for Box2iAttribute {
    type SysPointee = sys::Imf_Box2iAttribute_t;
    type Pointee = Box2iAttribute;
}

pub type Box2iAttributeRef<'a, P = Box2iAttribute> = Ref<'a, P>;
pub type Box2iAttributeRefMut<'a, P = Box2iAttribute> = RefMut<'a, P>;

impl Box2iAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> Box2iAttribute
    where
        T: Bound2<i32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_Box2iAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_Box2i_t,
            )
            .into_result()
            .unwrap();
        }

        Box2iAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Bound2<i32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_Box2iAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_Box2i_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Bound2<i32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_Box2iAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_Box2i_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Box2iAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for Box2iAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_Box2iAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// Box2fAttribute
#[repr(transparent)]
pub struct Box2fAttribute(pub(crate) *mut sys::Imf_Box2fAttribute_t);

unsafe impl OpaquePtr for Box2fAttribute {
    type SysPointee = sys::Imf_Box2fAttribute_t;
    type Pointee = Box2fAttribute;
}

pub type Box2fAttributeRef<'a, P = Box2fAttribute> = Ref<'a, P>;
pub type Box2fAttributeRefMut<'a, P = Box2fAttribute> = RefMut<'a, P>;

impl Box2fAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> Box2fAttribute
    where
        T: Bound2<f32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_Box2fAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_Box2f_t,
            )
            .into_result()
            .unwrap();
        }

        Box2fAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Bound2<f32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_Box2fAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_Box2f_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Bound2<f32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_Box2fAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_Box2f_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Box2fAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for Box2fAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_Box2fAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// IntAttribute
#[repr(transparent)]
pub struct IntAttribute(pub(crate) *mut sys::Imf_IntAttribute_t);

unsafe impl OpaquePtr for IntAttribute {
    type SysPointee = sys::Imf_IntAttribute_t;
    type Pointee = IntAttribute;
}

pub type IntAttributeRef<'a, P = IntAttribute> = Ref<'a, P>;
pub type IntAttributeRefMut<'a, P = IntAttribute> = RefMut<'a, P>;

impl IntAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: i32) -> IntAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_IntAttribute_from_value(&mut inner, &value)
                .into_result()
                .unwrap();
        }

        IntAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> &i32 {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_IntAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const i32)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut i32 {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_IntAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut i32)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_IntAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for IntAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_IntAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// FloatAttribute
#[repr(transparent)]
pub struct FloatAttribute(pub(crate) *mut sys::Imf_FloatAttribute_t);

unsafe impl OpaquePtr for FloatAttribute {
    type SysPointee = sys::Imf_FloatAttribute_t;
    type Pointee = FloatAttribute;
}

pub type FloatAttributeRef<'a, P = FloatAttribute> = Ref<'a, P>;
pub type FloatAttributeRefMut<'a, P = FloatAttribute> = RefMut<'a, P>;

impl FloatAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: f32) -> FloatAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_FloatAttribute_from_value(&mut inner, &value)
                .into_result()
                .unwrap();
        }

        FloatAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> &f32 {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FloatAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const f32)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut f32 {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FloatAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut f32)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_FloatAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for FloatAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_FloatAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// DoubleAttribute
#[repr(transparent)]
pub struct DoubleAttribute(pub(crate) *mut sys::Imf_DoubleAttribute_t);

unsafe impl OpaquePtr for DoubleAttribute {
    type SysPointee = sys::Imf_DoubleAttribute_t;
    type Pointee = DoubleAttribute;
}

pub type DoubleAttributeRef<'a, P = DoubleAttribute> = Ref<'a, P>;
pub type DoubleAttributeRefMut<'a, P = DoubleAttribute> = RefMut<'a, P>;

impl DoubleAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: f64) -> DoubleAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_DoubleAttribute_from_value(&mut inner, &value)
                .into_result()
                .unwrap();
        }

        DoubleAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> &f64 {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DoubleAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const f64)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut f64 {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DoubleAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut f64)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DoubleAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for DoubleAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_DoubleAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// ChromaticitiesAttribute
#[repr(transparent)]
pub struct ChromaticitiesAttribute(
    pub(crate) *mut sys::Imf_ChromaticitiesAttribute_t,
);

unsafe impl OpaquePtr for ChromaticitiesAttribute {
    type SysPointee = sys::Imf_ChromaticitiesAttribute_t;
    type Pointee = ChromaticitiesAttribute;
}

pub type ChromaticitiesAttributeRef<'a, P = ChromaticitiesAttribute> =
    Ref<'a, P>;
pub type ChromaticitiesAttributeRefMut<'a, P = ChromaticitiesAttribute> =
    RefMut<'a, P>;

impl ChromaticitiesAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &Chromaticities) -> ChromaticitiesAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_ChromaticitiesAttribute_from_value(&mut inner, value)
                .into_result()
                .unwrap();
        }

        ChromaticitiesAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> &Chromaticities {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_ChromaticitiesAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const Chromaticities)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut Chromaticities {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_ChromaticitiesAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut Chromaticities)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_ChromaticitiesAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for ChromaticitiesAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_ChromaticitiesAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// CompressionAttribute
#[repr(transparent)]
pub struct CompressionAttribute(
    pub(crate) *mut sys::Imf_CompressionAttribute_t,
);

unsafe impl OpaquePtr for CompressionAttribute {
    type SysPointee = sys::Imf_CompressionAttribute_t;
    type Pointee = CompressionAttribute;
}

pub type CompressionAttributeRef<'a, P = CompressionAttribute> = Ref<'a, P>;
pub type CompressionAttributeRefMut<'a, P = CompressionAttribute> =
    RefMut<'a, P>;

impl CompressionAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &Compression) -> CompressionAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_CompressionAttribute_from_value(
                &mut inner,
                value as *const Compression as *const sys::Imf_Compression,
            )
            .into_result()
            .unwrap();
        }

        CompressionAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> Compression {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_CompressionAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *(ptr as *const Compression)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut Compression {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_CompressionAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut Compression)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_CompressionAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for CompressionAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_CompressionAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// DeepImageStateAttribute
#[repr(transparent)]
pub struct DeepImageStateAttribute(
    pub(crate) *mut sys::Imf_DeepImageStateAttribute_t,
);

unsafe impl OpaquePtr for DeepImageStateAttribute {
    type SysPointee = sys::Imf_DeepImageStateAttribute_t;
    type Pointee = DeepImageStateAttribute;
}

pub type DeepImageStateAttributeRef<'a, P = DeepImageStateAttribute> =
    Ref<'a, P>;
pub type DeepImageStateAttributeRefMut<'a, P = DeepImageStateAttribute> =
    RefMut<'a, P>;

impl DeepImageStateAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &DeepImageState) -> DeepImageStateAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepImageStateAttribute_from_value(
                &mut inner,
                value as *const DeepImageState
                    as *const sys::Imf_DeepImageState,
            )
            .into_result()
            .unwrap();
        }

        DeepImageStateAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> DeepImageState {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepImageStateAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *(ptr as *const DeepImageState)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut DeepImageState {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepImageStateAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut DeepImageState)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepImageStateAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for DeepImageStateAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_DeepImageStateAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// EnvmapAttribute
#[repr(transparent)]
pub struct EnvmapAttribute(pub(crate) *mut sys::Imf_EnvmapAttribute_t);

unsafe impl OpaquePtr for EnvmapAttribute {
    type SysPointee = sys::Imf_EnvmapAttribute_t;
    type Pointee = EnvmapAttribute;
}

pub type EnvmapAttributeRef<'a, P = EnvmapAttribute> = Ref<'a, P>;
pub type EnvmapAttributeRefMut<'a, P = EnvmapAttribute> = RefMut<'a, P>;

impl EnvmapAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &Envmap) -> EnvmapAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_EnvmapAttribute_from_value(
                &mut inner,
                value as *const Envmap as *const sys::Imf_Envmap,
            )
            .into_result()
            .unwrap();
        }

        EnvmapAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> Envmap {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_EnvmapAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *(ptr as *const Envmap)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut Envmap {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_EnvmapAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut Envmap)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_EnvmapAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for EnvmapAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_EnvmapAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// ChannelListAttribute
#[repr(transparent)]
pub struct ChannelListAttribute(
    pub(crate) *mut sys::Imf_ChannelListAttribute_t,
);

unsafe impl OpaquePtr for ChannelListAttribute {
    type SysPointee = sys::Imf_ChannelListAttribute_t;
    type Pointee = ChannelListAttribute;
}

pub type ChannelListAttributeRef<'a, P = ChannelListAttribute> = Ref<'a, P>;
pub type ChannelListAttributeRefMut<'a, P = ChannelListAttribute> =
    RefMut<'a, P>;

impl ChannelListAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &ChannelList) -> ChannelListAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_ChannelListAttribute_from_value(&mut inner, value.0)
                .into_result()
                .unwrap();
        }

        ChannelListAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> ChannelListRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_ChannelListAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            ChannelListRef::new(ptr)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> ChannelListRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_ChannelListAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            ChannelListRefMut::new(ptr)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_ChannelListAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for ChannelListAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_ChannelListAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// CppVectorFloatAttribute
#[repr(transparent)]
pub struct CppVectorFloatAttribute(
    pub(crate) *mut sys::Imf_CppVectorFloatAttribute_t,
);

unsafe impl OpaquePtr for CppVectorFloatAttribute {
    type SysPointee = sys::Imf_CppVectorFloatAttribute_t;
    type Pointee = CppVectorFloatAttribute;
}

pub type CppVectorFloatAttributeRef<'a, P = CppVectorFloatAttribute> =
    Ref<'a, P>;
pub type CppVectorFloatAttributeRefMut<'a, P = CppVectorFloatAttribute> =
    RefMut<'a, P>;

impl CppVectorFloatAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &CppVectorFloat) -> CppVectorFloatAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_CppVectorFloatAttribute_from_value(&mut inner, value.0)
                .into_result()
                .unwrap();
        }

        CppVectorFloatAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> CppVectorFloatRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_CppVectorFloatAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            CppVectorFloatRef::new(ptr)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> CppVectorFloatRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_CppVectorFloatAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            CppVectorFloatRefMut::new(ptr)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_CppVectorFloatAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for CppVectorFloatAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_CppVectorFloatAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// CppVectorStringAttribute
#[repr(transparent)]
pub struct CppVectorStringAttribute(
    pub(crate) *mut sys::Imf_CppVectorStringAttribute_t,
);

unsafe impl OpaquePtr for CppVectorStringAttribute {
    type SysPointee = sys::Imf_CppVectorStringAttribute_t;
    type Pointee = CppVectorStringAttribute;
}

pub type CppVectorStringAttributeRef<'a, P = CppVectorStringAttribute> =
    Ref<'a, P>;
pub type CppVectorStringAttributeRefMut<'a, P = CppVectorStringAttribute> =
    RefMut<'a, P>;

impl CppVectorStringAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &CppVectorString) -> CppVectorStringAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_CppVectorStringAttribute_from_value(&mut inner, value.0)
                .into_result()
                .unwrap();
        }

        CppVectorStringAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> CppVectorStringRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_CppVectorStringAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            CppVectorStringRef::new(ptr)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> CppVectorStringRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_CppVectorStringAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            CppVectorStringRefMut::new(ptr)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_CppVectorStringAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for CppVectorStringAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_CppVectorStringAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// CppStringAttribute
#[repr(transparent)]
pub struct CppStringAttribute(pub(crate) *mut sys::Imf_CppStringAttribute_t);

unsafe impl OpaquePtr for CppStringAttribute {
    type SysPointee = sys::Imf_CppStringAttribute_t;
    type Pointee = CppStringAttribute;
}

pub type CppStringAttributeRef<'a, P = CppStringAttribute> = Ref<'a, P>;
pub type CppStringAttributeRefMut<'a, P = CppStringAttribute> = RefMut<'a, P>;

impl CppStringAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &str) -> CppStringAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(value);

            sys::Imf_CppStringAttribute_from_value(&mut inner, s.0)
                .into_result()
                .unwrap();
        }

        CppStringAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> &str {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_CppStringAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            let mut cptr = std::ptr::null();
            sys::std_string_c_str(ptr, &mut cptr);
            CStr::from_ptr(cptr).to_str().unwrap()
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_CppStringAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for CppStringAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_CppStringAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// LineOrderAttribute
#[repr(transparent)]
pub struct LineOrderAttribute(pub(crate) *mut sys::Imf_LineOrderAttribute_t);

unsafe impl OpaquePtr for LineOrderAttribute {
    type SysPointee = sys::Imf_LineOrderAttribute_t;
    type Pointee = LineOrderAttribute;
}

pub type LineOrderAttributeRef<'a, P = LineOrderAttribute> = Ref<'a, P>;
pub type LineOrderAttributeRefMut<'a, P = LineOrderAttribute> = RefMut<'a, P>;

impl LineOrderAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &LineOrder) -> LineOrderAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_LineOrderAttribute_from_value(
                &mut inner,
                value as *const LineOrder as *const sys::Imf_LineOrder,
            )
            .into_result()
            .unwrap();
        }

        LineOrderAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> LineOrder {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_LineOrderAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *(ptr as *const LineOrder)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut LineOrder {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_LineOrderAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut LineOrder)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_LineOrderAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for LineOrderAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_LineOrderAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// M33fAttribute
#[repr(transparent)]
pub struct M33fAttribute(pub(crate) *mut sys::Imf_M33fAttribute_t);

unsafe impl OpaquePtr for M33fAttribute {
    type SysPointee = sys::Imf_M33fAttribute_t;
    type Pointee = M33fAttribute;
}

pub type M33fAttributeRef<'a, P = M33fAttribute> = Ref<'a, P>;
pub type M33fAttributeRefMut<'a, P = M33fAttribute> = RefMut<'a, P>;

impl M33fAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> M33fAttribute
    where
        T: Matrix33<f32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_M33fAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_M33f_t,
            )
            .into_result()
            .unwrap();
        }

        M33fAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Matrix33<f32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_M33fAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_M33f_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Matrix33<f32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_M33fAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_M33f_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_M33fAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

// ----------------------------------------------------------------------------
// M33dAttribute
#[repr(transparent)]
pub struct M33dAttribute(pub(crate) *mut sys::Imf_M33dAttribute_t);

unsafe impl OpaquePtr for M33dAttribute {
    type SysPointee = sys::Imf_M33dAttribute_t;
    type Pointee = M33dAttribute;
}

pub type M33dAttributeRef<'a, P = M33dAttribute> = Ref<'a, P>;
pub type M33dAttributeRefMut<'a, P = M33dAttribute> = RefMut<'a, P>;

impl M33dAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> M33dAttribute
    where
        T: Matrix33<f64>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_M33dAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_M33d_t,
            )
            .into_result()
            .unwrap();
        }

        M33dAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Matrix33<f64>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_M33dAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_M33d_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Matrix33<f64>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_M33dAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_M33d_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_M33dAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for M33dAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_M33dAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// M44fAttribute
#[repr(transparent)]
pub struct M44fAttribute(pub(crate) *mut sys::Imf_M44fAttribute_t);

unsafe impl OpaquePtr for M44fAttribute {
    type SysPointee = sys::Imf_M44fAttribute_t;
    type Pointee = M44fAttribute;
}

pub type M44fAttributeRef<'a, P = M44fAttribute> = Ref<'a, P>;
pub type M44fAttributeRefMut<'a, P = M44fAttribute> = RefMut<'a, P>;

impl M44fAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> M44fAttribute
    where
        T: Matrix44<f32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_M44fAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_M44f_t,
            )
            .into_result()
            .unwrap();
        }

        M44fAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Matrix44<f32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_M44fAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_M44f_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Matrix44<f32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_M44fAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_M44f_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_M44fAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for M44fAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_M44fAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// M44dAttribute
#[repr(transparent)]
pub struct M44dAttribute(pub(crate) *mut sys::Imf_M44dAttribute_t);

unsafe impl OpaquePtr for M44dAttribute {
    type SysPointee = sys::Imf_M44dAttribute_t;
    type Pointee = M44dAttribute;
}

pub type M44dAttributeRef<'a, P = M44dAttribute> = Ref<'a, P>;
pub type M44dAttributeRefMut<'a, P = M44dAttribute> = RefMut<'a, P>;

impl M44dAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> M44dAttribute
    where
        T: Matrix44<f64>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_M44dAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_M44d_t,
            )
            .into_result()
            .unwrap();
        }

        M44dAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Matrix44<f64>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_M44dAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_M44d_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Matrix44<f64>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_M44dAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_M44d_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_M44dAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for M44dAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_M44dAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// PreviewImageAttribute
#[repr(transparent)]
pub struct PreviewImageAttribute(
    pub(crate) *mut sys::Imf_PreviewImageAttribute_t,
);

unsafe impl OpaquePtr for PreviewImageAttribute {
    type SysPointee = sys::Imf_PreviewImageAttribute_t;
    type Pointee = PreviewImageAttribute;
}

pub type PreviewImageAttributeRef<'a, P = PreviewImageAttribute> = Ref<'a, P>;
pub type PreviewImageAttributeRefMut<'a, P = PreviewImageAttribute> =
    RefMut<'a, P>;

impl PreviewImageAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &PreviewImage) -> PreviewImageAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_PreviewImageAttribute_from_value(&mut inner, value.0)
                .into_result()
                .unwrap();
        }

        PreviewImageAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> PreviewImageRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_PreviewImageAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            PreviewImageRef::new(ptr)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> PreviewImageRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_PreviewImageAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            PreviewImageRefMut::new(ptr)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_PreviewImageAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for PreviewImageAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_PreviewImageAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// TileDescriptionAttribute
#[repr(transparent)]
pub struct TileDescriptionAttribute(
    pub(crate) *mut sys::Imf_TileDescriptionAttribute_t,
);

unsafe impl OpaquePtr for TileDescriptionAttribute {
    type SysPointee = sys::Imf_TileDescriptionAttribute_t;
    type Pointee = TileDescriptionAttribute;
}

pub type TileDescriptionAttributeRef<'a, P = TileDescriptionAttribute> =
    Ref<'a, P>;
pub type TileDescriptionAttributeRefMut<'a, P = TileDescriptionAttribute> =
    RefMut<'a, P>;

impl TileDescriptionAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value(value: &TileDescription) -> TileDescriptionAttribute {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_TileDescriptionAttribute_from_value(
                &mut inner,
                &(*value).into(),
            )
            .into_result()
            .unwrap();
        }

        TileDescriptionAttribute(inner)
    }

    /// Access to the contained value
    pub fn value(&self) -> &TileDescription {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_TileDescriptionAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const TileDescription)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut(&mut self) -> &mut TileDescription {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_TileDescriptionAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut TileDescription)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_TileDescriptionAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for TileDescriptionAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_TileDescriptionAttribute_t
            as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// V2iAttribute
#[repr(transparent)]
pub struct V2iAttribute(pub(crate) *mut sys::Imf_V2iAttribute_t);

unsafe impl OpaquePtr for V2iAttribute {
    type SysPointee = sys::Imf_V2iAttribute_t;
    type Pointee = V2iAttribute;
}

pub type V2iAttributeRef<'a, P = V2iAttribute> = Ref<'a, P>;
pub type V2iAttributeRefMut<'a, P = V2iAttribute> = RefMut<'a, P>;

impl V2iAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> V2iAttribute
    where
        T: Vec2<i32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_V2iAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_V2i_t,
            )
            .into_result()
            .unwrap();
        }

        V2iAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Vec2<i32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_V2iAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_V2i_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Vec2<i32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_V2iAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_V2i_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_V2iAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for V2iAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_V2iAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// V2fAttribute
#[repr(transparent)]
pub struct V2fAttribute(pub(crate) *mut sys::Imf_V2fAttribute_t);

unsafe impl OpaquePtr for V2fAttribute {
    type SysPointee = sys::Imf_V2fAttribute_t;
    type Pointee = V2fAttribute;
}

pub type V2fAttributeRef<'a, P = V2fAttribute> = Ref<'a, P>;
pub type V2fAttributeRefMut<'a, P = V2fAttribute> = RefMut<'a, P>;

impl V2fAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> V2fAttribute
    where
        T: Vec2<f32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_V2fAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_V2f_t,
            )
            .into_result()
            .unwrap();
        }

        V2fAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Vec2<f32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_V2fAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_V2f_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Vec2<f32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_V2fAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_V2f_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_V2fAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for V2fAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_V2fAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// V2dAttribute
#[repr(transparent)]
pub struct V2dAttribute(pub(crate) *mut sys::Imf_V2dAttribute_t);

unsafe impl OpaquePtr for V2dAttribute {
    type SysPointee = sys::Imf_V2dAttribute_t;
    type Pointee = V2dAttribute;
}

pub type V2dAttributeRef<'a, P = V2dAttribute> = Ref<'a, P>;
pub type V2dAttributeRefMut<'a, P = V2dAttribute> = RefMut<'a, P>;

impl V2dAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> V2dAttribute
    where
        T: Vec2<f64>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_V2dAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_V2d_t,
            )
            .into_result()
            .unwrap();
        }

        V2dAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Vec2<f64>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_V2dAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_V2d_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Vec2<f64>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_V2dAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_V2d_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_V2dAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for V2dAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_V2dAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// V3iAttribute
#[repr(transparent)]
pub struct V3iAttribute(pub(crate) *mut sys::Imf_V3iAttribute_t);

unsafe impl OpaquePtr for V3iAttribute {
    type SysPointee = sys::Imf_V3iAttribute_t;
    type Pointee = V3iAttribute;
}

pub type V3iAttributeRef<'a, P = V3iAttribute> = Ref<'a, P>;
pub type V3iAttributeRefMut<'a, P = V3iAttribute> = RefMut<'a, P>;

impl V3iAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> V3iAttribute
    where
        T: Vec3<i32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_V3iAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_V3i_t,
            )
            .into_result()
            .unwrap();
        }

        V3iAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Vec3<i32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_V3iAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_V3i_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Vec3<i32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_V3iAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_V3i_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_V3iAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for V3iAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_V3iAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// V3fAttribute
#[repr(transparent)]
pub struct V3fAttribute(pub(crate) *mut sys::Imf_V3fAttribute_t);

unsafe impl OpaquePtr for V3fAttribute {
    type SysPointee = sys::Imf_V3fAttribute_t;
    type Pointee = V3fAttribute;
}

pub type V3fAttributeRef<'a, P = V3fAttribute> = Ref<'a, P>;
pub type V3fAttributeRefMut<'a, P = V3fAttribute> = RefMut<'a, P>;

impl V3fAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> V3fAttribute
    where
        T: Vec3<f32>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_V3fAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_V3f_t,
            )
            .into_result()
            .unwrap();
        }

        V3fAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Vec3<f32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_V3fAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_V3f_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Vec3<f32>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_V3fAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_V3f_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_V3fAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for V3fAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_V3fAttribute_t as *const sys::Imf_Attribute_t
    }
}

// ----------------------------------------------------------------------------
// V3dAttribute
#[repr(transparent)]
pub struct V3dAttribute(pub(crate) *mut sys::Imf_V3dAttribute_t);

unsafe impl OpaquePtr for V3dAttribute {
    type SysPointee = sys::Imf_V3dAttribute_t;
    type Pointee = V3dAttribute;
}

pub type V3dAttributeRef<'a, P = V3dAttribute> = Ref<'a, P>;
pub type V3dAttributeRefMut<'a, P = V3dAttribute> = RefMut<'a, P>;

impl V3dAttribute {
    /// Create a new attribute wrapping the given value
    pub fn from_value<T>(value: &T) -> V3dAttribute
    where
        T: Vec3<f64>,
    {
        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_V3dAttribute_from_value(
                &mut inner,
                value as *const T as *const sys::Imath_V3d_t,
            )
            .into_result()
            .unwrap();
        }

        V3dAttribute(inner)
    }

    /// Access to the contained value
    pub fn value<T>(&self) -> &T
    where
        T: Vec3<f64>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_V3dAttribute_value_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_V3d_t as *const T)
        }
    }

    /// Mutable access to the contained value
    pub fn value_mut<T>(&mut self) -> &mut T
    where
        T: Vec3<f64>,
    {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_V3dAttribute_value(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_V3d_t as *mut T)
        }
    }

    /// Get this attribute's type name
    pub fn type_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_V3dAttribute_typeName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }
}

impl TypedAttribute for V3dAttribute {
    fn as_attribute_ptr(&self) -> *const sys::Imf_Attribute_t {
        self.0 as *const sys::Imf_V3dAttribute_t as *const sys::Imf_Attribute_t
    }
}
