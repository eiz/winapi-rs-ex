// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{ULONG64, ULONG_PTR};
use shared::guiddef::GUID;
use shared::minwindef::{UCHAR, ULONG};
use um::winnt::{HANDLE, LARGE_INTEGER, STANDARD_RIGHTS_READ, SYNCHRONIZE, WCHAR};
STRUCT! {struct WNODE_HEADER_u1_s {
    Version: ULONG,
    Linkage: ULONG,
}}
UNION! {union WNODE_HEADER_u1 {
    [u64; 1],
    HistoricalContext HistoricalContext_mut: ULONG64,
    s s_mut: WNODE_HEADER_u1_s,
}}
UNION! {union WNODE_HEADER_u2 {
    [u64; 1],
    CountLost CountLost_mut: ULONG,
    KernelHandle KernelHandle_mut: HANDLE,
    TimeStamp TimeStamp_mut: LARGE_INTEGER,
}}
STRUCT! {struct WNODE_HEADER {
    BufferSize: ULONG,
    ProviderId: ULONG,
    u1: WNODE_HEADER_u1,
    u2: WNODE_HEADER_u2,
    Guid: GUID,
    ClientContext: ULONG,
    Flags: ULONG,
}}
pub type PWNODE_HEADER = *mut WNODE_HEADER;
pub const WNODE_FLAG_ALL_DATA: ULONG = 0x00000001;
pub const WNODE_FLAG_SINGLE_INSTANCE: ULONG = 0x00000002;
pub const WNODE_FLAG_SINGLE_ITEM: ULONG = 0x00000004;
pub const WNODE_FLAG_EVENT_ITEM: ULONG = 0x00000008;
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: ULONG = 0x00000010;
pub const WNODE_FLAG_TOO_SMALL: ULONG = 0x00000020;
pub const WNODE_FLAG_INSTANCES_SAME: ULONG = 0x00000040;
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: ULONG = 0x00000080;
pub const WNODE_FLAG_INTERNAL: ULONG = 0x00000100;
pub const WNODE_FLAG_USE_TIMESTAMP: ULONG = 0x00000200;
pub const WNODE_FLAG_PERSIST_EVENT: ULONG = 0x00000400;
pub const WNODE_FLAG_EVENT_REFERENCE: ULONG = 0x00002000;
pub const WNODE_FLAG_ANSI_INSTANCENAMES: ULONG = 0x00004000;
pub const WNODE_FLAG_METHOD_ITEM: ULONG = 0x00008000;
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: ULONG = 0x00010000;
pub const WNODE_FLAG_TRACED_GUID: ULONG = 0x00020000;
pub const WNODE_FLAG_LOG_WNODE: ULONG = 0x00040000;
pub const WNODE_FLAG_USE_GUID_PTR: ULONG = 0x00080000;
pub const WNODE_FLAG_USE_MOF_PTR: ULONG = 0x00100000;
pub const WNODE_FLAG_NO_HEADER: ULONG = 0x00200000;
pub const WNODE_FLAG_SEND_DATA_BLOCK: ULONG = 0x00400000;
pub const WNODE_FLAG_VERSIONED_PROPERTIES: ULONG = 0x00800000;
pub const WNODE_FLAG_SEVERITY_MASK: ULONG = 0xff000000;
STRUCT! {struct OFFSETINSTANCEDATAANDLENGTH {
    OffsetInstanceData: ULONG,
    LengthInstanceData: ULONG,
}}
pub type POFFSETINSTANCEDATAANDLENGTH = *mut OFFSETINSTANCEDATAANDLENGTH;
UNION! {union WNODE_ALL_DATA_u {
    [u32; 2],
    FixedInstanceSize FixedInstanceSize_mut: ULONG,
    OffsetInstanceDataAndLength OffsetInstanceDataAndLength_mut:
        [OFFSETINSTANCEDATAANDLENGTH; 0],
}}
STRUCT! {struct WNODE_ALL_DATA {
    WnodeHeader: WNODE_HEADER,
    DataBlockOffset: ULONG,
    InstanceCount: ULONG,
    OffsetInstanceNameOffsets: ULONG,
    u: WNODE_ALL_DATA_u,
}}
pub type PWNODE_ALL_DATA = *mut WNODE_ALL_DATA;
STRUCT! {struct WNODE_SINGLE_INSTANCE {
    WnodeHeader: WNODE_HEADER,
    OffsetInstanceName: ULONG,
    InstanceIndex: ULONG,
    DataBlockOffset: ULONG,
    SizeDataBlock: ULONG,
    VariableData: [UCHAR; 0],
}}
pub type PWNODE_SINGLE_INSTANCE = *mut WNODE_SINGLE_INSTANCE;
STRUCT! {struct WNODE_SINGLE_ITEM {
    WnodeHeader: WNODE_HEADER,
    OffsetInstanceName: ULONG,
    InstanceIndex: ULONG,
    ItemId: ULONG,
    DataBlockOffset: ULONG,
    SizeDataItem: ULONG,
    VariableData: [UCHAR; 0],
}}
pub type PWNODE_SINGLE_ITEM = *mut WNODE_SINGLE_ITEM;
STRUCT! {struct WNODE_METHOD_ITEM {
    WnodeHeader: WNODE_HEADER,
    OffsetInstanceName: ULONG,
    InstanceIndex: ULONG,
    MethodId: ULONG,
    DataBlockOffset: ULONG,
    SizeDataBlock: ULONG,
    VariableData: [UCHAR; 0],
}}
pub type PWNODE_METHOD_ITEM = *mut WNODE_METHOD_ITEM;
STRUCT! {struct WNODE_EVENT_ITEM {
    WnodeHeader: WNODE_HEADER,
}}
pub type PWNODE_EVENT_ITEM = *mut WNODE_EVENT_ITEM;
UNION! {union WNODE_EVENT_REFERENCE_u {
    [u32; 1],
    TargetInstanceIndex TargetInstanceIndex_mut: ULONG,
    TargetInstanceName TargetInstanceName_mut: [WCHAR; 0],
}}
STRUCT! {struct WNODE_EVENT_REFERENCE {
    WnodeHeader: WNODE_HEADER,
    TargetGuid: GUID,
    TargetDataBlockSize: ULONG,
    u: WNODE_EVENT_REFERENCE_u,
}}
pub type PWNODE_EVENT_REFERENCE = *mut WNODE_EVENT_REFERENCE;
STRUCT! {struct WNODE_TOO_SMALL {
    WnodeHeader: WNODE_HEADER,
    SizeNeeded: ULONG,
}}
pub type PWNODE_TOO_SMALL = *mut WNODE_TOO_SMALL;
UNION! {union WMIREGGUIDW_u {
    [usize; 1],
    InstanceNameList InstanceNameList_mut: ULONG,
    BaseNameOffset BaseNameOffset_mut: ULONG,
    Pdo Pdo_mut: ULONG_PTR,
    InstanceInfo InstanceInfo_mut: ULONG_PTR,
}}
STRUCT! {struct WMIREGGUIDW {
    Guid: GUID,
    Flags: ULONG,
    InstanceCount: ULONG,
    u: WMIREGGUIDW_u,
}}
pub type PWMIREGGUIDW = *mut WMIREGGUIDW;
pub const WMIREG_FLAG_EXPENSIVE: ULONG = 0x00000001;
pub const WMIREG_FLAG_INSTANCE_LIST: ULONG = 0x00000004;
pub const WMIREG_FLAG_INSTANCE_BASENAME: ULONG = 0x00000008;
pub const WMIREG_FLAG_INSTANCE_PDO: ULONG = 0x00000020;
pub const WMIREG_FLAG_REMOVE_GUID: ULONG = 0x00010000;
pub const WMIREG_FLAG_RESERVED1: ULONG = 0x00020000;
pub const WMIREG_FLAG_RESERVED2: ULONG = 0x00040000;
pub const WMIREG_FLAG_TRACED_GUID: ULONG = 0x00080000;
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: ULONG = 0x00001000;
pub const WMIREG_FLAG_EVENT_ONLY_GUID: ULONG = 0x00000040;
STRUCT! {struct WMIREGINFOW {
    BufferSize: ULONG,
    NextWmiRegInfo: ULONG,
    RegistryPath: ULONG,
    MofResourceName: ULONG,
    GuidGount: ULONG,
    WmiRegGuid: [WMIREGGUIDW; 0],
}}
pub type PWMIREGINFOW = *mut WMIREGINFOW;
ENUM! {enum WMIDPREQUESTCODE {
    WMI_GET_ALL_DATA = 0,
    WMI_GET_SINGLE_INSTANCE = 1,
    WMI_SET_SINGLE_INSTANCE = 2,
    WMI_SET_SINGLE_ITEM = 3,
    WMI_ENABLE_EVENTS = 4,
    WMI_DISABLE_EVENTS = 5,
    WMI_ENABLE_COLLECTION = 6,
    WMI_DISABLE_COLLECTION = 7,
    WMI_REGINFO = 8,
    WMI_EXECUTE_METHOD = 9,
    WMI_CAPTURE_STATE = 10,
}}
pub const WMI_GUIDTYPE_TRACECONTROL: ULONG = 0;
pub const WMI_GUIDTYPE_TRACE: ULONG = 1;
pub const WMI_GUIDTYPE_DATA: ULONG = 2;
pub const WMI_GUIDTYPE_EVENT: ULONG = 3;
pub const WMIGUID_QUERY: ULONG = 0x0001;
pub const WMIGUID_SET: ULONG = 0x0002;
pub const WMIGUID_NOTIFICATION: ULONG = 0x0004;
pub const WMIGUID_READ_DESCRIPTION: ULONG = 0x0008;
pub const WMIGUID_EXECUTE: ULONG = 0x0010;
pub const TRACELOG_CREATE_REALTIME: ULONG = 0x0020;
pub const TRACELOG_CREATE_ONDISK: ULONG = 0x0040;
pub const TRACELOG_GUID_ENABLE: ULONG = 0x0080;
pub const TRACELOG_ACCESS_KERNEL_LOGGER: ULONG = 0x0100;
pub const TRACELOG_LOG_EVENT: ULONG = 0x0200;
pub const TRACELOG_CREATE_INPROC: ULONG = 0x0200;
pub const TRACELOG_ACCESS_REALTIME: ULONG = 0x0400;
pub const TRACELOG_REGISTER_GUIDS: ULONG = 0x0800;
pub const TRACELOG_JOIN_GROUP: ULONG = 0x1000;
pub const WMIGUID_ALL_ACCESS_WIN2K: ULONG = STANDARD_RIGHTS_READ
    | WMIGUID_QUERY
    | WMIGUID_SET
    | WMIGUID_NOTIFICATION
    | WMIGUID_READ_DESCRIPTION
    | WMIGUID_EXECUTE
    | TRACELOG_CREATE_REALTIME
    | TRACELOG_CREATE_ONDISK
    | TRACELOG_GUID_ENABLE
    | TRACELOG_ACCESS_KERNEL_LOGGER
    | TRACELOG_CREATE_INPROC
    | TRACELOG_ACCESS_REALTIME;
pub const WMIGUID_ALL_ACCESS_WINXP: ULONG =
    WMIGUID_ALL_ACCESS_WIN2K | SYNCHRONIZE | TRACELOG_REGISTER_GUIDS;
pub const WMIGUID_ALL_ACCESS_RS1: ULONG = WMIGUID_ALL_ACCESS_WINXP | TRACELOG_JOIN_GROUP;
pub const WMIGUID_ALL_ACCESS: ULONG = WMIGUID_ALL_ACCESS_RS1;
pub const WMI_GLOBAL_LOGGER_ID: ULONG = 0x0001;
