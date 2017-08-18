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
pub struct ListChecksum {
    // message fields
    version: ::std::option::Option<i32>,
    sha1: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListChecksum {}

impl ListChecksum {
    pub fn new() -> ListChecksum {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListChecksum {
        static mut instance: ::protobuf::lazy::Lazy<ListChecksum> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListChecksum,
        };
        unsafe {
            instance.get(ListChecksum::new)
        }
    }

    // optional int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }

    // optional bytes sha1 = 4;

    pub fn clear_sha1(&mut self) {
        self.sha1.clear();
    }

    pub fn has_sha1(&self) -> bool {
        self.sha1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha1(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha1(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha1.is_none() {
            self.sha1.set_default();
        };
        self.sha1.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha1(&mut self) -> ::std::vec::Vec<u8> {
        self.sha1.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_sha1(&self) -> &[u8] {
        match self.sha1.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_sha1_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.sha1
    }

    fn mut_sha1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.sha1
    }
}

impl ::protobuf::Message for ListChecksum {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha1)?;
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sha1.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.sha1.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for ListChecksum {
    fn new() -> ListChecksum {
        ListChecksum::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListChecksum>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    ListChecksum::get_version_for_reflect,
                    ListChecksum::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha1",
                    ListChecksum::get_sha1_for_reflect,
                    ListChecksum::mut_sha1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListChecksum>(
                    "ListChecksum",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListChecksum {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_sha1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListChecksum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListChecksum {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DownloadFormat {
    // message fields
    codec: ::std::option::Option<DownloadFormat_Codec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadFormat {}

impl DownloadFormat {
    pub fn new() -> DownloadFormat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadFormat {
        static mut instance: ::protobuf::lazy::Lazy<DownloadFormat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadFormat,
        };
        unsafe {
            instance.get(DownloadFormat::new)
        }
    }

    // optional .DownloadFormat.Codec codec = 1;

    pub fn clear_codec(&mut self) {
        self.codec = ::std::option::Option::None;
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: DownloadFormat_Codec) {
        self.codec = ::std::option::Option::Some(v);
    }

    pub fn get_codec(&self) -> DownloadFormat_Codec {
        self.codec.unwrap_or(DownloadFormat_Codec::CODEC_UNKNOWN)
    }

    fn get_codec_for_reflect(&self) -> &::std::option::Option<DownloadFormat_Codec> {
        &self.codec
    }

    fn mut_codec_for_reflect(&mut self) -> &mut ::std::option::Option<DownloadFormat_Codec> {
        &mut self.codec
    }
}

impl ::protobuf::Message for DownloadFormat {
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
                    self.codec = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.codec {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.codec {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for DownloadFormat {
    fn new() -> DownloadFormat {
        DownloadFormat::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadFormat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DownloadFormat_Codec>>(
                    "codec",
                    DownloadFormat::get_codec_for_reflect,
                    DownloadFormat::mut_codec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadFormat>(
                    "DownloadFormat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadFormat {
    fn clear(&mut self) {
        self.clear_codec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DownloadFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DownloadFormat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DownloadFormat_Codec {
    CODEC_UNKNOWN = 0,
    OGG_VORBIS = 1,
    FLAC = 2,
    MPEG_1_LAYER_3 = 3,
}

impl ::protobuf::ProtobufEnum for DownloadFormat_Codec {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DownloadFormat_Codec> {
        match value {
            0 => ::std::option::Option::Some(DownloadFormat_Codec::CODEC_UNKNOWN),
            1 => ::std::option::Option::Some(DownloadFormat_Codec::OGG_VORBIS),
            2 => ::std::option::Option::Some(DownloadFormat_Codec::FLAC),
            3 => ::std::option::Option::Some(DownloadFormat_Codec::MPEG_1_LAYER_3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DownloadFormat_Codec] = &[
            DownloadFormat_Codec::CODEC_UNKNOWN,
            DownloadFormat_Codec::OGG_VORBIS,
            DownloadFormat_Codec::FLAC,
            DownloadFormat_Codec::MPEG_1_LAYER_3,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DownloadFormat_Codec>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DownloadFormat_Codec", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DownloadFormat_Codec {
}

impl ::protobuf::reflect::ProtobufValue for DownloadFormat_Codec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListAttributes {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularField<::std::string::String>,
    picture: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    collaborative: ::std::option::Option<bool>,
    pl3_version: ::protobuf::SingularField<::std::string::String>,
    deleted_by_owner: ::std::option::Option<bool>,
    restricted_collaborative: ::std::option::Option<bool>,
    deprecated_client_id: ::std::option::Option<i64>,
    public_starred: ::std::option::Option<bool>,
    client_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListAttributes {}

impl ListAttributes {
    pub fn new() -> ListAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListAttributes {
        static mut instance: ::protobuf::lazy::Lazy<ListAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListAttributes,
        };
        unsafe {
            instance.get(ListAttributes::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }

    // optional bytes picture = 3;

    pub fn clear_picture(&mut self) {
        self.picture.clear();
    }

    pub fn has_picture(&self) -> bool {
        self.picture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_picture(&mut self, v: ::std::vec::Vec<u8>) {
        self.picture = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_picture(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.picture.is_none() {
            self.picture.set_default();
        };
        self.picture.as_mut().unwrap()
    }

    // Take field
    pub fn take_picture(&mut self) -> ::std::vec::Vec<u8> {
        self.picture.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_picture(&self) -> &[u8] {
        match self.picture.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_picture_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.picture
    }

    fn mut_picture_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.picture
    }

    // optional bool collaborative = 4;

    pub fn clear_collaborative(&mut self) {
        self.collaborative = ::std::option::Option::None;
    }

    pub fn has_collaborative(&self) -> bool {
        self.collaborative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collaborative(&mut self, v: bool) {
        self.collaborative = ::std::option::Option::Some(v);
    }

    pub fn get_collaborative(&self) -> bool {
        self.collaborative.unwrap_or(false)
    }

    fn get_collaborative_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.collaborative
    }

    fn mut_collaborative_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.collaborative
    }

    // optional string pl3_version = 5;

    pub fn clear_pl3_version(&mut self) {
        self.pl3_version.clear();
    }

    pub fn has_pl3_version(&self) -> bool {
        self.pl3_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pl3_version(&mut self, v: ::std::string::String) {
        self.pl3_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pl3_version(&mut self) -> &mut ::std::string::String {
        if self.pl3_version.is_none() {
            self.pl3_version.set_default();
        };
        self.pl3_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_pl3_version(&mut self) -> ::std::string::String {
        self.pl3_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pl3_version(&self) -> &str {
        match self.pl3_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pl3_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pl3_version
    }

    fn mut_pl3_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pl3_version
    }

    // optional bool deleted_by_owner = 6;

    pub fn clear_deleted_by_owner(&mut self) {
        self.deleted_by_owner = ::std::option::Option::None;
    }

    pub fn has_deleted_by_owner(&self) -> bool {
        self.deleted_by_owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleted_by_owner(&mut self, v: bool) {
        self.deleted_by_owner = ::std::option::Option::Some(v);
    }

    pub fn get_deleted_by_owner(&self) -> bool {
        self.deleted_by_owner.unwrap_or(false)
    }

    fn get_deleted_by_owner_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deleted_by_owner
    }

    fn mut_deleted_by_owner_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deleted_by_owner
    }

    // optional bool restricted_collaborative = 7;

    pub fn clear_restricted_collaborative(&mut self) {
        self.restricted_collaborative = ::std::option::Option::None;
    }

    pub fn has_restricted_collaborative(&self) -> bool {
        self.restricted_collaborative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_restricted_collaborative(&mut self, v: bool) {
        self.restricted_collaborative = ::std::option::Option::Some(v);
    }

    pub fn get_restricted_collaborative(&self) -> bool {
        self.restricted_collaborative.unwrap_or(false)
    }

    fn get_restricted_collaborative_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.restricted_collaborative
    }

    fn mut_restricted_collaborative_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.restricted_collaborative
    }

    // optional int64 deprecated_client_id = 8;

    pub fn clear_deprecated_client_id(&mut self) {
        self.deprecated_client_id = ::std::option::Option::None;
    }

    pub fn has_deprecated_client_id(&self) -> bool {
        self.deprecated_client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated_client_id(&mut self, v: i64) {
        self.deprecated_client_id = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated_client_id(&self) -> i64 {
        self.deprecated_client_id.unwrap_or(0)
    }

    fn get_deprecated_client_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.deprecated_client_id
    }

    fn mut_deprecated_client_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.deprecated_client_id
    }

    // optional bool public_starred = 9;

    pub fn clear_public_starred(&mut self) {
        self.public_starred = ::std::option::Option::None;
    }

    pub fn has_public_starred(&self) -> bool {
        self.public_starred.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_starred(&mut self, v: bool) {
        self.public_starred = ::std::option::Option::Some(v);
    }

    pub fn get_public_starred(&self) -> bool {
        self.public_starred.unwrap_or(false)
    }

    fn get_public_starred_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.public_starred
    }

    fn mut_public_starred_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.public_starred
    }

    // optional string client_id = 10;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::string::String) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::string::String {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::string::String {
        self.client_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_client_id(&self) -> &str {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_client_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.client_id
    }

    fn mut_client_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.client_id
    }
}

impl ::protobuf::Message for ListAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.picture)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.collaborative = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pl3_version)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.deleted_by_owner = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.restricted_collaborative = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.deprecated_client_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.public_starred = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.picture.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.collaborative {
            my_size += 2;
        };
        if let Some(v) = self.pl3_version.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.deleted_by_owner {
            my_size += 2;
        };
        if let Some(v) = self.restricted_collaborative {
            my_size += 2;
        };
        if let Some(v) = self.deprecated_client_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.public_starred {
            my_size += 2;
        };
        if let Some(v) = self.client_id.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.description.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.picture.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.collaborative {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.pl3_version.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.deleted_by_owner {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.restricted_collaborative {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.deprecated_client_id {
            os.write_int64(8, v)?;
        };
        if let Some(v) = self.public_starred {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.client_id.as_ref() {
            os.write_string(10, &v)?;
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

impl ::protobuf::MessageStatic for ListAttributes {
    fn new() -> ListAttributes {
        ListAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ListAttributes::get_name_for_reflect,
                    ListAttributes::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    ListAttributes::get_description_for_reflect,
                    ListAttributes::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "picture",
                    ListAttributes::get_picture_for_reflect,
                    ListAttributes::mut_picture_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "collaborative",
                    ListAttributes::get_collaborative_for_reflect,
                    ListAttributes::mut_collaborative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pl3_version",
                    ListAttributes::get_pl3_version_for_reflect,
                    ListAttributes::mut_pl3_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deleted_by_owner",
                    ListAttributes::get_deleted_by_owner_for_reflect,
                    ListAttributes::mut_deleted_by_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "restricted_collaborative",
                    ListAttributes::get_restricted_collaborative_for_reflect,
                    ListAttributes::mut_restricted_collaborative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "deprecated_client_id",
                    ListAttributes::get_deprecated_client_id_for_reflect,
                    ListAttributes::mut_deprecated_client_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "public_starred",
                    ListAttributes::get_public_starred_for_reflect,
                    ListAttributes::mut_public_starred_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client_id",
                    ListAttributes::get_client_id_for_reflect,
                    ListAttributes::mut_client_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListAttributes>(
                    "ListAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListAttributes {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_description();
        self.clear_picture();
        self.clear_collaborative();
        self.clear_pl3_version();
        self.clear_deleted_by_owner();
        self.clear_restricted_collaborative();
        self.clear_deprecated_client_id();
        self.clear_public_starred();
        self.clear_client_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ItemAttributes {
    // message fields
    added_by: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<i64>,
    message: ::protobuf::SingularField<::std::string::String>,
    seen: ::std::option::Option<bool>,
    download_count: ::std::option::Option<i64>,
    download_format: ::protobuf::SingularPtrField<DownloadFormat>,
    sevendigital_id: ::protobuf::SingularField<::std::string::String>,
    sevendigital_left: ::std::option::Option<i64>,
    seen_at: ::std::option::Option<i64>,
    public: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ItemAttributes {}

impl ItemAttributes {
    pub fn new() -> ItemAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ItemAttributes {
        static mut instance: ::protobuf::lazy::Lazy<ItemAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ItemAttributes,
        };
        unsafe {
            instance.get(ItemAttributes::new)
        }
    }

    // optional string added_by = 1;

    pub fn clear_added_by(&mut self) {
        self.added_by.clear();
    }

    pub fn has_added_by(&self) -> bool {
        self.added_by.is_some()
    }

    // Param is passed by value, moved
    pub fn set_added_by(&mut self, v: ::std::string::String) {
        self.added_by = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_added_by(&mut self) -> &mut ::std::string::String {
        if self.added_by.is_none() {
            self.added_by.set_default();
        };
        self.added_by.as_mut().unwrap()
    }

    // Take field
    pub fn take_added_by(&mut self) -> ::std::string::String {
        self.added_by.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_added_by(&self) -> &str {
        match self.added_by.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_added_by_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.added_by
    }

    fn mut_added_by_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.added_by
    }

    // optional int64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional bool seen = 4;

    pub fn clear_seen(&mut self) {
        self.seen = ::std::option::Option::None;
    }

    pub fn has_seen(&self) -> bool {
        self.seen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seen(&mut self, v: bool) {
        self.seen = ::std::option::Option::Some(v);
    }

    pub fn get_seen(&self) -> bool {
        self.seen.unwrap_or(false)
    }

    fn get_seen_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.seen
    }

    fn mut_seen_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.seen
    }

    // optional int64 download_count = 5;

    pub fn clear_download_count(&mut self) {
        self.download_count = ::std::option::Option::None;
    }

    pub fn has_download_count(&self) -> bool {
        self.download_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_download_count(&mut self, v: i64) {
        self.download_count = ::std::option::Option::Some(v);
    }

    pub fn get_download_count(&self) -> i64 {
        self.download_count.unwrap_or(0)
    }

    fn get_download_count_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.download_count
    }

    fn mut_download_count_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.download_count
    }

    // optional .DownloadFormat download_format = 6;

    pub fn clear_download_format(&mut self) {
        self.download_format.clear();
    }

    pub fn has_download_format(&self) -> bool {
        self.download_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_download_format(&mut self, v: DownloadFormat) {
        self.download_format = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_download_format(&mut self) -> &mut DownloadFormat {
        if self.download_format.is_none() {
            self.download_format.set_default();
        };
        self.download_format.as_mut().unwrap()
    }

    // Take field
    pub fn take_download_format(&mut self) -> DownloadFormat {
        self.download_format.take().unwrap_or_else(|| DownloadFormat::new())
    }

    pub fn get_download_format(&self) -> &DownloadFormat {
        self.download_format.as_ref().unwrap_or_else(|| DownloadFormat::default_instance())
    }

    fn get_download_format_for_reflect(&self) -> &::protobuf::SingularPtrField<DownloadFormat> {
        &self.download_format
    }

    fn mut_download_format_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DownloadFormat> {
        &mut self.download_format
    }

    // optional string sevendigital_id = 7;

    pub fn clear_sevendigital_id(&mut self) {
        self.sevendigital_id.clear();
    }

    pub fn has_sevendigital_id(&self) -> bool {
        self.sevendigital_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sevendigital_id(&mut self, v: ::std::string::String) {
        self.sevendigital_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sevendigital_id(&mut self) -> &mut ::std::string::String {
        if self.sevendigital_id.is_none() {
            self.sevendigital_id.set_default();
        };
        self.sevendigital_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_sevendigital_id(&mut self) -> ::std::string::String {
        self.sevendigital_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sevendigital_id(&self) -> &str {
        match self.sevendigital_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sevendigital_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sevendigital_id
    }

    fn mut_sevendigital_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sevendigital_id
    }

    // optional int64 sevendigital_left = 8;

    pub fn clear_sevendigital_left(&mut self) {
        self.sevendigital_left = ::std::option::Option::None;
    }

    pub fn has_sevendigital_left(&self) -> bool {
        self.sevendigital_left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sevendigital_left(&mut self, v: i64) {
        self.sevendigital_left = ::std::option::Option::Some(v);
    }

    pub fn get_sevendigital_left(&self) -> i64 {
        self.sevendigital_left.unwrap_or(0)
    }

    fn get_sevendigital_left_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sevendigital_left
    }

    fn mut_sevendigital_left_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sevendigital_left
    }

    // optional int64 seen_at = 9;

    pub fn clear_seen_at(&mut self) {
        self.seen_at = ::std::option::Option::None;
    }

    pub fn has_seen_at(&self) -> bool {
        self.seen_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seen_at(&mut self, v: i64) {
        self.seen_at = ::std::option::Option::Some(v);
    }

    pub fn get_seen_at(&self) -> i64 {
        self.seen_at.unwrap_or(0)
    }

    fn get_seen_at_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.seen_at
    }

    fn mut_seen_at_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.seen_at
    }

    // optional bool public = 10;

    pub fn clear_public(&mut self) {
        self.public = ::std::option::Option::None;
    }

    pub fn has_public(&self) -> bool {
        self.public.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public(&mut self, v: bool) {
        self.public = ::std::option::Option::Some(v);
    }

    pub fn get_public(&self) -> bool {
        self.public.unwrap_or(false)
    }

    fn get_public_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.public
    }

    fn mut_public_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.public
    }
}

impl ::protobuf::Message for ItemAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.added_by)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.seen = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.download_count = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.download_format)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sevendigital_id)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.sevendigital_left = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.seen_at = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.public = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.added_by.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.seen {
            my_size += 2;
        };
        if let Some(v) = self.download_count {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.download_format.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.sevendigital_id.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.sevendigital_left {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seen_at {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.public {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.added_by.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_int64(2, v)?;
        };
        if let Some(v) = self.message.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.seen {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.download_count {
            os.write_int64(5, v)?;
        };
        if let Some(v) = self.download_format.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.sevendigital_id.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.sevendigital_left {
            os.write_int64(8, v)?;
        };
        if let Some(v) = self.seen_at {
            os.write_int64(9, v)?;
        };
        if let Some(v) = self.public {
            os.write_bool(10, v)?;
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

impl ::protobuf::MessageStatic for ItemAttributes {
    fn new() -> ItemAttributes {
        ItemAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<ItemAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "added_by",
                    ItemAttributes::get_added_by_for_reflect,
                    ItemAttributes::mut_added_by_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    ItemAttributes::get_timestamp_for_reflect,
                    ItemAttributes::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    ItemAttributes::get_message_for_reflect,
                    ItemAttributes::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "seen",
                    ItemAttributes::get_seen_for_reflect,
                    ItemAttributes::mut_seen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "download_count",
                    ItemAttributes::get_download_count_for_reflect,
                    ItemAttributes::mut_download_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DownloadFormat>>(
                    "download_format",
                    ItemAttributes::get_download_format_for_reflect,
                    ItemAttributes::mut_download_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sevendigital_id",
                    ItemAttributes::get_sevendigital_id_for_reflect,
                    ItemAttributes::mut_sevendigital_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "sevendigital_left",
                    ItemAttributes::get_sevendigital_left_for_reflect,
                    ItemAttributes::mut_sevendigital_left_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "seen_at",
                    ItemAttributes::get_seen_at_for_reflect,
                    ItemAttributes::mut_seen_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "public",
                    ItemAttributes::get_public_for_reflect,
                    ItemAttributes::mut_public_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ItemAttributes>(
                    "ItemAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ItemAttributes {
    fn clear(&mut self) {
        self.clear_added_by();
        self.clear_timestamp();
        self.clear_message();
        self.clear_seen();
        self.clear_download_count();
        self.clear_download_format();
        self.clear_sevendigital_id();
        self.clear_sevendigital_left();
        self.clear_seen_at();
        self.clear_public();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ItemAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ItemAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StringAttribute {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringAttribute {}

impl StringAttribute {
    pub fn new() -> StringAttribute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringAttribute {
        static mut instance: ::protobuf::lazy::Lazy<StringAttribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringAttribute,
        };
        unsafe {
            instance.get(StringAttribute::new)
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for StringAttribute {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for StringAttribute {
    fn new() -> StringAttribute {
        StringAttribute::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringAttribute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    StringAttribute::get_key_for_reflect,
                    StringAttribute::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    StringAttribute::get_value_for_reflect,
                    StringAttribute::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringAttribute>(
                    "StringAttribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringAttribute {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StringAttribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StringAttribute {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StringAttributes {
    // message fields
    attribute: ::protobuf::RepeatedField<StringAttribute>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringAttributes {}

impl StringAttributes {
    pub fn new() -> StringAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringAttributes {
        static mut instance: ::protobuf::lazy::Lazy<StringAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringAttributes,
        };
        unsafe {
            instance.get(StringAttributes::new)
        }
    }

    // repeated .StringAttribute attribute = 1;

    pub fn clear_attribute(&mut self) {
        self.attribute.clear();
    }

    // Param is passed by value, moved
    pub fn set_attribute(&mut self, v: ::protobuf::RepeatedField<StringAttribute>) {
        self.attribute = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attribute(&mut self) -> &mut ::protobuf::RepeatedField<StringAttribute> {
        &mut self.attribute
    }

    // Take field
    pub fn take_attribute(&mut self) -> ::protobuf::RepeatedField<StringAttribute> {
        ::std::mem::replace(&mut self.attribute, ::protobuf::RepeatedField::new())
    }

    pub fn get_attribute(&self) -> &[StringAttribute] {
        &self.attribute
    }

    fn get_attribute_for_reflect(&self) -> &::protobuf::RepeatedField<StringAttribute> {
        &self.attribute
    }

    fn mut_attribute_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StringAttribute> {
        &mut self.attribute
    }
}

impl ::protobuf::Message for StringAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attribute)?;
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
        for value in &self.attribute {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.attribute {
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

impl ::protobuf::MessageStatic for StringAttributes {
    fn new() -> StringAttributes {
        StringAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StringAttribute>>(
                    "attribute",
                    StringAttributes::get_attribute_for_reflect,
                    StringAttributes::mut_attribute_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringAttributes>(
                    "StringAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringAttributes {
    fn clear(&mut self) {
        self.clear_attribute();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StringAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StringAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x6d, 0x65, 0x74, 0x61, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3c, 0x0a, 0x0c, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x12, 0x0a, 0x04, 0x73, 0x68, 0x61, 0x31, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x73,
    0x68, 0x61, 0x31, 0x22, 0x87, 0x01, 0x0a, 0x0e, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x2b, 0x0a, 0x05, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x52, 0x05, 0x63, 0x6f,
    0x64, 0x65, 0x63, 0x22, 0x48, 0x0a, 0x05, 0x43, 0x6f, 0x64, 0x65, 0x63, 0x12, 0x11, 0x0a, 0x0d,
    0x43, 0x4f, 0x44, 0x45, 0x43, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12,
    0x0e, 0x0a, 0x0a, 0x4f, 0x47, 0x47, 0x5f, 0x56, 0x4f, 0x52, 0x42, 0x49, 0x53, 0x10, 0x01, 0x12,
    0x08, 0x0a, 0x04, 0x46, 0x4c, 0x41, 0x43, 0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x4d, 0x50, 0x45,
    0x47, 0x5f, 0x31, 0x5f, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x5f, 0x33, 0x10, 0x03, 0x22, 0x82, 0x03,
    0x0a, 0x0e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73,
    0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x69, 0x63, 0x74, 0x75, 0x72,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x69, 0x63, 0x74, 0x75, 0x72, 0x65,
    0x12, 0x24, 0x0a, 0x0d, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x62, 0x6f, 0x72, 0x61, 0x74, 0x69, 0x76,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x62, 0x6f,
    0x72, 0x61, 0x74, 0x69, 0x76, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x33, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x70, 0x6c, 0x33,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x64, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0e, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x42, 0x79, 0x4f, 0x77, 0x6e, 0x65,
    0x72, 0x12, 0x39, 0x0a, 0x18, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x65, 0x64, 0x5f,
    0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x62, 0x6f, 0x72, 0x61, 0x74, 0x69, 0x76, 0x65, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x17, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x65, 0x64, 0x43,
    0x6f, 0x6c, 0x6c, 0x61, 0x62, 0x6f, 0x72, 0x61, 0x74, 0x69, 0x76, 0x65, 0x12, 0x30, 0x0a, 0x14,
    0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x03, 0x52, 0x12, 0x64, 0x65, 0x70, 0x72,
    0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x25,
    0x0a, 0x0e, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x72, 0x65, 0x64,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x53, 0x74,
    0x61, 0x72, 0x72, 0x65, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x49, 0x64, 0x22, 0xdf, 0x02, 0x0a, 0x0e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x61, 0x64, 0x64, 0x65, 0x64, 0x5f, 0x62,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x65, 0x64, 0x42, 0x79,
    0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x18,
    0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x65, 0x65, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x73, 0x65, 0x65, 0x6e, 0x12, 0x25, 0x0a, 0x0e,
    0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x0d, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x43, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x38, 0x0a, 0x0f, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x5f,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x44,
    0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x52, 0x0e, 0x64,
    0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x27, 0x0a,
    0x0f, 0x73, 0x65, 0x76, 0x65, 0x6e, 0x64, 0x69, 0x67, 0x69, 0x74, 0x61, 0x6c, 0x5f, 0x69, 0x64,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x73, 0x65, 0x76, 0x65, 0x6e, 0x64, 0x69, 0x67,
    0x69, 0x74, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x2b, 0x0a, 0x11, 0x73, 0x65, 0x76, 0x65, 0x6e, 0x64,
    0x69, 0x67, 0x69, 0x74, 0x61, 0x6c, 0x5f, 0x6c, 0x65, 0x66, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x03, 0x52, 0x10, 0x73, 0x65, 0x76, 0x65, 0x6e, 0x64, 0x69, 0x67, 0x69, 0x74, 0x61, 0x6c, 0x4c,
    0x65, 0x66, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x73, 0x65, 0x65, 0x6e, 0x5f, 0x61, 0x74, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x06, 0x73, 0x65, 0x65, 0x6e, 0x41, 0x74, 0x12, 0x16, 0x0a, 0x06,
    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x70, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x22, 0x39, 0x0a, 0x0f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x74,
    0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x42, 0x0a, 0x10, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x12, 0x2e, 0x0a, 0x09, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x41,
    0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x52, 0x09, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x4a, 0xfc, 0x10, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x32, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x03, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x03, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04,
    0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1a, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x07, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x08, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x08, 0x1b, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12,
    0x04, 0x09, 0x04, 0x0e, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x09, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a,
    0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a,
    0x18, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x15,
    0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x0f, 0x12,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x1d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x16, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x19, 0x1c, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x11, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x11, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x12, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x14, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x13, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x13, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x22,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x14, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x14, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x15, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x15, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x12, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x15, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x16,
    0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x16, 0x22, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x04, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x17, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x17, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x17, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x18,
    0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x18, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x18, 0x12, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x18, 0x2d, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x07, 0x12, 0x03, 0x19, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03,
    0x19, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x19, 0x13,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x19, 0x2a, 0x2d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1a, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x08, 0x05, 0x12, 0x03, 0x1a, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x1a, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x1a, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x1b, 0x04,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1b, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1b, 0x20, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x1e, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x04, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1f, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x20, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x13, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x12, 0x03, 0x21, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x21, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21,
    0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x1e, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x22, 0x04, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x22, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x22, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x22, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x23,
    0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x23, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x23, 0x24, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x06, 0x12, 0x03,
    0x24, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x24, 0x1c,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x24, 0x2e, 0x31, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x25, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x25, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x25, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x25, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12, 0x03, 0x26, 0x04,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x26, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x26, 0x13, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x26, 0x27, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x08, 0x12, 0x03, 0x27, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x04, 0x12,
    0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x05, 0x12, 0x03, 0x27,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01, 0x12, 0x03, 0x27, 0x13, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x03, 0x27, 0x1d, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x03, 0x28, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x09, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x09, 0x05, 0x12, 0x03, 0x28, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x28, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x03, 0x12, 0x03,
    0x28, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x2c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x2c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c,
    0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1a, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x04, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x2d, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x2d, 0x1c, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x30, 0x00, 0x32,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x30, 0x08, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x31, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x31, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x31, 0x1d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31,
    0x29, 0x2c,
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
