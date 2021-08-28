use openexr_sys as sys;
pub use sys::Imf_Channel_t as Channel;

use crate::core::{
    cppstd::CppString,
    refptr::{OpaquePtr, Ref, RefMut},
};

use std::ffi::{CStr, CString};
use std::marker::PhantomData;

#[repr(transparent)]
pub struct ChannelList(pub(crate) *mut sys::Imf_ChannelList_t);

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct ChannelListConstIterator(
    pub(crate) sys::Imf_ChannelList_ConstIterator_t,
);

// #[repr(transparent)]
// pub(crate) struct ChannelListIterator(
//     pub(crate) sys::Imf_ChannelList_Iterator_t,
// );

pub const CHANNEL_HALF: Channel = Channel {
    type_: sys::Imf_PixelType_HALF,
    x_sampling: 1,
    y_sampling: 1,
    p_linear: true,
};
pub const CHANNEL_FLOAT: Channel = Channel {
    type_: sys::Imf_PixelType_FLOAT,
    x_sampling: 1,
    y_sampling: 1,
    p_linear: true,
};
pub const CHANNEL_UINT: Channel = Channel {
    type_: sys::Imf_PixelType_UINT,
    x_sampling: 1,
    y_sampling: 1,
    p_linear: true,
};

unsafe impl OpaquePtr for ChannelList {
    type SysPointee = sys::Imf_ChannelList_t;
    type Pointee = ChannelList;
}

pub type ChannelListRef<'a, P = ChannelList> = Ref<'a, P>;
pub type ChannelListRefMut<'a, P = ChannelList> = RefMut<'a, P>;

impl ChannelList {
    /// Create a Default channel list
    ///
    pub fn new() -> ChannelList {
        ChannelList::default()
    }

    /// Insert a channel
    ///
    pub fn insert(&mut self, name: &str, channel: &Channel) {
        unsafe {
            let cname = CString::new(name).expect("Inner NUL bytes in name");
            sys::Imf_ChannelList_insert(self.0, cname.as_ptr(), channel)
                .into_result()
                .unwrap();
        }
    }

    /// Get a reference to a channel by name.
    ///
    /// # Returns
    /// * `Some(&Channel)` - if the channel called `name` exists
    /// * `None` - otherwise
    ///
    pub fn get(&self, name: &str) -> Option<&Channel> {
        unsafe {
            let cname = CString::new(name).expect("Inner NUL bytes in name");
            let mut ptr = std::ptr::null();
            sys::Imf_ChannelList_findChannel_const(
                self.0,
                &mut ptr,
                cname.as_ptr(),
            )
            .into_result()
            .unwrap();

            if ptr.is_null() {
                None
            } else {
                Some(&*ptr)
            }
        }
    }

    /// Get a mutable reference to a channel by name.
    ///
    /// # Returns
    /// * `Some(&Channel)` - if the channel called `name` exists
    /// * `None` - otherwise
    ///
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Channel> {
        unsafe {
            let cname = CString::new(name).expect("Inner NUL bytes in name");
            let mut ptr = std::ptr::null_mut();

            sys::Imf_ChannelList_findChannel(self.0, &mut ptr, cname.as_ptr())
                .into_result()
                .unwrap();

            if ptr.is_null() {
                None
            } else {
                Some(&mut *ptr)
            }
        }
    }

    /// Get an iterator over the channels in the channel list
    ///
    pub fn iter(&self) -> ChannelListIter {
        unsafe {
            let mut ptr = sys::Imf_ChannelList_ConstIterator_t::default();
            sys::Imf_ChannelList_begin_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            let ptr = ChannelListConstIterator(ptr);

            let mut end = sys::Imf_ChannelList_ConstIterator_t::default();
            sys::Imf_ChannelList_end_const(self.0, &mut end)
                .into_result()
                .unwrap();
            let end = ChannelListConstIterator(end);

            ChannelListIter {
                ptr,
                end,
                _p: PhantomData,
            }
        }
    }
}

impl ChannelList {
    //! # Layers
    //! In an image file with many channels it is sometimes useful to
    //! group the channels into "layers", that is, into sets of channels
    //! that logically belong together.  Grouping channels into layers
    //! is done using a naming convention:  channel C in layer L is
    //! called "L.C".
    //!
    //! For example, a computer graphic image may contain separate
    //! R, G and B channels for light that originated at each of
    //! several different virtual light sources.  The channels in
    //! this image might be called "light1.R", "light1.G", "light1.B",
    //! "light2.R", "light2.G", "light2.B", etc.
    //!
    //! Note that this naming convention allows layers to be nested;
    //! for example, "light1.specular.R" identifies the "R" channel
    //! in the "specular" sub-layer of layer "light1".
    //!
    //! Channel names that don't contain a "." or that contain a
    //! "." only at the beginning or at the end are not considered
    //! to be part of any layer.

    /// Get the set of layers in the file.
    ///
    ///
    pub fn layers(&self) -> Vec<String> {
        unsafe {
            // FIXME: dealing with STL at the boundary is pretty gnarly.
            // Should probably provide a high-level pub(crate) mod for these
            let mut set = std::ptr::null_mut();
            sys::std_set_string_ctor(&mut set);
            sys::Imf_ChannelList_layers(self.0, set);

            let mut ptr = sys::std_set_string_iterator_t::default();
            sys::std_set_string_cbegin(set, &mut ptr)
                .into_result()
                .unwrap();
            let mut end = sys::std_set_string_iterator_t::default();
            sys::std_set_string_cend(set, &mut end)
                .into_result()
                .unwrap();

            let mut size = 0;
            sys::std_set_string_size(set, &mut size)
                .into_result()
                .unwrap();

            let mut result = Vec::with_capacity(size as usize);

            loop {
                let mut eq = false;
                sys::std_set_string_const_iterator_eq(&mut eq, &ptr, &end);
                if eq {
                    break;
                }

                let mut str_ptr = std::ptr::null();
                sys::std_set_string_iterator_deref(&ptr, &mut str_ptr)
                    .into_result()
                    .unwrap();

                let mut char_ptr = std::ptr::null();
                sys::std_string_c_str(str_ptr, &mut char_ptr)
                    .into_result()
                    .unwrap();

                result.push(
                    CStr::from_ptr(char_ptr).to_string_lossy().to_string(),
                );

                let mut dummy = std::ptr::null_mut();
                sys::std_set_string_iterator_inc(&mut ptr, &mut dummy)
                    .into_result()
                    .unwrap();
            }

            result
        }
    }

    /// Get an iterator over the channels belonging to a particular layer
    ///
    pub fn channels_in_layer(&self, layer: &str) -> ChannelListIter {
        let mut ptr = sys::Imf_ChannelList_ConstIterator_t::default();
        let mut end = sys::Imf_ChannelList_ConstIterator_t::default();
        unsafe {
            let s = CppString::new(layer);
            sys::Imf_ChannelList_channelsInLayer_const(
                self.0, s.0, &mut ptr, &mut end,
            )
            .into_result()
            .unwrap();

            ChannelListIter {
                ptr: ChannelListConstIterator(ptr),
                end: ChannelListConstIterator(end),
                _p: PhantomData,
            }
        }
    }

    /// Get an iterator over all channels whose name starts with prefix `prefix`
    ///
    pub fn channels_with_prefix(&self, prefix: &str) -> ChannelListIter {
        let mut ptr = sys::Imf_ChannelList_ConstIterator_t::default();
        let mut end = sys::Imf_ChannelList_ConstIterator_t::default();
        unsafe {
            let cprefix = CString::new(prefix).expect("NUL bytes in prefix");
            sys::Imf_ChannelList_channelsWithPrefix_const(
                self.0,
                cprefix.as_ptr(),
                &mut ptr,
                &mut end,
            )
            .into_result()
            .unwrap();

            ChannelListIter {
                ptr: ChannelListConstIterator(ptr),
                end: ChannelListConstIterator(end),
                _p: PhantomData,
            }
        }
    }
}

impl Default for ChannelList {
    fn default() -> ChannelList {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_ChannelList_ctor(&mut ptr);
            ChannelList(ptr)
        }
    }
}

impl PartialEq for ChannelList {
    fn eq(&self, rhs: &ChannelList) -> bool {
        unsafe {
            let mut result = false;
            sys::Imf_ChannelList__eq(self.0, &mut result, rhs.0)
                .into_result()
                .unwrap();

            result
        }
    }
}

pub struct ChannelListIter<'a> {
    ptr: ChannelListConstIterator,
    end: ChannelListConstIterator,
    _p: PhantomData<&'a ChannelList>,
}

impl<'a> IntoIterator for &'a ChannelList {
    type Item = (&'a str, &'a Channel);
    type IntoIter = ChannelListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for ChannelListIter<'a> {
    type Item = (&'a str, &'a Channel);

    fn next(&mut self) -> Option<(&'a str, &'a Channel)> {
        let ptr_curr = self.ptr.clone();
        let mut ptr_next = self.ptr.clone();
        unsafe {
            let mut dummy = std::ptr::null_mut();
            sys::Imf_ChannelList_ConstIterator__op_inc(
                &mut ptr_next.0,
                &mut dummy,
            )
            .into_result()
            .unwrap();
        }

        if ptr_curr == self.end {
            None
        } else {
            self.ptr = ptr_next;
            unsafe {
                let mut nameptr = std::ptr::null();
                sys::Imf_ChannelList_ConstIterator_name(
                    &ptr_curr.0,
                    &mut nameptr,
                )
                .into_result()
                .unwrap();

                if nameptr.is_null() {
                    panic!("ChannelList::ConstIterator::name() returned NULL");
                }

                let mut chanptr = std::ptr::null();
                sys::Imf_ChannelList_ConstIterator_channel(
                    &ptr_curr.0,
                    &mut chanptr,
                )
                .into_result()
                .unwrap();

                Some((
                    CStr::from_ptr(nameptr)
                        .to_str()
                        .expect("NUL bytes in channel name"),
                    &*chanptr,
                ))
            }
        }
    }
}

impl PartialEq for ChannelListConstIterator {
    fn eq(&self, rhs: &ChannelListConstIterator) -> bool {
        unsafe {
            let mut result = false;
            sys::Imf_channel_list_const_iter_eq(&mut result, &self.0, &rhs.0)
                .into_result()
                .unwrap();

            result
        }
    }
}

#[cfg(test)]
#[test]
fn iter1() {
    use crate::core::PixelType;
    let mut list = ChannelList::new();
    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    list.insert("R", &channel);
    list.insert("G", &channel);
    list.insert("B", &channel);
    list.insert("A", &channel);

    assert_eq!(
        list.iter().map(|(name, _)| { name }).collect::<Vec<&str>>(),
        ["A", "B", "G", "R"]
    )
}

#[cfg(test)]
#[test]
fn iter2() {
    use crate::core::PixelType;
    let mut list = ChannelList::new();
    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    list.insert("R", &channel);
    list.insert("G", &channel);
    list.insert("B", &channel);
    list.insert("A", &channel);

    for (_name, _channel) in &list {
        // ...
    }
}

#[cfg(test)]
#[test]
fn eq() {
    use crate::core::PixelType;
    let mut list1 = ChannelList::new();
    let mut list2 = ChannelList::new();
    let mut list3 = ChannelList::new();
    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    list1.insert("R", &channel);
    list1.insert("G", &channel);
    list1.insert("B", &channel);
    list1.insert("A", &channel);

    list2.insert("R", &channel);
    list2.insert("G", &channel);
    list2.insert("B", &channel);
    list2.insert("A", &channel);

    list3.insert("R", &channel);
    list3.insert("G", &channel);
    list3.insert("B", &channel);

    assert!(list1 == list2);
    assert!(list1 != list3);
}

#[cfg(test)]
#[test]
fn layers() {
    use crate::core::PixelType;
    let mut list = ChannelList::new();
    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    list.insert("diffuse.R", &channel);
    list.insert("diffuse.G", &channel);
    list.insert("diffuse.B", &channel);

    list.insert("specular.R", &channel);
    list.insert("specular.G", &channel);
    list.insert("specular.B", &channel);

    let layers = list.layers();

    assert_eq!(layers, ["diffuse", "specular"]);
}

#[cfg(test)]
#[test]
fn channels_in_layer() {
    use crate::core::PixelType;
    let mut list = ChannelList::new();
    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    list.insert("diffuse.R", &channel);
    list.insert("diffuse.G", &channel);
    list.insert("diffuse.B", &channel);

    list.insert("specular.R", &channel);
    list.insert("specular.G", &channel);
    list.insert("specular.B", &channel);

    assert_eq!(
        list.channels_in_layer("diffuse")
            .map(|(name, _)| { name })
            .collect::<Vec<&str>>(),
        ["diffuse.B", "diffuse.G", "diffuse.R"]
    )
}

#[cfg(test)]
#[test]
fn channels_with_prefix() {
    use crate::core::PixelType;
    let mut list = ChannelList::new();
    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    list.insert("diffuse.R", &channel);
    list.insert("diffuse.G", &channel);
    list.insert("diffuse.B", &channel);

    list.insert("specular.R", &channel);
    list.insert("specular.G", &channel);
    list.insert("specular.B", &channel);

    assert_eq!(
        list.channels_with_prefix("spec")
            .map(|(name, _)| { name })
            .collect::<Vec<&str>>(),
        ["specular.B", "specular.G", "specular.R"]
    )
}

// pub struct ChannelListIterMut {
//     ptr: ChannelListIterator,
//     end: ChannelListIterator,
// }

// impl PartialEq for ChannelListIterator {
//     fn eq(&self, rhs: &ChannelListIterator) -> bool {
//         let mut a = sys::Imf_ChannelList_ConstIterator_t::default();
//         let mut b = sys::Imf_ChannelList_ConstIterator_t::default();
//         unsafe {
//             sys::Imf_ChannelList_ConstIterator_from_mut(&mut a, &self.0);
//             sys::Imf_ChannelList_ConstIterator_from_mut(&mut b, &rhs.0);
//             sys::Imf_channel_list_const_iter_eq(&a, &b)
//         }
//     }
// }
