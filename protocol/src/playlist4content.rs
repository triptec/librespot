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
pub struct Item {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    attributes: ::protobuf::SingularPtrField<super::playlist4meta::ItemAttributes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Item {}

impl Item {
    pub fn new() -> Item {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Item {
        static mut instance: ::protobuf::lazy::Lazy<Item> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Item,
        };
        unsafe {
            instance.get(Item::new)
        }
    }

    // optional string uri = 1;

    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    pub fn has_uri(&self) -> bool {
        self.uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        if self.uri.is_none() {
            self.uri.set_default();
        };
        self.uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        self.uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uri(&self) -> &str {
        match self.uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.uri
    }

    fn mut_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.uri
    }

    // optional .ItemAttributes attributes = 2;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    pub fn has_attributes(&self) -> bool {
        self.attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: super::playlist4meta::ItemAttributes) {
        self.attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attributes(&mut self) -> &mut super::playlist4meta::ItemAttributes {
        if self.attributes.is_none() {
            self.attributes.set_default();
        };
        self.attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_attributes(&mut self) -> super::playlist4meta::ItemAttributes {
        self.attributes.take().unwrap_or_else(|| super::playlist4meta::ItemAttributes::new())
    }

    pub fn get_attributes(&self) -> &super::playlist4meta::ItemAttributes {
        self.attributes.as_ref().unwrap_or_else(|| super::playlist4meta::ItemAttributes::default_instance())
    }

    fn get_attributes_for_reflect(&self) -> &::protobuf::SingularPtrField<super::playlist4meta::ItemAttributes> {
        &self.attributes
    }

    fn mut_attributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::playlist4meta::ItemAttributes> {
        &mut self.attributes
    }
}

impl ::protobuf::Message for Item {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.attributes)?;
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
        if let Some(v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uri.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.attributes.as_ref() {
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

impl ::protobuf::MessageStatic for Item {
    fn new() -> Item {
        Item::new()
    }

    fn descriptor_static(_: ::std::option::Option<Item>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    Item::get_uri_for_reflect,
                    Item::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4meta::ItemAttributes>>(
                    "attributes",
                    Item::get_attributes_for_reflect,
                    Item::mut_attributes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Item>(
                    "Item",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Item {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_attributes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Item {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Item {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListItems {
    // message fields
    pos: ::std::option::Option<i32>,
    truncated: ::std::option::Option<bool>,
    items: ::protobuf::RepeatedField<Item>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListItems {}

impl ListItems {
    pub fn new() -> ListItems {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListItems {
        static mut instance: ::protobuf::lazy::Lazy<ListItems> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListItems,
        };
        unsafe {
            instance.get(ListItems::new)
        }
    }

    // optional int32 pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos = ::std::option::Option::None;
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: i32) {
        self.pos = ::std::option::Option::Some(v);
    }

    pub fn get_pos(&self) -> i32 {
        self.pos.unwrap_or(0)
    }

    fn get_pos_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pos
    }

    // optional bool truncated = 2;

    pub fn clear_truncated(&mut self) {
        self.truncated = ::std::option::Option::None;
    }

    pub fn has_truncated(&self) -> bool {
        self.truncated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_truncated(&mut self, v: bool) {
        self.truncated = ::std::option::Option::Some(v);
    }

    pub fn get_truncated(&self) -> bool {
        self.truncated.unwrap_or(false)
    }

    fn get_truncated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.truncated
    }

    fn mut_truncated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.truncated
    }

    // repeated .Item items = 3;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Item>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Item> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Item> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Item] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<Item> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Item> {
        &mut self.items
    }
}

impl ::protobuf::Message for ListItems {
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
                    self.pos = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.truncated = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        if let Some(v) = self.pos {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.truncated {
            my_size += 2;
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pos {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.truncated {
            os.write_bool(2, v)?;
        };
        for v in &self.items {
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

impl ::protobuf::MessageStatic for ListItems {
    fn new() -> ListItems {
        ListItems::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListItems>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pos",
                    ListItems::get_pos_for_reflect,
                    ListItems::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "truncated",
                    ListItems::get_truncated_for_reflect,
                    ListItems::mut_truncated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Item>>(
                    "items",
                    ListItems::get_items_for_reflect,
                    ListItems::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListItems>(
                    "ListItems",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListItems {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_truncated();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListItems {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListItems {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContentRange {
    // message fields
    pos: ::std::option::Option<i32>,
    length: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContentRange {}

impl ContentRange {
    pub fn new() -> ContentRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContentRange {
        static mut instance: ::protobuf::lazy::Lazy<ContentRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContentRange,
        };
        unsafe {
            instance.get(ContentRange::new)
        }
    }

    // optional int32 pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos = ::std::option::Option::None;
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: i32) {
        self.pos = ::std::option::Option::Some(v);
    }

    pub fn get_pos(&self) -> i32 {
        self.pos.unwrap_or(0)
    }

    fn get_pos_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pos
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
}

impl ::protobuf::Message for ContentRange {
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
                    self.pos = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pos {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pos {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.length {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for ContentRange {
    fn new() -> ContentRange {
        ContentRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContentRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pos",
                    ContentRange::get_pos_for_reflect,
                    ContentRange::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    ContentRange::get_length_for_reflect,
                    ContentRange::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContentRange>(
                    "ContentRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContentRange {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContentRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContentRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListContentSelection {
    // message fields
    wantRevision: ::std::option::Option<bool>,
    wantLength: ::std::option::Option<bool>,
    wantAttributes: ::std::option::Option<bool>,
    wantChecksum: ::std::option::Option<bool>,
    wantContent: ::std::option::Option<bool>,
    contentRange: ::protobuf::SingularPtrField<ContentRange>,
    wantDiff: ::std::option::Option<bool>,
    baseRevision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    hintRevision: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    wantNothingIfUpToDate: ::std::option::Option<bool>,
    wantResolveAction: ::std::option::Option<bool>,
    issues: ::protobuf::RepeatedField<super::playlist4issues::ClientIssue>,
    resolveAction: ::protobuf::RepeatedField<super::playlist4issues::ClientResolveAction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListContentSelection {}

impl ListContentSelection {
    pub fn new() -> ListContentSelection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListContentSelection {
        static mut instance: ::protobuf::lazy::Lazy<ListContentSelection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListContentSelection,
        };
        unsafe {
            instance.get(ListContentSelection::new)
        }
    }

    // optional bool wantRevision = 1;

    pub fn clear_wantRevision(&mut self) {
        self.wantRevision = ::std::option::Option::None;
    }

    pub fn has_wantRevision(&self) -> bool {
        self.wantRevision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantRevision(&mut self, v: bool) {
        self.wantRevision = ::std::option::Option::Some(v);
    }

    pub fn get_wantRevision(&self) -> bool {
        self.wantRevision.unwrap_or(false)
    }

    fn get_wantRevision_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantRevision
    }

    fn mut_wantRevision_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantRevision
    }

    // optional bool wantLength = 2;

    pub fn clear_wantLength(&mut self) {
        self.wantLength = ::std::option::Option::None;
    }

    pub fn has_wantLength(&self) -> bool {
        self.wantLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantLength(&mut self, v: bool) {
        self.wantLength = ::std::option::Option::Some(v);
    }

    pub fn get_wantLength(&self) -> bool {
        self.wantLength.unwrap_or(false)
    }

    fn get_wantLength_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantLength
    }

    fn mut_wantLength_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantLength
    }

    // optional bool wantAttributes = 3;

    pub fn clear_wantAttributes(&mut self) {
        self.wantAttributes = ::std::option::Option::None;
    }

    pub fn has_wantAttributes(&self) -> bool {
        self.wantAttributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantAttributes(&mut self, v: bool) {
        self.wantAttributes = ::std::option::Option::Some(v);
    }

    pub fn get_wantAttributes(&self) -> bool {
        self.wantAttributes.unwrap_or(false)
    }

    fn get_wantAttributes_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantAttributes
    }

    fn mut_wantAttributes_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantAttributes
    }

    // optional bool wantChecksum = 4;

    pub fn clear_wantChecksum(&mut self) {
        self.wantChecksum = ::std::option::Option::None;
    }

    pub fn has_wantChecksum(&self) -> bool {
        self.wantChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantChecksum(&mut self, v: bool) {
        self.wantChecksum = ::std::option::Option::Some(v);
    }

    pub fn get_wantChecksum(&self) -> bool {
        self.wantChecksum.unwrap_or(false)
    }

    fn get_wantChecksum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantChecksum
    }

    fn mut_wantChecksum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantChecksum
    }

    // optional bool wantContent = 5;

    pub fn clear_wantContent(&mut self) {
        self.wantContent = ::std::option::Option::None;
    }

    pub fn has_wantContent(&self) -> bool {
        self.wantContent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantContent(&mut self, v: bool) {
        self.wantContent = ::std::option::Option::Some(v);
    }

    pub fn get_wantContent(&self) -> bool {
        self.wantContent.unwrap_or(false)
    }

    fn get_wantContent_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantContent
    }

    fn mut_wantContent_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantContent
    }

    // optional .ContentRange contentRange = 6;

    pub fn clear_contentRange(&mut self) {
        self.contentRange.clear();
    }

    pub fn has_contentRange(&self) -> bool {
        self.contentRange.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentRange(&mut self, v: ContentRange) {
        self.contentRange = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contentRange(&mut self) -> &mut ContentRange {
        if self.contentRange.is_none() {
            self.contentRange.set_default();
        };
        self.contentRange.as_mut().unwrap()
    }

    // Take field
    pub fn take_contentRange(&mut self) -> ContentRange {
        self.contentRange.take().unwrap_or_else(|| ContentRange::new())
    }

    pub fn get_contentRange(&self) -> &ContentRange {
        self.contentRange.as_ref().unwrap_or_else(|| ContentRange::default_instance())
    }

    fn get_contentRange_for_reflect(&self) -> &::protobuf::SingularPtrField<ContentRange> {
        &self.contentRange
    }

    fn mut_contentRange_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ContentRange> {
        &mut self.contentRange
    }

    // optional bool wantDiff = 7;

    pub fn clear_wantDiff(&mut self) {
        self.wantDiff = ::std::option::Option::None;
    }

    pub fn has_wantDiff(&self) -> bool {
        self.wantDiff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantDiff(&mut self, v: bool) {
        self.wantDiff = ::std::option::Option::Some(v);
    }

    pub fn get_wantDiff(&self) -> bool {
        self.wantDiff.unwrap_or(false)
    }

    fn get_wantDiff_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantDiff
    }

    fn mut_wantDiff_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantDiff
    }

    // optional bytes baseRevision = 8;

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

    // optional bytes hintRevision = 9;

    pub fn clear_hintRevision(&mut self) {
        self.hintRevision.clear();
    }

    pub fn has_hintRevision(&self) -> bool {
        self.hintRevision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hintRevision(&mut self, v: ::std::vec::Vec<u8>) {
        self.hintRevision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hintRevision(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hintRevision.is_none() {
            self.hintRevision.set_default();
        };
        self.hintRevision.as_mut().unwrap()
    }

    // Take field
    pub fn take_hintRevision(&mut self) -> ::std::vec::Vec<u8> {
        self.hintRevision.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hintRevision(&self) -> &[u8] {
        match self.hintRevision.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hintRevision_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hintRevision
    }

    fn mut_hintRevision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hintRevision
    }

    // optional bool wantNothingIfUpToDate = 10;

    pub fn clear_wantNothingIfUpToDate(&mut self) {
        self.wantNothingIfUpToDate = ::std::option::Option::None;
    }

    pub fn has_wantNothingIfUpToDate(&self) -> bool {
        self.wantNothingIfUpToDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantNothingIfUpToDate(&mut self, v: bool) {
        self.wantNothingIfUpToDate = ::std::option::Option::Some(v);
    }

    pub fn get_wantNothingIfUpToDate(&self) -> bool {
        self.wantNothingIfUpToDate.unwrap_or(false)
    }

    fn get_wantNothingIfUpToDate_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantNothingIfUpToDate
    }

    fn mut_wantNothingIfUpToDate_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantNothingIfUpToDate
    }

    // optional bool wantResolveAction = 12;

    pub fn clear_wantResolveAction(&mut self) {
        self.wantResolveAction = ::std::option::Option::None;
    }

    pub fn has_wantResolveAction(&self) -> bool {
        self.wantResolveAction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantResolveAction(&mut self, v: bool) {
        self.wantResolveAction = ::std::option::Option::Some(v);
    }

    pub fn get_wantResolveAction(&self) -> bool {
        self.wantResolveAction.unwrap_or(false)
    }

    fn get_wantResolveAction_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wantResolveAction
    }

    fn mut_wantResolveAction_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wantResolveAction
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

    // repeated .ClientResolveAction resolveAction = 14;

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
}

impl ::protobuf::Message for ListContentSelection {
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
                    let tmp = is.read_bool()?;
                    self.wantRevision = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantLength = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantAttributes = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantChecksum = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantContent = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contentRange)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantDiff = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.baseRevision)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hintRevision)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantNothingIfUpToDate = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.wantResolveAction = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.issues)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resolveAction)?;
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
        if let Some(v) = self.wantRevision {
            my_size += 2;
        };
        if let Some(v) = self.wantLength {
            my_size += 2;
        };
        if let Some(v) = self.wantAttributes {
            my_size += 2;
        };
        if let Some(v) = self.wantChecksum {
            my_size += 2;
        };
        if let Some(v) = self.wantContent {
            my_size += 2;
        };
        if let Some(v) = self.contentRange.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.wantDiff {
            my_size += 2;
        };
        if let Some(v) = self.baseRevision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        };
        if let Some(v) = self.hintRevision.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        };
        if let Some(v) = self.wantNothingIfUpToDate {
            my_size += 2;
        };
        if let Some(v) = self.wantResolveAction {
            my_size += 2;
        };
        for value in &self.issues {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.resolveAction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.wantRevision {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.wantLength {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.wantAttributes {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.wantChecksum {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.wantContent {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.contentRange.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.wantDiff {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.baseRevision.as_ref() {
            os.write_bytes(8, &v)?;
        };
        if let Some(v) = self.hintRevision.as_ref() {
            os.write_bytes(9, &v)?;
        };
        if let Some(v) = self.wantNothingIfUpToDate {
            os.write_bool(10, v)?;
        };
        if let Some(v) = self.wantResolveAction {
            os.write_bool(12, v)?;
        };
        for v in &self.issues {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.resolveAction {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ListContentSelection {
    fn new() -> ListContentSelection {
        ListContentSelection::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListContentSelection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantRevision",
                    ListContentSelection::get_wantRevision_for_reflect,
                    ListContentSelection::mut_wantRevision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantLength",
                    ListContentSelection::get_wantLength_for_reflect,
                    ListContentSelection::mut_wantLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantAttributes",
                    ListContentSelection::get_wantAttributes_for_reflect,
                    ListContentSelection::mut_wantAttributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantChecksum",
                    ListContentSelection::get_wantChecksum_for_reflect,
                    ListContentSelection::mut_wantChecksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantContent",
                    ListContentSelection::get_wantContent_for_reflect,
                    ListContentSelection::mut_wantContent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ContentRange>>(
                    "contentRange",
                    ListContentSelection::get_contentRange_for_reflect,
                    ListContentSelection::mut_contentRange_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantDiff",
                    ListContentSelection::get_wantDiff_for_reflect,
                    ListContentSelection::mut_wantDiff_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "baseRevision",
                    ListContentSelection::get_baseRevision_for_reflect,
                    ListContentSelection::mut_baseRevision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hintRevision",
                    ListContentSelection::get_hintRevision_for_reflect,
                    ListContentSelection::mut_hintRevision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantNothingIfUpToDate",
                    ListContentSelection::get_wantNothingIfUpToDate_for_reflect,
                    ListContentSelection::mut_wantNothingIfUpToDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wantResolveAction",
                    ListContentSelection::get_wantResolveAction_for_reflect,
                    ListContentSelection::mut_wantResolveAction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4issues::ClientIssue>>(
                    "issues",
                    ListContentSelection::get_issues_for_reflect,
                    ListContentSelection::mut_issues_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::playlist4issues::ClientResolveAction>>(
                    "resolveAction",
                    ListContentSelection::get_resolveAction_for_reflect,
                    ListContentSelection::mut_resolveAction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListContentSelection>(
                    "ListContentSelection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListContentSelection {
    fn clear(&mut self) {
        self.clear_wantRevision();
        self.clear_wantLength();
        self.clear_wantAttributes();
        self.clear_wantChecksum();
        self.clear_wantContent();
        self.clear_contentRange();
        self.clear_wantDiff();
        self.clear_baseRevision();
        self.clear_hintRevision();
        self.clear_wantNothingIfUpToDate();
        self.clear_wantResolveAction();
        self.clear_issues();
        self.clear_resolveAction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListContentSelection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListContentSelection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x70, 0x6c, 0x61, 0x79, 0x6c, 0x69,
    0x73, 0x74, 0x34, 0x6d, 0x65, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x70,
    0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x34, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x49, 0x0a, 0x04, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x10, 0x0a, 0x03,
    0x75, 0x72, 0x69, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x69, 0x12, 0x2f,
    0x0a, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x22,
    0x58, 0x0a, 0x09, 0x4c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x10, 0x0a, 0x03,
    0x70, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x03, 0x70, 0x6f, 0x73, 0x12, 0x1c,
    0x0a, 0x09, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x09, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x12, 0x1b, 0x0a, 0x05,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x22, 0x38, 0x0a, 0x0c, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x70, 0x6f, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x03, 0x70, 0x6f, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c, 0x65, 0x6e,
    0x67, 0x74, 0x68, 0x22, 0xa5, 0x04, 0x0a, 0x14, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x0c,
    0x77, 0x61, 0x6e, 0x74, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0c, 0x77, 0x61, 0x6e, 0x74, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e,
    0x12, 0x1e, 0x0a, 0x0a, 0x77, 0x61, 0x6e, 0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x77, 0x61, 0x6e, 0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68,
    0x12, 0x26, 0x0a, 0x0e, 0x77, 0x61, 0x6e, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x77, 0x61, 0x6e, 0x74, 0x41, 0x74,
    0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x22, 0x0a, 0x0c, 0x77, 0x61, 0x6e, 0x74,
    0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c,
    0x77, 0x61, 0x6e, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x20, 0x0a, 0x0b,
    0x77, 0x61, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0b, 0x77, 0x61, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x31,
    0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61,
    0x6e, 0x67, 0x65, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67,
    0x65, 0x12, 0x1a, 0x0a, 0x08, 0x77, 0x61, 0x6e, 0x74, 0x44, 0x69, 0x66, 0x66, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x08, 0x77, 0x61, 0x6e, 0x74, 0x44, 0x69, 0x66, 0x66, 0x12, 0x22, 0x0a,
    0x0c, 0x62, 0x61, 0x73, 0x65, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x0c, 0x62, 0x61, 0x73, 0x65, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f,
    0x6e, 0x12, 0x22, 0x0a, 0x0c, 0x68, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x68, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x76,
    0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x34, 0x0a, 0x15, 0x77, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x74,
    0x68, 0x69, 0x6e, 0x67, 0x49, 0x66, 0x55, 0x70, 0x54, 0x6f, 0x44, 0x61, 0x74, 0x65, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x15, 0x77, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x74, 0x68, 0x69, 0x6e,
    0x67, 0x49, 0x66, 0x55, 0x70, 0x54, 0x6f, 0x44, 0x61, 0x74, 0x65, 0x12, 0x2c, 0x0a, 0x11, 0x77,
    0x61, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x11, 0x77, 0x61, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x06, 0x69, 0x73, 0x73,
    0x75, 0x65, 0x73, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52, 0x06, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73, 0x12,
    0x3a, 0x0a, 0x0d, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x0e, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4a, 0xec, 0x0b, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x23, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x03, 0x07, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00,
    0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x06, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x06, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x07, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x1c, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x29, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x0a, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0b, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0b, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c,
    0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x0d, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x0d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x10, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x11, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x13, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x12, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x12, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x1c,
    0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x15, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x15, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x16, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x12, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x21, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x17, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x17, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x17, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x18, 0x04, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03,
    0x12, 0x03, 0x19, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x19, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x19, 0x12, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x19, 0x21, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x1a, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x1a, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1a,
    0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x04, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x06, 0x12, 0x03, 0x1b, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1b, 0x1a, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x1b, 0x29, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12,
    0x03, 0x1c, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1c,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1c, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x07, 0x12, 0x03, 0x1d, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x1d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x1d, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1d, 0x22,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x03, 0x1e, 0x04, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x1e, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08,
    0x03, 0x12, 0x03, 0x1e, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x03,
    0x1f, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x04, 0x12, 0x03, 0x1f, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1f, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1f, 0x12, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1f, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x0a, 0x12, 0x03, 0x20, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a,
    0x04, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x05, 0x12,
    0x03, 0x20, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x20,
    0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x20, 0x26, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0b, 0x12, 0x03, 0x21, 0x04, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x21, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x21, 0x19, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x03,
    0x12, 0x03, 0x21, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0c, 0x12, 0x03, 0x22,
    0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x06, 0x12, 0x03, 0x22, 0x0d, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x22, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x22, 0x31, 0x34,
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
