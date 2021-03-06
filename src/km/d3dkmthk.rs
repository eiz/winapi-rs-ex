// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! longhorn display driver model kernel mode thunk interfaces
use shared::basetsd::{UINT32, UINT64};
use shared::d3dukmdt::{
    D3DDDICB_SIGNALFLAGS, D3DDDI_ALLOCATIONLIST, D3DDDI_CREATECONTEXTFLAGS,
    D3DDDI_MAX_BROADCAST_CONTEXT, D3DDDI_MAX_OBJECT_SIGNALED, D3DDDI_MAX_OBJECT_WAITED_ON,
    D3DDDI_PATCHLOCATIONLIST, D3DDDI_SYNCHRONIZATIONOBJECTINFO, D3DDDI_SYNCHRONIZATIONOBJECTINFO2,
    D3DDDI_VIDEO_PRESENT_SOURCE_ID, D3DGPU_VIRTUAL_ADDRESS, D3DKMT_HANDLE,
};
use shared::minwindef::{BOOL, UCHAR, UINT, ULONG};
use shared::ntdef::{HANDLE, LUID, PCWSTR, ULONGLONG, VOID, WCHAR};
use shared::windef::HDC;
STRUCT! {struct D3DKMT_CREATEDEVICEFLAGS {
    bitfield: UINT,
}}
BITFIELD! {D3DKMT_CREATEDEVICEFLAGS bitfield: UINT [
    LegacyMode set_LegacyMode[0..1],
    RequestVSync set_RequestVSync[1..2],
    DisableGpuTimeout set_DisableGpuTimeout[2..3],
    Reserved set_Reserved[3..32],
]}
UNION! {union D3DKMT_CREATEDEVICE_u {
    [usize; 1],
    hAdapter hAdapter_mut: D3DKMT_HANDLE,
    pAdapter pAdapter_mut: *mut VOID,
}}
STRUCT! {struct D3DKMT_CREATEDEVICE {
    u: D3DKMT_CREATEDEVICE_u,
    Flags: D3DKMT_CREATEDEVICEFLAGS,
    hDevice: D3DKMT_HANDLE,
    pCommandBuffer: *mut VOID,
    CommandBufferSize: UINT,
    pAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    AllocationListSize: UINT,
    pPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    PatchLocationListSize: UINT,
}}
STRUCT! {struct D3DKMT_DESTROYDEVICE {
    hDevice: D3DKMT_HANDLE,
}}
ENUM! {enum D3DKMT_CLIENTHINT {
    D3DKMT_CLIENTHINT_UNKNOWN = 0,
    D3DKMT_CLIENTHINT_OPENGL = 1,
    D3DKMT_CLIENTHINT_CDD = 2,
    D3DKMT_CLIENTHINT_DX7 = 7,
    D3DKMT_CLIENTHINT_DX8 = 8,
    D3DKMT_CLIENTHINT_DX9 = 9,
    D3DKMT_CLIENTHINT_DX10 = 10,
}}
STRUCT! {struct D3DKMT_CREATECONTEXT {
    hDevice: D3DKMT_HANDLE,
    NodeOrdinal: UINT,
    EngineAffinity: UINT,
    Flags: D3DDDI_CREATECONTEXTFLAGS,
    pPrivateDriverData: *mut VOID,
    PrivateDriverDataSize: UINT,
    ClientHint: D3DKMT_CLIENTHINT,
    hContext: D3DKMT_HANDLE,
    pCommandBuffer: *mut VOID,
    CommandBufferSize: UINT,
    pAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    AllocationListSize: UINT,
    pPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    PatchLocationListSize: UINT,
    CommandBuffer: D3DGPU_VIRTUAL_ADDRESS,
}}
STRUCT! {struct D3DKMT_DESTROYCONTEXT {
    hContext: D3DKMT_HANDLE,
}}
STRUCT! {struct D3DKMT_CREATESYNCHRONIZATIONOBJECT {
    hDevice: D3DKMT_HANDLE,
    Info: D3DDDI_SYNCHRONIZATIONOBJECTINFO,
    hSyncObject: D3DKMT_HANDLE,
}}
STRUCT! {struct D3DKMT_CREATESYNCHRONIZATIONOBJECT2 {
    hDevice: D3DKMT_HANDLE,
    Info: D3DDDI_SYNCHRONIZATIONOBJECTINFO2,
    hSyncObject: D3DKMT_HANDLE,
}}
STRUCT! {struct D3DKMT_DESTROYSYNCHRONIZATIONOBJECT {
    hSyncObject: D3DKMT_HANDLE,
}}
STRUCT! {struct D3DKMT_OPENSYNCHRONIZATIONOBJECT {
    hSharedHandle: D3DKMT_HANDLE,
    hSyncObject: D3DKMT_HANDLE,
    Reserved: [UINT64; 8],
}}
STRUCT! {struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_WAITED_ON],
}}
STRUCT! {struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_Fence {
    FenceValue: UINT64,
}}
UNION! {union D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_u {
    [u64; 8],
    Fence Fence_mut: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_Fence,
    Reserved Reserved_mut: [UINT64; 8],
}}
STRUCT! {struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_WAITED_ON],
    u: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_u,
}}
STRUCT! {struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_SIGNALED],
    Flags: D3DDDICB_SIGNALFLAGS,
}}
STRUCT! {struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_Fence {
    FenceValue: UINT64,
}}
UNION! {union D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_u {
    [u64; 8],
    Fence Fence_mut: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_Fence,
    CpuEventHandle CpuEventHandle_mut: HANDLE,
    Reserved Reserved_mut: [UINT64; 8],
}}
STRUCT! {struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_SIGNALED],
    Flags: D3DDDICB_SIGNALFLAGS,
    BroadcastContextCount: ULONG,
    BroadcastContext: [D3DKMT_HANDLE; D3DDDI_MAX_BROADCAST_CONTEXT],
    u: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_u,
}}
//1512
STRUCT! {struct D3DKMT_SEGMENTSIZEINFO {
    DedicatedVideoMemorySize: ULONGLONG,
    DedicatedSystemMemorySize: ULONGLONG,
    SharedSystemMemorySize: ULONGLONG,
}}
//1617
STRUCT! {struct D3DKMT_ADAPTERTYPE {
    Value: UINT,
}}
BITFIELD! {D3DKMT_ADAPTERTYPE Value: UINT [
    RenderSupported set_RenderSupported[0..1],
    DisplaySupported set_DisplaySupported[1..2],
    SoftwareDevice set_SoftwareDevice[2..3],
    PostDevice set_PostDevice[3..4],
    HybridDiscrete set_HybridDiscrete[4..5],
    HybridIntegrated set_HybridIntegrated[5..6],
    IndirectDisplayDevice set_IndirectDisplayDevice[6..7],
    Paravirtualized set_Paravirtualized[7..8],
    ACGSupported set_ACGSupported[8..9],
    SupportSetTimingsFromVidPn set_SupportSetTimingsFromVidPn[9..10],
    Detachable set_Detachable[10..11],
    Reserved set_Reserved[11..32],
]}
//1852
STRUCT! {struct D3DKMT_NODE_PERFDATA {
    NodeOrdinal: UINT32,
    PhysicalAdapterIndex: UINT32,
    Frequency: ULONGLONG,
    MaxFrequency: ULONGLONG,
    MaxFrequencyOC: ULONGLONG,
    Voltage: ULONG,
    VoltageMax: ULONG,
    VoltageMaxOC: ULONG,
    MaxTransitionLatency: ULONGLONG,
}}
STRUCT! {struct D3DKMT_ADAPTER_PERFDATA {
    PhysicalAdapterIndex: UINT32,
    MemoryFrequency: ULONGLONG,
    MaxMemoryFrequency: ULONGLONG,
    MaxMemoryFrequencyOC: ULONGLONG,
    MemoryBandwidth: ULONGLONG,
    PCIEBandwidth: ULONGLONG,
    FanRPM: ULONG,
    Power: ULONG,
    Temperature: ULONG,
    PowerStateOverride: UCHAR,
}}
STRUCT! {struct D3DKMT_ADAPTER_PERFDATACAPS {
    PhysicalAdapterIndex: UINT32,
    MaxMemoryBandwidth: ULONGLONG,
    MaxPCIEBandwidth: ULONGLONG,
    MaxFanRPM: ULONG,
    TemperatureMax: ULONG,
    TemperatureWarning: ULONG,
}}
pub const DXGK_MAX_GPUVERSION_NAME_LENGTH: usize = 32;
STRUCT! {struct D3DKMT_GPUVERSION {
    PhysicalAdapterIndex: UINT32,
    BiosVersion: [WCHAR; DXGK_MAX_GPUVERSION_NAME_LENGTH],
    GpuArchitecture: [WCHAR; DXGK_MAX_GPUVERSION_NAME_LENGTH],
}}
ENUM! {enum KMTQUERYADAPTERINFOTYPE {
    KMTQAITYPE_UMDRIVERPRIVATE = 0,
    KMTQAITYPE_UMDRIVERNAME = 1,
    KMTQAITYPE_UMOPENGLINFO = 2,
    KMTQAITYPE_GETSEGMENTSIZE = 3,
    KMTQAITYPE_ADAPTERGUID = 4,
    KMTQAITYPE_FLIPQUEUEINFO = 5,
    KMTQAITYPE_ADAPTERADDRESS = 6,
    KMTQAITYPE_SETWORKINGSETINFO = 7,
    KMTQAITYPE_ADAPTERREGISTRYINFO = 8,
    KMTQAITYPE_CURRENTDISPLAYMODE = 9,
    KMTQAITYPE_MODELIST = 10,
    KMTQAITYPE_CHECKDRIVERUPDATESTATUS = 11,
    KMTQAITYPE_VIRTUALADDRESSINFO = 12,
    KMTQAITYPE_DRIVERVERSION = 13,
    KMTQAITYPE_ADAPTERTYPE = 15,
    KMTQAITYPE_OUTPUTDUPLCONTEXTSCOUNT = 16,
    KMTQAITYPE_WDDM_1_2_CAPS = 17,
    KMTQAITYPE_UMD_DRIVER_VERSION = 18,
    KMTQAITYPE_DIRECTFLIP_SUPPORT = 19,
    KMTQAITYPE_MULTIPLANEOVERLAY_SUPPORT = 20,
    KMTQAITYPE_DLIST_DRIVER_NAME = 21,
    KMTQAITYPE_WDDM_1_3_CAPS = 22,
    KMTQAITYPE_MULTIPLANEOVERLAY_HUD_SUPPORT = 23,
    KMTQAITYPE_WDDM_2_0_CAPS = 24,
    KMTQAITYPE_NODEMETADATA = 25,
    KMTQAITYPE_CPDRIVERNAME = 26,
    KMTQAITYPE_XBOX = 27,
    KMTQAITYPE_INDEPENDENTFLIP_SUPPORT = 28,
    KMTQAITYPE_MIRACASTCOMPANIONDRIVERNAME = 29,
    KMTQAITYPE_PHYSICALADAPTERCOUNT = 30,
    KMTQAITYPE_PHYSICALADAPTERDEVICEIDS = 31,
    KMTQAITYPE_DRIVERCAPS_EXT = 32,
    KMTQAITYPE_QUERY_MIRACAST_DRIVER_TYPE = 33,
    KMTQAITYPE_QUERY_GPUMMU_CAPS = 34,
    KMTQAITYPE_QUERY_MULTIPLANEOVERLAY_DECODE_SUPPORT = 35,
    KMTQAITYPE_QUERY_HW_PROTECTION_TEARDOWN_COUNT = 36,
    KMTQAITYPE_QUERY_ISBADDRIVERFORHWPROTECTIONDISABLED = 37,
    KMTQAITYPE_MULTIPLANEOVERLAY_SECONDARY_SUPPORT = 38,
    KMTQAITYPE_INDEPENDENTFLIP_SECONDARY_SUPPORT = 39,
    KMTQAITYPE_PANELFITTER_SUPPORT = 40,
    KMTQAITYPE_PHYSICALADAPTERPNPKEY = 41,
    KMTQAITYPE_GETSEGMENTGROUPSIZE = 42,
    KMTQAITYPE_MPO3DDI_SUPPORT = 43,
    KMTQAITYPE_HWDRM_SUPPORT = 44,
    KMTQAITYPE_MPOKERNELCAPS_SUPPORT = 45,
    KMTQAITYPE_MULTIPLANEOVERLAY_STRETCH_SUPPORT = 46,
    KMTQAITYPE_GET_DEVICE_VIDPN_OWNERSHIP_INFO = 47,
    KMTQAITYPE_QUERYREGISTRY = 48,
    KMTQAITYPE_KMD_DRIVER_VERSION = 49,
    KMTQAITYPE_BLOCKLIST_KERNEL = 50,
    KMTQAITYPE_BLOCKLIST_RUNTIME = 51,
    KMTQAITYPE_ADAPTERGUID_RENDER = 52,
    KMTQAITYPE_ADAPTERADDRESS_RENDER = 53,
    KMTQAITYPE_ADAPTERREGISTRYINFO_RENDER = 54,
    KMTQAITYPE_CHECKDRIVERUPDATESTATUS_RENDER = 55,
    KMTQAITYPE_DRIVERVERSION_RENDER = 56,
    KMTQAITYPE_ADAPTERTYPE_RENDER = 57,
    KMTQAITYPE_WDDM_1_2_CAPS_RENDER = 58,
    KMTQAITYPE_WDDM_1_3_CAPS_RENDER = 59,
    KMTQAITYPE_QUERY_ADAPTER_UNIQUE_GUID = 60,
    KMTQAITYPE_NODEPERFDATA = 61,
    KMTQAITYPE_ADAPTERPERFDATA = 62,
    KMTQAITYPE_ADAPTERPERFDATA_CAPS = 63,
    KMTQUITYPE_GPUVERSION = 64,
}}
STRUCT! {struct D3DKMT_QUERYADAPTERINFO {
    hAdapter: D3DKMT_HANDLE,
    Type: KMTQUERYADAPTERINFOTYPE,
    pPrivateDriverData: *mut VOID,
    PrivateDriverDataSize: UINT,
}}
STRUCT! {struct D3DKMT_OPENADAPTERFROMHDC {
    hDc: HDC,
    hAdapter: D3DKMT_HANDLE,
    AdapterLuid: LUID,
    VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}}
STRUCT! {struct D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME {
    DeviceName: [WCHAR; 32],
    hAdapter: D3DKMT_HANDLE,
    AdapterLuid: LUID,
    VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}}
STRUCT! {struct D3DKMT_OPENADAPTERFROMDEVICENAME {
    pDeviceName: PCWSTR,
    hAdapter: D3DKMT_HANDLE,
    AdapterLuid: LUID,
}}
pub const MAX_ENUM_ADAPTERS: usize = 16;
STRUCT! {struct D3DKMT_ADAPTERINFO {
    hAdapter: D3DKMT_HANDLE,
    AdapterLuid: LUID,
    NumOfSources: ULONG,
    bPresentMoveRegionsPreferred: BOOL,
}}
STRUCT! {struct D3DKMT_ENUMADAPTERS {
    NumAdapters: ULONG,
    Adapters: [D3DKMT_ADAPTERINFO; MAX_ENUM_ADAPTERS],
}}
STRUCT! {struct D3DKMT_ENUMADAPTERS2 {
    NumAdapters: ULONG,
    pAdapters: *mut D3DKMT_ADAPTERINFO,
}}
STRUCT! {struct D3DKMT_OPENADAPTERFROMLUID {
    AdapterLuid: LUID,
    hAdapter: D3DKMT_HANDLE,
}}
STRUCT! {struct D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME {
    DeviceName: [WCHAR; 32],
    VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}}
STRUCT! {struct D3DKMT_CLOSEADAPTER {
    hAdapter: D3DKMT_HANDLE,
}}
