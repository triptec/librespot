// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Add {
    // message fields
    fromIndex: ::std::option::Option<i32>,
    items: ::protobuf::RepeatedField<super::playlist4content::Item>,
    list_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    addLast: ::std::option::Option<bool>,
    addFirst: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Add {}

impl Add {
    pub fn new() -> Add {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Add {
        static mut instance: ::protobuf::lazy::Lazy<Add> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Add,
        };
        unsafe {
            instance.get(Add::new)
        }
    }

    // optional int32 fromIndex = 1;

    pub fn clear_fromIndex(&mut self) {
        self.fromIndex = ::std::option::Option::None;
    }

    pub fn has_fromIndex(&self) -> bool {
        self.fromIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromIndex(&mut self, v: i32) {
        self.fromIndex = ::std::option::Option::Some(v);
    }

    pub fn get_fromIndex(&self) -> i32 {
        self.fromIndex.unwrap_or(0)
    }

    fn get_fromIndex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.fromIndex
    }

    fn mut_fromIndex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.fromIndex
    }

    // repeated .Item items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<super::playlist4content::Item>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4content::Item> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<super::playlist4content::Item> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[super::playlist4content::Item] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<super::playlist4content::Item> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4content::Item> {
        &mut self.items
    }

    // optional .ListChecksum list_checksum = 3;

    pub fn clear_list_checksum(&mut self) {
        self.list_checksum.clear();
    }

    pub fn has_list_checksum(&self) -> bool {
        self.list_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.list_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.list_checksum.is_none() {
            self.list_checksum.set_default();
        };
        self.list_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.list_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_list_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.list_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_list_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.list_checksum
    }

    fn mut_list_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.list_checksum
    }

    // optional bool addLast = 4;

    pub fn clear_addLast(&mut self) {
        self.addLast = ::std::option::Option::None;
    }

    pub fn has_addLast(&self) -> bool {
        self.addLast.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addLast(&mut self, v: bool) {
        self.addLast = ::std::option::Option::Some(v);
    }

    pub fn get_addLast(&self) -> bool {
        self.addLast.unwrap_or(false)
    }

    fn get_addLast_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.addLast
    }

    fn mut_addLast_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.addLast
    }

    // optional bool addFirst = 5;

    pub fn clear_addFirst(&mut self) {
        self.addFirst = ::std::option::Option::None;
    }

    pub fn has_addFirst(&self) -> bool {
        self.addFirst.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addFirst(&mut self, v: bool) {
        self.addFirst = ::std::option::Option::Some(v);
    }

    pub fn get_addFirst(&self) -> bool {
        self.addFirst.unwrap_or(false)
    }

    fn get_addFirst_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.addFirst
    }

    fn mut_addFirst_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.addFirst
    }
}

impl ::protobuf::Message for Add {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.fromIndex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_checksum)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.addLast = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.addFirst = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.fromIndex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.addLast {
            my_size += 2;
        };
        if let Some(v) = self.addFirst {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fromIndex {
            os.write_int32(1, v)?;
        };
        for v in &self.items {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.addLast {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.addFirst {
            os.write_bool(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Add {
    fn new() -> Add {
        Add::new()
    }

    fn descriptor_static(_: ::std::option::Option<Add>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fromIndex",
                    Add::get_fromIndex_for_reflect,
                    Add::mut_fromIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4content::Item>>(
                    "items",
                    Add::get_items_for_reflect,
                    Add::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "list_checksum",
                    Add::get_list_checksum_for_reflect,
                    Add::mut_list_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "addLast",
                    Add::get_addLast_for_reflect,
                    Add::mut_addLast_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "addFirst",
                    Add::get_addFirst_for_reflect,
                    Add::mut_addFirst_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Add>(
                    "Add",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Add {
    fn clear(&mut self) {
        self.clear_fromIndex();
        self.clear_items();
        self.clear_list_checksum();
        self.clear_addLast();
        self.clear_addFirst();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Add {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Add {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Rem {
    // message fields
    fromIndex: ::std::option::Option<i32>,
    length: ::std::option::Option<i32>,
    items: ::protobuf::RepeatedField<super::playlist4content::Item>,
    list_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    items_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    uris_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    itemsAsKey: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Rem {}

impl Rem {
    pub fn new() -> Rem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Rem {
        static mut instance: ::protobuf::lazy::Lazy<Rem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Rem,
        };
        unsafe {
            instance.get(Rem::new)
        }
    }

    // optional int32 fromIndex = 1;

    pub fn clear_fromIndex(&mut self) {
        self.fromIndex = ::std::option::Option::None;
    }

    pub fn has_fromIndex(&self) -> bool {
        self.fromIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromIndex(&mut self, v: i32) {
        self.fromIndex = ::std::option::Option::Some(v);
    }

    pub fn get_fromIndex(&self) -> i32 {
        self.fromIndex.unwrap_or(0)
    }

    fn get_fromIndex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.fromIndex
    }

    fn mut_fromIndex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.fromIndex
    }

    // optional int32 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: i32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> i32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.length
    }

    // repeated .Item items = 3;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<super::playlist4content::Item>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4content::Item> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<super::playlist4content::Item> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[super::playlist4content::Item] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<super::playlist4content::Item> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4content::Item> {
        &mut self.items
    }

    // optional .ListChecksum list_checksum = 4;

    pub fn clear_list_checksum(&mut self) {
        self.list_checksum.clear();
    }

    pub fn has_list_checksum(&self) -> bool {
        self.list_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.list_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.list_checksum.is_none() {
            self.list_checksum.set_default();
        };
        self.list_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.list_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_list_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.list_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_list_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.list_checksum
    }

    fn mut_list_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.list_checksum
    }

    // optional .ListChecksum items_checksum = 5;

    pub fn clear_items_checksum(&mut self) {
        self.items_checksum.clear();
    }

    pub fn has_items_checksum(&self) -> bool {
        self.items_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_items_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.items_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_items_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.items_checksum.is_none() {
            self.items_checksum.set_default();
        };
        self.items_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_items_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.items_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_items_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.items_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_items_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.items_checksum
    }

    fn mut_items_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.items_checksum
    }

    // optional .ListChecksum uris_checksum = 6;

    pub fn clear_uris_checksum(&mut self) {
        self.uris_checksum.clear();
    }

    pub fn has_uris_checksum(&self) -> bool {
        self.uris_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uris_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.uris_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uris_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.uris_checksum.is_none() {
            self.uris_checksum.set_default();
        };
        self.uris_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_uris_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.uris_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_uris_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.uris_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_uris_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.uris_checksum
    }

    fn mut_uris_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.uris_checksum
    }

    // optional bool itemsAsKey = 7;

    pub fn clear_itemsAsKey(&mut self) {
        self.itemsAsKey = ::std::option::Option::None;
    }

    pub fn has_itemsAsKey(&self) -> bool {
        self.itemsAsKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemsAsKey(&mut self, v: bool) {
        self.itemsAsKey = ::std::option::Option::Some(v);
    }

    pub fn get_itemsAsKey(&self) -> bool {
        self.itemsAsKey.unwrap_or(false)
    }

    fn get_itemsAsKey_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.itemsAsKey
    }

    fn mut_itemsAsKey_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.itemsAsKey
    }
}

impl ::protobuf::Message for Rem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.fromIndex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_checksum)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.items_checksum)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.uris_checksum)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.itemsAsKey = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.fromIndex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.items_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.uris_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.itemsAsKey {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fromIndex {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.length {
            os.write_int32(2, v)?;
        };
        for v in &self.items {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.items_checksum.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.uris_checksum.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.itemsAsKey {
            os.write_bool(7, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Rem {
    fn new() -> Rem {
        Rem::new()
    }

    fn descriptor_static(_: ::std::option::Option<Rem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fromIndex",
                    Rem::get_fromIndex_for_reflect,
                    Rem::mut_fromIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    Rem::get_length_for_reflect,
                    Rem::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4content::Item>>(
                    "items",
                    Rem::get_items_for_reflect,
                    Rem::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "list_checksum",
                    Rem::get_list_checksum_for_reflect,
                    Rem::mut_list_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "items_checksum",
                    Rem::get_items_checksum_for_reflect,
                    Rem::mut_items_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "uris_checksum",
                    Rem::get_uris_checksum_for_reflect,
                    Rem::mut_uris_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "itemsAsKey",
                    Rem::get_itemsAsKey_for_reflect,
                    Rem::mut_itemsAsKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Rem>(
                    "Rem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Rem {
    fn clear(&mut self) {
        self.clear_fromIndex();
        self.clear_length();
        self.clear_items();
        self.clear_list_checksum();
        self.clear_items_checksum();
        self.clear_uris_checksum();
        self.clear_itemsAsKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Rem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Rem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mov {
    // message fields
    fromIndex: ::std::option::Option<i32>,
    length: ::std::option::Option<i32>,
    toIndex: ::std::option::Option<i32>,
    list_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    items_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    uris_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mov {}

impl Mov {
    pub fn new() -> Mov {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mov {
        static mut instance: ::protobuf::lazy::Lazy<Mov> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mov,
        };
        unsafe {
            instance.get(Mov::new)
        }
    }

    // optional int32 fromIndex = 1;

    pub fn clear_fromIndex(&mut self) {
        self.fromIndex = ::std::option::Option::None;
    }

    pub fn has_fromIndex(&self) -> bool {
        self.fromIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromIndex(&mut self, v: i32) {
        self.fromIndex = ::std::option::Option::Some(v);
    }

    pub fn get_fromIndex(&self) -> i32 {
        self.fromIndex.unwrap_or(0)
    }

    fn get_fromIndex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.fromIndex
    }

    fn mut_fromIndex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.fromIndex
    }

    // optional int32 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: i32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> i32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.length
    }

    // optional int32 toIndex = 3;

    pub fn clear_toIndex(&mut self) {
        self.toIndex = ::std::option::Option::None;
    }

    pub fn has_toIndex(&self) -> bool {
        self.toIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_toIndex(&mut self, v: i32) {
        self.toIndex = ::std::option::Option::Some(v);
    }

    pub fn get_toIndex(&self) -> i32 {
        self.toIndex.unwrap_or(0)
    }

    fn get_toIndex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.toIndex
    }

    fn mut_toIndex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.toIndex
    }

    // optional .ListChecksum list_checksum = 4;

    pub fn clear_list_checksum(&mut self) {
        self.list_checksum.clear();
    }

    pub fn has_list_checksum(&self) -> bool {
        self.list_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.list_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.list_checksum.is_none() {
            self.list_checksum.set_default();
        };
        self.list_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.list_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_list_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.list_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_list_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.list_checksum
    }

    fn mut_list_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.list_checksum
    }

    // optional .ListChecksum items_checksum = 5;

    pub fn clear_items_checksum(&mut self) {
        self.items_checksum.clear();
    }

    pub fn has_items_checksum(&self) -> bool {
        self.items_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_items_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.items_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_items_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.items_checksum.is_none() {
            self.items_checksum.set_default();
        };
        self.items_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_items_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.items_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_items_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.items_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_items_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.items_checksum
    }

    fn mut_items_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.items_checksum
    }

    // optional .ListChecksum uris_checksum = 6;

    pub fn clear_uris_checksum(&mut self) {
        self.uris_checksum.clear();
    }

    pub fn has_uris_checksum(&self) -> bool {
        self.uris_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uris_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.uris_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uris_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.uris_checksum.is_none() {
            self.uris_checksum.set_default();
        };
        self.uris_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_uris_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.uris_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_uris_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.uris_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_uris_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.uris_checksum
    }

    fn mut_uris_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.uris_checksum
    }
}

impl ::protobuf::Message for Mov {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.fromIndex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.toIndex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_checksum)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.items_checksum)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.uris_checksum)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.fromIndex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.toIndex {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.list_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.items_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.uris_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fromIndex {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.length {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.toIndex {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.items_checksum.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.uris_checksum.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mov {
    fn new() -> Mov {
        Mov::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mov>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fromIndex",
                    Mov::get_fromIndex_for_reflect,
                    Mov::mut_fromIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    Mov::get_length_for_reflect,
                    Mov::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "toIndex",
                    Mov::get_toIndex_for_reflect,
                    Mov::mut_toIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "list_checksum",
                    Mov::get_list_checksum_for_reflect,
                    Mov::mut_list_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "items_checksum",
                    Mov::get_items_checksum_for_reflect,
                    Mov::mut_items_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "uris_checksum",
                    Mov::get_uris_checksum_for_reflect,
                    Mov::mut_uris_checksum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mov>(
                    "Mov",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mov {
    fn clear(&mut self) {
        self.clear_fromIndex();
        self.clear_length();
        self.clear_toIndex();
        self.clear_list_checksum();
        self.clear_items_checksum();
        self.clear_uris_checksum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mov {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mov {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ItemAttributesPartialState {
    // message fields
    values: ::protobuf::SingularPtrField<super::playlist4meta::ItemAttributes>,
    no_value: ::std::vec::Vec<ItemAttributesPartialState_ItemAttributeKind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ItemAttributesPartialState {}

impl ItemAttributesPartialState {
    pub fn new() -> ItemAttributesPartialState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ItemAttributesPartialState {
        static mut instance: ::protobuf::lazy::Lazy<ItemAttributesPartialState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ItemAttributesPartialState,
        };
        unsafe {
            instance.get(ItemAttributesPartialState::new)
        }
    }

    // optional .ItemAttributes values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    pub fn has_values(&self) -> bool {
        self.values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: super::playlist4meta::ItemAttributes) {
        self.values = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_values(&mut self) -> &mut super::playlist4meta::ItemAttributes {
        if self.values.is_none() {
            self.values.set_default();
        };
        self.values.as_mut().unwrap()
    }

    // Take field
    pub fn take_values(&mut self) -> super::playlist4meta::ItemAttributes {
        self.values.take().unwrap_or_else(|| super::playlist4meta::ItemAttributes::new())
    }

    pub fn get_values(&self) -> &super::playlist4meta::ItemAttributes {
        self.values.as_ref().unwrap_or_else(|| super::playlist4meta::ItemAttributes::default_instance())
    }

    fn get_values_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ItemAttributes> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ItemAttributes> {
        &mut self.values
    }

    // repeated .ItemAttributesPartialState.ItemAttributeKind no_value = 2;

    pub fn clear_no_value(&mut self) {
        self.no_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_no_value(&mut self, v: ::std::vec::Vec<ItemAttributesPartialState_ItemAttributeKind>) {
        self.no_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_no_value(&mut self) -> &mut ::std::vec::Vec<ItemAttributesPartialState_ItemAttributeKind> {
        &mut self.no_value
    }

    // Take field
    pub fn take_no_value(&mut self) -> ::std::vec::Vec<ItemAttributesPartialState_ItemAttributeKind> {
        ::std::mem::replace(&mut self.no_value, ::std::vec::Vec::new())
    }

    pub fn get_no_value(&self) -> &[ItemAttributesPartialState_ItemAttributeKind] {
        &self.no_value
    }

    fn get_no_value_for_reflect(&self) -> &::std::vec::Vec<ItemAttributesPartialState_ItemAttributeKind> {
        &self.no_value
    }

    fn mut_no_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<ItemAttributesPartialState_ItemAttributeKind> {
        &mut self.no_value
    }
}

impl ::protobuf::Message for ItemAttributesPartialState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.values)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.no_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.values.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.no_value {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.values.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.no_value {
            os.write_enum(2, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ItemAttributesPartialState {
    fn new() -> ItemAttributesPartialState {
        ItemAttributesPartialState::new()
    }

    fn descriptor_static(_: ::std::option::Option<ItemAttributesPartialState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ItemAttributes>>(
                    "values",
                    ItemAttributesPartialState::get_values_for_reflect,
                    ItemAttributesPartialState::mut_values_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ItemAttributesPartialState_ItemAttributeKind>>(
                    "no_value",
                    ItemAttributesPartialState::get_no_value_for_reflect,
                    ItemAttributesPartialState::mut_no_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ItemAttributesPartialState>(
                    "ItemAttributesPartialState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ItemAttributesPartialState {
    fn clear(&mut self) {
        self.clear_values();
        self.clear_no_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ItemAttributesPartialState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ItemAttributesPartialState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ItemAttributesPartialState_ItemAttributeKind {
    ITEM_UNKNOWN = 0,
    ITEM_ADDED_BY = 1,
    ITEM_TIMESTAMP = 2,
    ITEM_MESSAGE = 3,
    ITEM_SEEN = 4,
    ITEM_DOWNLOAD_COUNT = 5,
    ITEM_DOWNLOAD_FORMAT = 6,
    ITEM_SEVENDIGITAL_ID = 7,
    ITEM_SEVENDIGITAL_LEFT = 8,
    ITEM_SEEN_AT = 9,
    ITEM_PUBLIC = 10,
}

impl ::protobuf::ProtobufEnum for ItemAttributesPartialState_ItemAttributeKind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ItemAttributesPartialState_ItemAttributeKind> {
        match value {
            0 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_UNKNOWN),
            1 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_ADDED_BY),
            2 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_TIMESTAMP),
            3 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_MESSAGE),
            4 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_SEEN),
            5 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_DOWNLOAD_COUNT),
            6 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_DOWNLOAD_FORMAT),
            7 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_SEVENDIGITAL_ID),
            8 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_SEVENDIGITAL_LEFT),
            9 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_SEEN_AT),
            10 => ::std::option::Option::Some(ItemAttributesPartialState_ItemAttributeKind::ITEM_PUBLIC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ItemAttributesPartialState_ItemAttributeKind] = &[
            ItemAttributesPartialState_ItemAttributeKind::ITEM_UNKNOWN,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_ADDED_BY,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_TIMESTAMP,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_MESSAGE,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_SEEN,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_DOWNLOAD_COUNT,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_DOWNLOAD_FORMAT,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_SEVENDIGITAL_ID,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_SEVENDIGITAL_LEFT,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_SEEN_AT,
            ItemAttributesPartialState_ItemAttributeKind::ITEM_PUBLIC,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ItemAttributesPartialState_ItemAttributeKind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ItemAttributesPartialState_ItemAttributeKind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ItemAttributesPartialState_ItemAttributeKind {
}

impl ::protobuf::reflect::ProtobufValue for ItemAttributesPartialState_ItemAttributeKind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListAttributesPartialState {
    // message fields
    values: ::protobuf::SingularPtrField<super::playlist4meta::ListAttributes>,
    no_value: ::std::vec::Vec<ListAttributesPartialState_ListAttributeKind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListAttributesPartialState {}

impl ListAttributesPartialState {
    pub fn new() -> ListAttributesPartialState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListAttributesPartialState {
        static mut instance: ::protobuf::lazy::Lazy<ListAttributesPartialState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListAttributesPartialState,
        };
        unsafe {
            instance.get(ListAttributesPartialState::new)
        }
    }

    // optional .ListAttributes values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    pub fn has_values(&self) -> bool {
        self.values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: super::playlist4meta::ListAttributes) {
        self.values = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_values(&mut self) -> &mut super::playlist4meta::ListAttributes {
        if self.values.is_none() {
            self.values.set_default();
        };
        self.values.as_mut().unwrap()
    }

    // Take field
    pub fn take_values(&mut self) -> super::playlist4meta::ListAttributes {
        self.values.take().unwrap_or_else(|| super::playlist4meta::ListAttributes::new())
    }

    pub fn get_values(&self) -> &super::playlist4meta::ListAttributes {
        self.values.as_ref().unwrap_or_else(|| super::playlist4meta::ListAttributes::default_instance())
    }

    fn get_values_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListAttributes> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListAttributes> {
        &mut self.values
    }

    // repeated .ListAttributesPartialState.ListAttributeKind no_value = 2;

    pub fn clear_no_value(&mut self) {
        self.no_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_no_value(&mut self, v: ::std::vec::Vec<ListAttributesPartialState_ListAttributeKind>) {
        self.no_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_no_value(&mut self) -> &mut ::std::vec::Vec<ListAttributesPartialState_ListAttributeKind> {
        &mut self.no_value
    }

    // Take field
    pub fn take_no_value(&mut self) -> ::std::vec::Vec<ListAttributesPartialState_ListAttributeKind> {
        ::std::mem::replace(&mut self.no_value, ::std::vec::Vec::new())
    }

    pub fn get_no_value(&self) -> &[ListAttributesPartialState_ListAttributeKind] {
        &self.no_value
    }

    fn get_no_value_for_reflect(&self) -> &::std::vec::Vec<ListAttributesPartialState_ListAttributeKind> {
        &self.no_value
    }

    fn mut_no_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<ListAttributesPartialState_ListAttributeKind> {
        &mut self.no_value
    }
}

impl ::protobuf::Message for ListAttributesPartialState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.values)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.no_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.values.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.no_value {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.values.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.no_value {
            os.write_enum(2, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListAttributesPartialState {
    fn new() -> ListAttributesPartialState {
        ListAttributesPartialState::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListAttributesPartialState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListAttributes>>(
                    "values",
                    ListAttributesPartialState::get_values_for_reflect,
                    ListAttributesPartialState::mut_values_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ListAttributesPartialState_ListAttributeKind>>(
                    "no_value",
                    ListAttributesPartialState::get_no_value_for_reflect,
                    ListAttributesPartialState::mut_no_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListAttributesPartialState>(
                    "ListAttributesPartialState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListAttributesPartialState {
    fn clear(&mut self) {
        self.clear_values();
        self.clear_no_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListAttributesPartialState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListAttributesPartialState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ListAttributesPartialState_ListAttributeKind {
    LIST_UNKNOWN = 0,
    LIST_NAME = 1,
    LIST_DESCRIPTION = 2,
    LIST_PICTURE = 3,
    LIST_COLLABORATIVE = 4,
    LIST_PL3_VERSION = 5,
    LIST_DELETED_BY_OWNER = 6,
    LIST_RESTRICTED_COLLABORATIVE = 7,
}

impl ::protobuf::ProtobufEnum for ListAttributesPartialState_ListAttributeKind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ListAttributesPartialState_ListAttributeKind> {
        match value {
            0 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_UNKNOWN),
            1 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_NAME),
            2 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_DESCRIPTION),
            3 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_PICTURE),
            4 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_COLLABORATIVE),
            5 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_PL3_VERSION),
            6 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_DELETED_BY_OWNER),
            7 => ::std::option::Option::Some(ListAttributesPartialState_ListAttributeKind::LIST_RESTRICTED_COLLABORATIVE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ListAttributesPartialState_ListAttributeKind] = &[
            ListAttributesPartialState_ListAttributeKind::LIST_UNKNOWN,
            ListAttributesPartialState_ListAttributeKind::LIST_NAME,
            ListAttributesPartialState_ListAttributeKind::LIST_DESCRIPTION,
            ListAttributesPartialState_ListAttributeKind::LIST_PICTURE,
            ListAttributesPartialState_ListAttributeKind::LIST_COLLABORATIVE,
            ListAttributesPartialState_ListAttributeKind::LIST_PL3_VERSION,
            ListAttributesPartialState_ListAttributeKind::LIST_DELETED_BY_OWNER,
            ListAttributesPartialState_ListAttributeKind::LIST_RESTRICTED_COLLABORATIVE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ListAttributesPartialState_ListAttributeKind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ListAttributesPartialState_ListAttributeKind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ListAttributesPartialState_ListAttributeKind {
}

impl ::protobuf::reflect::ProtobufValue for ListAttributesPartialState_ListAttributeKind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateItemAttributes {
    // message fields
    index: ::std::option::Option<i32>,
    new_attributes: ::protobuf::SingularPtrField<ItemAttributesPartialState>,
    old_attributes: ::protobuf::SingularPtrField<ItemAttributesPartialState>,
    list_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    old_attributes_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateItemAttributes {}

impl UpdateItemAttributes {
    pub fn new() -> UpdateItemAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateItemAttributes {
        static mut instance: ::protobuf::lazy::Lazy<UpdateItemAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateItemAttributes,
        };
        unsafe {
            instance.get(UpdateItemAttributes::new)
        }
    }

    // optional int32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> i32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.index
    }

    // optional .ItemAttributesPartialState new_attributes = 2;

    pub fn clear_new_attributes(&mut self) {
        self.new_attributes.clear();
    }

    pub fn has_new_attributes(&self) -> bool {
        self.new_attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_attributes(&mut self, v: ItemAttributesPartialState) {
        self.new_attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_attributes(&mut self) -> &mut ItemAttributesPartialState {
        if self.new_attributes.is_none() {
            self.new_attributes.set_default();
        };
        self.new_attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_attributes(&mut self) -> ItemAttributesPartialState {
        self.new_attributes.take().unwrap_or_else(|| ItemAttributesPartialState::new())
    }

    pub fn get_new_attributes(&self) -> &ItemAttributesPartialState {
        self.new_attributes.as_ref().unwrap_or_else(|| ItemAttributesPartialState::default_instance())
    }

    fn get_new_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<ItemAttributesPartialState> {
        &self.new_attributes
    }

    fn mut_new_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ItemAttributesPartialState> {
        &mut self.new_attributes
    }

    // optional .ItemAttributesPartialState old_attributes = 3;

    pub fn clear_old_attributes(&mut self) {
        self.old_attributes.clear();
    }

    pub fn has_old_attributes(&self) -> bool {
        self.old_attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_attributes(&mut self, v: ItemAttributesPartialState) {
        self.old_attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_attributes(&mut self) -> &mut ItemAttributesPartialState {
        if self.old_attributes.is_none() {
            self.old_attributes.set_default();
        };
        self.old_attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_attributes(&mut self) -> ItemAttributesPartialState {
        self.old_attributes.take().unwrap_or_else(|| ItemAttributesPartialState::new())
    }

    pub fn get_old_attributes(&self) -> &ItemAttributesPartialState {
        self.old_attributes.as_ref().unwrap_or_else(|| ItemAttributesPartialState::default_instance())
    }

    fn get_old_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<ItemAttributesPartialState> {
        &self.old_attributes
    }

    fn mut_old_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ItemAttributesPartialState> {
        &mut self.old_attributes
    }

    // optional .ListChecksum list_checksum = 4;

    pub fn clear_list_checksum(&mut self) {
        self.list_checksum.clear();
    }

    pub fn has_list_checksum(&self) -> bool {
        self.list_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.list_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.list_checksum.is_none() {
            self.list_checksum.set_default();
        };
        self.list_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.list_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_list_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.list_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_list_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.list_checksum
    }

    fn mut_list_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.list_checksum
    }

    // optional .ListChecksum old_attributes_checksum = 5;

    pub fn clear_old_attributes_checksum(&mut self) {
        self.old_attributes_checksum.clear();
    }

    pub fn has_old_attributes_checksum(&self) -> bool {
        self.old_attributes_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_attributes_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.old_attributes_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_attributes_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.old_attributes_checksum.is_none() {
            self.old_attributes_checksum.set_default();
        };
        self.old_attributes_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_attributes_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.old_attributes_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_old_attributes_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.old_attributes_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_old_attributes_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.old_attributes_checksum
    }

    fn mut_old_attributes_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.old_attributes_checksum
    }
}

impl ::protobuf::Message for UpdateItemAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new_attributes)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_attributes)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_checksum)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_attributes_checksum)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.new_attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.old_attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.old_attributes_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.new_attributes.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.old_attributes.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.old_attributes_checksum.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateItemAttributes {
    fn new() -> UpdateItemAttributes {
        UpdateItemAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateItemAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "index",
                    UpdateItemAttributes::get_index_for_reflect,
                    UpdateItemAttributes::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ItemAttributesPartialState>>(
                    "new_attributes",
                    UpdateItemAttributes::get_new_attributes_for_reflect,
                    UpdateItemAttributes::mut_new_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ItemAttributesPartialState>>(
                    "old_attributes",
                    UpdateItemAttributes::get_old_attributes_for_reflect,
                    UpdateItemAttributes::mut_old_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "list_checksum",
                    UpdateItemAttributes::get_list_checksum_for_reflect,
                    UpdateItemAttributes::mut_list_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "old_attributes_checksum",
                    UpdateItemAttributes::get_old_attributes_checksum_for_reflect,
                    UpdateItemAttributes::mut_old_attributes_checksum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateItemAttributes>(
                    "UpdateItemAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateItemAttributes {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_new_attributes();
        self.clear_old_attributes();
        self.clear_list_checksum();
        self.clear_old_attributes_checksum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateItemAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateItemAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateListAttributes {
    // message fields
    new_attributes: ::protobuf::SingularPtrField<ListAttributesPartialState>,
    old_attributes: ::protobuf::SingularPtrField<ListAttributesPartialState>,
    list_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    old_attributes_checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateListAttributes {}

impl UpdateListAttributes {
    pub fn new() -> UpdateListAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateListAttributes {
        static mut instance: ::protobuf::lazy::Lazy<UpdateListAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateListAttributes,
        };
        unsafe {
            instance.get(UpdateListAttributes::new)
        }
    }

    // optional .ListAttributesPartialState new_attributes = 1;

    pub fn clear_new_attributes(&mut self) {
        self.new_attributes.clear();
    }

    pub fn has_new_attributes(&self) -> bool {
        self.new_attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_attributes(&mut self, v: ListAttributesPartialState) {
        self.new_attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_attributes(&mut self) -> &mut ListAttributesPartialState {
        if self.new_attributes.is_none() {
            self.new_attributes.set_default();
        };
        self.new_attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_attributes(&mut self) -> ListAttributesPartialState {
        self.new_attributes.take().unwrap_or_else(|| ListAttributesPartialState::new())
    }

    pub fn get_new_attributes(&self) -> &ListAttributesPartialState {
        self.new_attributes.as_ref().unwrap_or_else(|| ListAttributesPartialState::default_instance())
    }

    fn get_new_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<ListAttributesPartialState> {
        &self.new_attributes
    }

    fn mut_new_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ListAttributesPartialState> {
        &mut self.new_attributes
    }

    // optional .ListAttributesPartialState old_attributes = 2;

    pub fn clear_old_attributes(&mut self) {
        self.old_attributes.clear();
    }

    pub fn has_old_attributes(&self) -> bool {
        self.old_attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_attributes(&mut self, v: ListAttributesPartialState) {
        self.old_attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_attributes(&mut self) -> &mut ListAttributesPartialState {
        if self.old_attributes.is_none() {
            self.old_attributes.set_default();
        };
        self.old_attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_attributes(&mut self) -> ListAttributesPartialState {
        self.old_attributes.take().unwrap_or_else(|| ListAttributesPartialState::new())
    }

    pub fn get_old_attributes(&self) -> &ListAttributesPartialState {
        self.old_attributes.as_ref().unwrap_or_else(|| ListAttributesPartialState::default_instance())
    }

    fn get_old_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<ListAttributesPartialState> {
        &self.old_attributes
    }

    fn mut_old_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ListAttributesPartialState> {
        &mut self.old_attributes
    }

    // optional .ListChecksum list_checksum = 3;

    pub fn clear_list_checksum(&mut self) {
        self.list_checksum.clear();
    }

    pub fn has_list_checksum(&self) -> bool {
        self.list_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.list_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.list_checksum.is_none() {
            self.list_checksum.set_default();
        };
        self.list_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.list_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_list_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.list_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_list_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.list_checksum
    }

    fn mut_list_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.list_checksum
    }

    // optional .ListChecksum old_attributes_checksum = 4;

    pub fn clear_old_attributes_checksum(&mut self) {
        self.old_attributes_checksum.clear();
    }

    pub fn has_old_attributes_checksum(&self) -> bool {
        self.old_attributes_checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_attributes_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.old_attributes_checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_attributes_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.old_attributes_checksum.is_none() {
            self.old_attributes_checksum.set_default();
        };
        self.old_attributes_checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_attributes_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.old_attributes_checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_old_attributes_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.old_attributes_checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_old_attributes_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.old_attributes_checksum
    }

    fn mut_old_attributes_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.old_attributes_checksum
    }
}

impl ::protobuf::Message for UpdateListAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new_attributes)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_attributes)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_checksum)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_attributes_checksum)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.new_attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.old_attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.old_attributes_checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_attributes.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.old_attributes.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.list_checksum.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.old_attributes_checksum.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateListAttributes {
    fn new() -> UpdateListAttributes {
        UpdateListAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateListAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListAttributesPartialState>>(
                    "new_attributes",
                    UpdateListAttributes::get_new_attributes_for_reflect,
                    UpdateListAttributes::mut_new_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListAttributesPartialState>>(
                    "old_attributes",
                    UpdateListAttributes::get_old_attributes_for_reflect,
                    UpdateListAttributes::mut_old_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "list_checksum",
                    UpdateListAttributes::get_list_checksum_for_reflect,
                    UpdateListAttributes::mut_list_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "old_attributes_checksum",
                    UpdateListAttributes::get_old_attributes_checksum_for_reflect,
                    UpdateListAttributes::mut_old_attributes_checksum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateListAttributes>(
                    "UpdateListAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateListAttributes {
    fn clear(&mut self) {
        self.clear_new_attributes();
        self.clear_old_attributes();
        self.clear_list_checksum();
        self.clear_old_attributes_checksum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateListAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateListAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Op {
    // message fields
    kind: ::std::option::Option<Op_Kind>,
    add: ::protobuf::SingularPtrField<Add>,
    rem: ::protobuf::SingularPtrField<Rem>,
    mov: ::protobuf::SingularPtrField<Mov>,
    update_item_attributes: ::protobuf::SingularPtrField<UpdateItemAttributes>,
    update_list_attributes: ::protobuf::SingularPtrField<UpdateListAttributes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Op {}

impl Op {
    pub fn new() -> Op {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Op {
        static mut instance: ::protobuf::lazy::Lazy<Op> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Op,
        };
        unsafe {
            instance.get(Op::new)
        }
    }

    // optional .Op.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: Op_Kind) {
        self.kind = ::std::option::Option::Some(v);
    }

    pub fn get_kind(&self) -> Op_Kind {
        self.kind.unwrap_or(Op_Kind::KIND_UNKNOWN)
    }

    fn get_kind_for_reflect(&self) -> &::std::option::Option<Op_Kind> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::std::option::Option<Op_Kind> {
        &mut self.kind
    }

    // optional .Add add = 2;

    pub fn clear_add(&mut self) {
        self.add.clear();
    }

    pub fn has_add(&self) -> bool {
        self.add.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add(&mut self, v: Add) {
        self.add = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add(&mut self) -> &mut Add {
        if self.add.is_none() {
            self.add.set_default();
        };
        self.add.as_mut().unwrap()
    }

    // Take field
    pub fn take_add(&mut self) -> Add {
        self.add.take().unwrap_or_else(|| Add::new())
    }

    pub fn get_add(&self) -> &Add {
        self.add.as_ref().unwrap_or_else(|| Add::default_instance())
    }

    fn get_add_for_reflect(&self) -> &::protobuf::SingularPtrField<Add> {
        &self.add
    }

    fn mut_add_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Add> {
        &mut self.add
    }

    // optional .Rem rem = 3;

    pub fn clear_rem(&mut self) {
        self.rem.clear();
    }

    pub fn has_rem(&self) -> bool {
        self.rem.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rem(&mut self, v: Rem) {
        self.rem = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rem(&mut self) -> &mut Rem {
        if self.rem.is_none() {
            self.rem.set_default();
        };
        self.rem.as_mut().unwrap()
    }

    // Take field
    pub fn take_rem(&mut self) -> Rem {
        self.rem.take().unwrap_or_else(|| Rem::new())
    }

    pub fn get_rem(&self) -> &Rem {
        self.rem.as_ref().unwrap_or_else(|| Rem::default_instance())
    }

    fn get_rem_for_reflect(&self) -> &::protobuf::SingularPtrField<Rem> {
        &self.rem
    }

    fn mut_rem_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Rem> {
        &mut self.rem
    }

    // optional .Mov mov = 4;

    pub fn clear_mov(&mut self) {
        self.mov.clear();
    }

    pub fn has_mov(&self) -> bool {
        self.mov.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mov(&mut self, v: Mov) {
        self.mov = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mov(&mut self) -> &mut Mov {
        if self.mov.is_none() {
            self.mov.set_default();
        };
        self.mov.as_mut().unwrap()
    }

    // Take field
    pub fn take_mov(&mut self) -> Mov {
        self.mov.take().unwrap_or_else(|| Mov::new())
    }

    pub fn get_mov(&self) -> &Mov {
        self.mov.as_ref().unwrap_or_else(|| Mov::default_instance())
    }

    fn get_mov_for_reflect(&self) -> &::protobuf::SingularPtrField<Mov> {
        &self.mov
    }

    fn mut_mov_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Mov> {
        &mut self.mov
    }

    // optional .UpdateItemAttributes update_item_attributes = 5;

    pub fn clear_update_item_attributes(&mut self) {
        self.update_item_attributes.clear();
    }

    pub fn has_update_item_attributes(&self) -> bool {
        self.update_item_attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_item_attributes(&mut self, v: UpdateItemAttributes) {
        self.update_item_attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_item_attributes(&mut self) -> &mut UpdateItemAttributes {
        if self.update_item_attributes.is_none() {
            self.update_item_attributes.set_default();
        };
        self.update_item_attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_item_attributes(&mut self) -> UpdateItemAttributes {
        self.update_item_attributes.take().unwrap_or_else(|| UpdateItemAttributes::new())
    }

    pub fn get_update_item_attributes(&self) -> &UpdateItemAttributes {
        self.update_item_attributes.as_ref().unwrap_or_else(|| UpdateItemAttributes::default_instance())
    }

    fn get_update_item_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<UpdateItemAttributes> {
        &self.update_item_attributes
    }

    fn mut_update_item_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UpdateItemAttributes> {
        &mut self.update_item_attributes
    }

    // optional .UpdateListAttributes update_list_attributes = 6;

    pub fn clear_update_list_attributes(&mut self) {
        self.update_list_attributes.clear();
    }

    pub fn has_update_list_attributes(&self) -> bool {
        self.update_list_attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_list_attributes(&mut self, v: UpdateListAttributes) {
        self.update_list_attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_list_attributes(&mut self) -> &mut UpdateListAttributes {
        if self.update_list_attributes.is_none() {
            self.update_list_attributes.set_default();
        };
        self.update_list_attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_list_attributes(&mut self) -> UpdateListAttributes {
        self.update_list_attributes.take().unwrap_or_else(|| UpdateListAttributes::new())
    }

    pub fn get_update_list_attributes(&self) -> &UpdateListAttributes {
        self.update_list_attributes.as_ref().unwrap_or_else(|| UpdateListAttributes::default_instance())
    }

    fn get_update_list_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<UpdateListAttributes> {
        &self.update_list_attributes
    }

    fn mut_update_list_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UpdateListAttributes> {
        &mut self.update_list_attributes
    }
}

impl ::protobuf::Message for Op {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.kind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rem)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mov)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_item_attributes)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_list_attributes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.kind {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.add.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.rem.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.mov.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_item_attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_list_attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kind {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.add.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.rem.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.mov.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_item_attributes.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_list_attributes.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Op {
    fn new() -> Op {
        Op::new()
    }

    fn descriptor_static(_: ::std::option::Option<Op>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Op_Kind>>(
                    "kind",
                    Op::get_kind_for_reflect,
                    Op::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Add>>(
                    "add",
                    Op::get_add_for_reflect,
                    Op::mut_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Rem>>(
                    "rem",
                    Op::get_rem_for_reflect,
                    Op::mut_rem_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mov>>(
                    "mov",
                    Op::get_mov_for_reflect,
                    Op::mut_mov_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UpdateItemAttributes>>(
                    "update_item_attributes",
                    Op::get_update_item_attributes_for_reflect,
                    Op::mut_update_item_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UpdateListAttributes>>(
                    "update_list_attributes",
                    Op::get_update_list_attributes_for_reflect,
                    Op::mut_update_list_attributes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Op>(
                    "Op",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Op {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_add();
        self.clear_rem();
        self.clear_mov();
        self.clear_update_item_attributes();
        self.clear_update_list_attributes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Op {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Op {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Op_Kind {
    KIND_UNKNOWN = 0,
    ADD = 2,
    REM = 3,
    MOV = 4,
    UPDATE_ITEM_ATTRIBUTES = 5,
    UPDATE_LIST_ATTRIBUTES = 6,
}

impl ::protobuf::ProtobufEnum for Op_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Op_Kind> {
        match value {
            0 => ::std::option::Option::Some(Op_Kind::KIND_UNKNOWN),
            2 => ::std::option::Option::Some(Op_Kind::ADD),
            3 => ::std::option::Option::Some(Op_Kind::REM),
            4 => ::std::option::Option::Some(Op_Kind::MOV),
            5 => ::std::option::Option::Some(Op_Kind::UPDATE_ITEM_ATTRIBUTES),
            6 => ::std::option::Option::Some(Op_Kind::UPDATE_LIST_ATTRIBUTES),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Op_Kind] = &[
            Op_Kind::KIND_UNKNOWN,
            Op_Kind::ADD,
            Op_Kind::REM,
            Op_Kind::MOV,
            Op_Kind::UPDATE_ITEM_ATTRIBUTES,
            Op_Kind::UPDATE_LIST_ATTRIBUTES,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Op_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Op_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Op_Kind {
}

impl ::protobuf::reflect::ProtobufValue for Op_Kind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpList {
    // message fields
    ops: ::protobuf::RepeatedField<Op>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpList {}

impl OpList {
    pub fn new() -> OpList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpList {
        static mut instance: ::protobuf::lazy::Lazy<OpList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpList,
        };
        unsafe {
            instance.get(OpList::new)
        }
    }

    // repeated .Op ops = 1;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::protobuf::RepeatedField<Op>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::protobuf::RepeatedField<Op> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::protobuf::RepeatedField<Op> {
        ::std::mem::replace(&mut self.ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_ops(&self) -> &[Op] {
        &self.ops
    }

    fn get_ops_for_reflect(&self) -> &::protobuf::RepeatedField<Op> {
        &self.ops
    }

    fn mut_ops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Op> {
        &mut self.ops
    }
}

impl ::protobuf::Message for OpList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ops {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpList {
    fn new() -> OpList {
        OpList::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Op>>(
                    "ops",
                    OpList::get_ops_for_reflect,
                    OpList::mut_ops_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpList>(
                    "OpList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpList {
    fn clear(&mut self) {
        self.clear_ops();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x6f, 0x70, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x6d,
    0x65, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x70, 0x6c, 0x61, 0x79, 0x6c,
    0x69, 0x73, 0x74, 0x34, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xaa, 0x01, 0x0a, 0x03, 0x41, 0x64, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x66, 0x72, 0x6f,
    0x6d, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x66, 0x72,
    0x6f, 0x6d, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1b, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69,
    0x74, 0x65, 0x6d, 0x73, 0x12, 0x32, 0x0a, 0x0d, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69,
    0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x0c, 0x6c, 0x69, 0x73, 0x74,
    0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x4c,
    0x61, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x61, 0x64, 0x64, 0x4c, 0x61,
    0x73, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x61, 0x64, 0x64, 0x46, 0x69, 0x72, 0x73, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x61, 0x64, 0x64, 0x46, 0x69, 0x72, 0x73, 0x74, 0x22, 0x96,
    0x02, 0x0a, 0x03, 0x52, 0x65, 0x6d, 0x12, 0x1c, 0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x49, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x1b, 0x0a, 0x05,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x32, 0x0a, 0x0d, 0x6c, 0x69, 0x73,
    0x74, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52,
    0x0c, 0x6c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x34, 0x0a,
    0x0e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63,
    0x6b, 0x73, 0x75, 0x6d, 0x52, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x73, 0x75, 0x6d, 0x12, 0x32, 0x0a, 0x0d, 0x75, 0x72, 0x69, 0x73, 0x5f, 0x63, 0x68, 0x65, 0x63,
    0x6b, 0x73, 0x75, 0x6d, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73,
    0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x0c, 0x75, 0x72, 0x69, 0x73, 0x43,
    0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x1e, 0x0a, 0x0a, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x41, 0x73, 0x4b, 0x65, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x74, 0x65,
    0x6d, 0x73, 0x41, 0x73, 0x4b, 0x65, 0x79, 0x22, 0xf3, 0x01, 0x0a, 0x03, 0x4d, 0x6f, 0x76, 0x12,
    0x1c, 0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x16, 0x0a,
    0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x18, 0x0a, 0x07, 0x74, 0x6f, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x74, 0x6f, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x32, 0x0a, 0x0d, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x0c, 0x6c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x73, 0x75, 0x6d, 0x12, 0x34, 0x0a, 0x0e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69,
    0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x0d, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x32, 0x0a, 0x0d, 0x75, 0x72, 0x69,
    0x73, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52,
    0x0c, 0x75, 0x72, 0x69, 0x73, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x22, 0x8b, 0x03,
    0x0a, 0x1a, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x27, 0x0a, 0x06,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x06, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x12, 0x48, 0x0a, 0x08, 0x6e, 0x6f, 0x5f, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x2d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74,
    0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x4b, 0x69, 0x6e, 0x64, 0x52, 0x07, 0x6e, 0x6f, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0xf9, 0x01, 0x0a, 0x11, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x10, 0x0a, 0x0c, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x49, 0x54, 0x45, 0x4d, 0x5f,
    0x41, 0x44, 0x44, 0x45, 0x44, 0x5f, 0x42, 0x59, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x54,
    0x45, 0x4d, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x53, 0x54, 0x41, 0x4d, 0x50, 0x10, 0x02, 0x12, 0x10,
    0x0a, 0x0c, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x10, 0x03,
    0x12, 0x0d, 0x0a, 0x09, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x53, 0x45, 0x45, 0x4e, 0x10, 0x04, 0x12,
    0x17, 0x0a, 0x13, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x44, 0x4f, 0x57, 0x4e, 0x4c, 0x4f, 0x41, 0x44,
    0x5f, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x10, 0x05, 0x12, 0x18, 0x0a, 0x14, 0x49, 0x54, 0x45, 0x4d,
    0x5f, 0x44, 0x4f, 0x57, 0x4e, 0x4c, 0x4f, 0x41, 0x44, 0x5f, 0x46, 0x4f, 0x52, 0x4d, 0x41, 0x54,
    0x10, 0x06, 0x12, 0x18, 0x0a, 0x14, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x53, 0x45, 0x56, 0x45, 0x4e,
    0x44, 0x49, 0x47, 0x49, 0x54, 0x41, 0x4c, 0x5f, 0x49, 0x44, 0x10, 0x07, 0x12, 0x1a, 0x0a, 0x16,
    0x49, 0x54, 0x45, 0x4d, 0x5f, 0x53, 0x45, 0x56, 0x45, 0x4e, 0x44, 0x49, 0x47, 0x49, 0x54, 0x41,
    0x4c, 0x5f, 0x4c, 0x45, 0x46, 0x54, 0x10, 0x08, 0x12, 0x10, 0x0a, 0x0c, 0x49, 0x54, 0x45, 0x4d,
    0x5f, 0x53, 0x45, 0x45, 0x4e, 0x5f, 0x41, 0x54, 0x10, 0x09, 0x12, 0x0f, 0x0a, 0x0b, 0x49, 0x54,
    0x45, 0x4d, 0x5f, 0x50, 0x55, 0x42, 0x4c, 0x49, 0x43, 0x10, 0x0a, 0x22, 0xda, 0x02, 0x0a, 0x1a,
    0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x27, 0x0a, 0x06, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x4c, 0x69, 0x73,
    0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x06, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x12, 0x48, 0x0a, 0x08, 0x6e, 0x6f, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x2d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x4b, 0x69, 0x6e, 0x64, 0x52, 0x07, 0x6e, 0x6f, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xc8, 0x01,
    0x0a, 0x11, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x4b,
    0x69, 0x6e, 0x64, 0x12, 0x10, 0x0a, 0x0c, 0x4c, 0x49, 0x53, 0x54, 0x5f, 0x55, 0x4e, 0x4b, 0x4e,
    0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09, 0x4c, 0x49, 0x53, 0x54, 0x5f, 0x4e, 0x41,
    0x4d, 0x45, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x4c, 0x49, 0x53, 0x54, 0x5f, 0x44, 0x45, 0x53,
    0x43, 0x52, 0x49, 0x50, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x4c, 0x49,
    0x53, 0x54, 0x5f, 0x50, 0x49, 0x43, 0x54, 0x55, 0x52, 0x45, 0x10, 0x03, 0x12, 0x16, 0x0a, 0x12,
    0x4c, 0x49, 0x53, 0x54, 0x5f, 0x43, 0x4f, 0x4c, 0x4c, 0x41, 0x42, 0x4f, 0x52, 0x41, 0x54, 0x49,
    0x56, 0x45, 0x10, 0x04, 0x12, 0x14, 0x0a, 0x10, 0x4c, 0x49, 0x53, 0x54, 0x5f, 0x50, 0x4c, 0x33,
    0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x05, 0x12, 0x19, 0x0a, 0x15, 0x4c, 0x49,
    0x53, 0x54, 0x5f, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x44, 0x5f, 0x42, 0x59, 0x5f, 0x4f, 0x57,
    0x4e, 0x45, 0x52, 0x10, 0x06, 0x12, 0x21, 0x0a, 0x1d, 0x4c, 0x49, 0x53, 0x54, 0x5f, 0x52, 0x45,
    0x53, 0x54, 0x52, 0x49, 0x43, 0x54, 0x45, 0x44, 0x5f, 0x43, 0x4f, 0x4c, 0x4c, 0x41, 0x42, 0x4f,
    0x52, 0x41, 0x54, 0x49, 0x56, 0x45, 0x10, 0x07, 0x22, 0xaf, 0x02, 0x0a, 0x14, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x42, 0x0a, 0x0e, 0x6e, 0x65, 0x77, 0x5f, 0x61,
    0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x0d, 0x6e, 0x65,
    0x77, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x42, 0x0a, 0x0e, 0x6f,
    0x6c, 0x64, 0x5f, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x73, 0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x52, 0x0d, 0x6f, 0x6c, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12,
    0x32, 0x0a, 0x0d, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x0c, 0x6c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x73, 0x75, 0x6d, 0x12, 0x45, 0x0a, 0x17, 0x6f, 0x6c, 0x64, 0x5f, 0x61, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x73, 0x75, 0x6d, 0x52, 0x15, 0x6f, 0x6c, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x73, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x22, 0x99, 0x02, 0x0a, 0x14, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x12, 0x42, 0x0a, 0x0e, 0x6e, 0x65, 0x77, 0x5f, 0x61, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x4c, 0x69,
    0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x50, 0x61, 0x72, 0x74,
    0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x0d, 0x6e, 0x65, 0x77, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x42, 0x0a, 0x0e, 0x6f, 0x6c, 0x64, 0x5f, 0x61,
    0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x0d, 0x6f, 0x6c,
    0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x32, 0x0a, 0x0d, 0x6c,
    0x69, 0x73, 0x74, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75,
    0x6d, 0x52, 0x0c, 0x6c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12,
    0x45, 0x0a, 0x17, 0x6f, 0x6c, 0x64, 0x5f, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52,
    0x15, 0x6f, 0x6c, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x43, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x22, 0xf1, 0x02, 0x0a, 0x02, 0x4f, 0x70, 0x12, 0x1c, 0x0a,
    0x04, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x08, 0x2e, 0x4f, 0x70,
    0x2e, 0x4b, 0x69, 0x6e, 0x64, 0x52, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x12, 0x16, 0x0a, 0x03, 0x61,
    0x64, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x41, 0x64, 0x64, 0x52, 0x03,
    0x61, 0x64, 0x64, 0x12, 0x16, 0x0a, 0x03, 0x72, 0x65, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x04, 0x2e, 0x52, 0x65, 0x6d, 0x52, 0x03, 0x72, 0x65, 0x6d, 0x12, 0x16, 0x0a, 0x03, 0x6d,
    0x6f, 0x76, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x4d, 0x6f, 0x76, 0x52, 0x03,
    0x6d, 0x6f, 0x76, 0x12, 0x4b, 0x0a, 0x16, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x69, 0x74,
    0x65, 0x6d, 0x5f, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x49, 0x74, 0x65, 0x6d,
    0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x14, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73,
    0x12, 0x4b, 0x0a, 0x16, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x5f,
    0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x15, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x14, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4c,
    0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x22, 0x6b, 0x0a,
    0x04, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x10, 0x0a, 0x0c, 0x4b, 0x49, 0x4e, 0x44, 0x5f, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x44, 0x44, 0x10, 0x02,
    0x12, 0x07, 0x0a, 0x03, 0x52, 0x45, 0x4d, 0x10, 0x03, 0x12, 0x07, 0x0a, 0x03, 0x4d, 0x4f, 0x56,
    0x10, 0x04, 0x12, 0x1a, 0x0a, 0x16, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x49, 0x54, 0x45,
    0x4d, 0x5f, 0x41, 0x54, 0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x45, 0x53, 0x10, 0x05, 0x12, 0x1a,
    0x0a, 0x16, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x4c, 0x49, 0x53, 0x54, 0x5f, 0x41, 0x54,
    0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x45, 0x53, 0x10, 0x06, 0x22, 0x1f, 0x0a, 0x06, 0x4f, 0x70,
    0x4c, 0x69, 0x73, 0x74, 0x12, 0x15, 0x0a, 0x03, 0x6f, 0x70, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x03, 0x2e, 0x4f, 0x70, 0x52, 0x03, 0x6f, 0x70, 0x73, 0x4a, 0xa9, 0x20, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x65, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x03, 0x07, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00,
    0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x06, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x06, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x07, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x08, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x08, 0x0d,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x1a, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x09, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x09, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09,
    0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0a, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x0a, 0x1d, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d,
    0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0e, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0e, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x10, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x10,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x1a, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x11, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x11, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x11, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x11, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x12, 0x04, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x12, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12, 0x1a, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x12, 0x2b, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05,
    0x12, 0x03, 0x13, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x13, 0x0d,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x1a, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x13, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x14, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x14, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x14, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x14,
    0x1f, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x17, 0x00, 0x1e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x18, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x18, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x13,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x1f, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x19, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x19, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x19, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x04,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1b, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1b,
    0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x1a, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1b, 0x2a, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x06, 0x12, 0x03, 0x1c, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x1c, 0x1a, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x1c, 0x2b, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x04, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x1d, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1d, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x1d, 0x2a, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x20, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x20, 0x08, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x21, 0x04, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x21, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x21, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x22,
    0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x22, 0x0d, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x1f, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x2a, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03,
    0x04, 0x00, 0x12, 0x04, 0x24, 0x04, 0x30, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x24, 0x09, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x25, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x25, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x25, 0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x26, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x26, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x26, 0x18, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x27, 0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x27, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x27, 0x19, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x28,
    0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x28,
    0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x28,
    0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x29, 0x08,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x29, 0x08,
    0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x29, 0x14,
    0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x2a, 0x08, 0x22,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x1b,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x2a, 0x1e, 0x21,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x2b, 0x08, 0x23, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x1c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x2b, 0x1f, 0x22, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x2c, 0x08, 0x23, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x1c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x2c, 0x1f, 0x22, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x2d, 0x08, 0x25, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x1e, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x2d, 0x21, 0x24, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x2e, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x2e, 0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x2f, 0x08, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x2f, 0x16, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x33, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03,
    0x33, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x34, 0x04, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x34, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x35, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x35, 0x0d,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x1f, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x2a, 0x2d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x37, 0x04, 0x40, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x37, 0x09, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x38, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x38, 0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x39, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x39, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x39, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x3a, 0x08, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x3a, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x3a, 0x1b, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x3b, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x3b, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x3b, 0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x3c, 0x08, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x3c, 0x08, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x3c, 0x1d, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x3d, 0x08, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x3d, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03,
    0x3d, 0x1b, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x3e,
    0x08, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3e,
    0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x3e,
    0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x3f, 0x08,
    0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x3f, 0x08,
    0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x3f, 0x28,
    0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x43, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x44, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x13, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x1b, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x45, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x45, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x45, 0x0d, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x45, 0x28, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x45, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x46, 0x04, 0x3d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x46, 0x0d, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x46, 0x28, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x46, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03,
    0x12, 0x03, 0x47, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x47, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x47, 0x0d,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x47, 0x1a, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x47, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x48, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x06, 0x12, 0x03, 0x48, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x48, 0x1a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x48,
    0x34, 0x37, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4b, 0x00, 0x50, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x4c, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x4c, 0x0d, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x28,
    0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x39, 0x3c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x4d, 0x0d, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x4d, 0x28, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x4d, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x4e, 0x04,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4e, 0x0d, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x03, 0x12, 0x03, 0x4f, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4f,
    0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4f, 0x1a, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4f, 0x34, 0x37, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x52, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07,
    0x01, 0x12, 0x03, 0x52, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03,
    0x53, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x53, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x07, 0x04, 0x00, 0x12, 0x04, 0x54, 0x04, 0x5b, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x54, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x55, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x55, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x55, 0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x56, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x56, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x56, 0x0e, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x57, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x57, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x57, 0x0e, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x58, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x58, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x58, 0x0e, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x59,
    0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x59,
    0x08, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x59,
    0x21, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x5a, 0x08,
    0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x5a, 0x08,
    0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x5a, 0x21,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x04, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5c, 0x0d, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x5c, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x5c, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03,
    0x5d, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5d, 0x0d, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x11, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5d, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x5e, 0x0d, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5e,
    0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5e, 0x17, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x3f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x04, 0x06, 0x12, 0x03, 0x5f, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x5f, 0x22, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x5f, 0x3b, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x03, 0x60,
    0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x06, 0x12, 0x03, 0x60, 0x0d, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x60, 0x22, 0x38, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x60, 0x3b, 0x3e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x63, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x63,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x64, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x10, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x64, 0x16, 0x19,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
