// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `steammessages_clientlanp2p.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CMsgClientLANP2PRequestChunks {
    // message fields
    chunk_keys: ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunks_ChunkKey>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CMsgClientLANP2PRequestChunks {
    fn default() -> &'a CMsgClientLANP2PRequestChunks {
        <CMsgClientLANP2PRequestChunks as ::protobuf::Message>::default_instance()
    }
}

impl CMsgClientLANP2PRequestChunks {
    pub fn new() -> CMsgClientLANP2PRequestChunks {
        ::std::default::Default::default()
    }

    // repeated .CMsgClientLANP2PRequestChunks.ChunkKey chunk_keys = 1;


    pub fn get_chunk_keys(&self) -> &[CMsgClientLANP2PRequestChunks_ChunkKey] {
        &self.chunk_keys
    }
    pub fn clear_chunk_keys(&mut self) {
        self.chunk_keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunk_keys(&mut self, v: ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunks_ChunkKey>) {
        self.chunk_keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunk_keys(&mut self) -> &mut ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunks_ChunkKey> {
        &mut self.chunk_keys
    }

    // Take field
    pub fn take_chunk_keys(&mut self) -> ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunks_ChunkKey> {
        ::std::mem::replace(&mut self.chunk_keys, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CMsgClientLANP2PRequestChunks {
    fn is_initialized(&self) -> bool {
        for v in &self.chunk_keys {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunk_keys)?;
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
        for value in &self.chunk_keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunk_keys {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CMsgClientLANP2PRequestChunks {
        CMsgClientLANP2PRequestChunks::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgClientLANP2PRequestChunks_ChunkKey>>(
                    "chunk_keys",
                    |m: &CMsgClientLANP2PRequestChunks| { &m.chunk_keys },
                    |m: &mut CMsgClientLANP2PRequestChunks| { &mut m.chunk_keys },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientLANP2PRequestChunks>(
                    "CMsgClientLANP2PRequestChunks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CMsgClientLANP2PRequestChunks {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientLANP2PRequestChunks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientLANP2PRequestChunks,
        };
        unsafe {
            instance.get(CMsgClientLANP2PRequestChunks::new)
        }
    }
}

impl ::protobuf::Clear for CMsgClientLANP2PRequestChunks {
    fn clear(&mut self) {
        self.chunk_keys.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientLANP2PRequestChunks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientLANP2PRequestChunks {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CMsgClientLANP2PRequestChunks_ChunkKey {
    // message fields
    depot_id: ::std::option::Option<u32>,
    sha: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CMsgClientLANP2PRequestChunks_ChunkKey {
    fn default() -> &'a CMsgClientLANP2PRequestChunks_ChunkKey {
        <CMsgClientLANP2PRequestChunks_ChunkKey as ::protobuf::Message>::default_instance()
    }
}

impl CMsgClientLANP2PRequestChunks_ChunkKey {
    pub fn new() -> CMsgClientLANP2PRequestChunks_ChunkKey {
        ::std::default::Default::default()
    }

    // optional uint32 depot_id = 1;


    pub fn get_depot_id(&self) -> u32 {
        self.depot_id.unwrap_or(0)
    }
    pub fn clear_depot_id(&mut self) {
        self.depot_id = ::std::option::Option::None;
    }

    pub fn has_depot_id(&self) -> bool {
        self.depot_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_depot_id(&mut self, v: u32) {
        self.depot_id = ::std::option::Option::Some(v);
    }

    // optional bytes sha = 2;


    pub fn get_sha(&self) -> &[u8] {
        match self.sha.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_sha(&mut self) {
        self.sha.clear();
    }

    pub fn has_sha(&self) -> bool {
        self.sha.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha.is_none() {
            self.sha.set_default();
        }
        self.sha.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha(&mut self) -> ::std::vec::Vec<u8> {
        self.sha.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CMsgClientLANP2PRequestChunks_ChunkKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.depot_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha)?;
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
        if let Some(v) = self.depot_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.sha.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.depot_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.sha.as_ref() {
            os.write_bytes(2, &v)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CMsgClientLANP2PRequestChunks_ChunkKey {
        CMsgClientLANP2PRequestChunks_ChunkKey::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "depot_id",
                    |m: &CMsgClientLANP2PRequestChunks_ChunkKey| { &m.depot_id },
                    |m: &mut CMsgClientLANP2PRequestChunks_ChunkKey| { &mut m.depot_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha",
                    |m: &CMsgClientLANP2PRequestChunks_ChunkKey| { &m.sha },
                    |m: &mut CMsgClientLANP2PRequestChunks_ChunkKey| { &mut m.sha },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientLANP2PRequestChunks_ChunkKey>(
                    "CMsgClientLANP2PRequestChunks_ChunkKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CMsgClientLANP2PRequestChunks_ChunkKey {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientLANP2PRequestChunks_ChunkKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientLANP2PRequestChunks_ChunkKey,
        };
        unsafe {
            instance.get(CMsgClientLANP2PRequestChunks_ChunkKey::new)
        }
    }
}

impl ::protobuf::Clear for CMsgClientLANP2PRequestChunks_ChunkKey {
    fn clear(&mut self) {
        self.depot_id = ::std::option::Option::None;
        self.sha.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientLANP2PRequestChunks_ChunkKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientLANP2PRequestChunks_ChunkKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CMsgClientLANP2PRequestChunksResponse {
    // message fields
    chunk_responses: ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunksResponse_ChunkData>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CMsgClientLANP2PRequestChunksResponse {
    fn default() -> &'a CMsgClientLANP2PRequestChunksResponse {
        <CMsgClientLANP2PRequestChunksResponse as ::protobuf::Message>::default_instance()
    }
}

impl CMsgClientLANP2PRequestChunksResponse {
    pub fn new() -> CMsgClientLANP2PRequestChunksResponse {
        ::std::default::Default::default()
    }

    // repeated .CMsgClientLANP2PRequestChunksResponse.ChunkData chunk_responses = 1;


    pub fn get_chunk_responses(&self) -> &[CMsgClientLANP2PRequestChunksResponse_ChunkData] {
        &self.chunk_responses
    }
    pub fn clear_chunk_responses(&mut self) {
        self.chunk_responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunk_responses(&mut self, v: ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunksResponse_ChunkData>) {
        self.chunk_responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunk_responses(&mut self) -> &mut ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunksResponse_ChunkData> {
        &mut self.chunk_responses
    }

    // Take field
    pub fn take_chunk_responses(&mut self) -> ::protobuf::RepeatedField<CMsgClientLANP2PRequestChunksResponse_ChunkData> {
        ::std::mem::replace(&mut self.chunk_responses, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CMsgClientLANP2PRequestChunksResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.chunk_responses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunk_responses)?;
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
        for value in &self.chunk_responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunk_responses {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CMsgClientLANP2PRequestChunksResponse {
        CMsgClientLANP2PRequestChunksResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgClientLANP2PRequestChunksResponse_ChunkData>>(
                    "chunk_responses",
                    |m: &CMsgClientLANP2PRequestChunksResponse| { &m.chunk_responses },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse| { &mut m.chunk_responses },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientLANP2PRequestChunksResponse>(
                    "CMsgClientLANP2PRequestChunksResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CMsgClientLANP2PRequestChunksResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientLANP2PRequestChunksResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientLANP2PRequestChunksResponse,
        };
        unsafe {
            instance.get(CMsgClientLANP2PRequestChunksResponse::new)
        }
    }
}

impl ::protobuf::Clear for CMsgClientLANP2PRequestChunksResponse {
    fn clear(&mut self) {
        self.chunk_responses.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientLANP2PRequestChunksResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientLANP2PRequestChunksResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CMsgClientLANP2PRequestChunksResponse_ChunkData {
    // message fields
    result: ::std::option::Option<u32>,
    depot_id: ::std::option::Option<u32>,
    sha: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    chunk_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    encrypted: ::std::option::Option<bool>,
    compressed: ::std::option::Option<bool>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CMsgClientLANP2PRequestChunksResponse_ChunkData {
    fn default() -> &'a CMsgClientLANP2PRequestChunksResponse_ChunkData {
        <CMsgClientLANP2PRequestChunksResponse_ChunkData as ::protobuf::Message>::default_instance()
    }
}

impl CMsgClientLANP2PRequestChunksResponse_ChunkData {
    pub fn new() -> CMsgClientLANP2PRequestChunksResponse_ChunkData {
        ::std::default::Default::default()
    }

    // optional uint32 result = 1;


    pub fn get_result(&self) -> u32 {
        self.result.unwrap_or(0)
    }
    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(v);
    }

    // optional uint32 depot_id = 2;


    pub fn get_depot_id(&self) -> u32 {
        self.depot_id.unwrap_or(0)
    }
    pub fn clear_depot_id(&mut self) {
        self.depot_id = ::std::option::Option::None;
    }

    pub fn has_depot_id(&self) -> bool {
        self.depot_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_depot_id(&mut self, v: u32) {
        self.depot_id = ::std::option::Option::Some(v);
    }

    // optional bytes sha = 3;


    pub fn get_sha(&self) -> &[u8] {
        match self.sha.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_sha(&mut self) {
        self.sha.clear();
    }

    pub fn has_sha(&self) -> bool {
        self.sha.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sha(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sha.is_none() {
            self.sha.set_default();
        }
        self.sha.as_mut().unwrap()
    }

    // Take field
    pub fn take_sha(&mut self) -> ::std::vec::Vec<u8> {
        self.sha.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes chunk_data = 4;


    pub fn get_chunk_data(&self) -> &[u8] {
        match self.chunk_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_chunk_data(&mut self) {
        self.chunk_data.clear();
    }

    pub fn has_chunk_data(&self) -> bool {
        self.chunk_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunk_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.chunk_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chunk_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.chunk_data.is_none() {
            self.chunk_data.set_default();
        }
        self.chunk_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_chunk_data(&mut self) -> ::std::vec::Vec<u8> {
        self.chunk_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bool encrypted = 5;


    pub fn get_encrypted(&self) -> bool {
        self.encrypted.unwrap_or(false)
    }
    pub fn clear_encrypted(&mut self) {
        self.encrypted = ::std::option::Option::None;
    }

    pub fn has_encrypted(&self) -> bool {
        self.encrypted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted(&mut self, v: bool) {
        self.encrypted = ::std::option::Option::Some(v);
    }

    // optional bool compressed = 6;


    pub fn get_compressed(&self) -> bool {
        self.compressed.unwrap_or(false)
    }
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
}

impl ::protobuf::Message for CMsgClientLANP2PRequestChunksResponse_ChunkData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.depot_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sha)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.chunk_data)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.encrypted = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.compressed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.depot_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.sha.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.chunk_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.encrypted {
            my_size += 2;
        }
        if let Some(v) = self.compressed {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.depot_id {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.sha.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.chunk_data.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.encrypted {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.compressed {
            os.write_bool(6, v)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CMsgClientLANP2PRequestChunksResponse_ChunkData {
        CMsgClientLANP2PRequestChunksResponse_ChunkData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    |m: &CMsgClientLANP2PRequestChunksResponse_ChunkData| { &m.result },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse_ChunkData| { &mut m.result },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "depot_id",
                    |m: &CMsgClientLANP2PRequestChunksResponse_ChunkData| { &m.depot_id },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse_ChunkData| { &mut m.depot_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha",
                    |m: &CMsgClientLANP2PRequestChunksResponse_ChunkData| { &m.sha },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse_ChunkData| { &mut m.sha },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "chunk_data",
                    |m: &CMsgClientLANP2PRequestChunksResponse_ChunkData| { &m.chunk_data },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse_ChunkData| { &mut m.chunk_data },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "encrypted",
                    |m: &CMsgClientLANP2PRequestChunksResponse_ChunkData| { &m.encrypted },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse_ChunkData| { &mut m.encrypted },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "compressed",
                    |m: &CMsgClientLANP2PRequestChunksResponse_ChunkData| { &m.compressed },
                    |m: &mut CMsgClientLANP2PRequestChunksResponse_ChunkData| { &mut m.compressed },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientLANP2PRequestChunksResponse_ChunkData>(
                    "CMsgClientLANP2PRequestChunksResponse_ChunkData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CMsgClientLANP2PRequestChunksResponse_ChunkData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientLANP2PRequestChunksResponse_ChunkData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientLANP2PRequestChunksResponse_ChunkData,
        };
        unsafe {
            instance.get(CMsgClientLANP2PRequestChunksResponse_ChunkData::new)
        }
    }
}

impl ::protobuf::Clear for CMsgClientLANP2PRequestChunksResponse_ChunkData {
    fn clear(&mut self) {
        self.result = ::std::option::Option::None;
        self.depot_id = ::std::option::Option::None;
        self.sha.clear();
        self.chunk_data.clear();
        self.encrypted = ::std::option::Option::None;
        self.compressed = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientLANP2PRequestChunksResponse_ChunkData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientLANP2PRequestChunksResponse_ChunkData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20steammessages_clientlanp2p.proto\x1a\x18steammessages_base.proto\"\
    \xa0\x01\n\x1dCMsgClientLANP2PRequestChunks\x12F\n\nchunk_keys\x18\x01\
    \x20\x03(\x0b2'.CMsgClientLANP2PRequestChunks.ChunkKeyR\tchunkKeys\x1a7\
    \n\x08ChunkKey\x12\x19\n\x08depot_id\x18\x01\x20\x01(\rR\x07depotId\x12\
    \x10\n\x03sha\x18\x02\x20\x01(\x0cR\x03sha\"\xb2\x02\n%CMsgClientLANP2PR\
    equestChunksResponse\x12Y\n\x0fchunk_responses\x18\x01\x20\x03(\x0b20.CM\
    sgClientLANP2PRequestChunksResponse.ChunkDataR\x0echunkResponses\x1a\xad\
    \x01\n\tChunkData\x12\x16\n\x06result\x18\x01\x20\x01(\rR\x06result\x12\
    \x19\n\x08depot_id\x18\x02\x20\x01(\rR\x07depotId\x12\x10\n\x03sha\x18\
    \x03\x20\x01(\x0cR\x03sha\x12\x1d\n\nchunk_data\x18\x04\x20\x01(\x0cR\tc\
    hunkData\x12\x1c\n\tencrypted\x18\x05\x20\x01(\x08R\tencrypted\x12\x1e\n\
    \ncompressed\x18\x06\x20\x01(\x08R\ncompressedB\x05H\x01\x80\x01\0\
";

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
