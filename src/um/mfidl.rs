ENUM! {enum MFSESSION_SETTOPOLOGY_FLAGS {
    MFSESSION_SETTOPOLOGY_IMMEDIATE = 0x1,
    MFSESSION_SETTOPOLOGY_NORESOLUTION = 0x2,
    MFSESSION_SETTOPOLOGY_CLEAR_CURRENT = 0x4,
}}
ENUM! {enum MFSESSION_GETFULLTOPOLOGY_FLAGS {
    MFSESSION_GETFULLTOPOLOGY_CURRENT = 0x1,
}}
ENUM! {enum MFMPSESSION_CREATION_FLAGS {
    MFPMPSESSION_UNPROTECTED_PROCESS = 0x1,
    MFPMPSESSION_IN_PROCESS = 0x2,
}}
pub type TOPOID = __int64;
DEFINE_GUID! {MF_WVC1_PROG_SINGLE_SLICE_CONTENT,
0x67EC2559, 0x0F2F, 0x4420, 0xA4, 0xDD, 0x2F, 0x8E, 0xE7, 0xA5, 0x73, 0x8B}
DEFINE_GUID! {MF_PROGRESSIVE_CODING_CONTENT,
0x8F020EEA, 0x1508, 0x471F, 0x9D, 0xA6, 0x50, 0x7D, 0x7C, 0xFA, 0x40, 0xDB}
DEFINE_GUID! {MF_NALU_LENGTH_SET,
0xA7911D53, 0x12A4, 0x4965, 0xAE, 0x70, 0x6E, 0xAD, 0xD6, 0xFF, 0x05, 0x51}
DEFINE_GUID! {MF_NALU_LENGTH_INFORMATION,
0x19124E7C, 0xAD4B, 0x465F, 0xBB, 0x18, 0x20, 0x18, 0x62, 0x87, 0xB6, 0xAF}
DEFINE_GUID! {MF_USER_DATA_PAYLOAD,
0xd1d4985d, 0xdc92, 0x457a, 0xb3, 0xa0, 0x65, 0x1a, 0x33, 0xa3, 0x10, 0x47}
DEFINE_GUID! {MF_MPEG4SINK_SPSPPS_PASSTHROUGH,
0x5601a134, 0x2005, 0x4ad2, 0xb3, 0x7d, 0x22, 0xa6, 0xc5, 0x54, 0xde, 0xb2}
DEFINE_GUID! {MF_MPEG4SINK_MOOV_BEFORE_MDAT,
0xf672e3ac, 0xe1e6, 0x4f10, 0xb5, 0xec, 0x5f, 0x3b, 0x30, 0x82, 0x88, 0x16}
DEFINE_GUID! {MF_MPEG4SINK_MINIMUM_PROPERTIES_SIZE,
0xdca1ed52, 0x450e, 0x4a22, 0x8c, 0x62, 0x4e, 0xd4, 0x52, 0xf7, 0xa1, 0x87}
DEFINE_GUID! {MF_MPEG4SINK_MIN_FRAGMENT_DURATION,
0xa30b570c, 0x8efd, 0x45e8, 0x94, 0xfe, 0x27, 0xc8, 0x4b, 0x5b, 0xdf, 0xf6}
DEFINE_GUID! {MF_MPEG4SINK_MAX_CODED_SEQUENCES_PER_FRAGMENT,
0xfc1b3bd6, 0x692d, 0x4ce5, 0x92, 0x99, 0x73, 0x8a, 0xa5, 0x46, 0x3e, 0x9a}
RIDL! {#[uuid(0x90377834, 0x21d0, 0x4dee, 0x82, 0x14, 0xba, 0x2e, 0x3e, 0x6c, 0x11, 0x27)]
interface IMFMediaSession(IMFMediaSessionVtbl)
    : IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn SetTopology(
        dwSetTopologyFlags: DWORD,
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
    fn ClearTopologies() -> HRESULT,
    fn Start(
        pguidTimeFormat: *mut GUID,
        pvarStartPosition: *const PROPVARIANT,
    ) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Stop() -> HRESULT,
    fn Close() -> HRESULT,
    fn Shutdown() -> HRESULT,
    fn GetClock(
        ppClock: *mut *mut IMFClock,
    ) -> HRESULT,
    fn GetSessionCapabilities(
        pdwCaps: *mut DWORD,
    ) -> HRESULT,
    fn GetFullTopology(
        dwGetFullTopologyFlags: DWORD,
        TopoId: TOPOID,
        ppFullTopology: *mut *mut IMFTopology,
    ) -> HRESULT,
}}
DEFINE_GUID! {MF_SESSION_TOPOLOADER,
0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x71}
DEFINE_GUID! {MF_SESSION_GLOBAL_TIME,
0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x72}
DEFINE_GUID! {MF_SESSION_QUALITY_MANAGER,
0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x73}
DEFINE_GUID! {MF_SESSION_CONTENT_PROTECTION_MANAGER,
0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x74}
DEFINE_GUID! {MF_SESSION_SERVER_CONTEXT,
0xafe5b291, 0x50fa, 0x46e8, 0xb9, 0xbe, 0xc, 0xc, 0x3c, 0xe4, 0xb3, 0xa5}
DEFINE_GUID! {MF_SESSION_REMOTE_SOURCE_MODE,
0xf4033ef4, 0x9bb3, 0x4378, 0x94, 0x1f, 0x85, 0xa0, 0x85, 0x6b, 0xc2, 0x44}
DEFINE_GUID! {MF_SESSION_APPROX_EVENT_OCCURRENCE_TIME,
0x190e852f, 0x6238, 0x42d1, 0xb5, 0xaf, 0x69, 0xea, 0x33, 0x8e, 0xf8, 0x50}
DEFINE_GUID! {MF_PMP_SERVER_CONTEXT,
0x2f00c910, 0xd2cf, 0x4278, 0x8b, 0x6a, 0xd0, 0x77, 0xfa, 0xc3, 0xa2, 0x5f}
extern "system" {
    pub fn MFCreateMediaSession(
        pConfiguration: *mut IMFAttributes,
        ppMediaSession: *mut *mut IMFMediaSession,
    ) -> HRESULT;
    pub fn MFCreatePMPMediaSession(
        dwCreationFlags: DWORD,
        pConfiguration: *mut IMFAttributes,
        ppMediaSession: *mut *mut IMFMediaSession,
        ppEnablerActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
}
ENUM! {enum MF_OBJECT_TYPE {
    MF_OBJECT_MEDIASOURCE = 0,
    MF_OBJECT_BYTESTREAM = MF_OBJECT_MEDIASOURCE + 1,
    MF_OBJECT_INVALID = MF_OBJECT_BYTESTREAM + 1,
}}
ENUM! {enum __MIDL___MIDL_itf_mfidl_0000_0001_0001 {
    MF_RESOLUTION_MEDIASOURCE = 0x1,
    MF_RESOLUTION_BYTESTREAM = 0x2,
    MF_RESOLUTION_CONTENT_DOES_NOT_HAVE_TO_MATCH_EXTENSION_OR_MIME_TYPE = 0x10,
    MF_RESOLUTION_KEEP_BYTE_STREAM_ALIVE_ON_FAIL = 0x20,
    MF_RESOLUTION_DISABLE_LOCAL_PLUGINS = 0x40,
    MF_RESOLUTION_PLUGIN_CONTROL_POLICY_APPROVED_ONLY = 0x80,
    MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY = 0x100,
    MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY_EDGEMODE = 0x200,
    MF_RESOLUTION_ENABLE_STORE_PLUGINS = 0x400,
    MF_RESOLUTION_READ = 0x10000,
    MF_RESOLUTION_WRITE = 0x20000,
}}
ENUM! {enum MF_CONNECT_METHOD {
    MF_CONNECT_DIRECT = 0,
    MF_CONNECT_ALLOW_CONVERTER = 0x1,
    MF_CONNECT_ALLOW_DECODER = 0x3,
    MF_CONNECT_RESOLVE_INDEPENDENT_OUTPUTTYPES = 0x4,
    MF_CONNECT_AS_OPTIONAL = 0x10000,
    MF_CONNECT_AS_OPTIONAL_BRANCH = 0x20000,
}}
ENUM! {enum MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS {
    MF_TOPOLOGY_RESOLUTION_SUCCEEDED = 0,
    MF_OPTIONAL_NODE_REJECTED_MEDIA_TYPE = 0x1,
    MF_OPTIONAL_NODE_REJECTED_PROTECTED_PROCESS = 0x2,
}}
RIDL! {#[uuid(0xfbe5a32d, 0xa497, 0x4b61, 0xbb, 0x85, 0x97, 0xb1, 0xa8, 0x48, 0xa6, 0xe3)]
interface IMFSourceResolver(IMFSourceResolverVtbl): IUnknown(IUnknownVtbl) {
    fn CreateObjectFromURL(
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CreateObjectFromByteStream(
        pByteStream: *mut IMFByteStream,
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn BeginCreateObjectFromURL(
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        ppIUnknownCancelCookie: *mut *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndCreateObjectFromURL(
        pResult: *mut IMFAsyncResult,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn BeginCreateObjectFromByteStream(
        pByteStream: *mut IMFByteStream,
        pwszURL: LPCWSTR,
        dwFlags: DWORD,
        pProps: *mut IPropertyStore,
        ppIUnknownCancelCookie: *mut *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndCreateObjectFromByteStream(
        pResult: *mut IMFAsyncResult,
        pObjectType: *mut MF_OBJECT_TYPE,
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CancelObjectCreation(
        pIUnknownCancelCookie: *mut IUnknown,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSourceResolver(ppISourceResolver: *mut *mut IMFSourceResolver) -> HRESULT;
    pub fn CreatePropertyStore(ppStore: *mut *mut IPropertyStore) -> HRESULT;
    pub fn MFGetSupportedSchemes(pPropVarSchemeArray: *mut PROPVARIANT) -> HRESULT;
    pub fn MFGetSupportedMimeTypes(pPropVarMimeTypeArray: *mut PROPVARIANT) -> HRESULT;
}
DEFINE_PROPERTYKEY! {MFPKEY_SourceOpenMonitor,
0x074d4637, 0xb5ae, 0x465d, 0xaf, 0x17, 0x1a, 0x53, 0x8d, 0x28, 0x59, 0xdd, 0x02}
DEFINE_PROPERTYKEY! {MFPKEY_ASFMediaSource_ApproxSeek,
0xb4cd270f, 0x244d, 0x4969, 0xbb, 0x92, 0x3f, 0x0f, 0xb8, 0x31, 0x6f, 0x10, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_ASFMediaSource_IterativeSeekIfNoIndex,
0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_ASFMediaSource_IterativeSeek_Max_Count,
0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c, 0x02}
DEFINE_PROPERTYKEY! {MFPKEY_ASFMediaSource_IterativeSeek_Tolerance_In_MilliSecond,
0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c, 0x03}
DEFINE_PROPERTYKEY! {MFPKEY_Content_DLNA_Profile_ID,
0xcfa31b45, 0x525d, 0x4998, 0xbb, 0x44, 0x3f, 0x7d, 0x81, 0x54, 0x2f, 0xa4, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_MediaSource_DisableReadAhead,
0x26366c14, 0xc5bf, 0x4c76, 0x88, 0x7b, 0x9f, 0x17, 0x54, 0xdb, 0x5f, 0x9, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_SBESourceMode,
0x3fae10bb, 0xf859, 0x4192, 0xb5, 0x62, 0x18, 0x68, 0xd3, 0xda, 0x3a, 0x02, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_PMP_Creation_Callback,
0x28bb4de2, 0x26a2, 0x4870, 0xb7, 0x20, 0xd2, 0x6b, 0xbe, 0xb1, 0x49, 0x42, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Enable_Urlmon,
0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Urlmon_Bind_Flags,
0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 0x02}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Urlmon_Security_Id,
0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 0x03}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Urlmon_Window,
0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 0x04}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Urlmon_Callback_QueryService,
0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92, 0x05}
DEFINE_PROPERTYKEY! {MFPKEY_MediaProtectionSystemId,
0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_MediaProtectionSystemContext,
0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd, 0x02}
DEFINE_PROPERTYKEY! {MFPKEY_MediaProtectionSystemIdMapping,
0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd, 0x03}
DEFINE_PROPERTYKEY! {MFPKEY_MediaProtectionContainerGuid,
0x42af3d7c, 0xcf, 0x4a0f, 0x81, 0xf0, 0xad, 0xf5, 0x24, 0xa5, 0xa5, 0xb5, 0x1}
DEFINE_PROPERTYKEY! {MFPKEY_MediaProtectionSystemContextsPerTrack,
0x4454b092, 0xd3da, 0x49b0, 0x84, 0x52, 0x68, 0x50, 0xc7, 0xdb, 0x76, 0x4d, 0x03}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Download_Mode,
0x817f11b7, 0xa982, 0x46ec, 0xa4, 0x49, 0xef, 0x58, 0xae, 0xd5, 0x3c, 0xa8, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Caching_Mode,
0x86a2403e, 0xc78b, 0x44d7, 0x8b, 0xc8, 0xff, 0x72, 0x58, 0x11, 0x75, 0x08, 0x01}
DEFINE_PROPERTYKEY! {MFPKEY_HTTP_ByteStream_Cache_Limit,
0x86a2403e, 0xc78b, 0x44d7, 0x8b, 0xc8, 0xff, 0x72, 0x58, 0x11, 0x75, 0x08, 0x02}
ENUM! {enum MFMEDIASOURCE_CHARACTERISTICS{
    MFMEDIASOURCE_IS_LIVE = 0x1,
    MFMEDIASOURCE_CAN_SEEK = 0x2,
    MFMEDIASOURCE_CAN_PAUSE = 0x4,
    MFMEDIASOURCE_HAS_SLOW_SEEK = 0x8,
    MFMEDIASOURCE_HAS_MULTIPLE_PRESENTATIONS = 0x10,
    MFMEDIASOURCE_CAN_SKIPFORWARD = 0x20,
    MFMEDIASOURCE_CAN_SKIPBACKWARD = 0x40,
    MFMEDIASOURCE_DOES_NOT_USE_NETWORK = 0x80,
}}
RIDL! {#[uuid(0x279a808d, 0xaec7, 0x40c8, 0x9c, 0x6b, 0xa6, 0xb4, 0x92, 0xc7, 0x8a, 0x66)]
interface IMFMediaSource(IMFMediaSourceVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn CreatePresentationDescriptor(
        ppPresentationDescriptor: *mut IMFPresentationDescriptor,
    ) -> HRESULT,
    fn Start(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
        pguidTimeFormat: *const GUID,
        pvarStartPosition: *const PROPVARIANT,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Pause() -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
RIDL! {#[uuid(0x3c9b2eb9, 0x86d5, 0x4514, 0xa3, 0x94, 0xf5, 0x66, 0x64, 0xf9, 0xf0, 0xd8)]
interface IMFMediaSourceEx(IMFMediaSourceExVtbl): IMFMediaSource(IMFMediaSourceVtbl) {
    fn GetSourceAttributes(
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn GetStreamAttributes(
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
    fn SetD3DManager(
        pManager: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x6ef2a662, 0x47c0, 0x4666, 0xb1, 0x3d, 0xcb, 0xb7, 0x17, 0xf2, 0xfa, 0x2c)]
interface IMFClockConsumer(IMFClockConsumerVtbl): IUnknown(IUnknownVtbl) {
    fn SetPresentationClock(
        pPresentationClock: *mut IMFPresentationClock,
    ) -> HRESULT,
    fn GetPresentationClock(
        ppPresentationClock: *mut *mut IMFPresentationClock,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xd182108f, 0x4ec6, 0x443f, 0xaa, 0x42, 0xa7, 0x11, 0x06, 0xec, 0x82, 0x5f)]
interface IMFMediaStream(IMFMediaStreamVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetMediaSource(
        ppMediaSource: *mut *mut IMFMediaSource,
    ) -> HRESULT,
    fn GetStreamDescriptor(
        ppStreamDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT,
    fn RequestSample(
        pToken: *mut IUnknown,
    ) -> HRESULT,
}}
pub const MEDIASINK_FIXED_STREAMS: DWORD = 0x00000001;
pub const MEDIASINK_CANNOT_MATCH_CLOCK: DWORD = 0x00000002;
pub const MEDIASINK_RATELESS: DWORD = 0x00000004;
pub const MEDIASINK_CLOCK_REQUIRED: DWORD = 0x00000008;
pub const MEDIASINK_CAN_PREROLL: DWORD = 0x00000010;
pub const MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE: DWORD = 0x00000020;
ENUM! {enum MF_TRANSFER_VIDEO_FRAME_FLAGS {
    MF_TRANSFER_VIDEO_FRAME_DEFAULT = 0,
    MF_TRANSFER_VIDEO_FRAME_STRETCH = 1,
    MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR = 2,
}}
DEFINE_GUID! {MF_SINK_VIDEO_PTS,
0x2162bde7, 0x421e, 0x4b90, 0x9b, 0x33, 0xe5, 0x8f, 0xbf, 0x1d, 0x58, 0xb6}
DEFINE_GUID! {MF_SINK_VIDEO_NATIVE_WIDTH,
0xe6d6a707, 0x1505, 0x4747, 0x9b, 0x10, 0x72, 0xd2, 0xd1, 0x58, 0xcb, 0x3a}
DEFINE_GUID! {MF_SINK_VIDEO_NATIVE_HEIGHT,
0xf0ca6705, 0x490c, 0x43e8, 0x94, 0x1c, 0xc0, 0xb3, 0x20, 0x6b, 0x9a, 0x65}
DEFINE_GUID! {MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_NUMERATOR,
0xd0f33b22, 0xb78a, 0x4879, 0xb4, 0x55, 0xf0, 0x3e, 0xf3, 0xfa, 0x82, 0xcd}
DEFINE_GUID! {MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_DENOMINATOR,
0x6ea1eb97, 0x1fe0, 0x4f10, 0xa6, 0xe4, 0x1f, 0x4f, 0x66, 0x15, 0x64, 0xe0}
DEFINE_GUID! {MF_BD_MVC_PLANE_OFFSET_METADATA,
0x62a654e4, 0xb76c, 0x4901, 0x98, 0x23, 0x2c, 0xb6, 0x15, 0xd4, 0x73, 0x18}
DEFINE_GUID! {MF_LUMA_KEY_ENABLE,
0x7369820f, 0x76de, 0x43ca, 0x92, 0x84, 0x47, 0xb8, 0xf3, 0x7e, 0x06, 0x49}
DEFINE_GUID! {MF_LUMA_KEY_LOWER,
0x93d7b8d5, 0x0b81, 0x4715, 0xae, 0xa0, 0x87, 0x25, 0x87, 0x16, 0x21, 0xe9}
DEFINE_GUID! {MF_LUMA_KEY_UPPER,
0xd09f39bb, 0x4602, 0x4c31, 0xa7, 0x06, 0xa1, 0x21, 0x71, 0xa5, 0x11, 0x0a}
DEFINE_GUID! {MF_USER_EXTENDED_ATTRIBUTES,
0xc02abac6, 0xfeb2, 0x4541, 0x92, 0x2f, 0x92, 0x0b, 0x43, 0x70, 0x27, 0x22}
DEFINE_GUID! {MF_INDEPENDENT_STILL_IMAGE,
0xea12af41, 0x0710, 0x42c9, 0xa1, 0x27, 0xda, 0xa3, 0xe7, 0x84, 0x83, 0xa5}
RIDL! {#[uuid(0x6ef2a660, 0x47c0, 0x4666, 0xb1, 0x3d, 0xcb, 0xb7, 0x17, 0xf2, 0xfa, 0x2c)]
interface IMFMediaSink(IMFMediaSinkVtbl): IUnknown(IUnknownVtbl) {
    fn GetCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn AddStreamSink(
        dwStreamSinkIdentifier: DWORD,
        pMediaType: *mut IMFMediaType,
        ppStreamSink: *mut *mut IMFStreamSink,
    ) -> HRESULT,
    fn RemoveStreamSink(
        dwStreamSinkIdentifier: DWORD,
    ) -> HRESULT,
    fn GetStreamSinkCount(
        pcStreamSinkCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamSinkByIndex(
        dwIndex: DWORD,
        ppStreamSink: *mut *mut IMFStreamSink,
    ) -> HRESULT,
    fn GetStreamSinkById(
        dwStreamSinkIdentifier: DWORD,
        ppStreamSink: *mut *mut IMFStreamSink,
    ) -> HRESULT,
    fn SetPresentationClock(
        pPresentationClock: *mut IMFPresentationClock,
    ) -> HRESULT,
    fn GetPresentationClock(
        ppPresentationClock: *mut *mut IMFPresentationClock,
    ) -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
