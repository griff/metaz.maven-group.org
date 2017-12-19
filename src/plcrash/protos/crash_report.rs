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
pub struct CrashReport {
    // message fields
    system_info: ::protobuf::SingularPtrField<CrashReport_SystemInfo>,
    application_info: ::protobuf::SingularPtrField<CrashReport_ApplicationInfo>,
    threads: ::protobuf::RepeatedField<CrashReport_Thread>,
    binary_images: ::protobuf::RepeatedField<CrashReport_BinaryImage>,
    exception: ::protobuf::SingularPtrField<CrashReport_Exception>,
    signal: ::protobuf::SingularPtrField<CrashReport_Signal>,
    process_info: ::protobuf::SingularPtrField<CrashReport_ProcessInfo>,
    machine_info: ::protobuf::SingularPtrField<CrashReport_MachineInfo>,
    report_info: ::protobuf::SingularPtrField<CrashReport_ReportInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport {}

impl CrashReport {
    pub fn new() -> CrashReport {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport,
        };
        unsafe {
            instance.get(CrashReport::new)
        }
    }

    // required .plcrash.CrashReport.SystemInfo system_info = 1;

    pub fn clear_system_info(&mut self) {
        self.system_info.clear();
    }

    pub fn has_system_info(&self) -> bool {
        self.system_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_system_info(&mut self, v: CrashReport_SystemInfo) {
        self.system_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_system_info(&mut self) -> &mut CrashReport_SystemInfo {
        if self.system_info.is_none() {
            self.system_info.set_default();
        }
        self.system_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_system_info(&mut self) -> CrashReport_SystemInfo {
        self.system_info.take().unwrap_or_else(|| CrashReport_SystemInfo::new())
    }

    pub fn get_system_info(&self) -> &CrashReport_SystemInfo {
        self.system_info.as_ref().unwrap_or_else(|| CrashReport_SystemInfo::default_instance())
    }

    fn get_system_info_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_SystemInfo> {
        &self.system_info
    }

    fn mut_system_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_SystemInfo> {
        &mut self.system_info
    }

    // required .plcrash.CrashReport.ApplicationInfo application_info = 2;

    pub fn clear_application_info(&mut self) {
        self.application_info.clear();
    }

    pub fn has_application_info(&self) -> bool {
        self.application_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_info(&mut self, v: CrashReport_ApplicationInfo) {
        self.application_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application_info(&mut self) -> &mut CrashReport_ApplicationInfo {
        if self.application_info.is_none() {
            self.application_info.set_default();
        }
        self.application_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_application_info(&mut self) -> CrashReport_ApplicationInfo {
        self.application_info.take().unwrap_or_else(|| CrashReport_ApplicationInfo::new())
    }

    pub fn get_application_info(&self) -> &CrashReport_ApplicationInfo {
        self.application_info.as_ref().unwrap_or_else(|| CrashReport_ApplicationInfo::default_instance())
    }

    fn get_application_info_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_ApplicationInfo> {
        &self.application_info
    }

    fn mut_application_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_ApplicationInfo> {
        &mut self.application_info
    }

    // repeated .plcrash.CrashReport.Thread threads = 3;

    pub fn clear_threads(&mut self) {
        self.threads.clear();
    }

    // Param is passed by value, moved
    pub fn set_threads(&mut self, v: ::protobuf::RepeatedField<CrashReport_Thread>) {
        self.threads = v;
    }

    // Mutable pointer to the field.
    pub fn mut_threads(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread> {
        &mut self.threads
    }

    // Take field
    pub fn take_threads(&mut self) -> ::protobuf::RepeatedField<CrashReport_Thread> {
        ::std::mem::replace(&mut self.threads, ::protobuf::RepeatedField::new())
    }

    pub fn get_threads(&self) -> &[CrashReport_Thread] {
        &self.threads
    }

    fn get_threads_for_reflect(&self) -> &::protobuf::RepeatedField<CrashReport_Thread> {
        &self.threads
    }

    fn mut_threads_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread> {
        &mut self.threads
    }

    // repeated .plcrash.CrashReport.BinaryImage binary_images = 4;

    pub fn clear_binary_images(&mut self) {
        self.binary_images.clear();
    }

    // Param is passed by value, moved
    pub fn set_binary_images(&mut self, v: ::protobuf::RepeatedField<CrashReport_BinaryImage>) {
        self.binary_images = v;
    }

    // Mutable pointer to the field.
    pub fn mut_binary_images(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_BinaryImage> {
        &mut self.binary_images
    }

    // Take field
    pub fn take_binary_images(&mut self) -> ::protobuf::RepeatedField<CrashReport_BinaryImage> {
        ::std::mem::replace(&mut self.binary_images, ::protobuf::RepeatedField::new())
    }

    pub fn get_binary_images(&self) -> &[CrashReport_BinaryImage] {
        &self.binary_images
    }

    fn get_binary_images_for_reflect(&self) -> &::protobuf::RepeatedField<CrashReport_BinaryImage> {
        &self.binary_images
    }

    fn mut_binary_images_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_BinaryImage> {
        &mut self.binary_images
    }

    // optional .plcrash.CrashReport.Exception exception = 5;

    pub fn clear_exception(&mut self) {
        self.exception.clear();
    }

    pub fn has_exception(&self) -> bool {
        self.exception.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exception(&mut self, v: CrashReport_Exception) {
        self.exception = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exception(&mut self) -> &mut CrashReport_Exception {
        if self.exception.is_none() {
            self.exception.set_default();
        }
        self.exception.as_mut().unwrap()
    }

    // Take field
    pub fn take_exception(&mut self) -> CrashReport_Exception {
        self.exception.take().unwrap_or_else(|| CrashReport_Exception::new())
    }

    pub fn get_exception(&self) -> &CrashReport_Exception {
        self.exception.as_ref().unwrap_or_else(|| CrashReport_Exception::default_instance())
    }

    fn get_exception_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_Exception> {
        &self.exception
    }

    fn mut_exception_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_Exception> {
        &mut self.exception
    }

    // required .plcrash.CrashReport.Signal signal = 6;

    pub fn clear_signal(&mut self) {
        self.signal.clear();
    }

    pub fn has_signal(&self) -> bool {
        self.signal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signal(&mut self, v: CrashReport_Signal) {
        self.signal = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signal(&mut self) -> &mut CrashReport_Signal {
        if self.signal.is_none() {
            self.signal.set_default();
        }
        self.signal.as_mut().unwrap()
    }

    // Take field
    pub fn take_signal(&mut self) -> CrashReport_Signal {
        self.signal.take().unwrap_or_else(|| CrashReport_Signal::new())
    }

    pub fn get_signal(&self) -> &CrashReport_Signal {
        self.signal.as_ref().unwrap_or_else(|| CrashReport_Signal::default_instance())
    }

    fn get_signal_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_Signal> {
        &self.signal
    }

    fn mut_signal_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_Signal> {
        &mut self.signal
    }

    // optional .plcrash.CrashReport.ProcessInfo process_info = 7;

    pub fn clear_process_info(&mut self) {
        self.process_info.clear();
    }

    pub fn has_process_info(&self) -> bool {
        self.process_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process_info(&mut self, v: CrashReport_ProcessInfo) {
        self.process_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_process_info(&mut self) -> &mut CrashReport_ProcessInfo {
        if self.process_info.is_none() {
            self.process_info.set_default();
        }
        self.process_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_process_info(&mut self) -> CrashReport_ProcessInfo {
        self.process_info.take().unwrap_or_else(|| CrashReport_ProcessInfo::new())
    }

    pub fn get_process_info(&self) -> &CrashReport_ProcessInfo {
        self.process_info.as_ref().unwrap_or_else(|| CrashReport_ProcessInfo::default_instance())
    }

    fn get_process_info_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_ProcessInfo> {
        &self.process_info
    }

    fn mut_process_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_ProcessInfo> {
        &mut self.process_info
    }

    // optional .plcrash.CrashReport.MachineInfo machine_info = 8;

    pub fn clear_machine_info(&mut self) {
        self.machine_info.clear();
    }

    pub fn has_machine_info(&self) -> bool {
        self.machine_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_machine_info(&mut self, v: CrashReport_MachineInfo) {
        self.machine_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_machine_info(&mut self) -> &mut CrashReport_MachineInfo {
        if self.machine_info.is_none() {
            self.machine_info.set_default();
        }
        self.machine_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_machine_info(&mut self) -> CrashReport_MachineInfo {
        self.machine_info.take().unwrap_or_else(|| CrashReport_MachineInfo::new())
    }

    pub fn get_machine_info(&self) -> &CrashReport_MachineInfo {
        self.machine_info.as_ref().unwrap_or_else(|| CrashReport_MachineInfo::default_instance())
    }

    fn get_machine_info_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_MachineInfo> {
        &self.machine_info
    }

    fn mut_machine_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_MachineInfo> {
        &mut self.machine_info
    }

    // optional .plcrash.CrashReport.ReportInfo report_info = 9;

    pub fn clear_report_info(&mut self) {
        self.report_info.clear();
    }

    pub fn has_report_info(&self) -> bool {
        self.report_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_info(&mut self, v: CrashReport_ReportInfo) {
        self.report_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report_info(&mut self) -> &mut CrashReport_ReportInfo {
        if self.report_info.is_none() {
            self.report_info.set_default();
        }
        self.report_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_report_info(&mut self) -> CrashReport_ReportInfo {
        self.report_info.take().unwrap_or_else(|| CrashReport_ReportInfo::new())
    }

    pub fn get_report_info(&self) -> &CrashReport_ReportInfo {
        self.report_info.as_ref().unwrap_or_else(|| CrashReport_ReportInfo::default_instance())
    }

    fn get_report_info_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_ReportInfo> {
        &self.report_info
    }

    fn mut_report_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_ReportInfo> {
        &mut self.report_info
    }
}

impl ::protobuf::Message for CrashReport {
    fn is_initialized(&self) -> bool {
        if self.system_info.is_none() {
            return false;
        }
        if self.application_info.is_none() {
            return false;
        }
        if self.signal.is_none() {
            return false;
        }
        for v in &self.system_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.application_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.threads {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.binary_images {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.exception {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.signal {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.process_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.machine_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.report_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.system_info)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.application_info)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.threads)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.binary_images)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.exception)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signal)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.process_info)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.machine_info)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.report_info)?;
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
        if let Some(ref v) = self.system_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.application_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.threads {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.binary_images {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.exception.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.signal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.process_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.machine_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.report_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.system_info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.application_info.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.threads {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.binary_images {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.exception.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.signal.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.process_info.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.machine_info.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.report_info.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for CrashReport {
    fn new() -> CrashReport {
        CrashReport::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_SystemInfo>>(
                    "system_info",
                    CrashReport::get_system_info_for_reflect,
                    CrashReport::mut_system_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_ApplicationInfo>>(
                    "application_info",
                    CrashReport::get_application_info_for_reflect,
                    CrashReport::mut_application_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Thread>>(
                    "threads",
                    CrashReport::get_threads_for_reflect,
                    CrashReport::mut_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_BinaryImage>>(
                    "binary_images",
                    CrashReport::get_binary_images_for_reflect,
                    CrashReport::mut_binary_images_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Exception>>(
                    "exception",
                    CrashReport::get_exception_for_reflect,
                    CrashReport::mut_exception_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Signal>>(
                    "signal",
                    CrashReport::get_signal_for_reflect,
                    CrashReport::mut_signal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_ProcessInfo>>(
                    "process_info",
                    CrashReport::get_process_info_for_reflect,
                    CrashReport::mut_process_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_MachineInfo>>(
                    "machine_info",
                    CrashReport::get_machine_info_for_reflect,
                    CrashReport::mut_machine_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_ReportInfo>>(
                    "report_info",
                    CrashReport::get_report_info_for_reflect,
                    CrashReport::mut_report_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport>(
                    "CrashReport",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport {
    fn clear(&mut self) {
        self.clear_system_info();
        self.clear_application_info();
        self.clear_threads();
        self.clear_binary_images();
        self.clear_exception();
        self.clear_signal();
        self.clear_process_info();
        self.clear_machine_info();
        self.clear_report_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Processor {
    // message fields
    encoding: ::std::option::Option<CrashReport_Processor_TypeEncoding>,
    field_type: ::std::option::Option<u64>,
    subtype: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Processor {}

impl CrashReport_Processor {
    pub fn new() -> CrashReport_Processor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Processor {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Processor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Processor,
        };
        unsafe {
            instance.get(CrashReport_Processor::new)
        }
    }

    // optional .plcrash.CrashReport.Processor.TypeEncoding encoding = 1;

    pub fn clear_encoding(&mut self) {
        self.encoding = ::std::option::Option::None;
    }

    pub fn has_encoding(&self) -> bool {
        self.encoding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encoding(&mut self, v: CrashReport_Processor_TypeEncoding) {
        self.encoding = ::std::option::Option::Some(v);
    }

    pub fn get_encoding(&self) -> CrashReport_Processor_TypeEncoding {
        self.encoding.unwrap_or(CrashReport_Processor_TypeEncoding::TYPE_ENCODING_UNKNOWN)
    }

    fn get_encoding_for_reflect(&self) -> &::std::option::Option<CrashReport_Processor_TypeEncoding> {
        &self.encoding
    }

    fn mut_encoding_for_reflect(&mut self) -> &mut ::std::option::Option<CrashReport_Processor_TypeEncoding> {
        &mut self.encoding
    }

    // required uint64 type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: u64) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> u64 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.field_type
    }

    // required uint64 subtype = 3;

    pub fn clear_subtype(&mut self) {
        self.subtype = ::std::option::Option::None;
    }

    pub fn has_subtype(&self) -> bool {
        self.subtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subtype(&mut self, v: u64) {
        self.subtype = ::std::option::Option::Some(v);
    }

    pub fn get_subtype(&self) -> u64 {
        self.subtype.unwrap_or(0)
    }

    fn get_subtype_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.subtype
    }

    fn mut_subtype_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.subtype
    }
}

impl ::protobuf::Message for CrashReport_Processor {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.subtype.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.encoding = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.subtype = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.encoding {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.subtype {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encoding {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.field_type {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.subtype {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for CrashReport_Processor {
    fn new() -> CrashReport_Processor {
        CrashReport_Processor::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Processor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CrashReport_Processor_TypeEncoding>>(
                    "encoding",
                    CrashReport_Processor::get_encoding_for_reflect,
                    CrashReport_Processor::mut_encoding_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "type",
                    CrashReport_Processor::get_field_type_for_reflect,
                    CrashReport_Processor::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "subtype",
                    CrashReport_Processor::get_subtype_for_reflect,
                    CrashReport_Processor::mut_subtype_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Processor>(
                    "CrashReport_Processor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Processor {
    fn clear(&mut self) {
        self.clear_encoding();
        self.clear_field_type();
        self.clear_subtype();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Processor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Processor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CrashReport_Processor_TypeEncoding {
    TYPE_ENCODING_UNKNOWN = 0,
    TYPE_ENCODING_MACH = 1,
}

impl ::protobuf::ProtobufEnum for CrashReport_Processor_TypeEncoding {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CrashReport_Processor_TypeEncoding> {
        match value {
            0 => ::std::option::Option::Some(CrashReport_Processor_TypeEncoding::TYPE_ENCODING_UNKNOWN),
            1 => ::std::option::Option::Some(CrashReport_Processor_TypeEncoding::TYPE_ENCODING_MACH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CrashReport_Processor_TypeEncoding] = &[
            CrashReport_Processor_TypeEncoding::TYPE_ENCODING_UNKNOWN,
            CrashReport_Processor_TypeEncoding::TYPE_ENCODING_MACH,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CrashReport_Processor_TypeEncoding>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CrashReport_Processor_TypeEncoding", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CrashReport_Processor_TypeEncoding {
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Processor_TypeEncoding {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_SystemInfo {
    // message fields
    operating_system: ::std::option::Option<CrashReport_SystemInfo_OperatingSystem>,
    os_version: ::protobuf::SingularField<::std::string::String>,
    architecture: ::std::option::Option<Architecture>,
    timestamp: ::std::option::Option<i64>,
    os_build: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_SystemInfo {}

impl CrashReport_SystemInfo {
    pub fn new() -> CrashReport_SystemInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_SystemInfo {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_SystemInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_SystemInfo,
        };
        unsafe {
            instance.get(CrashReport_SystemInfo::new)
        }
    }

    // optional .plcrash.CrashReport.SystemInfo.OperatingSystem operating_system = 1;

    pub fn clear_operating_system(&mut self) {
        self.operating_system = ::std::option::Option::None;
    }

    pub fn has_operating_system(&self) -> bool {
        self.operating_system.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operating_system(&mut self, v: CrashReport_SystemInfo_OperatingSystem) {
        self.operating_system = ::std::option::Option::Some(v);
    }

    pub fn get_operating_system(&self) -> CrashReport_SystemInfo_OperatingSystem {
        self.operating_system.unwrap_or(CrashReport_SystemInfo_OperatingSystem::OS_UNKNOWN)
    }

    fn get_operating_system_for_reflect(&self) -> &::std::option::Option<CrashReport_SystemInfo_OperatingSystem> {
        &self.operating_system
    }

    fn mut_operating_system_for_reflect(&mut self) -> &mut ::std::option::Option<CrashReport_SystemInfo_OperatingSystem> {
        &mut self.operating_system
    }

    // required string os_version = 2;

    pub fn clear_os_version(&mut self) {
        self.os_version.clear();
    }

    pub fn has_os_version(&self) -> bool {
        self.os_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os_version(&mut self, v: ::std::string::String) {
        self.os_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_os_version(&mut self) -> &mut ::std::string::String {
        if self.os_version.is_none() {
            self.os_version.set_default();
        }
        self.os_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_os_version(&mut self) -> ::std::string::String {
        self.os_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_os_version(&self) -> &str {
        match self.os_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_os_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.os_version
    }

    fn mut_os_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.os_version
    }

    // required .plcrash.Architecture architecture = 3;

    pub fn clear_architecture(&mut self) {
        self.architecture = ::std::option::Option::None;
    }

    pub fn has_architecture(&self) -> bool {
        self.architecture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_architecture(&mut self, v: Architecture) {
        self.architecture = ::std::option::Option::Some(v);
    }

    pub fn get_architecture(&self) -> Architecture {
        self.architecture.unwrap_or(Architecture::ARCHITECTURE_UNKNOWN)
    }

    fn get_architecture_for_reflect(&self) -> &::std::option::Option<Architecture> {
        &self.architecture
    }

    fn mut_architecture_for_reflect(&mut self) -> &mut ::std::option::Option<Architecture> {
        &mut self.architecture
    }

    // required int64 timestamp = 4;

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

    // optional string os_build = 5;

    pub fn clear_os_build(&mut self) {
        self.os_build.clear();
    }

    pub fn has_os_build(&self) -> bool {
        self.os_build.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os_build(&mut self, v: ::std::string::String) {
        self.os_build = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_os_build(&mut self) -> &mut ::std::string::String {
        if self.os_build.is_none() {
            self.os_build.set_default();
        }
        self.os_build.as_mut().unwrap()
    }

    // Take field
    pub fn take_os_build(&mut self) -> ::std::string::String {
        self.os_build.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_os_build(&self) -> &str {
        match self.os_build.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_os_build_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.os_build
    }

    fn mut_os_build_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.os_build
    }
}

impl ::protobuf::Message for CrashReport_SystemInfo {
    fn is_initialized(&self) -> bool {
        if self.os_version.is_none() {
            return false;
        }
        if self.architecture.is_none() {
            return false;
        }
        if self.timestamp.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.operating_system = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.os_version)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.architecture = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.os_build)?;
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
        if let Some(v) = self.operating_system {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.os_version.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.architecture {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.os_build.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.operating_system {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.os_version.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.architecture {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.timestamp {
            os.write_int64(4, v)?;
        }
        if let Some(ref v) = self.os_build.as_ref() {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for CrashReport_SystemInfo {
    fn new() -> CrashReport_SystemInfo {
        CrashReport_SystemInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_SystemInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CrashReport_SystemInfo_OperatingSystem>>(
                    "operating_system",
                    CrashReport_SystemInfo::get_operating_system_for_reflect,
                    CrashReport_SystemInfo::mut_operating_system_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "os_version",
                    CrashReport_SystemInfo::get_os_version_for_reflect,
                    CrashReport_SystemInfo::mut_os_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Architecture>>(
                    "architecture",
                    CrashReport_SystemInfo::get_architecture_for_reflect,
                    CrashReport_SystemInfo::mut_architecture_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    CrashReport_SystemInfo::get_timestamp_for_reflect,
                    CrashReport_SystemInfo::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "os_build",
                    CrashReport_SystemInfo::get_os_build_for_reflect,
                    CrashReport_SystemInfo::mut_os_build_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_SystemInfo>(
                    "CrashReport_SystemInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_SystemInfo {
    fn clear(&mut self) {
        self.clear_operating_system();
        self.clear_os_version();
        self.clear_architecture();
        self.clear_timestamp();
        self.clear_os_build();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_SystemInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_SystemInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CrashReport_SystemInfo_OperatingSystem {
    MAC_OS_X = 0,
    IPHONE_OS = 1,
    IPHONE_SIMULATOR = 2,
    OS_UNKNOWN = 3,
}

impl ::protobuf::ProtobufEnum for CrashReport_SystemInfo_OperatingSystem {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CrashReport_SystemInfo_OperatingSystem> {
        match value {
            0 => ::std::option::Option::Some(CrashReport_SystemInfo_OperatingSystem::MAC_OS_X),
            1 => ::std::option::Option::Some(CrashReport_SystemInfo_OperatingSystem::IPHONE_OS),
            2 => ::std::option::Option::Some(CrashReport_SystemInfo_OperatingSystem::IPHONE_SIMULATOR),
            3 => ::std::option::Option::Some(CrashReport_SystemInfo_OperatingSystem::OS_UNKNOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CrashReport_SystemInfo_OperatingSystem] = &[
            CrashReport_SystemInfo_OperatingSystem::MAC_OS_X,
            CrashReport_SystemInfo_OperatingSystem::IPHONE_OS,
            CrashReport_SystemInfo_OperatingSystem::IPHONE_SIMULATOR,
            CrashReport_SystemInfo_OperatingSystem::OS_UNKNOWN,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CrashReport_SystemInfo_OperatingSystem>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CrashReport_SystemInfo_OperatingSystem", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CrashReport_SystemInfo_OperatingSystem {
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_SystemInfo_OperatingSystem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_ApplicationInfo {
    // message fields
    identifier: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_ApplicationInfo {}

impl CrashReport_ApplicationInfo {
    pub fn new() -> CrashReport_ApplicationInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_ApplicationInfo {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_ApplicationInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_ApplicationInfo,
        };
        unsafe {
            instance.get(CrashReport_ApplicationInfo::new)
        }
    }

    // required string identifier = 1;

    pub fn clear_identifier(&mut self) {
        self.identifier.clear();
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: ::std::string::String) {
        self.identifier = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier(&mut self) -> &mut ::std::string::String {
        if self.identifier.is_none() {
            self.identifier.set_default();
        }
        self.identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier(&mut self) -> ::std::string::String {
        self.identifier.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_identifier(&self) -> &str {
        match self.identifier.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_identifier_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.identifier
    }

    fn mut_identifier_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.identifier
    }

    // required string version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }
}

impl ::protobuf::Message for CrashReport_ApplicationInfo {
    fn is_initialized(&self) -> bool {
        if self.identifier.is_none() {
            return false;
        }
        if self.version.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.identifier)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
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
        if let Some(ref v) = self.identifier.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.identifier.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CrashReport_ApplicationInfo {
    fn new() -> CrashReport_ApplicationInfo {
        CrashReport_ApplicationInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_ApplicationInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identifier",
                    CrashReport_ApplicationInfo::get_identifier_for_reflect,
                    CrashReport_ApplicationInfo::mut_identifier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    CrashReport_ApplicationInfo::get_version_for_reflect,
                    CrashReport_ApplicationInfo::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_ApplicationInfo>(
                    "CrashReport_ApplicationInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_ApplicationInfo {
    fn clear(&mut self) {
        self.clear_identifier();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_ApplicationInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_ApplicationInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Symbol {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    start_address: ::std::option::Option<u64>,
    end_address: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Symbol {}

impl CrashReport_Symbol {
    pub fn new() -> CrashReport_Symbol {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Symbol {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Symbol> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Symbol,
        };
        unsafe {
            instance.get(CrashReport_Symbol::new)
        }
    }

    // required string name = 1;

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
        }
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

    // required uint64 start_address = 2;

    pub fn clear_start_address(&mut self) {
        self.start_address = ::std::option::Option::None;
    }

    pub fn has_start_address(&self) -> bool {
        self.start_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_address(&mut self, v: u64) {
        self.start_address = ::std::option::Option::Some(v);
    }

    pub fn get_start_address(&self) -> u64 {
        self.start_address.unwrap_or(0)
    }

    fn get_start_address_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_address
    }

    fn mut_start_address_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_address
    }

    // optional uint64 end_address = 3;

    pub fn clear_end_address(&mut self) {
        self.end_address = ::std::option::Option::None;
    }

    pub fn has_end_address(&self) -> bool {
        self.end_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_address(&mut self, v: u64) {
        self.end_address = ::std::option::Option::Some(v);
    }

    pub fn get_end_address(&self) -> u64 {
        self.end_address.unwrap_or(0)
    }

    fn get_end_address_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.end_address
    }

    fn mut_end_address_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.end_address
    }
}

impl ::protobuf::Message for CrashReport_Symbol {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.start_address.is_none() {
            return false;
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_address = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.end_address = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.start_address {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.end_address {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.start_address {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.end_address {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for CrashReport_Symbol {
    fn new() -> CrashReport_Symbol {
        CrashReport_Symbol::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Symbol>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CrashReport_Symbol::get_name_for_reflect,
                    CrashReport_Symbol::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_address",
                    CrashReport_Symbol::get_start_address_for_reflect,
                    CrashReport_Symbol::mut_start_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "end_address",
                    CrashReport_Symbol::get_end_address_for_reflect,
                    CrashReport_Symbol::mut_end_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Symbol>(
                    "CrashReport_Symbol",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Symbol {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_start_address();
        self.clear_end_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Symbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Symbol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Thread {
    // message fields
    thread_number: ::std::option::Option<u32>,
    frames: ::protobuf::RepeatedField<CrashReport_Thread_StackFrame>,
    crashed: ::std::option::Option<bool>,
    registers: ::protobuf::RepeatedField<CrashReport_Thread_RegisterValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Thread {}

impl CrashReport_Thread {
    pub fn new() -> CrashReport_Thread {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Thread {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Thread> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Thread,
        };
        unsafe {
            instance.get(CrashReport_Thread::new)
        }
    }

    // required uint32 thread_number = 1;

    pub fn clear_thread_number(&mut self) {
        self.thread_number = ::std::option::Option::None;
    }

    pub fn has_thread_number(&self) -> bool {
        self.thread_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thread_number(&mut self, v: u32) {
        self.thread_number = ::std::option::Option::Some(v);
    }

    pub fn get_thread_number(&self) -> u32 {
        self.thread_number.unwrap_or(0)
    }

    fn get_thread_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.thread_number
    }

    fn mut_thread_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.thread_number
    }

    // repeated .plcrash.CrashReport.Thread.StackFrame frames = 2;

    pub fn clear_frames(&mut self) {
        self.frames.clear();
    }

    // Param is passed by value, moved
    pub fn set_frames(&mut self, v: ::protobuf::RepeatedField<CrashReport_Thread_StackFrame>) {
        self.frames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_frames(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        &mut self.frames
    }

    // Take field
    pub fn take_frames(&mut self) -> ::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        ::std::mem::replace(&mut self.frames, ::protobuf::RepeatedField::new())
    }

    pub fn get_frames(&self) -> &[CrashReport_Thread_StackFrame] {
        &self.frames
    }

    fn get_frames_for_reflect(&self) -> &::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        &self.frames
    }

    fn mut_frames_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        &mut self.frames
    }

    // required bool crashed = 3;

    pub fn clear_crashed(&mut self) {
        self.crashed = ::std::option::Option::None;
    }

    pub fn has_crashed(&self) -> bool {
        self.crashed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crashed(&mut self, v: bool) {
        self.crashed = ::std::option::Option::Some(v);
    }

    pub fn get_crashed(&self) -> bool {
        self.crashed.unwrap_or(false)
    }

    fn get_crashed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.crashed
    }

    fn mut_crashed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.crashed
    }

    // repeated .plcrash.CrashReport.Thread.RegisterValue registers = 4;

    pub fn clear_registers(&mut self) {
        self.registers.clear();
    }

    // Param is passed by value, moved
    pub fn set_registers(&mut self, v: ::protobuf::RepeatedField<CrashReport_Thread_RegisterValue>) {
        self.registers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_registers(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread_RegisterValue> {
        &mut self.registers
    }

    // Take field
    pub fn take_registers(&mut self) -> ::protobuf::RepeatedField<CrashReport_Thread_RegisterValue> {
        ::std::mem::replace(&mut self.registers, ::protobuf::RepeatedField::new())
    }

    pub fn get_registers(&self) -> &[CrashReport_Thread_RegisterValue] {
        &self.registers
    }

    fn get_registers_for_reflect(&self) -> &::protobuf::RepeatedField<CrashReport_Thread_RegisterValue> {
        &self.registers
    }

    fn mut_registers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread_RegisterValue> {
        &mut self.registers
    }
}

impl ::protobuf::Message for CrashReport_Thread {
    fn is_initialized(&self) -> bool {
        if self.thread_number.is_none() {
            return false;
        }
        if self.crashed.is_none() {
            return false;
        }
        for v in &self.frames {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.registers {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.thread_number = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.frames)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.crashed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.registers)?;
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
        if let Some(v) = self.thread_number {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.frames {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.crashed {
            my_size += 2;
        }
        for value in &self.registers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.thread_number {
            os.write_uint32(1, v)?;
        }
        for v in &self.frames {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.crashed {
            os.write_bool(3, v)?;
        }
        for v in &self.registers {
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

impl ::protobuf::MessageStatic for CrashReport_Thread {
    fn new() -> CrashReport_Thread {
        CrashReport_Thread::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Thread>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "thread_number",
                    CrashReport_Thread::get_thread_number_for_reflect,
                    CrashReport_Thread::mut_thread_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Thread_StackFrame>>(
                    "frames",
                    CrashReport_Thread::get_frames_for_reflect,
                    CrashReport_Thread::mut_frames_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "crashed",
                    CrashReport_Thread::get_crashed_for_reflect,
                    CrashReport_Thread::mut_crashed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Thread_RegisterValue>>(
                    "registers",
                    CrashReport_Thread::get_registers_for_reflect,
                    CrashReport_Thread::mut_registers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Thread>(
                    "CrashReport_Thread",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Thread {
    fn clear(&mut self) {
        self.clear_thread_number();
        self.clear_frames();
        self.clear_crashed();
        self.clear_registers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Thread {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Thread {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Thread_StackFrame {
    // message fields
    pc: ::std::option::Option<u64>,
    symbol: ::protobuf::SingularPtrField<CrashReport_Symbol>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Thread_StackFrame {}

impl CrashReport_Thread_StackFrame {
    pub fn new() -> CrashReport_Thread_StackFrame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Thread_StackFrame {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Thread_StackFrame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Thread_StackFrame,
        };
        unsafe {
            instance.get(CrashReport_Thread_StackFrame::new)
        }
    }

    // required uint64 pc = 3;

    pub fn clear_pc(&mut self) {
        self.pc = ::std::option::Option::None;
    }

    pub fn has_pc(&self) -> bool {
        self.pc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pc(&mut self, v: u64) {
        self.pc = ::std::option::Option::Some(v);
    }

    pub fn get_pc(&self) -> u64 {
        self.pc.unwrap_or(0)
    }

    fn get_pc_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.pc
    }

    fn mut_pc_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.pc
    }

    // optional .plcrash.CrashReport.Symbol symbol = 6;

    pub fn clear_symbol(&mut self) {
        self.symbol.clear();
    }

    pub fn has_symbol(&self) -> bool {
        self.symbol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symbol(&mut self, v: CrashReport_Symbol) {
        self.symbol = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symbol(&mut self) -> &mut CrashReport_Symbol {
        if self.symbol.is_none() {
            self.symbol.set_default();
        }
        self.symbol.as_mut().unwrap()
    }

    // Take field
    pub fn take_symbol(&mut self) -> CrashReport_Symbol {
        self.symbol.take().unwrap_or_else(|| CrashReport_Symbol::new())
    }

    pub fn get_symbol(&self) -> &CrashReport_Symbol {
        self.symbol.as_ref().unwrap_or_else(|| CrashReport_Symbol::default_instance())
    }

    fn get_symbol_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_Symbol> {
        &self.symbol
    }

    fn mut_symbol_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_Symbol> {
        &mut self.symbol
    }
}

impl ::protobuf::Message for CrashReport_Thread_StackFrame {
    fn is_initialized(&self) -> bool {
        if self.pc.is_none() {
            return false;
        }
        for v in &self.symbol {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.pc = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.symbol)?;
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
        if let Some(v) = self.pc {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.symbol.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pc {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.symbol.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for CrashReport_Thread_StackFrame {
    fn new() -> CrashReport_Thread_StackFrame {
        CrashReport_Thread_StackFrame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Thread_StackFrame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "pc",
                    CrashReport_Thread_StackFrame::get_pc_for_reflect,
                    CrashReport_Thread_StackFrame::mut_pc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Symbol>>(
                    "symbol",
                    CrashReport_Thread_StackFrame::get_symbol_for_reflect,
                    CrashReport_Thread_StackFrame::mut_symbol_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Thread_StackFrame>(
                    "CrashReport_Thread_StackFrame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Thread_StackFrame {
    fn clear(&mut self) {
        self.clear_pc();
        self.clear_symbol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Thread_StackFrame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Thread_StackFrame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Thread_RegisterValue {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Thread_RegisterValue {}

impl CrashReport_Thread_RegisterValue {
    pub fn new() -> CrashReport_Thread_RegisterValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Thread_RegisterValue {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Thread_RegisterValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Thread_RegisterValue,
        };
        unsafe {
            instance.get(CrashReport_Thread_RegisterValue::new)
        }
    }

    // required string name = 1;

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
        }
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

    // required uint64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> u64 {
        self.value.unwrap_or(0)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.value
    }
}

impl ::protobuf::Message for CrashReport_Thread_RegisterValue {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.value.is_none() {
            return false;
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.value = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.value {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for CrashReport_Thread_RegisterValue {
    fn new() -> CrashReport_Thread_RegisterValue {
        CrashReport_Thread_RegisterValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Thread_RegisterValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CrashReport_Thread_RegisterValue::get_name_for_reflect,
                    CrashReport_Thread_RegisterValue::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "value",
                    CrashReport_Thread_RegisterValue::get_value_for_reflect,
                    CrashReport_Thread_RegisterValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Thread_RegisterValue>(
                    "CrashReport_Thread_RegisterValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Thread_RegisterValue {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Thread_RegisterValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Thread_RegisterValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_BinaryImage {
    // message fields
    base_address: ::std::option::Option<u64>,
    size: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    code_type: ::protobuf::SingularPtrField<CrashReport_Processor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_BinaryImage {}

impl CrashReport_BinaryImage {
    pub fn new() -> CrashReport_BinaryImage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_BinaryImage {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_BinaryImage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_BinaryImage,
        };
        unsafe {
            instance.get(CrashReport_BinaryImage::new)
        }
    }

    // required uint64 base_address = 1;

    pub fn clear_base_address(&mut self) {
        self.base_address = ::std::option::Option::None;
    }

    pub fn has_base_address(&self) -> bool {
        self.base_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_address(&mut self, v: u64) {
        self.base_address = ::std::option::Option::Some(v);
    }

    pub fn get_base_address(&self) -> u64 {
        self.base_address.unwrap_or(0)
    }

    fn get_base_address_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.base_address
    }

    fn mut_base_address_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.base_address
    }

    // required uint64 size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u64 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.size
    }

    // required string name = 3;

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
        }
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

    // optional bytes uuid = 4;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        }
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.uuid
    }

    // optional .plcrash.CrashReport.Processor code_type = 5;

    pub fn clear_code_type(&mut self) {
        self.code_type.clear();
    }

    pub fn has_code_type(&self) -> bool {
        self.code_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_type(&mut self, v: CrashReport_Processor) {
        self.code_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code_type(&mut self) -> &mut CrashReport_Processor {
        if self.code_type.is_none() {
            self.code_type.set_default();
        }
        self.code_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_code_type(&mut self) -> CrashReport_Processor {
        self.code_type.take().unwrap_or_else(|| CrashReport_Processor::new())
    }

    pub fn get_code_type(&self) -> &CrashReport_Processor {
        self.code_type.as_ref().unwrap_or_else(|| CrashReport_Processor::default_instance())
    }

    fn get_code_type_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_Processor> {
        &self.code_type
    }

    fn mut_code_type_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_Processor> {
        &mut self.code_type
    }
}

impl ::protobuf::Message for CrashReport_BinaryImage {
    fn is_initialized(&self) -> bool {
        if self.base_address.is_none() {
            return false;
        }
        if self.size.is_none() {
            return false;
        }
        if self.name.is_none() {
            return false;
        }
        for v in &self.code_type {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.base_address = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.code_type)?;
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
        if let Some(v) = self.base_address {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.code_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base_address {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.size {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.uuid.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.code_type.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for CrashReport_BinaryImage {
    fn new() -> CrashReport_BinaryImage {
        CrashReport_BinaryImage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_BinaryImage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_address",
                    CrashReport_BinaryImage::get_base_address_for_reflect,
                    CrashReport_BinaryImage::mut_base_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    CrashReport_BinaryImage::get_size_for_reflect,
                    CrashReport_BinaryImage::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CrashReport_BinaryImage::get_name_for_reflect,
                    CrashReport_BinaryImage::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    CrashReport_BinaryImage::get_uuid_for_reflect,
                    CrashReport_BinaryImage::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Processor>>(
                    "code_type",
                    CrashReport_BinaryImage::get_code_type_for_reflect,
                    CrashReport_BinaryImage::mut_code_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_BinaryImage>(
                    "CrashReport_BinaryImage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_BinaryImage {
    fn clear(&mut self) {
        self.clear_base_address();
        self.clear_size();
        self.clear_name();
        self.clear_uuid();
        self.clear_code_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_BinaryImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_BinaryImage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Exception {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    reason: ::protobuf::SingularField<::std::string::String>,
    frames: ::protobuf::RepeatedField<CrashReport_Thread_StackFrame>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Exception {}

impl CrashReport_Exception {
    pub fn new() -> CrashReport_Exception {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Exception {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Exception> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Exception,
        };
        unsafe {
            instance.get(CrashReport_Exception::new)
        }
    }

    // required string name = 1;

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
        }
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

    // required string reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        if self.reason.is_none() {
            self.reason.set_default();
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        self.reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.reason
    }

    // repeated .plcrash.CrashReport.Thread.StackFrame frames = 3;

    pub fn clear_frames(&mut self) {
        self.frames.clear();
    }

    // Param is passed by value, moved
    pub fn set_frames(&mut self, v: ::protobuf::RepeatedField<CrashReport_Thread_StackFrame>) {
        self.frames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_frames(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        &mut self.frames
    }

    // Take field
    pub fn take_frames(&mut self) -> ::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        ::std::mem::replace(&mut self.frames, ::protobuf::RepeatedField::new())
    }

    pub fn get_frames(&self) -> &[CrashReport_Thread_StackFrame] {
        &self.frames
    }

    fn get_frames_for_reflect(&self) -> &::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        &self.frames
    }

    fn mut_frames_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CrashReport_Thread_StackFrame> {
        &mut self.frames
    }
}

impl ::protobuf::Message for CrashReport_Exception {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.reason.is_none() {
            return false;
        }
        for v in &self.frames {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.frames)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.frames {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.frames {
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

impl ::protobuf::MessageStatic for CrashReport_Exception {
    fn new() -> CrashReport_Exception {
        CrashReport_Exception::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Exception>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CrashReport_Exception::get_name_for_reflect,
                    CrashReport_Exception::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    CrashReport_Exception::get_reason_for_reflect,
                    CrashReport_Exception::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Thread_StackFrame>>(
                    "frames",
                    CrashReport_Exception::get_frames_for_reflect,
                    CrashReport_Exception::mut_frames_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Exception>(
                    "CrashReport_Exception",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Exception {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_reason();
        self.clear_frames();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Exception {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Exception {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Signal {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    code: ::protobuf::SingularField<::std::string::String>,
    address: ::std::option::Option<u64>,
    mach_exception: ::protobuf::SingularPtrField<CrashReport_Signal_MachException>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Signal {}

impl CrashReport_Signal {
    pub fn new() -> CrashReport_Signal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Signal {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Signal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Signal,
        };
        unsafe {
            instance.get(CrashReport_Signal::new)
        }
    }

    // required string name = 1;

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
        }
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

    // required string code = 2;

    pub fn clear_code(&mut self) {
        self.code.clear();
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::std::string::String) {
        self.code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code(&mut self) -> &mut ::std::string::String {
        if self.code.is_none() {
            self.code.set_default();
        }
        self.code.as_mut().unwrap()
    }

    // Take field
    pub fn take_code(&mut self) -> ::std::string::String {
        self.code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_code(&self) -> &str {
        match self.code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.code
    }

    // required uint64 address = 3;

    pub fn clear_address(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: u64) {
        self.address = ::std::option::Option::Some(v);
    }

    pub fn get_address(&self) -> u64 {
        self.address.unwrap_or(0)
    }

    fn get_address_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.address
    }

    // optional .plcrash.CrashReport.Signal.MachException mach_exception = 4;

    pub fn clear_mach_exception(&mut self) {
        self.mach_exception.clear();
    }

    pub fn has_mach_exception(&self) -> bool {
        self.mach_exception.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mach_exception(&mut self, v: CrashReport_Signal_MachException) {
        self.mach_exception = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mach_exception(&mut self) -> &mut CrashReport_Signal_MachException {
        if self.mach_exception.is_none() {
            self.mach_exception.set_default();
        }
        self.mach_exception.as_mut().unwrap()
    }

    // Take field
    pub fn take_mach_exception(&mut self) -> CrashReport_Signal_MachException {
        self.mach_exception.take().unwrap_or_else(|| CrashReport_Signal_MachException::new())
    }

    pub fn get_mach_exception(&self) -> &CrashReport_Signal_MachException {
        self.mach_exception.as_ref().unwrap_or_else(|| CrashReport_Signal_MachException::default_instance())
    }

    fn get_mach_exception_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_Signal_MachException> {
        &self.mach_exception
    }

    fn mut_mach_exception_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_Signal_MachException> {
        &mut self.mach_exception
    }
}

impl ::protobuf::Message for CrashReport_Signal {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.code.is_none() {
            return false;
        }
        if self.address.is_none() {
            return false;
        }
        for v in &self.mach_exception {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.code)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.address = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mach_exception)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.code.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.address {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.mach_exception.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.code.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.address {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.mach_exception.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for CrashReport_Signal {
    fn new() -> CrashReport_Signal {
        CrashReport_Signal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Signal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CrashReport_Signal::get_name_for_reflect,
                    CrashReport_Signal::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "code",
                    CrashReport_Signal::get_code_for_reflect,
                    CrashReport_Signal::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "address",
                    CrashReport_Signal::get_address_for_reflect,
                    CrashReport_Signal::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Signal_MachException>>(
                    "mach_exception",
                    CrashReport_Signal::get_mach_exception_for_reflect,
                    CrashReport_Signal::mut_mach_exception_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Signal>(
                    "CrashReport_Signal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Signal {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_code();
        self.clear_address();
        self.clear_mach_exception();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Signal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Signal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_Signal_MachException {
    // message fields
    field_type: ::std::option::Option<u64>,
    codes: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_Signal_MachException {}

impl CrashReport_Signal_MachException {
    pub fn new() -> CrashReport_Signal_MachException {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_Signal_MachException {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_Signal_MachException> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_Signal_MachException,
        };
        unsafe {
            instance.get(CrashReport_Signal_MachException::new)
        }
    }

    // required uint64 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: u64) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> u64 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.field_type
    }

    // repeated uint64 codes = 2;

    pub fn clear_codes(&mut self) {
        self.codes.clear();
    }

    // Param is passed by value, moved
    pub fn set_codes(&mut self, v: ::std::vec::Vec<u64>) {
        self.codes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_codes(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.codes
    }

    // Take field
    pub fn take_codes(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.codes, ::std::vec::Vec::new())
    }

    pub fn get_codes(&self) -> &[u64] {
        &self.codes
    }

    fn get_codes_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.codes
    }

    fn mut_codes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.codes
    }
}

impl ::protobuf::Message for CrashReport_Signal_MachException {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.codes)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.codes {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_uint64(1, v)?;
        }
        for v in &self.codes {
            os.write_uint64(2, *v)?;
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

impl ::protobuf::MessageStatic for CrashReport_Signal_MachException {
    fn new() -> CrashReport_Signal_MachException {
        CrashReport_Signal_MachException::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_Signal_MachException>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "type",
                    CrashReport_Signal_MachException::get_field_type_for_reflect,
                    CrashReport_Signal_MachException::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "codes",
                    CrashReport_Signal_MachException::get_codes_for_reflect,
                    CrashReport_Signal_MachException::mut_codes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_Signal_MachException>(
                    "CrashReport_Signal_MachException",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_Signal_MachException {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_codes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_Signal_MachException {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_Signal_MachException {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_ProcessInfo {
    // message fields
    process_name: ::protobuf::SingularField<::std::string::String>,
    process_id: ::std::option::Option<u32>,
    process_path: ::protobuf::SingularField<::std::string::String>,
    parent_process_name: ::protobuf::SingularField<::std::string::String>,
    parent_process_id: ::std::option::Option<u32>,
    native: ::std::option::Option<bool>,
    start_time: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_ProcessInfo {}

impl CrashReport_ProcessInfo {
    pub fn new() -> CrashReport_ProcessInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_ProcessInfo {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_ProcessInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_ProcessInfo,
        };
        unsafe {
            instance.get(CrashReport_ProcessInfo::new)
        }
    }

    // optional string process_name = 1;

    pub fn clear_process_name(&mut self) {
        self.process_name.clear();
    }

    pub fn has_process_name(&self) -> bool {
        self.process_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process_name(&mut self, v: ::std::string::String) {
        self.process_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_process_name(&mut self) -> &mut ::std::string::String {
        if self.process_name.is_none() {
            self.process_name.set_default();
        }
        self.process_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_process_name(&mut self) -> ::std::string::String {
        self.process_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_process_name(&self) -> &str {
        match self.process_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_process_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.process_name
    }

    fn mut_process_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.process_name
    }

    // required uint32 process_id = 2;

    pub fn clear_process_id(&mut self) {
        self.process_id = ::std::option::Option::None;
    }

    pub fn has_process_id(&self) -> bool {
        self.process_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process_id(&mut self, v: u32) {
        self.process_id = ::std::option::Option::Some(v);
    }

    pub fn get_process_id(&self) -> u32 {
        self.process_id.unwrap_or(0)
    }

    fn get_process_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.process_id
    }

    fn mut_process_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.process_id
    }

    // optional string process_path = 3;

    pub fn clear_process_path(&mut self) {
        self.process_path.clear();
    }

    pub fn has_process_path(&self) -> bool {
        self.process_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process_path(&mut self, v: ::std::string::String) {
        self.process_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_process_path(&mut self) -> &mut ::std::string::String {
        if self.process_path.is_none() {
            self.process_path.set_default();
        }
        self.process_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_process_path(&mut self) -> ::std::string::String {
        self.process_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_process_path(&self) -> &str {
        match self.process_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_process_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.process_path
    }

    fn mut_process_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.process_path
    }

    // optional string parent_process_name = 4;

    pub fn clear_parent_process_name(&mut self) {
        self.parent_process_name.clear();
    }

    pub fn has_parent_process_name(&self) -> bool {
        self.parent_process_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_process_name(&mut self, v: ::std::string::String) {
        self.parent_process_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_process_name(&mut self) -> &mut ::std::string::String {
        if self.parent_process_name.is_none() {
            self.parent_process_name.set_default();
        }
        self.parent_process_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent_process_name(&mut self) -> ::std::string::String {
        self.parent_process_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_parent_process_name(&self) -> &str {
        match self.parent_process_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_parent_process_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.parent_process_name
    }

    fn mut_parent_process_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.parent_process_name
    }

    // required uint32 parent_process_id = 5;

    pub fn clear_parent_process_id(&mut self) {
        self.parent_process_id = ::std::option::Option::None;
    }

    pub fn has_parent_process_id(&self) -> bool {
        self.parent_process_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_process_id(&mut self, v: u32) {
        self.parent_process_id = ::std::option::Option::Some(v);
    }

    pub fn get_parent_process_id(&self) -> u32 {
        self.parent_process_id.unwrap_or(0)
    }

    fn get_parent_process_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.parent_process_id
    }

    fn mut_parent_process_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.parent_process_id
    }

    // required bool native = 6;

    pub fn clear_native(&mut self) {
        self.native = ::std::option::Option::None;
    }

    pub fn has_native(&self) -> bool {
        self.native.is_some()
    }

    // Param is passed by value, moved
    pub fn set_native(&mut self, v: bool) {
        self.native = ::std::option::Option::Some(v);
    }

    pub fn get_native(&self) -> bool {
        self.native.unwrap_or(false)
    }

    fn get_native_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.native
    }

    fn mut_native_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.native
    }

    // optional uint64 start_time = 7;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: u64) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> u64 {
        self.start_time.unwrap_or(0)
    }

    fn get_start_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_time
    }
}

impl ::protobuf::Message for CrashReport_ProcessInfo {
    fn is_initialized(&self) -> bool {
        if self.process_id.is_none() {
            return false;
        }
        if self.parent_process_id.is_none() {
            return false;
        }
        if self.native.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.process_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.process_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.process_path)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.parent_process_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.parent_process_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.native = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_time = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.process_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.process_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.process_path.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.parent_process_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.parent_process_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.native {
            my_size += 2;
        }
        if let Some(v) = self.start_time {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.process_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.process_id {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.process_path.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.parent_process_name.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.parent_process_id {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.native {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.start_time {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for CrashReport_ProcessInfo {
    fn new() -> CrashReport_ProcessInfo {
        CrashReport_ProcessInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_ProcessInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "process_name",
                    CrashReport_ProcessInfo::get_process_name_for_reflect,
                    CrashReport_ProcessInfo::mut_process_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "process_id",
                    CrashReport_ProcessInfo::get_process_id_for_reflect,
                    CrashReport_ProcessInfo::mut_process_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "process_path",
                    CrashReport_ProcessInfo::get_process_path_for_reflect,
                    CrashReport_ProcessInfo::mut_process_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "parent_process_name",
                    CrashReport_ProcessInfo::get_parent_process_name_for_reflect,
                    CrashReport_ProcessInfo::mut_parent_process_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "parent_process_id",
                    CrashReport_ProcessInfo::get_parent_process_id_for_reflect,
                    CrashReport_ProcessInfo::mut_parent_process_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "native",
                    CrashReport_ProcessInfo::get_native_for_reflect,
                    CrashReport_ProcessInfo::mut_native_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_time",
                    CrashReport_ProcessInfo::get_start_time_for_reflect,
                    CrashReport_ProcessInfo::mut_start_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_ProcessInfo>(
                    "CrashReport_ProcessInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_ProcessInfo {
    fn clear(&mut self) {
        self.clear_process_name();
        self.clear_process_id();
        self.clear_process_path();
        self.clear_parent_process_name();
        self.clear_parent_process_id();
        self.clear_native();
        self.clear_start_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_ProcessInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_ProcessInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_MachineInfo {
    // message fields
    model: ::protobuf::SingularField<::std::string::String>,
    processor: ::protobuf::SingularPtrField<CrashReport_Processor>,
    processor_count: ::std::option::Option<u32>,
    logical_processor_count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_MachineInfo {}

impl CrashReport_MachineInfo {
    pub fn new() -> CrashReport_MachineInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_MachineInfo {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_MachineInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_MachineInfo,
        };
        unsafe {
            instance.get(CrashReport_MachineInfo::new)
        }
    }

    // optional string model = 1;

    pub fn clear_model(&mut self) {
        self.model.clear();
    }

    pub fn has_model(&self) -> bool {
        self.model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model(&mut self, v: ::std::string::String) {
        self.model = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model(&mut self) -> &mut ::std::string::String {
        if self.model.is_none() {
            self.model.set_default();
        }
        self.model.as_mut().unwrap()
    }

    // Take field
    pub fn take_model(&mut self) -> ::std::string::String {
        self.model.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_model(&self) -> &str {
        match self.model.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_model_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.model
    }

    fn mut_model_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.model
    }

    // required .plcrash.CrashReport.Processor processor = 2;

    pub fn clear_processor(&mut self) {
        self.processor.clear();
    }

    pub fn has_processor(&self) -> bool {
        self.processor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_processor(&mut self, v: CrashReport_Processor) {
        self.processor = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_processor(&mut self) -> &mut CrashReport_Processor {
        if self.processor.is_none() {
            self.processor.set_default();
        }
        self.processor.as_mut().unwrap()
    }

    // Take field
    pub fn take_processor(&mut self) -> CrashReport_Processor {
        self.processor.take().unwrap_or_else(|| CrashReport_Processor::new())
    }

    pub fn get_processor(&self) -> &CrashReport_Processor {
        self.processor.as_ref().unwrap_or_else(|| CrashReport_Processor::default_instance())
    }

    fn get_processor_for_reflect(&self) -> &::protobuf::SingularPtrField<CrashReport_Processor> {
        &self.processor
    }

    fn mut_processor_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrashReport_Processor> {
        &mut self.processor
    }

    // required uint32 processor_count = 3;

    pub fn clear_processor_count(&mut self) {
        self.processor_count = ::std::option::Option::None;
    }

    pub fn has_processor_count(&self) -> bool {
        self.processor_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_processor_count(&mut self, v: u32) {
        self.processor_count = ::std::option::Option::Some(v);
    }

    pub fn get_processor_count(&self) -> u32 {
        self.processor_count.unwrap_or(0)
    }

    fn get_processor_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.processor_count
    }

    fn mut_processor_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.processor_count
    }

    // required uint32 logical_processor_count = 4;

    pub fn clear_logical_processor_count(&mut self) {
        self.logical_processor_count = ::std::option::Option::None;
    }

    pub fn has_logical_processor_count(&self) -> bool {
        self.logical_processor_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logical_processor_count(&mut self, v: u32) {
        self.logical_processor_count = ::std::option::Option::Some(v);
    }

    pub fn get_logical_processor_count(&self) -> u32 {
        self.logical_processor_count.unwrap_or(0)
    }

    fn get_logical_processor_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.logical_processor_count
    }

    fn mut_logical_processor_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.logical_processor_count
    }
}

impl ::protobuf::Message for CrashReport_MachineInfo {
    fn is_initialized(&self) -> bool {
        if self.processor.is_none() {
            return false;
        }
        if self.processor_count.is_none() {
            return false;
        }
        if self.logical_processor_count.is_none() {
            return false;
        }
        for v in &self.processor {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.model)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.processor)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.processor_count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.logical_processor_count = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.model.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.processor.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.processor_count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.logical_processor_count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.model.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.processor.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.processor_count {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.logical_processor_count {
            os.write_uint32(4, v)?;
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

impl ::protobuf::MessageStatic for CrashReport_MachineInfo {
    fn new() -> CrashReport_MachineInfo {
        CrashReport_MachineInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_MachineInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "model",
                    CrashReport_MachineInfo::get_model_for_reflect,
                    CrashReport_MachineInfo::mut_model_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrashReport_Processor>>(
                    "processor",
                    CrashReport_MachineInfo::get_processor_for_reflect,
                    CrashReport_MachineInfo::mut_processor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "processor_count",
                    CrashReport_MachineInfo::get_processor_count_for_reflect,
                    CrashReport_MachineInfo::mut_processor_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "logical_processor_count",
                    CrashReport_MachineInfo::get_logical_processor_count_for_reflect,
                    CrashReport_MachineInfo::mut_logical_processor_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_MachineInfo>(
                    "CrashReport_MachineInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_MachineInfo {
    fn clear(&mut self) {
        self.clear_model();
        self.clear_processor();
        self.clear_processor_count();
        self.clear_logical_processor_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_MachineInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_MachineInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CrashReport_ReportInfo {
    // message fields
    user_requested: ::std::option::Option<bool>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrashReport_ReportInfo {}

impl CrashReport_ReportInfo {
    pub fn new() -> CrashReport_ReportInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrashReport_ReportInfo {
        static mut instance: ::protobuf::lazy::Lazy<CrashReport_ReportInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrashReport_ReportInfo,
        };
        unsafe {
            instance.get(CrashReport_ReportInfo::new)
        }
    }

    // required bool user_requested = 1;

    pub fn clear_user_requested(&mut self) {
        self.user_requested = ::std::option::Option::None;
    }

    pub fn has_user_requested(&self) -> bool {
        self.user_requested.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_requested(&mut self, v: bool) {
        self.user_requested = ::std::option::Option::Some(v);
    }

    pub fn get_user_requested(&self) -> bool {
        self.user_requested.unwrap_or(false)
    }

    fn get_user_requested_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.user_requested
    }

    fn mut_user_requested_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.user_requested
    }

    // optional bytes uuid = 2;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        }
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for CrashReport_ReportInfo {
    fn is_initialized(&self) -> bool {
        if self.user_requested.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.user_requested = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if let Some(v) = self.user_requested {
            my_size += 2;
        }
        if let Some(ref v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user_requested {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.uuid.as_ref() {
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

impl ::protobuf::MessageStatic for CrashReport_ReportInfo {
    fn new() -> CrashReport_ReportInfo {
        CrashReport_ReportInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrashReport_ReportInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "user_requested",
                    CrashReport_ReportInfo::get_user_requested_for_reflect,
                    CrashReport_ReportInfo::mut_user_requested_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    CrashReport_ReportInfo::get_uuid_for_reflect,
                    CrashReport_ReportInfo::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrashReport_ReportInfo>(
                    "CrashReport_ReportInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrashReport_ReportInfo {
    fn clear(&mut self) {
        self.clear_user_requested();
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrashReport_ReportInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrashReport_ReportInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Architecture {
    X86_32 = 0,
    X86_64 = 1,
    ARMV6 = 2,
    PPC = 3,
    PPC64 = 4,
    ARMV7 = 5,
    ARCHITECTURE_UNKNOWN = 6,
}

impl ::protobuf::ProtobufEnum for Architecture {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Architecture> {
        match value {
            0 => ::std::option::Option::Some(Architecture::X86_32),
            1 => ::std::option::Option::Some(Architecture::X86_64),
            2 => ::std::option::Option::Some(Architecture::ARMV6),
            3 => ::std::option::Option::Some(Architecture::PPC),
            4 => ::std::option::Option::Some(Architecture::PPC64),
            5 => ::std::option::Option::Some(Architecture::ARMV7),
            6 => ::std::option::Option::Some(Architecture::ARCHITECTURE_UNKNOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Architecture] = &[
            Architecture::X86_32,
            Architecture::X86_64,
            Architecture::ARMV6,
            Architecture::PPC,
            Architecture::PPC64,
            Architecture::ARMV7,
            Architecture::ARCHITECTURE_UNKNOWN,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Architecture>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Architecture", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Architecture {
}

impl ::protobuf::reflect::ProtobufValue for Architecture {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12crash_report.proto\x12\x07plcrash\"\xd9\x15\n\x0bCrashReport\x12@\
    \n\x0bsystem_info\x18\x01\x20\x02(\x0b2\x1f.plcrash.CrashReport.SystemIn\
    foR\nsystemInfo\x12O\n\x10application_info\x18\x02\x20\x02(\x0b2$.plcras\
    h.CrashReport.ApplicationInfoR\x0fapplicationInfo\x125\n\x07threads\x18\
    \x03\x20\x03(\x0b2\x1b.plcrash.CrashReport.ThreadR\x07threads\x12E\n\rbi\
    nary_images\x18\x04\x20\x03(\x0b2\x20.plcrash.CrashReport.BinaryImageR\
    \x0cbinaryImages\x12<\n\texception\x18\x05\x20\x01(\x0b2\x1e.plcrash.Cra\
    shReport.ExceptionR\texception\x123\n\x06signal\x18\x06\x20\x02(\x0b2\
    \x1b.plcrash.CrashReport.SignalR\x06signal\x12C\n\x0cprocess_info\x18\
    \x07\x20\x01(\x0b2\x20.plcrash.CrashReport.ProcessInfoR\x0bprocessInfo\
    \x12C\n\x0cmachine_info\x18\x08\x20\x01(\x0b2\x20.plcrash.CrashReport.Ma\
    chineInfoR\x0bmachineInfo\x12@\n\x0breport_info\x18\t\x20\x01(\x0b2\x1f.\
    plcrash.CrashReport.ReportInfoR\nreportInfo\x1a\xdc\x01\n\tProcessor\x12\
    ^\n\x08encoding\x18\x01\x20\x01(\x0e2+.plcrash.CrashReport.Processor.Typ\
    eEncoding:\x15TYPE_ENCODING_UNKNOWNR\x08encoding\x12\x12\n\x04type\x18\
    \x02\x20\x02(\x04R\x04type\x12\x18\n\x07subtype\x18\x03\x20\x02(\x04R\
    \x07subtype\"A\n\x0cTypeEncoding\x12\x19\n\x15TYPE_ENCODING_UNKNOWN\x10\
    \0\x12\x16\n\x12TYPE_ENCODING_MACH\x10\x01\x1a\xf3\x02\n\nSystemInfo\x12\
    f\n\x10operating_system\x18\x01\x20\x01(\x0e2/.plcrash.CrashReport.Syste\
    mInfo.OperatingSystem:\nOS_UNKNOWNR\x0foperatingSystem\x12\x1d\n\nos_ver\
    sion\x18\x02\x20\x02(\tR\tosVersion\x12O\n\x0carchitecture\x18\x03\x20\
    \x02(\x0e2\x15.plcrash.Architecture:\x14ARCHITECTURE_UNKNOWNR\x0carchite\
    cture\x12\x1c\n\ttimestamp\x18\x04\x20\x02(\x03R\ttimestamp\x12\x19\n\
    \x08os_build\x18\x05\x20\x01(\tR\x07osBuild\"T\n\x0fOperatingSystem\x12\
    \x0c\n\x08MAC_OS_X\x10\0\x12\r\n\tIPHONE_OS\x10\x01\x12\x14\n\x10IPHONE_\
    SIMULATOR\x10\x02\x12\x0e\n\nOS_UNKNOWN\x10\x03\x1aK\n\x0fApplicationInf\
    o\x12\x1e\n\nidentifier\x18\x01\x20\x02(\tR\nidentifier\x12\x18\n\x07ver\
    sion\x18\x02\x20\x02(\tR\x07version\x1ab\n\x06Symbol\x12\x12\n\x04name\
    \x18\x01\x20\x02(\tR\x04name\x12#\n\rstart_address\x18\x02\x20\x02(\x04R\
    \x0cstartAddress\x12\x1f\n\x0bend_address\x18\x03\x20\x01(\x04R\nendAddr\
    ess\x1a\xde\x02\n\x06Thread\x12#\n\rthread_number\x18\x01\x20\x02(\rR\
    \x0cthreadNumber\x12>\n\x06frames\x18\x02\x20\x03(\x0b2&.plcrash.CrashRe\
    port.Thread.StackFrameR\x06frames\x12\x18\n\x07crashed\x18\x03\x20\x02(\
    \x08R\x07crashed\x12G\n\tregisters\x18\x04\x20\x03(\x0b2).plcrash.CrashR\
    eport.Thread.RegisterValueR\tregisters\x1aQ\n\nStackFrame\x12\x0e\n\x02p\
    c\x18\x03\x20\x02(\x04R\x02pc\x123\n\x06symbol\x18\x06\x20\x01(\x0b2\x1b\
    .plcrash.CrashReport.SymbolR\x06symbol\x1a9\n\rRegisterValue\x12\x12\n\
    \x04name\x18\x01\x20\x02(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x02(\
    \x04R\x05value\x1a\xa9\x01\n\x0bBinaryImage\x12!\n\x0cbase_address\x18\
    \x01\x20\x02(\x04R\x0bbaseAddress\x12\x12\n\x04size\x18\x02\x20\x02(\x04\
    R\x04size\x12\x12\n\x04name\x18\x03\x20\x02(\tR\x04name\x12\x12\n\x04uui\
    d\x18\x04\x20\x01(\x0cR\x04uuid\x12;\n\tcode_type\x18\x05\x20\x01(\x0b2\
    \x1e.plcrash.CrashReport.ProcessorR\x08codeType\x1aw\n\tException\x12\
    \x12\n\x04name\x18\x01\x20\x02(\tR\x04name\x12\x16\n\x06reason\x18\x02\
    \x20\x02(\tR\x06reason\x12>\n\x06frames\x18\x03\x20\x03(\x0b2&.plcrash.C\
    rashReport.Thread.StackFrameR\x06frames\x1a\xd7\x01\n\x06Signal\x12\x12\
    \n\x04name\x18\x01\x20\x02(\tR\x04name\x12\x12\n\x04code\x18\x02\x20\x02\
    (\tR\x04code\x12\x18\n\x07address\x18\x03\x20\x02(\x04R\x07address\x12P\
    \n\x0emach_exception\x18\x04\x20\x01(\x0b2).plcrash.CrashReport.Signal.M\
    achExceptionR\rmachException\x1a9\n\rMachException\x12\x12\n\x04type\x18\
    \x01\x20\x02(\x04R\x04type\x12\x14\n\x05codes\x18\x02\x20\x03(\x04R\x05c\
    odes\x1a\x85\x02\n\x0bProcessInfo\x12!\n\x0cprocess_name\x18\x01\x20\x01\
    (\tR\x0bprocessName\x12\x1d\n\nprocess_id\x18\x02\x20\x02(\rR\tprocessId\
    \x12!\n\x0cprocess_path\x18\x03\x20\x01(\tR\x0bprocessPath\x12.\n\x13par\
    ent_process_name\x18\x04\x20\x01(\tR\x11parentProcessName\x12*\n\x11pare\
    nt_process_id\x18\x05\x20\x02(\rR\x0fparentProcessId\x12\x16\n\x06native\
    \x18\x06\x20\x02(\x08R\x06native\x12\x1d\n\nstart_time\x18\x07\x20\x01(\
    \x04R\tstartTime\x1a\xc2\x01\n\x0bMachineInfo\x12\x14\n\x05model\x18\x01\
    \x20\x01(\tR\x05model\x12<\n\tprocessor\x18\x02\x20\x02(\x0b2\x1e.plcras\
    h.CrashReport.ProcessorR\tprocessor\x12'\n\x0fprocessor_count\x18\x03\
    \x20\x02(\rR\x0eprocessorCount\x126\n\x17logical_processor_count\x18\x04\
    \x20\x02(\rR\x15logicalProcessorCount\x1aG\n\nReportInfo\x12%\n\x0euser_\
    requested\x18\x01\x20\x02(\x08R\ruserRequested\x12\x12\n\x04uuid\x18\x02\
    \x20\x01(\x0cR\x04uuid*j\n\x0cArchitecture\x12\n\n\x06X86_32\x10\0\x12\n\
    \n\x06X86_64\x10\x01\x12\t\n\x05ARMV6\x10\x02\x12\x07\n\x03PPC\x10\x03\
    \x12\t\n\x05PPC64\x10\x04\x12\t\n\x05ARMV7\x10\x05\x12\x18\n\x14ARCHITEC\
    TURE_UNKNOWN\x10\x06B.\n\x1ccoop.plausible.crashreporterB\x0eCrashReport\
    _pb\
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
