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
pub struct ChangeInfo {
    // message fields
    user: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<i32>,
    admin: ::std::option::Option<bool>,
    undo: ::std::option::Option<bool>,
    redo: ::std::option::Option<bool>,
    merge: ::std::option::Option<bool>,
    compressed: ::std::option::Option<bool>,
    migration: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangeInfo {}

impl ChangeInfo {
    pub fn new() -> ChangeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangeInfo {
        static mut instance: ::protobuf::lazy::Lazy<ChangeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangeInfo,
        };
        unsafe {
            instance.get(ChangeInfo::new)
        }
    }

    // optional string user = 1;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::string::String) {
        self.user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&mut self) -> &mut ::std::string::String {
        if self.user.is_none() {
            self.user.set_default();
        };
        self.user.as_mut().unwrap()
    }

    // Take field
    pub fn take_user(&mut self) -> ::std::string::String {
        self.user.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user(&self) -> &str {
        match self.user.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_user_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.user
    }

    fn mut_user_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.user
    }

    // optional int32 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i32 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.timestamp
    }

    // optional bool admin = 3;

    pub fn clear_admin(&mut self) {
        self.admin = ::std::option::Option::None;
    }

    pub fn has_admin(&self) -> bool {
        self.admin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin(&mut self, v: bool) {
        self.admin = ::std::option::Option::Some(v);
    }

    pub fn get_admin(&self) -> bool {
        self.admin.unwrap_or(false)
    }

    fn get_admin_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.admin
    }

    fn mut_admin_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.admin
    }

    // optional bool undo = 4;

    pub fn clear_undo(&mut self) {
        self.undo = ::std::option::Option::None;
    }

    pub fn has_undo(&self) -> bool {
        self.undo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_undo(&mut self, v: bool) {
        self.undo = ::std::option::Option::Some(v);
    }

    pub fn get_undo(&self) -> bool {
        self.undo.unwrap_or(false)
    }

    fn get_undo_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.undo
    }

    fn mut_undo_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.undo
    }

    // optional bool redo = 5;

    pub fn clear_redo(&mut self) {
        self.redo = ::std::option::Option::None;
    }

    pub fn has_redo(&self) -> bool {
        self.redo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redo(&mut self, v: bool) {
        self.redo = ::std::option::Option::Some(v);
    }

    pub fn get_redo(&self) -> bool {
        self.redo.unwrap_or(false)
    }

    fn get_redo_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.redo
    }

    fn mut_redo_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.redo
    }

    // optional bool merge = 6;

    pub fn clear_merge(&mut self) {
        self.merge = ::std::option::Option::None;
    }

    pub fn has_merge(&self) -> bool {
        self.merge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_merge(&mut self, v: bool) {
        self.merge = ::std::option::Option::Some(v);
    }

    pub fn get_merge(&self) -> bool {
        self.merge.unwrap_or(false)
    }

    fn get_merge_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.merge
    }

    fn mut_merge_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.merge
    }

    // optional bool compressed = 7;

    pub fn clear_compressed(&mut self) {
        self.compressed = ::std::option::Option::None;
    }

    pub fn has_compressed(&self) -> bool {
        self.compressed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compressed(&mut self, v: bool) {
        self.compressed = ::std::option::Option::Some(v);
    }

    pub fn get_compressed(&self) -> bool {
        self.compressed.unwrap_or(false)
    }

    fn get_compressed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.compressed
    }

    fn mut_compressed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.compressed
    }

    // optional bool migration = 8;

    pub fn clear_migration(&mut self) {
        self.migration = ::std::option::Option::None;
    }

    pub fn has_migration(&self) -> bool {
        self.migration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_migration(&mut self, v: bool) {
        self.migration = ::std::option::Option::Some(v);
    }

    pub fn get_migration(&self) -> bool {
        self.migration.unwrap_or(false)
    }

    fn get_migration_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.migration
    }

    fn mut_migration_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.migration
    }
}

impl ::protobuf::Message for ChangeInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.user)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.admin = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.undo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.redo = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.merge = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.compressed = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.migration = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.user.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.admin {
            my_size += 2;
        };
        if let Some(v) = self.undo {
            my_size += 2;
        };
        if let Some(v) = self.redo {
            my_size += 2;
        };
        if let Some(v) = self.merge {
            my_size += 2;
        };
        if let Some(v) = self.compressed {
            my_size += 2;
        };
        if let Some(v) = self.migration {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.admin {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.undo {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.redo {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.merge {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.compressed {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.migration {
            os.write_bool(8, v)?;
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

impl ::protobuf::MessageStatic for ChangeInfo {
    fn new() -> ChangeInfo {
        ChangeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChangeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user",
                    ChangeInfo::get_user_for_reflect,
                    ChangeInfo::mut_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "timestamp",
                    ChangeInfo::get_timestamp_for_reflect,
                    ChangeInfo::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "admin",
                    ChangeInfo::get_admin_for_reflect,
                    ChangeInfo::mut_admin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "undo",
                    ChangeInfo::get_undo_for_reflect,
                    ChangeInfo::mut_undo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "redo",
                    ChangeInfo::get_redo_for_reflect,
                    ChangeInfo::mut_redo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "merge",
                    ChangeInfo::get_merge_for_reflect,
                    ChangeInfo::mut_merge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "compressed",
                    ChangeInfo::get_compressed_for_reflect,
                    ChangeInfo::mut_compressed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "migration",
                    ChangeInfo::get_migration_for_reflect,
                    ChangeInfo::mut_migration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChangeInfo>(
                    "ChangeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChangeInfo {
    fn clear(&mut self) {
        self.clear_user();
        self.clear_timestamp();
        self.clear_admin();
        self.clear_undo();
        self.clear_redo();
        self.clear_merge();
        self.clear_compressed();
        self.clear_migration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChangeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Delta {
    // message fields
    base_version: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ops: ::protobuf::RepeatedField<super::playlist4ops::Op>,
    info: ::protobuf::SingularPtrField<ChangeInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Delta {}

impl Delta {
    pub fn new() -> Delta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Delta {
        static mut instance: ::protobuf::lazy::Lazy<Delta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Delta,
        };
        unsafe {
            instance.get(Delta::new)
        }
    }

    // optional bytes base_version = 1;

    pub fn clear_base_version(&mut self) {
        self.base_version.clear();
    }

    pub fn has_base_version(&self) -> bool {
        self.base_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_version(&mut self, v: ::std::vec::Vec<u8>) {
        self.base_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_version(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.base_version.is_none() {
            self.base_version.set_default();
        };
        self.base_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_base_version(&mut self) -> ::std::vec::Vec<u8> {
        self.base_version.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_base_version(&self) -> &[u8] {
        match self.base_version.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_base_version_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.base_version
    }

    fn mut_base_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.base_version
    }

    // repeated .Op ops = 2;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::protobuf::RepeatedField<super::playlist4ops::Op>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4ops::Op> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::protobuf::RepeatedField<super::playlist4ops::Op> {
        ::std::mem::replace(&mut self.ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_ops(&self) -> &[super::playlist4ops::Op] {
        &self.ops
    }

    fn get_ops_for_reflect(&self) -> &::protobuf::RepeatedField<super::playlist4ops::Op> {
        &self.ops
    }

    fn mut_ops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4ops::Op> {
        &mut self.ops
    }

    // optional .ChangeInfo info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ChangeInfo) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ChangeInfo {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ChangeInfo {
        self.info.take().unwrap_or_else(|| ChangeInfo::new())
    }

    pub fn get_info(&self) -> &ChangeInfo {
        self.info.as_ref().unwrap_or_else(|| ChangeInfo::default_instance())
    }

    fn get_info_for_reflect(&self) -> &::protobuf::SingularPtrField<ChangeInfo> {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChangeInfo> {
        &mut self.info
    }
}

impl ::protobuf::Message for Delta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.base_version)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info)?;
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
        if let Some(v) = self.base_version.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        for value in &self.ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base_version.as_ref() {
            os.write_bytes(1, &v)?;
        };
        for v in &self.ops {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.info.as_ref() {
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

impl ::protobuf::MessageStatic for Delta {
    fn new() -> Delta {
        Delta::new()
    }

    fn descriptor_static(_: ::std::option::Option<Delta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "base_version",
                    Delta::get_base_version_for_reflect,
                    Delta::mut_base_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4ops::Op>>(
                    "ops",
                    Delta::get_ops_for_reflect,
                    Delta::mut_ops_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChangeInfo>>(
                    "info",
                    Delta::get_info_for_reflect,
                    Delta::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Delta>(
                    "Delta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Delta {
    fn clear(&mut self) {
        self.clear_base_version();
        self.clear_ops();
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Delta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Delta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Merge {
    // message fields
    base_version: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    merge_version: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    info: ::protobuf::SingularPtrField<ChangeInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Merge {}

impl Merge {
    pub fn new() -> Merge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Merge {
        static mut instance: ::protobuf::lazy::Lazy<Merge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Merge,
        };
        unsafe {
            instance.get(Merge::new)
        }
    }

    // optional bytes base_version = 1;

    pub fn clear_base_version(&mut self) {
        self.base_version.clear();
    }

    pub fn has_base_version(&self) -> bool {
        self.base_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_version(&mut self, v: ::std::vec::Vec<u8>) {
        self.base_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_version(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.base_version.is_none() {
            self.base_version.set_default();
        };
        self.base_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_base_version(&mut self) -> ::std::vec::Vec<u8> {
        self.base_version.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_base_version(&self) -> &[u8] {
        match self.base_version.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_base_version_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.base_version
    }

    fn mut_base_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.base_version
    }

    // optional bytes merge_version = 2;

    pub fn clear_merge_version(&mut self) {
        self.merge_version.clear();
    }

    pub fn has_merge_version(&self) -> bool {
        self.merge_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_merge_version(&mut self, v: ::std::vec::Vec<u8>) {
        self.merge_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merge_version(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.merge_version.is_none() {
            self.merge_version.set_default();
        };
        self.merge_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_merge_version(&mut self) -> ::std::vec::Vec<u8> {
        self.merge_version.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_merge_version(&self) -> &[u8] {
        match self.merge_version.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_merge_version_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.merge_version
    }

    fn mut_merge_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.merge_version
    }

    // optional .ChangeInfo info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ChangeInfo) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ChangeInfo {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ChangeInfo {
        self.info.take().unwrap_or_else(|| ChangeInfo::new())
    }

    pub fn get_info(&self) -> &ChangeInfo {
        self.info.as_ref().unwrap_or_else(|| ChangeInfo::default_instance())
    }

    fn get_info_for_reflect(&self) -> &::protobuf::SingularPtrField<ChangeInfo> {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChangeInfo> {
        &mut self.info
    }
}

impl ::protobuf::Message for Merge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.base_version)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.merge_version)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info)?;
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
        if let Some(v) = self.base_version.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.merge_version.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base_version.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.merge_version.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.info.as_ref() {
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

impl ::protobuf::MessageStatic for Merge {
    fn new() -> Merge {
        Merge::new()
    }

    fn descriptor_static(_: ::std::option::Option<Merge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "base_version",
                    Merge::get_base_version_for_reflect,
                    Merge::mut_base_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merge_version",
                    Merge::get_merge_version_for_reflect,
                    Merge::mut_merge_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChangeInfo>>(
                    "info",
                    Merge::get_info_for_reflect,
                    Merge::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Merge>(
                    "Merge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Merge {
    fn clear(&mut self) {
        self.clear_base_version();
        self.clear_merge_version();
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Merge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Merge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChangeSet {
    // message fields
    kind: ::std::option::Option<ChangeSet_Kind>,
    delta: ::protobuf::SingularPtrField<Delta>,
    merge: ::protobuf::SingularPtrField<Merge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangeSet {}

impl ChangeSet {
    pub fn new() -> ChangeSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangeSet {
        static mut instance: ::protobuf::lazy::Lazy<ChangeSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangeSet,
        };
        unsafe {
            instance.get(ChangeSet::new)
        }
    }

    // optional .ChangeSet.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: ChangeSet_Kind) {
        self.kind = ::std::option::Option::Some(v);
    }

    pub fn get_kind(&self) -> ChangeSet_Kind {
        self.kind.unwrap_or(ChangeSet_Kind::KIND_UNKNOWN)
    }

    fn get_kind_for_reflect(&self) -> &::std::option::Option<ChangeSet_Kind> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::std::option::Option<ChangeSet_Kind> {
        &mut self.kind
    }

    // optional .Delta delta = 2;

    pub fn clear_delta(&mut self) {
        self.delta.clear();
    }

    pub fn has_delta(&self) -> bool {
        self.delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta(&mut self, v: Delta) {
        self.delta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delta(&mut self) -> &mut Delta {
        if self.delta.is_none() {
            self.delta.set_default();
        };
        self.delta.as_mut().unwrap()
    }

    // Take field
    pub fn take_delta(&mut self) -> Delta {
        self.delta.take().unwrap_or_else(|| Delta::new())
    }

    pub fn get_delta(&self) -> &Delta {
        self.delta.as_ref().unwrap_or_else(|| Delta::default_instance())
    }

    fn get_delta_for_reflect(&self) -> &::protobuf::SingularPtrField<Delta> {
        &self.delta
    }

    fn mut_delta_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Delta> {
        &mut self.delta
    }

    // optional .Merge merge = 3;

    pub fn clear_merge(&mut self) {
        self.merge.clear();
    }

    pub fn has_merge(&self) -> bool {
        self.merge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_merge(&mut self, v: Merge) {
        self.merge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merge(&mut self) -> &mut Merge {
        if self.merge.is_none() {
            self.merge.set_default();
        };
        self.merge.as_mut().unwrap()
    }

    // Take field
    pub fn take_merge(&mut self) -> Merge {
        self.merge.take().unwrap_or_else(|| Merge::new())
    }

    pub fn get_merge(&self) -> &Merge {
        self.merge.as_ref().unwrap_or_else(|| Merge::default_instance())
    }

    fn get_merge_for_reflect(&self) -> &::protobuf::SingularPtrField<Merge> {
        &self.merge
    }

    fn mut_merge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Merge> {
        &mut self.merge
    }
}

impl ::protobuf::Message for ChangeSet {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delta)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.merge)?;
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
        if let Some(v) = self.delta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.merge.as_ref() {
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
        if let Some(v) = self.delta.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.merge.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ChangeSet {
    fn new() -> ChangeSet {
        ChangeSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChangeSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChangeSet_Kind>>(
                    "kind",
                    ChangeSet::get_kind_for_reflect,
                    ChangeSet::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Delta>>(
                    "delta",
                    ChangeSet::get_delta_for_reflect,
                    ChangeSet::mut_delta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Merge>>(
                    "merge",
                    ChangeSet::get_merge_for_reflect,
                    ChangeSet::mut_merge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChangeSet>(
                    "ChangeSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChangeSet {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_delta();
        self.clear_merge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChangeSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChangeSet_Kind {
    KIND_UNKNOWN = 0,
    DELTA = 2,
    MERGE = 3,
}

impl ::protobuf::ProtobufEnum for ChangeSet_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChangeSet_Kind> {
        match value {
            0 => ::std::option::Option::Some(ChangeSet_Kind::KIND_UNKNOWN),
            2 => ::std::option::Option::Some(ChangeSet_Kind::DELTA),
            3 => ::std::option::Option::Some(ChangeSet_Kind::MERGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChangeSet_Kind] = &[
            ChangeSet_Kind::KIND_UNKNOWN,
            ChangeSet_Kind::DELTA,
            ChangeSet_Kind::MERGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ChangeSet_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChangeSet_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChangeSet_Kind {
}

impl ::protobuf::reflect::ProtobufValue for ChangeSet_Kind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RevisionTaggedChangeSet {
    // message fields
    revision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    change_set: ::protobuf::SingularPtrField<ChangeSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RevisionTaggedChangeSet {}

impl RevisionTaggedChangeSet {
    pub fn new() -> RevisionTaggedChangeSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RevisionTaggedChangeSet {
        static mut instance: ::protobuf::lazy::Lazy<RevisionTaggedChangeSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RevisionTaggedChangeSet,
        };
        unsafe {
            instance.get(RevisionTaggedChangeSet::new)
        }
    }

    // optional bytes revision = 1;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::vec::Vec<u8>) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.revision.is_none() {
            self.revision.set_default();
        };
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::vec::Vec<u8> {
        self.revision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_revision(&self) -> &[u8] {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.revision
    }

    // optional .ChangeSet change_set = 2;

    pub fn clear_change_set(&mut self) {
        self.change_set.clear();
    }

    pub fn has_change_set(&self) -> bool {
        self.change_set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_change_set(&mut self, v: ChangeSet) {
        self.change_set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_change_set(&mut self) -> &mut ChangeSet {
        if self.change_set.is_none() {
            self.change_set.set_default();
        };
        self.change_set.as_mut().unwrap()
    }

    // Take field
    pub fn take_change_set(&mut self) -> ChangeSet {
        self.change_set.take().unwrap_or_else(|| ChangeSet::new())
    }

    pub fn get_change_set(&self) -> &ChangeSet {
        self.change_set.as_ref().unwrap_or_else(|| ChangeSet::default_instance())
    }

    fn get_change_set_for_reflect(&self) -> &::protobuf::SingularPtrField<ChangeSet> {
        &self.change_set
    }

    fn mut_change_set_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChangeSet> {
        &mut self.change_set
    }
}

impl ::protobuf::Message for RevisionTaggedChangeSet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.revision)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.change_set)?;
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
        if let Some(v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.change_set.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.revision.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.change_set.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RevisionTaggedChangeSet {
    fn new() -> RevisionTaggedChangeSet {
        RevisionTaggedChangeSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<RevisionTaggedChangeSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "revision",
                    RevisionTaggedChangeSet::get_revision_for_reflect,
                    RevisionTaggedChangeSet::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChangeSet>>(
                    "change_set",
                    RevisionTaggedChangeSet::get_change_set_for_reflect,
                    RevisionTaggedChangeSet::mut_change_set_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RevisionTaggedChangeSet>(
                    "RevisionTaggedChangeSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RevisionTaggedChangeSet {
    fn clear(&mut self) {
        self.clear_revision();
        self.clear_change_set();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RevisionTaggedChangeSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RevisionTaggedChangeSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Diff {
    // message fields
    from_revision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ops: ::protobuf::RepeatedField<super::playlist4ops::Op>,
    to_revision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Diff {}

impl Diff {
    pub fn new() -> Diff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Diff {
        static mut instance: ::protobuf::lazy::Lazy<Diff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Diff,
        };
        unsafe {
            instance.get(Diff::new)
        }
    }

    // optional bytes from_revision = 1;

    pub fn clear_from_revision(&mut self) {
        self.from_revision.clear();
    }

    pub fn has_from_revision(&self) -> bool {
        self.from_revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_revision(&mut self, v: ::std::vec::Vec<u8>) {
        self.from_revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_revision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.from_revision.is_none() {
            self.from_revision.set_default();
        };
        self.from_revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_revision(&mut self) -> ::std::vec::Vec<u8> {
        self.from_revision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_from_revision(&self) -> &[u8] {
        match self.from_revision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_from_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.from_revision
    }

    fn mut_from_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.from_revision
    }

    // repeated .Op ops = 2;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::protobuf::RepeatedField<super::playlist4ops::Op>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4ops::Op> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::protobuf::RepeatedField<super::playlist4ops::Op> {
        ::std::mem::replace(&mut self.ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_ops(&self) -> &[super::playlist4ops::Op] {
        &self.ops
    }

    fn get_ops_for_reflect(&self) -> &::protobuf::RepeatedField<super::playlist4ops::Op> {
        &self.ops
    }

    fn mut_ops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4ops::Op> {
        &mut self.ops
    }

    // optional bytes to_revision = 3;

    pub fn clear_to_revision(&mut self) {
        self.to_revision.clear();
    }

    pub fn has_to_revision(&self) -> bool {
        self.to_revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_revision(&mut self, v: ::std::vec::Vec<u8>) {
        self.to_revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_revision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.to_revision.is_none() {
            self.to_revision.set_default();
        };
        self.to_revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_to_revision(&mut self) -> ::std::vec::Vec<u8> {
        self.to_revision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_to_revision(&self) -> &[u8] {
        match self.to_revision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_to_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.to_revision
    }

    fn mut_to_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.to_revision
    }
}

impl ::protobuf::Message for Diff {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.from_revision)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.to_revision)?;
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
        if let Some(v) = self.from_revision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        for value in &self.ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.to_revision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from_revision.as_ref() {
            os.write_bytes(1, &v)?;
        };
        for v in &self.ops {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.to_revision.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for Diff {
    fn new() -> Diff {
        Diff::new()
    }

    fn descriptor_static(_: ::std::option::Option<Diff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "from_revision",
                    Diff::get_from_revision_for_reflect,
                    Diff::mut_from_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4ops::Op>>(
                    "ops",
                    Diff::get_ops_for_reflect,
                    Diff::mut_ops_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "to_revision",
                    Diff::get_to_revision_for_reflect,
                    Diff::mut_to_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Diff>(
                    "Diff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Diff {
    fn clear(&mut self) {
        self.clear_from_revision();
        self.clear_ops();
        self.clear_to_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Diff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Diff {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListDump {
    // message fields
    latestRevision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<i32>,
    attributes: ::protobuf::SingularPtrField<super::playlist4meta::ListAttributes>,
    checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    contents: ::protobuf::SingularPtrField<super::playlist4content::ListItems>,
    pendingDeltas: ::protobuf::RepeatedField<Delta>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListDump {}

impl ListDump {
    pub fn new() -> ListDump {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListDump {
        static mut instance: ::protobuf::lazy::Lazy<ListDump> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListDump,
        };
        unsafe {
            instance.get(ListDump::new)
        }
    }

    // optional bytes latestRevision = 1;

    pub fn clear_latestRevision(&mut self) {
        self.latestRevision.clear();
    }

    pub fn has_latestRevision(&self) -> bool {
        self.latestRevision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latestRevision(&mut self, v: ::std::vec::Vec<u8>) {
        self.latestRevision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latestRevision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.latestRevision.is_none() {
            self.latestRevision.set_default();
        };
        self.latestRevision.as_mut().unwrap()
    }

    // Take field
    pub fn take_latestRevision(&mut self) -> ::std::vec::Vec<u8> {
        self.latestRevision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_latestRevision(&self) -> &[u8] {
        match self.latestRevision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_latestRevision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.latestRevision
    }

    fn mut_latestRevision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.latestRevision
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

    // optional .ListAttributes attributes = 3;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    pub fn has_attributes(&self) -> bool {
        self.attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: super::playlist4meta::ListAttributes) {
        self.attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attributes(&mut self) -> &mut super::playlist4meta::ListAttributes {
        if self.attributes.is_none() {
            self.attributes.set_default();
        };
        self.attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_attributes(&mut self) -> super::playlist4meta::ListAttributes {
        self.attributes.take().unwrap_or_else(|| super::playlist4meta::ListAttributes::new())
    }

    pub fn get_attributes(&self) -> &super::playlist4meta::ListAttributes {
        self.attributes.as_ref().unwrap_or_else(|| super::playlist4meta::ListAttributes::default_instance())
    }

    fn get_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListAttributes> {
        &self.attributes
    }

    fn mut_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListAttributes> {
        &mut self.attributes
    }

    // optional .ListChecksum checksum = 4;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.checksum.is_none() {
            self.checksum.set_default();
        };
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.checksum
    }

    // optional .ListItems contents = 5;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    pub fn has_contents(&self) -> bool {
        self.contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: super::playlist4content::ListItems) {
        self.contents = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut super::playlist4content::ListItems {
        if self.contents.is_none() {
            self.contents.set_default();
        };
        self.contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_contents(&mut self) -> super::playlist4content::ListItems {
        self.contents.take().unwrap_or_else(|| super::playlist4content::ListItems::new())
    }

    pub fn get_contents(&self) -> &super::playlist4content::ListItems {
        self.contents.as_ref().unwrap_or_else(|| super::playlist4content::ListItems::default_instance())
    }

    fn get_contents_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4content::ListItems> {
        &self.contents
    }

    fn mut_contents_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4content::ListItems> {
        &mut self.contents
    }

    // repeated .Delta pendingDeltas = 7;

    pub fn clear_pendingDeltas(&mut self) {
        self.pendingDeltas.clear();
    }

    // Param is passed by value, moved
    pub fn set_pendingDeltas(&mut self, v: ::protobuf::RepeatedField<Delta>) {
        self.pendingDeltas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pendingDeltas(&mut self) -> &mut ::protobuf::RepeatedField<Delta> {
        &mut self.pendingDeltas
    }

    // Take field
    pub fn take_pendingDeltas(&mut self) -> ::protobuf::RepeatedField<Delta> {
        ::std::mem::replace(&mut self.pendingDeltas, ::protobuf::RepeatedField::new())
    }

    pub fn get_pendingDeltas(&self) -> &[Delta] {
        &self.pendingDeltas
    }

    fn get_pendingDeltas_for_reflect(&self) -> &::protobuf::RepeatedField<Delta> {
        &self.pendingDeltas
    }

    fn mut_pendingDeltas_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Delta> {
        &mut self.pendingDeltas
    }
}

impl ::protobuf::Message for ListDump {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.latestRevision)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.attributes)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checksum)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contents)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pendingDeltas)?;
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
        if let Some(v) = self.latestRevision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.contents.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pendingDeltas {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.latestRevision.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.length {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.attributes.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.checksum.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.contents.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.pendingDeltas {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ListDump {
    fn new() -> ListDump {
        ListDump::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListDump>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "latestRevision",
                    ListDump::get_latestRevision_for_reflect,
                    ListDump::mut_latestRevision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    ListDump::get_length_for_reflect,
                    ListDump::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListAttributes>>(
                    "attributes",
                    ListDump::get_attributes_for_reflect,
                    ListDump::mut_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "checksum",
                    ListDump::get_checksum_for_reflect,
                    ListDump::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4content::ListItems>>(
                    "contents",
                    ListDump::get_contents_for_reflect,
                    ListDump::mut_contents_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Delta>>(
                    "pendingDeltas",
                    ListDump::get_pendingDeltas_for_reflect,
                    ListDump::mut_pendingDeltas_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListDump>(
                    "ListDump",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListDump {
    fn clear(&mut self) {
        self.clear_latestRevision();
        self.clear_length();
        self.clear_attributes();
        self.clear_checksum();
        self.clear_contents();
        self.clear_pendingDeltas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListDump {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListDump {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListChanges {
    // message fields
    baseRevision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    deltas: ::protobuf::RepeatedField<Delta>,
    wantResultingRevisions: ::std::option::Option<bool>,
    wantSyncResult: ::std::option::Option<bool>,
    dump: ::protobuf::SingularPtrField<ListDump>,
    nonces: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListChanges {}

impl ListChanges {
    pub fn new() -> ListChanges {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListChanges {
        static mut instance: ::protobuf::lazy::Lazy<ListChanges> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListChanges,
        };
        unsafe {
            instance.get(ListChanges::new)
        }
    }

    // optional bytes baseRevision = 1;

    pub fn clear_baseRevision(&mut self) {
        self.baseRevision.clear();
    }

    pub fn has_baseRevision(&self) -> bool {
        self.baseRevision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseRevision(&mut self, v: ::std::vec::Vec<u8>) {
        self.baseRevision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseRevision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.baseRevision.is_none() {
            self.baseRevision.set_default();
        };
        self.baseRevision.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseRevision(&mut self) -> ::std::vec::Vec<u8> {
        self.baseRevision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_baseRevision(&self) -> &[u8] {
        match self.baseRevision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_baseRevision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.baseRevision
    }

    fn mut_baseRevision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.baseRevision
    }

    // repeated .Delta deltas = 2;

    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
    }

    // Param is passed by value, moved
    pub fn set_deltas(&mut self, v: ::protobuf::RepeatedField<Delta>) {
        self.deltas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deltas(&mut self) -> &mut ::protobuf::RepeatedField<Delta> {
        &mut self.deltas
    }

    // Take field
    pub fn take_deltas(&mut self) -> ::protobuf::RepeatedField<Delta> {
        ::std::mem::replace(&mut self.deltas, ::protobuf::RepeatedField::new())
    }

    pub fn get_deltas(&self) -> &[Delta] {
        &self.deltas
    }

    fn get_deltas_for_reflect(&self) -> &::protobuf::RepeatedField<Delta> {
        &self.deltas
    }

    fn mut_deltas_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Delta> {
        &mut self.deltas
    }

    // optional bool wantResultingRevisions = 3;

    pub fn clear_wantResultingRevisions(&mut self) {
        self.wantResultingRevisions = ::std::option::Option::None;
    }

    pub fn has_wantResultingRevisions(&self) -> bool {
        self.wantResultingRevisions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantResultingRevisions(&mut self, v: bool) {
        self.wantResultingRevisions = ::std::option::Option::Some(v);
    }

    pub fn get_wantResultingRevisions(&self) -> bool {
        self.wantResultingRevisions.unwrap_or(false)
    }

    fn get_wantResultingRevisions_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantResultingRevisions
    }

    fn mut_wantResultingRevisions_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantResultingRevisions
    }

    // optional bool wantSyncResult = 4;

    pub fn clear_wantSyncResult(&mut self) {
        self.wantSyncResult = ::std::option::Option::None;
    }

    pub fn has_wantSyncResult(&self) -> bool {
        self.wantSyncResult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantSyncResult(&mut self, v: bool) {
        self.wantSyncResult = ::std::option::Option::Some(v);
    }

    pub fn get_wantSyncResult(&self) -> bool {
        self.wantSyncResult.unwrap_or(false)
    }

    fn get_wantSyncResult_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantSyncResult
    }

    fn mut_wantSyncResult_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantSyncResult
    }

    // optional .ListDump dump = 5;

    pub fn clear_dump(&mut self) {
        self.dump.clear();
    }

    pub fn has_dump(&self) -> bool {
        self.dump.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dump(&mut self, v: ListDump) {
        self.dump = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dump(&mut self) -> &mut ListDump {
        if self.dump.is_none() {
            self.dump.set_default();
        };
        self.dump.as_mut().unwrap()
    }

    // Take field
    pub fn take_dump(&mut self) -> ListDump {
        self.dump.take().unwrap_or_else(|| ListDump::new())
    }

    pub fn get_dump(&self) -> &ListDump {
        self.dump.as_ref().unwrap_or_else(|| ListDump::default_instance())
    }

    fn get_dump_for_reflect(&self) -> &::protobuf::SingularPtrField<ListDump> {
        &self.dump
    }

    fn mut_dump_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ListDump> {
        &mut self.dump
    }

    // repeated int32 nonces = 6;

    pub fn clear_nonces(&mut self) {
        self.nonces.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonces(&mut self, v: ::std::vec::Vec<i32>) {
        self.nonces = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nonces(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.nonces
    }

    // Take field
    pub fn take_nonces(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.nonces, ::std::vec::Vec::new())
    }

    pub fn get_nonces(&self) -> &[i32] {
        &self.nonces
    }

    fn get_nonces_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.nonces
    }

    fn mut_nonces_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.nonces
    }
}

impl ::protobuf::Message for ListChanges {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.baseRevision)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.deltas)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantResultingRevisions = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantSyncResult = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dump)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.nonces)?;
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
        if let Some(v) = self.baseRevision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        for value in &self.deltas {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.wantResultingRevisions {
            my_size += 2;
        };
        if let Some(v) = self.wantSyncResult {
            my_size += 2;
        };
        if let Some(v) = self.dump.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.nonces {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.baseRevision.as_ref() {
            os.write_bytes(1, &v)?;
        };
        for v in &self.deltas {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.wantResultingRevisions {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.wantSyncResult {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.dump.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.nonces {
            os.write_int32(6, *v)?;
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

impl ::protobuf::MessageStatic for ListChanges {
    fn new() -> ListChanges {
        ListChanges::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListChanges>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "baseRevision",
                    ListChanges::get_baseRevision_for_reflect,
                    ListChanges::mut_baseRevision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Delta>>(
                    "deltas",
                    ListChanges::get_deltas_for_reflect,
                    ListChanges::mut_deltas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantResultingRevisions",
                    ListChanges::get_wantResultingRevisions_for_reflect,
                    ListChanges::mut_wantResultingRevisions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantSyncResult",
                    ListChanges::get_wantSyncResult_for_reflect,
                    ListChanges::mut_wantSyncResult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListDump>>(
                    "dump",
                    ListChanges::get_dump_for_reflect,
                    ListChanges::mut_dump_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "nonces",
                    ListChanges::get_nonces_for_reflect,
                    ListChanges::mut_nonces_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListChanges>(
                    "ListChanges",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListChanges {
    fn clear(&mut self) {
        self.clear_baseRevision();
        self.clear_deltas();
        self.clear_wantResultingRevisions();
        self.clear_wantSyncResult();
        self.clear_dump();
        self.clear_nonces();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListChanges {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListChanges {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SelectedListContent {
    // message fields
    revision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<i32>,
    attributes: ::protobuf::SingularPtrField<super::playlist4meta::ListAttributes>,
    checksum: ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum>,
    contents: ::protobuf::SingularPtrField<super::playlist4content::ListItems>,
    diff: ::protobuf::SingularPtrField<Diff>,
    syncResult: ::protobuf::SingularPtrField<Diff>,
    resultingRevisions: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    multipleHeads: ::std::option::Option<bool>,
    upToDate: ::std::option::Option<bool>,
    resolveAction: ::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction>,
    issues: ::protobuf::RepeatedField<super::playlist4issues::ClientIssue>,
    nonces: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SelectedListContent {}

impl SelectedListContent {
    pub fn new() -> SelectedListContent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SelectedListContent {
        static mut instance: ::protobuf::lazy::Lazy<SelectedListContent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SelectedListContent,
        };
        unsafe {
            instance.get(SelectedListContent::new)
        }
    }

    // optional bytes revision = 1;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::vec::Vec<u8>) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.revision.is_none() {
            self.revision.set_default();
        };
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::vec::Vec<u8> {
        self.revision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_revision(&self) -> &[u8] {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.revision
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

    // optional .ListAttributes attributes = 3;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    pub fn has_attributes(&self) -> bool {
        self.attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: super::playlist4meta::ListAttributes) {
        self.attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attributes(&mut self) -> &mut super::playlist4meta::ListAttributes {
        if self.attributes.is_none() {
            self.attributes.set_default();
        };
        self.attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_attributes(&mut self) -> super::playlist4meta::ListAttributes {
        self.attributes.take().unwrap_or_else(|| super::playlist4meta::ListAttributes::new())
    }

    pub fn get_attributes(&self) -> &super::playlist4meta::ListAttributes {
        self.attributes.as_ref().unwrap_or_else(|| super::playlist4meta::ListAttributes::default_instance())
    }

    fn get_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListAttributes> {
        &self.attributes
    }

    fn mut_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListAttributes> {
        &mut self.attributes
    }

    // optional .ListChecksum checksum = 4;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: super::playlist4meta::ListChecksum) {
        self.checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum(&mut self) -> &mut super::playlist4meta::ListChecksum {
        if self.checksum.is_none() {
            self.checksum.set_default();
        };
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> super::playlist4meta::ListChecksum {
        self.checksum.take().unwrap_or_else(|| super::playlist4meta::ListChecksum::new())
    }

    pub fn get_checksum(&self) -> &super::playlist4meta::ListChecksum {
        self.checksum.as_ref().unwrap_or_else(|| super::playlist4meta::ListChecksum::default_instance())
    }

    fn get_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ListChecksum> {
        &mut self.checksum
    }

    // optional .ListItems contents = 5;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    pub fn has_contents(&self) -> bool {
        self.contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: super::playlist4content::ListItems) {
        self.contents = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut super::playlist4content::ListItems {
        if self.contents.is_none() {
            self.contents.set_default();
        };
        self.contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_contents(&mut self) -> super::playlist4content::ListItems {
        self.contents.take().unwrap_or_else(|| super::playlist4content::ListItems::new())
    }

    pub fn get_contents(&self) -> &super::playlist4content::ListItems {
        self.contents.as_ref().unwrap_or_else(|| super::playlist4content::ListItems::default_instance())
    }

    fn get_contents_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4content::ListItems> {
        &self.contents
    }

    fn mut_contents_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4content::ListItems> {
        &mut self.contents
    }

    // optional .Diff diff = 6;

    pub fn clear_diff(&mut self) {
        self.diff.clear();
    }

    pub fn has_diff(&self) -> bool {
        self.diff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diff(&mut self, v: Diff) {
        self.diff = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diff(&mut self) -> &mut Diff {
        if self.diff.is_none() {
            self.diff.set_default();
        };
        self.diff.as_mut().unwrap()
    }

    // Take field
    pub fn take_diff(&mut self) -> Diff {
        self.diff.take().unwrap_or_else(|| Diff::new())
    }

    pub fn get_diff(&self) -> &Diff {
        self.diff.as_ref().unwrap_or_else(|| Diff::default_instance())
    }

    fn get_diff_for_reflect(&self) -> &::protobuf::SingularPtrField<Diff> {
        &self.diff
    }

    fn mut_diff_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Diff> {
        &mut self.diff
    }

    // optional .Diff syncResult = 7;

    pub fn clear_syncResult(&mut self) {
        self.syncResult.clear();
    }

    pub fn has_syncResult(&self) -> bool {
        self.syncResult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncResult(&mut self, v: Diff) {
        self.syncResult = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_syncResult(&mut self) -> &mut Diff {
        if self.syncResult.is_none() {
            self.syncResult.set_default();
        };
        self.syncResult.as_mut().unwrap()
    }

    // Take field
    pub fn take_syncResult(&mut self) -> Diff {
        self.syncResult.take().unwrap_or_else(|| Diff::new())
    }

    pub fn get_syncResult(&self) -> &Diff {
        self.syncResult.as_ref().unwrap_or_else(|| Diff::default_instance())
    }

    fn get_syncResult_for_reflect(&self) -> &::protobuf::SingularPtrField<Diff> {
        &self.syncResult
    }

    fn mut_syncResult_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Diff> {
        &mut self.syncResult
    }

    // repeated bytes resultingRevisions = 8;

    pub fn clear_resultingRevisions(&mut self) {
        self.resultingRevisions.clear();
    }

    // Param is passed by value, moved
    pub fn set_resultingRevisions(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.resultingRevisions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resultingRevisions(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.resultingRevisions
    }

    // Take field
    pub fn take_resultingRevisions(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.resultingRevisions, ::protobuf::RepeatedField::new())
    }

    pub fn get_resultingRevisions(&self) -> &[::std::vec::Vec<u8>] {
        &self.resultingRevisions
    }

    fn get_resultingRevisions_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.resultingRevisions
    }

    fn mut_resultingRevisions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.resultingRevisions
    }

    // optional bool multipleHeads = 9;

    pub fn clear_multipleHeads(&mut self) {
        self.multipleHeads = ::std::option::Option::None;
    }

    pub fn has_multipleHeads(&self) -> bool {
        self.multipleHeads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_multipleHeads(&mut self, v: bool) {
        self.multipleHeads = ::std::option::Option::Some(v);
    }

    pub fn get_multipleHeads(&self) -> bool {
        self.multipleHeads.unwrap_or(false)
    }

    fn get_multipleHeads_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.multipleHeads
    }

    fn mut_multipleHeads_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.multipleHeads
    }

    // optional bool upToDate = 10;

    pub fn clear_upToDate(&mut self) {
        self.upToDate = ::std::option::Option::None;
    }

    pub fn has_upToDate(&self) -> bool {
        self.upToDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upToDate(&mut self, v: bool) {
        self.upToDate = ::std::option::Option::Some(v);
    }

    pub fn get_upToDate(&self) -> bool {
        self.upToDate.unwrap_or(false)
    }

    fn get_upToDate_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.upToDate
    }

    fn mut_upToDate_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.upToDate
    }

    // repeated .ClientResolveAction resolveAction = 12;

    pub fn clear_resolveAction(&mut self) {
        self.resolveAction.clear();
    }

    // Param is passed by value, moved
    pub fn set_resolveAction(&mut self, v: ::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction>) {
        self.resolveAction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resolveAction(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction> {
        &mut self.resolveAction
    }

    // Take field
    pub fn take_resolveAction(&mut self) -> ::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction> {
        ::std::mem::replace(&mut self.resolveAction, ::protobuf::RepeatedField::new())
    }

    pub fn get_resolveAction(&self) -> &[super::playlist4issues::ClientResolveAction] {
        &self.resolveAction
    }

    fn get_resolveAction_for_reflect(&self) -> &::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction> {
        &self.resolveAction
    }

    fn mut_resolveAction_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction> {
        &mut self.resolveAction
    }

    // repeated .ClientIssue issues = 13;

    pub fn clear_issues(&mut self) {
        self.issues.clear();
    }

    // Param is passed by value, moved
    pub fn set_issues(&mut self, v: ::protobuf::RepeatedField<super::playlist4issues::ClientIssue>) {
        self.issues = v;
    }

    // Mutable pointer to the field.
    pub fn mut_issues(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4issues::ClientIssue> {
        &mut self.issues
    }

    // Take field
    pub fn take_issues(&mut self) -> ::protobuf::RepeatedField<super::playlist4issues::ClientIssue> {
        ::std::mem::replace(&mut self.issues, ::protobuf::RepeatedField::new())
    }

    pub fn get_issues(&self) -> &[super::playlist4issues::ClientIssue] {
        &self.issues
    }

    fn get_issues_for_reflect(&self) -> &::protobuf::RepeatedField<super::playlist4issues::ClientIssue> {
        &self.issues
    }

    fn mut_issues_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::playlist4issues::ClientIssue> {
        &mut self.issues
    }

    // repeated int32 nonces = 14;

    pub fn clear_nonces(&mut self) {
        self.nonces.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonces(&mut self, v: ::std::vec::Vec<i32>) {
        self.nonces = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nonces(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.nonces
    }

    // Take field
    pub fn take_nonces(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.nonces, ::std::vec::Vec::new())
    }

    pub fn get_nonces(&self) -> &[i32] {
        &self.nonces
    }

    fn get_nonces_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.nonces
    }

    fn mut_nonces_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.nonces
    }
}

impl ::protobuf::Message for SelectedListContent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.revision)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.attributes)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checksum)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contents)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diff)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.syncResult)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.resultingRevisions)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.multipleHeads = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.upToDate = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resolveAction)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.issues)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.nonces)?;
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
        if let Some(v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.contents.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.diff.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.syncResult.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.resultingRevisions {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        if let Some(v) = self.multipleHeads {
            my_size += 2;
        };
        if let Some(v) = self.upToDate {
            my_size += 2;
        };
        for value in &self.resolveAction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.issues {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.nonces {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.revision.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.length {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.attributes.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.checksum.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.contents.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.diff.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.syncResult.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.resultingRevisions {
            os.write_bytes(8, &v)?;
        };
        if let Some(v) = self.multipleHeads {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.upToDate {
            os.write_bool(10, v)?;
        };
        for v in &self.resolveAction {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.issues {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.nonces {
            os.write_int32(14, *v)?;
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

impl ::protobuf::MessageStatic for SelectedListContent {
    fn new() -> SelectedListContent {
        SelectedListContent::new()
    }

    fn descriptor_static(_: ::std::option::Option<SelectedListContent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "revision",
                    SelectedListContent::get_revision_for_reflect,
                    SelectedListContent::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    SelectedListContent::get_length_for_reflect,
                    SelectedListContent::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListAttributes>>(
                    "attributes",
                    SelectedListContent::get_attributes_for_reflect,
                    SelectedListContent::mut_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ListChecksum>>(
                    "checksum",
                    SelectedListContent::get_checksum_for_reflect,
                    SelectedListContent::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4content::ListItems>>(
                    "contents",
                    SelectedListContent::get_contents_for_reflect,
                    SelectedListContent::mut_contents_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Diff>>(
                    "diff",
                    SelectedListContent::get_diff_for_reflect,
                    SelectedListContent::mut_diff_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Diff>>(
                    "syncResult",
                    SelectedListContent::get_syncResult_for_reflect,
                    SelectedListContent::mut_syncResult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "resultingRevisions",
                    SelectedListContent::get_resultingRevisions_for_reflect,
                    SelectedListContent::mut_resultingRevisions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "multipleHeads",
                    SelectedListContent::get_multipleHeads_for_reflect,
                    SelectedListContent::mut_multipleHeads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "upToDate",
                    SelectedListContent::get_upToDate_for_reflect,
                    SelectedListContent::mut_upToDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4issues::ClientResolveAction>>(
                    "resolveAction",
                    SelectedListContent::get_resolveAction_for_reflect,
                    SelectedListContent::mut_resolveAction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4issues::ClientIssue>>(
                    "issues",
                    SelectedListContent::get_issues_for_reflect,
                    SelectedListContent::mut_issues_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "nonces",
                    SelectedListContent::get_nonces_for_reflect,
                    SelectedListContent::mut_nonces_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SelectedListContent>(
                    "SelectedListContent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SelectedListContent {
    fn clear(&mut self) {
        self.clear_revision();
        self.clear_length();
        self.clear_attributes();
        self.clear_checksum();
        self.clear_contents();
        self.clear_diff();
        self.clear_syncResult();
        self.clear_resultingRevisions();
        self.clear_multipleHeads();
        self.clear_upToDate();
        self.clear_resolveAction();
        self.clear_issues();
        self.clear_nonces();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SelectedListContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelectedListContent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x12, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69,
    0x73, 0x74, 0x34, 0x6f, 0x70, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x70, 0x6c,
    0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x6d, 0x65, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x16, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x70, 0x6c, 0x61, 0x79, 0x6c,
    0x69, 0x73, 0x74, 0x34, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xd0, 0x01, 0x0a, 0x0a, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x12, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75,
    0x73, 0x65, 0x72, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x05, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x6e, 0x64, 0x6f, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x75, 0x6e, 0x64, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x72,
    0x65, 0x64, 0x6f, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x72, 0x65, 0x64, 0x6f, 0x12,
    0x14, 0x0a, 0x05, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05,
    0x6d, 0x65, 0x72, 0x67, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x63, 0x6f, 0x6d, 0x70, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x22, 0x62, 0x0a, 0x05, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x12, 0x21, 0x0a, 0x0c,
    0x62, 0x61, 0x73, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x0b, 0x62, 0x61, 0x73, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x15, 0x0a, 0x03, 0x6f, 0x70, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x03, 0x2e, 0x4f,
    0x70, 0x52, 0x03, 0x6f, 0x70, 0x73, 0x12, 0x1f, 0x0a, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x49, 0x6e, 0x66,
    0x6f, 0x52, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x22, 0x70, 0x0a, 0x05, 0x4d, 0x65, 0x72, 0x67, 0x65,
    0x12, 0x21, 0x0a, 0x0c, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x62, 0x61, 0x73, 0x65, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x6d, 0x65, 0x72, 0x67,
    0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x04, 0x69, 0x6e, 0x66, 0x6f,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x52, 0x04, 0x69, 0x6e, 0x66, 0x6f, 0x22, 0x9c, 0x01, 0x0a, 0x09, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x53, 0x65, 0x74, 0x12, 0x23, 0x0a, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x53, 0x65,
    0x74, 0x2e, 0x4b, 0x69, 0x6e, 0x64, 0x52, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x12, 0x1c, 0x0a, 0x05,
    0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x44, 0x65,
    0x6c, 0x74, 0x61, 0x52, 0x05, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x12, 0x1c, 0x0a, 0x05, 0x6d, 0x65,
    0x72, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x4d, 0x65, 0x72, 0x67,
    0x65, 0x52, 0x05, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x22, 0x2e, 0x0a, 0x04, 0x4b, 0x69, 0x6e, 0x64,
    0x12, 0x10, 0x0a, 0x0c, 0x4b, 0x49, 0x4e, 0x44, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e,
    0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x44, 0x45, 0x4c, 0x54, 0x41, 0x10, 0x02, 0x12, 0x09, 0x0a,
    0x05, 0x4d, 0x45, 0x52, 0x47, 0x45, 0x10, 0x03, 0x22, 0x60, 0x0a, 0x17, 0x52, 0x65, 0x76, 0x69,
    0x73, 0x69, 0x6f, 0x6e, 0x54, 0x61, 0x67, 0x67, 0x65, 0x64, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x53, 0x65, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x29, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x53, 0x65, 0x74, 0x52,
    0x09, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x53, 0x65, 0x74, 0x22, 0x63, 0x0a, 0x04, 0x44, 0x69,
    0x66, 0x66, 0x12, 0x23, 0x0a, 0x0d, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x72, 0x65, 0x76, 0x69, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x66, 0x72, 0x6f, 0x6d, 0x52,
    0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x15, 0x0a, 0x03, 0x6f, 0x70, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x03, 0x2e, 0x4f, 0x70, 0x52, 0x03, 0x6f, 0x70, 0x73, 0x12, 0x1f,
    0x0a, 0x0b, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x0a, 0x74, 0x6f, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x22,
    0xfc, 0x01, 0x0a, 0x08, 0x4c, 0x69, 0x73, 0x74, 0x44, 0x75, 0x6d, 0x70, 0x12, 0x26, 0x0a, 0x0e,
    0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x52, 0x65, 0x76, 0x69,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x2f, 0x0a, 0x0a,
    0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0f, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x29, 0x0a,
    0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x08,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x26, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x4c, 0x69, 0x73,
    0x74, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73,
    0x12, 0x2c, 0x0a, 0x0d, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x44, 0x65, 0x6c, 0x74, 0x61,
    0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x52,
    0x0d, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x73, 0x22, 0xe8,
    0x01, 0x0a, 0x0b, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x12, 0x22,
    0x0a, 0x0c, 0x62, 0x61, 0x73, 0x65, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x62, 0x61, 0x73, 0x65, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x06, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x06, 0x2e, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x52, 0x06, 0x64, 0x65, 0x6c, 0x74,
    0x61, 0x73, 0x12, 0x36, 0x0a, 0x16, 0x77, 0x61, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x16, 0x77, 0x61, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x69, 0x6e,
    0x67, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x26, 0x0a, 0x0e, 0x77, 0x61,
    0x6e, 0x74, 0x53, 0x79, 0x6e, 0x63, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0e, 0x77, 0x61, 0x6e, 0x74, 0x53, 0x79, 0x6e, 0x63, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x1d, 0x0a, 0x04, 0x64, 0x75, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x09, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x44, 0x75, 0x6d, 0x70, 0x52, 0x04, 0x64, 0x75, 0x6d,
    0x70, 0x12, 0x16, 0x0a, 0x06, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28,
    0x05, 0x52, 0x06, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x73, 0x22, 0xfb, 0x03, 0x0a, 0x13, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x16, 0x0a,
    0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x2f, 0x0a, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x4c, 0x69, 0x73, 0x74,
    0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x29, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73,
    0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43,
    0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75,
    0x6d, 0x12, 0x26, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x52,
    0x08, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x19, 0x0a, 0x04, 0x64, 0x69, 0x66,
    0x66, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x44, 0x69, 0x66, 0x66, 0x52, 0x04,
    0x64, 0x69, 0x66, 0x66, 0x12, 0x25, 0x0a, 0x0a, 0x73, 0x79, 0x6e, 0x63, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x44, 0x69, 0x66, 0x66, 0x52,
    0x0a, 0x73, 0x79, 0x6e, 0x63, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x2e, 0x0a, 0x12, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x12, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x24, 0x0a, 0x0d, 0x6d,
    0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64, 0x73, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0d, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64,
    0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x70, 0x54, 0x6f, 0x44, 0x61, 0x74, 0x65, 0x18, 0x0a, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x08, 0x75, 0x70, 0x54, 0x6f, 0x44, 0x61, 0x74, 0x65, 0x12, 0x3a, 0x0a,
    0x0d, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0c,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x72, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x06, 0x69, 0x73, 0x73,
    0x75, 0x65, 0x73, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52, 0x06, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73, 0x12,
    0x16, 0x0a, 0x06, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x73, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x05, 0x52,
    0x06, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x73, 0x4a, 0xea, 0x1c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x54, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03,
    0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x07, 0x1f, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x05, 0x07, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x07, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x08, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x08, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0a, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0a, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x1a, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0b, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0c, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x0d, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0d,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x0e, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x0e, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0f, 0x04, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x0f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x0f, 0x1e, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x12, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x13, 0x04, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x13, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x13, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x14,
    0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x14, 0x0d, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x10, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x15, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x15, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x18,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x1f, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x18, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x19, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x13, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x1a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1a, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x23,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x04, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1b, 0x1f, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1e, 0x00,
    0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x1f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1f, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x20, 0x04, 0x24,
    0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x20, 0x09, 0x0d, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x21, 0x08, 0x1b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x14, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x21, 0x17, 0x1a, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x14, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x08, 0x0d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x22, 0x10, 0x13, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x23, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x23, 0x10, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x25, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x25, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x25, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x1b,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x26, 0x04, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x26, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x26, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x29, 0x00,
    0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x29, 0x08, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x2a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2a, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2a, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x04, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2b, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2b, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x24, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x2e, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x0c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x2f, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x2f, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x30,
    0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x30, 0x0d, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x10, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x02, 0x12, 0x03, 0x31, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x31, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x31, 0x13,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x21, 0x24, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x34, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x06, 0x01, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12,
    0x03, 0x35, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x35,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x13, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x24, 0x27, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x36, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x36, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x36, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36, 0x1c,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x37, 0x04, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x37, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x1c, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x37, 0x29, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03,
    0x38, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x38, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x38, 0x0d, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x38, 0x1a, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x38, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x04, 0x12, 0x03, 0x39, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x39, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x39, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x39,
    0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x39, 0x22, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x3a, 0x04, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x05, 0x06, 0x12, 0x03, 0x3a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x3a, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x3a, 0x23, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3d, 0x00, 0x44,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x3e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x3e, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e,
    0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x3f, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12,
    0x03, 0x40, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x40,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x40, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40, 0x12, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x2b, 0x2e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x41, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x41, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x41, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x41, 0x23,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x42, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x06, 0x12, 0x03, 0x42, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x42, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x42, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x03,
    0x43, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x03, 0x43, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x05, 0x12, 0x03, 0x43, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x43, 0x13, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x43, 0x1c, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x08, 0x12, 0x04, 0x46, 0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03,
    0x46, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x47, 0x04, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01,
    0x12, 0x03, 0x48, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x13, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x49, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x49, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x49, 0x1c, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49,
    0x29, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x04, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4a, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x1a, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x4a, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12,
    0x03, 0x4b, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x03, 0x4b, 0x0d, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4b, 0x17, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4b, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x4c, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x4c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x4c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x4c, 0x19,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x06, 0x12, 0x03, 0x4d, 0x04, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x06, 0x06, 0x12, 0x03, 0x4d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x4d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x4d, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x07, 0x12, 0x03,
    0x4e, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x04, 0x12, 0x03, 0x4e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4e, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x13, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4e, 0x28, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x08, 0x12, 0x03, 0x4f, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x4f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4f,
    0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4f, 0x22, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x09, 0x12, 0x03, 0x50, 0x04, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x09, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x09, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x50, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x50, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x0a, 0x12, 0x03, 0x51,
    0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x51, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x51, 0x0d, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x51, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x51, 0x31, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x0b, 0x12, 0x03, 0x52, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0b, 0x06, 0x12, 0x03,
    0x52, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x52, 0x19,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x52, 0x22, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x0c, 0x12, 0x03, 0x53, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x53, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x53, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x53, 0x1c, 0x1f,
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
