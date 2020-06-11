// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::bthdef::{BTH_ADDR, BTH_DEVICE_INFO, MAX_UUIDS_IN_QUERY};
use shared::bthsdpdef::{SdpAttributeRange, SdpQueryUuid};
use shared::minwindef::{DWORD, UCHAR, ULONG, USHORT};
use shared::ntdef::{BOOLEAN, ULONGLONG};
use um::winioctl::{FILE_ANY_ACCESS, FILE_DEVICE_BLUETOOTH, METHOD_BUFFERED, METHOD_NEITHER};
pub const BTH_IOCTL_BASE: DWORD = 0;
pub const IOCTL_INTERNAL_BTH_SUBMIT_BRB: DWORD = BTH_KERNEL_CTL!(BTH_IOCTL_BASE + 0x00);
pub const IOCTL_INTERNAL_BTHENUM_GET_ENUMINFO: DWORD = BTH_KERNEL_CTL!(BTH_IOCTL_BASE + 0x01);
pub const IOCTL_INTERNAL_BTHENUM_GET_DEVINFO: DWORD = BTH_KERNEL_CTL!(BTH_IOCTL_BASE + 0x02);
pub const IOCTL_BTH_GET_LOCAL_INFO: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x00);
pub const IOCTL_BTH_GET_RADIO_INFO: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x01);
pub const IOCTL_BTH_GET_DEVICE_INFO: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x02);
pub const IOCTL_BTH_DISCONNECT_DEVICE: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x03);
pub const IOCTL_BTH_HCI_VENDOR_COMMAND: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x14);
pub const IOCTL_BTH_SDP_CONNECT: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x80);
pub const IOCTL_BTH_SDP_DISCONNECT: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x81);
pub const IOCTL_BTH_SDP_SERVICE_SEARCH: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x82);
pub const IOCTL_BTH_SDP_ATTRIBUTE_SEARCH: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x83);
pub const IOCTL_BTH_SDP_SERVICE_ATTRIBUTE_SEARCH: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x84);
pub const IOCTL_BTH_SDP_SUBMIT_RECORD: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x85);
pub const IOCTL_BTH_SDP_REMOVE_RECORD: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x86);
pub const IOCTL_BTH_SDP_SUBMIT_RECORD_WITH_INFO: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x87);
pub const IOCTL_BTH_GET_HOST_SUPPORTED_FEATURES: DWORD = BTH_CTL!(BTH_IOCTL_BASE + 0x88);
STRUCT! {#[repr(packed)] struct BTH_DEVICE_INFO_LIST {
    numOfDevices: ULONG,
    deviceList: [BTH_DEVICE_INFO; 1],
}}
pub type PBTH_DEVICE_INFO_LIST = *mut BTH_DEVICE_INFO_LIST;
STRUCT! {#[repr(packed)] struct BTH_RADIO_INFO {
    lmpSupportedFeatures: ULONGLONG,
    mfg: USHORT,
    lmpSubversion: USHORT,
    lmpVersion: UCHAR,
}}
pub type PBTH_RADIO_INFO = *mut BTH_RADIO_INFO;
STRUCT! {#[repr(packed)] struct BTH_LOCAL_RADIO_INFO {
    localInfo: BTH_DEVICE_INFO,
    flags: ULONG,
    hciRevision: USHORT,
    hciVersion: UCHAR,
    radioInfo: BTH_RADIO_INFO,
}}
pub type PBTH_LOCAL_RADIO_INFO = *mut BTH_LOCAL_RADIO_INFO;
pub const SDP_CONNECT_CACHE: ULONG = 0x00000001;
pub const SDP_CONNECT_ALLOW_PIN: ULONG = 0x00000002;
pub const SDP_REQUEST_TO_DEFAULT: UCHAR = 0;
pub const SDP_REQUEST_TO_MIN: UCHAR = 10;
pub const SDP_REQUEST_TO_MAX: UCHAR = 45;
pub const SERVICE_OPTION_DO_NOT_PUBLISH: ULONG = 0x00000002;
pub const SERVICE_OPTION_NO_PUBLIC_BROWSE: ULONG = 0x00000004;
pub const SERVICE_OPTION_DO_NOT_PUBLISH_EIR: ULONG = 0x00000008;
pub const SERVICE_SECURITY_USE_DEFAULTS: ULONG = 0x00000000;
pub const SERVICE_SECURITY_NONE: ULONG = 0x00000001;
pub const SERVICE_SECURITY_AUTHORIZE: ULONG = 0x00000002;
pub const SERVICE_SECURITY_AUTHENTICATE: ULONG = 0x00000004;
pub const SERVICE_SECURITY_ENCRYPT_REQUIRED: ULONG = 0x00000010;
pub const SERVICE_SECURITY_ENCRYPT_OPTIONAL: ULONG = 0x00000020;
pub const SERVICE_SECURITY_DISABLED: ULONG = 0x10000000;
pub const SERVICE_SECURITY_NO_ASK: ULONG = 0x20000000;
pub const SDP_SEARCH_NO_PARSE_CHECK: ULONG = 0x00000001;
pub const SDP_SEARCH_NO_FORMAT_CHECK: ULONG = 0x00000002;
pub type HANDLE_SDP = ULONGLONG;
pub type PHANDLE_SDP = *mut ULONGLONG;
pub type HANDLE_SDP_TYPE = HANDLE_SDP;
pub const HANDLE_SDP_NULL: HANDLE_SDP = 0x0;
pub const HANDLE_SDP_LOCAL: HANDLE_SDP = -2i64 as u64;
STRUCT! {#[repr(packed)] struct BTH_SDP_CONNECT {
    bthAddress: BTH_ADDR,
    fSdpConnect: ULONG,
    hConnection: HANDLE_SDP_TYPE,
    requestTimeout: UCHAR,
}}
pub type PBTH_SDP_CONNECT = *mut BTH_SDP_CONNECT;
STRUCT! {#[repr(packed)] struct BTH_SDP_DISCONNECT {
    hConnection: HANDLE_SDP_TYPE,
}}
pub type PBTH_SDP_DISCONNECT = *mut BTH_SDP_DISCONNECT;
STRUCT! {#[repr(packed)] struct BTH_SDP_RECORD {
    fSecurity: ULONG,
    fOptions: ULONG,
    fCodService: ULONG,
    recordLength: ULONG,
    record: [UCHAR; 1],
}}
pub type PBTH_SDP_RECORD = *mut BTH_SDP_RECORD;
STRUCT! {#[repr(packed)] struct BTH_SDP_SERVICE_SEARCH_REQUEST {
    hConnection: HANDLE_SDP_TYPE,
    uuids: [SdpQueryUuid; MAX_UUIDS_IN_QUERY],
}}
pub type PBTH_SDP_SERVICE_SEARCH_REQUEST = *mut BTH_SDP_SERVICE_SEARCH_REQUEST;
STRUCT! {#[repr(packed)] struct BTH_SDP_ATTRIBUTE_SEARCH_REQUEST {
    hConnection: HANDLE_SDP_TYPE,
    searchFlags: ULONG,
    recordHandle: ULONG,
    range: [SdpAttributeRange; 1],
}}
pub type PBTH_SDP_ATTRIBUTE_SEARCH_REQUEST = *mut BTH_SDP_ATTRIBUTE_SEARCH_REQUEST;
STRUCT! {#[repr(packed)] struct BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST {
    hConnection: HANDLE_SDP_TYPE,
    searchFlags: ULONG,
    uuids: [SdpQueryUuid; MAX_UUIDS_IN_QUERY],
    range: [SdpAttributeRange; 1],
}}
pub type PBTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST = *mut BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST;
STRUCT! {#[repr(packed)] struct BTH_SDP_STREAM_RESPONSE {
    requiredSize: ULONG,
    responseSize: ULONG,
    response: [UCHAR; 1],
}}
pub type PBTH_SDP_STREAM_RESPONSE = *mut BTH_SDP_STREAM_RESPONSE;
STRUCT! {#[repr(packed)] struct BTH_COMMAND_HEADER {
    OpCode: USHORT,
    TotalParameterLength: UCHAR,
}}
pub type PBTH_COMMAND_HEADER = *mut BTH_COMMAND_HEADER;
STRUCT! {#[repr(packed)] struct BTH_VENDOR_SPECIFIC_COMMAND {
    ManufacturerId: ULONG,
    LmpVersion: UCHAR,
    MatchAnySinglePattern: BOOLEAN,
    HciHeader: BTH_COMMAND_HEADER,
    Data: [UCHAR; 1],
}}
pub type PBTH_VENDOR_SPECIFIC_COMMAND = *mut BTH_VENDOR_SPECIFIC_COMMAND;
STRUCT! {#[repr(packed)] struct BTH_VENDOR_PATTERN {
    Offset: UCHAR,
    Size: UCHAR,
    Pattern: [UCHAR; 1],
}}
pub type PBTH_VENDOR_PATTERN = *mut BTH_VENDOR_PATTERN;
STRUCT! {#[repr(packed)] struct BTH_VENDOR_EVENT_INFO {
    BthAddress: BTH_ADDR,
    EventSize: ULONG,
    EventInfo: [UCHAR; 1],
}}
pub type PBTH_VENDOR_EVENT_INFO = *mut BTH_VENDOR_EVENT_INFO;
pub const BTH_HOST_FEATURE_ENHANCED_RETRANSMISSION_MODE: ULONGLONG = 0x0000000000000001;
pub const BTH_HOST_FEATURE_STREAMING_MODE: ULONGLONG = 0x0000000000000002;
pub const BTH_HOST_FEATURE_LOW_ENERGY: ULONGLONG = 0x0000000000000004;
pub const BTH_HOST_FEATURE_SCO_HCI: ULONGLONG = 0x0000000000000008;
pub const BTH_HOST_FEATURE_SCO_HCIBYPASS: ULONGLONG = 0x0000000000000010;
STRUCT! {#[repr(packed)] struct BTH_HOST_FEATURE_MASK {
    Mask: ULONGLONG,
    Reserved1: ULONGLONG,
    Reserved2: ULONGLONG,
}}
pub type PBTH_HOST_FEATURE_MASK = *mut BTH_HOST_FEATURE_MASK;
