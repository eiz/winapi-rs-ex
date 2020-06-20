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
ENUM! {enum MFSTREAMSINK_MARKER_TYPE {
    MFSTREAMSINK_MARKER_DEFAULT = 0,
    MFSTREAMSINK_MARKER_ENDOFSEGMENT = MFSTREAMSINK_MARKER_DEFAULT + 1,
    MFSTREAMSINK_MARKER_TICK = MFSTREAMSINK_MARKER_ENDOFSEGMENT + 1,
    MFSTREAMSINK_MARKER_EVENT = MFSTREAMSINK_MARKER_TICK + 1,
}}
RIDL! {#[uuid(0x0a97b3cf, 0x8e7c, 0x4a3d, 0x8f, 0x8c, 0x0c, 0x84, 0x3d, 0xc2, 0x47, 0xfb)]
interface IMFStreamSink(IMFStreamSinkVtbl): IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl) {
    fn GetMediaSink(
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT,
    fn GetIdentifier(
        pdwIdentifier: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeHandler(
        ppHandler: *mut *mut IMFMediaTypeHandler,
    ) -> HRESULT,
    fn ProcessSample(
        pSample: *mut IMFSample,
    ) -> HRESULT,
    fn PlaceMarker(
        eMarkerType: MFSTREAMSINK_MARKER_TYPE,
        pvarMarkerValue: *const PROPVARIANT,
        pvarContextValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn Flush() -> HRESULT,
}}
RIDL! {#[uuid(0x86cbc910, 0xe533, 0x4751, 0x8e, 0x3b, 0xf1, 0x9b, 0x5b, 0x80, 0x6a, 0x03)]
interface IMFVideoSampleAllocator(IMFVideoSampleAllocatorVtbl): IUnknown(IUnknownVtbl) {
    fn SetDirectXManager(
        pManager: *mut IUnknown,
    ) -> HRESULT,
    fn UninitializeSampleAllocator() -> HRESULT,
    fn InitializeSampleAllocator(
        cRequestedFrames: DWORD,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn AllocateSample(
        ppSample: *mut *mut IMFSample,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xa792cdbe, 0xc374, 0x4e89, 0x83, 0x35, 0x27, 0x8e, 0x7b, 0x99, 0x56, 0xa4)]
interface IMFVideoSampleAllocatorNotify(IMFVideoSampleAllocatorNotifyVtbl)
    : IUnknown(IUnknownVtbl) {
    fn NotifyRelease() -> HRESULT,
}}
RIDL! {#[uuid(0x3978aa1a, 0x6d5b, 0x4b7f, 0xa3, 0x40, 0x90, 0x89, 0x91, 0x89, 0xae, 0x34)]
interface IMFVideoSampleAllocatorNotifyEx(IMFVideoSampleAllocatorNotifyExVtbl)
    : IMFVideoSampleAllocatorNotify(IMFVideoSampleAllocatorNotifyVtbl) {
    fn NotifyPrune(
        __MIDL__IMFVideoSampleAllocatorNotifyEx0000: *mut IMFSample,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x992388b4, 0x3372, 0x4f67, 0x8b, 0x6f, 0xc8, 0x4c, 0x07, 0x1f, 0x47, 0x51)]
interface IMFVideoSampleAllocatorCallback(IMFVideoSampleAllocatorCallbackVtbl)
    : IUnknown(IUnknownVtbl) {
    fn SetCallback(
        pNotify: *mut IMFVideoSampleAllocatorNotify,
    ) -> HRESULT,
    fn GetFreeSampleCount(
        plSamples: *mut LONG,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x545b3a48, 0x3283, 0x4f62, 0x86, 0x6f, 0xa6, 0x2d, 0x8f, 0x59, 0x8f, 0x9f)]
interface IMFVideoSampleAllocatorEx(IMFVideoSampleAllocatorExVtbl)
    : IMFVideoSampleAllocator(IMFVideoSampleAllocatorVtbl) {
    fn InitializeSampleAllocatorEx(
        cInitialSamples: DWORD,
        cMaximumSamples: DWORD,
        pAttributes: *mut IMFAttributes,
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x20bc074b, 0x7a8d, 0x4609, 0x8c, 0x3b, 0x64, 0xa0, 0xa3, 0xb5, 0xd7, 0xce)]
interface IMFDXGIDeviceManagerSource(IMFDXGIDeviceManagerSourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetManager(
        ppManager: *mut *mut IMFDXGIDeviceManager,
    ) -> HRESULT,
}}
ENUM! {enum MF_VIDEO_PROCESSOR_ROTATION {
    ROTATION_NONE = 0,
    ROTATION_NORMAL = 1,
}}
ENUM! {enum MF_VIDEO_PROCESSOR_MIRROR {
    MIRROR_NONE = 0,
    MIRROR_HORIZONTAL = 1,
    MIRROR_VERTICAL = 2,
}}
RIDL! {#[uuid(0xa3f675d5, 0x6119, 0x4f7f, 0xa1, 0x00, 0x1d, 0x8b, 0x28, 0x0f, 0x0e, 0xfb)]
interface IMFVideoProcessorControl(IMFVideoProcessorControlVtbl): IUnknown(IUnknownVtbl) {
    fn SetBorderColor(
        pBorderColor: *mut MFARGB,
    ) -> HRESULT,
    fn SetSourceRectangle(
        pSrcRect: *mut RECT,
    ) -> HRESULT,
    fn SetDestinationRectangle(
        pDstRect: *mut RECT,
    ) -> HRESULT,
    fn SetMirror(
        eMirror: MF_VIDEO_PROCESSOR_MIRROR,
    ) -> HRESULT,
    fn SetRotation(
        eRotation: MF_VIDEO_PROCESSOR_ROTATION,
    ) -> HRESULT,
    fn SetConstrictionSize(
        pConstrictionSize: *mut SIZE,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xbde633d3, 0xe1dc, 0x4a7f, 0xa6, 0x93, 0xbb, 0xae, 0x39, 0x9c, 0x4a, 0x20)]
interface IMFVideoProcessorControl2(IMFVideoProcessorControl2Vtbl)
    : IMFVideoProcessorControl(IMFVideoProcessorControlVtbl) {
    fn SetRotationOverride(
        uiRotation: UINT,
    ) -> HRESULT,
    fn EnableHardwareEffects(
        fEnabled: BOOL,
    ) -> HRESULT,
    fn GetSupportedHardwareEffects(
        puiSupport: *mut UINT,
    ) -> HRESULT,
}}
ENUM! {enum MFVideoSphericalFormat {
    MFVideoSphericalFormat_Unsupported = 0,
    MFVideoSphericalFormat_Equirectangular = 1,
    MFVideoSphericalFormat_CubeMap = 2,
    MFVideoSphericalFormat_3DMesh = 3,
}}
ENUM! {enum MFVideoSphericalProjectionMode {
    MFVideoSphericalProjectionMode_Spherical = 0,
    MFVideoSphericalProjectionMode_Flat = MFVideoSphericalProjectionMode_Spherical + 1,
}}
RIDL! {#[uuid(0x2424b3f2, 0xeb23, 0x40f1, 0x91, 0xaa, 0x74, 0xbd, 0xde, 0xea, 0x08, 0x83)]
interface IMFVideoProcessorControl3(IMFVideoProcessorControl3Vtbl)
    : IMFVideoProcessorControl2(IMFVideoProcessorControl2Vtbl) {
    fn GetNaturalOutputType(
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn EnableSphericalVideoProcessing(
        fEnable: BOOL,
        eFormat: MFVideoSphericalFormat,
        eProjectionMode: MFVideoSphericalProjectionMode,
    ) -> HRESULT,
    fn SetSphericalVideoProperties(
        X: c_float,
        Y: c_float,
        Z: c_float,
        W: c_float,
        fieldOfView: c_float,
    ) -> HRESULT,
    fn SetOutputDevice(
        pOutputDevice: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x83cf873a, 0xf6da, 0x4bc8, 0x82, 0x3f, 0xba, 0xcf, 0xd5, 0x5d, 0xc4, 0x33)]
interface IMFTopology(IMFTopologyVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetTopologyID(
        pID: *mut TOPOID,
    ) -> HRESULT,
    fn AddNode(
        pNode: *mut IMFTopologyNode,
    ) -> HRESULT,
    fn RemoveNode(
        pNode: *mut IMFTopologyNode,
    ) -> HRESULT,
    fn GetNodeCount(
        pwNodes: *mut WORD,
    ) -> HRESULT,
    fn GetNode(
        wIndex: WORD,
        ppNode: *mut *mut IMFTopologyNode,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn CloneFrom(
        pTopology: *mut IMFTopology,
    ) -> HRESULT,
    fn GetNodeByID(
        qwTopoNodeID: TOPOID,
        ppNode: *mut *mut IMFTopologyNode,
    ) -> HRESULT,
    fn GetSourceNodeCollection(
        ppCollection: *mut *mut IMFCollection,
    ) -> HRESULT,
    fn GetOutputNodeCollection(
        ppCollection: *mut *mut IMFCollection,
    ) -> HRESULT,
}}
DEFINE_GUID! {MF_TOPOLOGY_PROJECTSTART,
0x7ed3f802, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80}
DEFINE_GUID! {MF_TOPOLOGY_PROJECTSTOP,
0x7ed3f803, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80}
DEFINE_GUID! {MF_TOPOLOGY_NO_MARKIN_MARKOUT,
0x7ed3f804, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80}
ENUM! {enum MFTOPOLOGY_DXVA_NODE {
    MFTOPOLOGY_DXVA_DEFAULT = 0,
    MFTOPOLOGY_DXVA_NONE = 1,
    MFTOPOLOGY_DXVA_FULL = 2,
}}
DEFINE_GUID! {MF_TOPOLOGY_DXVA_MODE,
0x1e8d34f6, 0xf5ab, 0x4e23, 0xbb, 0x88, 0x87, 0x4a, 0xa3, 0xa1, 0xa7, 0x4d}
DEFINE_GUID! {MF_TOPOLOGY_ENABLE_XVP_FOR_PLAYBACK,
0x1967731f, 0xcd78, 0x42fc, 0xb0, 0x26, 0x9, 0x92, 0xa5, 0x6e, 0x56, 0x93}
DEFINE_GUID! {MF_TOPOLOGY_STATIC_PLAYBACK_OPTIMIZATIONS,
0xb86cac42, 0x41a6, 0x4b79, 0x89, 0x7a, 0x1a, 0xb0, 0xe5, 0x2b, 0x4a, 0x1b}
DEFINE_GUID! {MF_TOPOLOGY_PLAYBACK_MAX_DIMS,
0x5715cf19, 0x5768, 0x44aa, 0xad, 0x6e, 0x87, 0x21, 0xf1, 0xb0, 0xf9, 0xbb}
ENUM! {enum MFTOPOLOGY_HARDWARE_NODE {
    MFTOPOLOGY_HWMODE_SOFTWARE_ONLY = 0,
    MFTOPOLOGY_HWMODE_USE_HARDWARE = 1,
    MFTOPOLOGY_HWMODE_USE_ONLY_HARDWARE = 2,
}}
DEFINE_GUID! {MF_TOPOLOGY_HARDWARE_MODE,
0xd2d362fd, 0x4e4f, 0x4191, 0xa5, 0x79, 0xc6, 0x18, 0xb6, 0x67, 0x6, 0xaf}
DEFINE_GUID! {MF_TOPOLOGY_PLAYBACK_FRAMERATE,
0xc164737a, 0xc2b1, 0x4553, 0x83, 0xbb, 0x5a, 0x52, 0x60, 0x72, 0x44, 0x8f}
DEFINE_GUID! {MF_TOPOLOGY_DYNAMIC_CHANGE_NOT_ALLOWED,
0xd529950b, 0xd484, 0x4527, 0xa9, 0xcd, 0xb1, 0x90, 0x95, 0x32, 0xb5, 0xb0}
DEFINE_GUID! {MF_TOPOLOGY_ENUMERATE_SOURCE_TYPES,
0x6248c36d, 0x5d0b, 0x4f40, 0xa0, 0xbb, 0xb0, 0xb3, 0x05, 0xf7, 0x76, 0x98}
DEFINE_GUID! {MF_TOPOLOGY_START_TIME_ON_PRESENTATION_SWITCH,
0xc8cc113f, 0x7951, 0x4548, 0xaa, 0xd6, 0x9e, 0xd6, 0x20, 0x2e, 0x62, 0xb3}
DEFINE_GUID! {MF_DISABLE_LOCALLY_REGISTERED_PLUGINS,
0x66b16da9, 0xadd4, 0x47e0, 0xa1, 0x6b, 0x5a, 0xf1, 0xfb, 0x48, 0x36, 0x34}
DEFINE_GUID! {MF_LOCAL_PLUGIN_CONTROL_POLICY,
0xd91b0085, 0xc86d, 0x4f81, 0x88, 0x22, 0x8c, 0x68, 0xe1, 0xd7, 0xfa, 0x04}
extern "system" {
    pub fn MFCreateTopology(ppTopo: *mut *mut IMFTopology) -> HRESULT;
}
ENUM! {enum MF_TOPOLOGY_TYPE {
    MF_TOPOLOGY_OUTPUT_NODE = 0,
    MF_TOPOLOGY_SOURCESTREAM_NODE = MF_TOPOLOGY_OUTPUT_NODE + 1,
    MF_TOPOLOGY_TRANSFORM_NODE = MF_TOPOLOGY_SOURCESTREAM_NODE + 1,
    MF_TOPOLOGY_TEE_NODE = MF_TOPOLOGY_TRANSFORM_NODE + 1,
    MF_TOPOLOGY_MAX = 0xffffffff,
}}
RIDL! {#[uuid(0x83cf873a, 0xf6da, 0x4bc8, 0x82, 0x3f, 0xba, 0xcf, 0xd5, 0x5d, 0xc4, 0x30)]
interface IMFTopologyNode(IMFTopologyNodeVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn SetObject(
        pObject: *mut IUnknown,
    ) -> HRESULT,
    fn GetObject(
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetNodeType(
        pType: *mut MF_TOPOLOGY_TYPE,
    ) -> HRESULT,
    fn GetTopoNodeID(
        pID: *mut TOPOID,
    ) -> HRESULT,
    fn SetTopoNodeID(
        ullTopoID: TOPOID,
    ) -> HRESULT,
    fn GetInputCount(
        pcInputs: *mut DWORD,
    ) -> HRESULT,
    fn GetOutputCount(
        pcOutputs: *mut DWORD,
    ) -> HRESULT,
    fn ConnectOutput(
        dwOutputIndex: DWORD,
        pDownstreamNode: *mut IMFTopologyNode,
        dwInputIndexOnDownstreamNode: DWORD,
    ) -> HRESULT,
    fn DisconnectOutput(
        dwOutputIndex: DWORD,
    ) -> HRESULT,
    fn GetInput(
        dwInputIndex: DWORD,
        ppUpstreamNode: *mut *mut IMFTopologyNode,
        pdwOutputIndexOnUpstreamNode: *mut DWORD,
    ) -> HRESULT,
    fn GetOutput(
        dwOutputIndex: DWORD,
        ppDownstreamNode: *mut *mut IMFTopologyNode,
        pdwInputIndexOnDownstreamNode: *mut DWORD,
    ) -> HRESULT,
    fn SetOutputPrefType(
        dwOutputIndex: DWORD,
        pType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetOutputPrefType(
        dwOutputIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetInputPrefType(
        dwInputIndex: DWORD,
        pType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetInputPRefType(
        dwInputIndex:
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn CloneFrom(
        pNode: *mut IMFTopologyNode,
    ) -> HRESULT,
}}
ENUM! {enum MF_TOPONODE_FLUSH_MODE {
    MF_TOPONODE_FLUSH_ALWAYS = 0,
    MF_TOPONODE_FLUSH_SEEK = MF_TOPONODE_FLUSH_ALWAYS + 1,
    MF_TOPONODE_FLUSH_NEVER = MF_TOPONODE_FLUSH_SEEK + 1,
}}
DEFINE_GUID! {MF_TOPONODE_FLUSH,
0x494bbce8, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
ENUM! {enum MF_TOPONODE_DRAIN_MODE {
    MF_TOPONODE_DRAIN_DEFAULT = 0,
    MF_TOPONODE_DRAIN_ALWAYS = MF_TOPONODE_DRAIN_DEFAULT + 1,
    MF_TOPONODE_DRAIN_NEVER = MF_TOPONODE_DRAIN_ALWAYS + 1,
}}
DEFINE_GUID! {MF_TOPONODE_DRAIN,
0x494bbce9, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_D3DAWARE,
0x494bbced, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPOLOGY_RESOLUTION_STATUS,
0x494bbcde, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_ERRORCODE,
0x494bbcee, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_CONNECT_METHOD,
0x494bbcf1, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_LOCKED,
0x494bbcf7, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_WORKQUEUE_ID,
0x494bbcf8, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_WORKQUEUE_MMCSS_CLASS,
0x494bbcf9, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_DECRYPTOR,
0x494bbcfa, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_DISCARDABLE,
0x494bbcfb, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_ERROR_MAJORTYPE,
0x494bbcfd, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_ERROR_SUBTYPE,
0x494bbcfe, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_WORKQUEUE_MMCSS_TASKID,
0x494bbcff, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_WORKQUEUE_MMCSS_PRIORITY,
0x5001f840, 0x2816, 0x48f4, 0x93, 0x64, 0xad, 0x1e, 0xf6, 0x61, 0xa1, 0x23}
DEFINE_GUID! {MF_TOPONODE_WORKQUEUE_ITEM_PRIORITY,
0xa1ff99be, 0x5e97, 0x4a53, 0xb4, 0x94, 0x56, 0x8c, 0x64, 0x2c, 0x0f, 0xf3}
DEFINE_GUID! {MF_TOPONODE_MARKIN_HERE,
0x494bbd00, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_MARKOUT_HERE,
0x494bbd01, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_DECODER,
0x494bbd02, 0xb031, 0x4e38, 0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc}
DEFINE_GUID! {MF_TOPONODE_MEDIASTART,
0x835c58ea, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID! {MF_TOPONODE_MEDIASTOP,
0x835c58eb, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID! {MF_TOPONODE_SOURCE,
0x835c58ec, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID! {MF_TOPONODE_PRESENTATION_DESCRIPTOR,
0x835c58ed, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID! {MF_TOPONODE_STREAM_DESCRIPTOR,
0x835c58ee, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID! {MF_TOPONODE_SEQUENCE_ELEMENTID,
0x835c58ef, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6}
DEFINE_GUID! {MF_TOPONODE_TRANSFORM_OBJECTID,
0x88dcc0c9, 0x293e, 0x4e8b, 0x9a, 0xeb, 0xa, 0xd6, 0x4c, 0xc0, 0x16, 0xb0}
DEFINE_GUID! {MF_TOPONODE_STREAMID,
0x14932f9b, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID! {MF_TOPONODE_NOSHUTDOWN_ON_REMOVE,
0x14932f9c, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID! {MF_TOPONODE_RATELESS,
0x14932f9d, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID! {MF_TOPONODE_DISABLE_PREROLL,
0x14932f9e, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04}
DEFINE_GUID! {MF_TOPONODE_PRIMARYOUTPUT,
0x6304ef99, 0x16b2, 0x4ebe, 0x9d, 0x67, 0xe4, 0xc5, 0x39, 0xb3, 0xa2, 0x59}
extern "system" {
    pub fn MFCreateTopologyNode(
        NodeType: MF_TOPOLOGY_TYPE,
        ppNode: *mut *mut IMFTopologyNode,
    ) -> HRESULT;
    pub fn MFGetTopoNodeCurrentType(
        pNode: *mut IMFTopologyNode,
        dwStreamIndex: DWORD,
        fOutput: BOOL,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT;
}
RIDL! {#[uuid(0xfa993888, 0x4383, 0x415a, 0xa9, 0x30, 0xdd, 0x47, 0x2a, 0x8c, 0xf6, 0xf7)]
interface IMFGetService(IMFGetServiceVtbl): IUnknown(IUnknownVtbl) {
    fn GetService(
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFGetService(
        punkObject: *mut IUnknown,
        guidService: REFGUID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT;
}
pub type MFTIME = LONGLONG;
ENUM! {enum MFCLOCK_CHARACTERISTICS_FLAGS {
    MFCLOCK_CHARACTERISTICS_FLAG_FREQUENCY_10MHZ = 0x2,
    MFCLOCK_CHARACTERISTICS_FLAG_ALWAYS_RUNNING = 0x4,
    MFCLOCK_CHARACTERISTICS_FLAG_IS_SYSTEM_CLOCK = 0x8,
}}
ENUM! {enum MFCLOCK_STATE {
    MFCLOCK_STATE_INVALID = 0,
    MFCLOCK_STATE_RUNNING = MFCLOCK_STATE_INVALID + 1,
    MFCLOCK_STATE_STOPPED = MFCLOCK_STATE_RUNNING + 1,
    MFCLOCK_STATE_PAUSED = MFCLOCK_STATE_STOPPED + 1,
}}
ENUM! {enum MFCLOCK_RELATIONAL_FLAGS {
    MFCLOCK_RELATIONAL_FLAG_JITTER_NEVER_AHEAD = 0x1,
}}
STRUCT! {struct MFCLOCK_PROPERTIES {
    qwCorrelationRate: __uint64,
    guidClockId: GUID,
    dwClockFlags: DWORD,
    qwClockFrequency: __uint64,
    dwClockTolerance: DWORD,
    dwClockJitter: DWORD,
}}
pub const MFCLOCK_FREQUENCY_HNS: __uint64 = 10000000;
pub const MFCLOCK_TOLERANCE_UNKNOWN: DWORD = 50000;
pub const MFCLOCK_JITTER_ISR: DWORD = 1000;
pub const MFCLOCK_JITTER_DPC: DWORD = 4000;
pub const MFCLOCK_JITTER_PASSIVE: DWORD = 10000;
RIDL! {#[uuid(0x2eb1e945, 0x18b8, 0x4139, 0x9b, 0x1a, 0xd5, 0xd5, 0x84, 0x81, 0x85, 0x30)]
interface IMFClock(IMFClockVtbl): IUnknown(IUnknownVtbl) {
    fn GetClockCharacteristics(
        pdwCharacteristics: *mut DWORD,
    ) -> HRESULT,
    fn GetCorrelatedTime(
        dwReserved: DWORD,
        pllClockTime: *mut LONGLONG,
        phnsSystemTime: *mut MFTIME,
    ) -> HRESULT,
    fn GetContinuityKey(
        pdwContinuityKey: *mut DWORD,
    ) -> HRESULT,
    fn GetState(
        dwReserved: DWORD,
        peClockState: *mut MFCLOCK_STATE,
    ) -> HRESULT,
    fn GetProperties(
        pClockProperties: *mut MFCLOCK_PROPERTIES,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFGetSystemTime() -> MFTIME;
}
pub const PRESENTATION_CURRENT_POSITION: LONGLONG = 0x7fffffffffffffff;
RIDL! {#[uuid(0x868ce85c, 0x8ea9, 0x4f55, 0xab, 0x82, 0xb0, 0x09, 0xa9, 0x10, 0xa8, 0x05)]
interface IMFPresentationClock(IMFPresentationClockVtbl): IMFClock(IMFClockVtbl) {
    fn SetTimeSource(
        pTimeSource: *mut IMFPresentationTimeSource,
    ) -> HRESULT,
    fn GetTimeSource(
        ppTimeSource: *mut *mut IMFPresentationTimeSource,
    ) -> HRESULT,
    fn GetTime(
        phnsClockTime: *mut MFTIME,
    ) -> HRESULT,
    fn AddClockStateSink(
        pStateSink: *mut IMFClockStateSink,
    ) -> HRESULT,
    fn RemoveClockStateSink(
        pStateSink: *mut IMFClockStateSink,
    ) -> HRESULT,
    fn Start(
        llClockStartOffset: LONGLONG,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Pause() -> HRESULT,
}}
extern "system" {
    pub fn MFCreatePresentationClock(
        ppPresentationClock: *mut *mut IMFPresentationClock,
    ) -> HRESULT;
}
RIDL! {#[uuid(0x7ff12cce, 0xf76f, 0x41c2, 0x86, 0x3b, 0x16, 0x66, 0xc8, 0xe5, 0xe1, 0x39)]
interface IMFPresentationTimeSource(IMFPresentationTimeSourceVtbl): IMFClock(IMFClockVtbl) {
    fn GetUnderlyingClock(
        ppClock: *mut *mut IMFClock,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSystemTimeSource(
        ppSystemTimeSource: *mut *mut IMFPresentationTimeSource,
    ) -> HRESULT;
}
RIDL! {#[uuid(0xf6696e82, 0x74f7, 0x4f3d, 0xa1, 0x78, 0x8a, 0x5e, 0x09, 0xc3, 0x65, 0x9f)]
interface IMFClockStateSink(IMFClockStateSinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnClockStart(
        hnsSystemTime: MFTIME,
        llClockStartOffset: LONGLONG,
    ) -> HRESULT,
    fn OnClockStop(
        hnsSystemTime: MFTIME,
    ) -> HRESULT,
    fn OnClockPause(
        hnsSystemTime: MFTIME,
    ) -> HRESULT,
    fn OnClockRestart(
        hnsSystemTime: MFTIME,
    ) -> HRESULT,
    fn OnClockSetRate(
        hnssystemTime: MFTIME,
        flRate: c_float,
    ) -> HRESULT,
}}
DEFINE_GUID! {MF_PD_PMPHOST_CONTEXT,
0x6c990d31, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_APP_CONTEXT,
0x6c990d32, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_DURATION,
0x6c990d33, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_TOTAL_FILE_SIZE,
0x6c990d34, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_AUDIO_ENCODING_BITRATE,
0x6c990d35, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_VIDEO_ENCODING_BITRATE,
0x6c990d36, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_MIME_TYPE,
0x6c990d37, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_LAST_MODIFIED_TIME,
0x6c990d38, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_PLAYBACK_ELEMENT_ID,
0x6c990d39, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_PREFERRED_LANGUAGE,
0x6c990d3A, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_PLAYBACK_BOUNDARY_TIME,
0x6c990d3b, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a}
DEFINE_GUID! {MF_PD_AUDIO_ISVARIABLEBITRATE,
0x33026ee0, 0xe387, 0x4582, 0xae, 0x0a, 0x34, 0xa2, 0xad, 0x3b, 0xaa, 0x18}
DEFINE_GUID! {MF_PD_ADAPTIVE_STREAMING,
0xEA0D5D97, 0x29F9, 0x488B, 0xAE, 0x6B, 0x7D, 0x6B, 0x41, 0x36, 0x11, 0x2B}
RIDL! {#[uuid(0x03cb2711, 0x24d7, 0x4db6, 0xa1, 0x7f, 0xf3, 0xa7, 0xa4, 0x79, 0xa5, 0x36)]
interface IMFPresentationDescriptor(IMFPresentationDescriptorVtbl)
    : IMFAttributes(IMFAttributesVtbl) {
    fn GetStreamDescriptorCount(
        pdwDescriptorCount: *mut DWORD,
    ) -> HRESULT,
    fn GetStreamDescriptorByIndex(
        dwIndex: DWORD,
        pfSelected: *mut BOOL,
        ppDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT,
    fn SelectStream(
        dwDescriptorIndex: DWORD,
    ) -> HRESULT,
    fn DeselectStream(
        dwDescriptorIndex: DWORD,
    ) -> HRESULT,
    fn Clone(
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreatePresentationDescriptor(
        cStreamDescriptors: DWORD,
        apStreamDescriptors: *mut *mut IMFStreamDescriptor,
        ppPresentationDescriptor: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT;
    pub fn MFRequireProtectedEnvironment(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
    ) -> HRESULT;
    pub fn MFSerializePresentationDescriptor(
        pPD: *mut IMFPresentationDescriptor,
        pcbData: *mut DWORD,
        ppbData: *mut *mut BYTE,
    ) -> HRESULT;
    pub fn MFDeserializePresentationDescriptor(
        cbData: DWORD,
        pbData: *mut BYTE,
        ppPD: *mut *mut IMFPresentationDescriptor,
    ) -> HRESULT;
}
DEFINE_GUID! {MF_SD_LANGUAGE,
0xaf2180, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x3, 0x59, 0x3b, 0xc1, 0x21}
DEFINE_GUID! {MF_SD_PROTECTED,
0xaf2181, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x3, 0x59, 0x3b, 0xc1, 0x21}
DEFINE_GUID! {MF_SD_STREAM_NAME,
0x4f1b099d, 0xd314, 0x41e5, 0xa7, 0x81, 0x7f, 0xef, 0xaa, 0x4c, 0x50, 0x1f}
DEFINE_GUID! {MF_SD_MUTUALLY_EXCLUSIVE,
0x23ef79c, 0x388d, 0x487f, 0xac, 0x17, 0x69, 0x6c, 0xd6, 0xe3, 0xc6, 0xf5}
RIDL! {#[uuid(0x56c03d9c, 0x9dbb, 0x45f5, 0xab, 0x4b, 0xd8, 0x0f, 0x47, 0xc0, 0x59, 0x38)]
interface IMFStreamDescriptor(IMFStreamDescriptorVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetStreamIdentifier(
        pdwStreamIdentifier: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeHandler(
        ppMediaTypeHandler: *mut *mut IMFMediaTypeHandler,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateStreamDescriptor(
        dwStreamIdentifier: DWORD,
        cMediaTypes: DWORD,
        apMediaTypes: *mut *mut IMFMediaType,
        ppDescriptor: *mut *mut IMFStreamDescriptor,
    ) -> HRESULT;
}
RIDL! {#[uuid(0xe93dcf6c, 0x4b07, 0x4e1e, 0x81, 0x23, 0xaa, 0x16, 0xed, 0x6e, 0xad, 0xf5)]
interface IMFMediaTypeHandler(IMFMediaTypeHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn IsMediaTypeSupported(
        pMediaType: *mut IMFMediaType,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetMediaTypeCount(
        pdwTypeCount: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaTypeByIndex(
        dwIndex: DWORD,
        ppType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn SetCurrentMediaType(
        pMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetCurrentMediaType(
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetMajorType(
        pguidMajorType: *mut GUID,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateSimpleTypeHandler(ppHandler: *mut *mut IMFMediaTypeHandler) -> HRESULT;
}
ENUM! {enum MFTIMER_FLAGS {
    MFTIMER_RELATIVE = 0x1,
}}
RIDL! {#[uuid(0xe56e4cbd, 0x8f70, 0x49d8, 0xa0, 0xf8, 0xed, 0xb3, 0xd6, 0xab, 0x9b, 0xf2)]
interface IMFTimer(IMFTimerVtbl): IUnknown(IUnknownVtbl) {
    fn SetTimer(
        dwFlags: DWORD,
        llClockTime: LONGLONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
        ppunkKey: *mut *mut IUnknown,
    ) -> HRESULT,
    fn CancelTimer(
        punkKey: *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID! {MF_ACTIVATE_CUSTOM_VIDEO_MIXER_CLSID,
0xba491360, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID! {MF_ACTIVATE_CUSTOM_VIDEO_MIXER_ACTIVATE,
0xba491361, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID! {MF_ACTIVATE_CUSTOM_VIDEO_MIXER_FLAGS,
0xba491362, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID! {MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_CLSID,
0xba491364, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID! {MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_ACTIVATE,
0xba491365, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
DEFINE_GUID! {MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_FLAGS,
0xba491366, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8}
ENUM! {enum __MIDL___MIDL_itf_mfidl_0000_0028_0001 {
    MF_ACTIVATE_CUSTOM_MIXER_ALLOWFAIL = 0x1,
}}
ENUM! {enum __MIDL___MIDL_itf_mfidl_0000_0028_0002 {
    MF_ACTIVATE_CUSTOM_PRESENTER_ALLOWFAIL = 0x1,
}}
DEFINE_GUID! {MF_ACTIVATE_MFT_LOCKED,
0xc1f6093c, 0x7f65, 0x4fbd, 0x9e, 0x39, 0x5f, 0xae, 0xc3, 0xc4, 0xfb, 0xd7}
DEFINE_GUID! {MF_ACTIVATE_VIDEO_WINDOW,
0x9a2dbbdd, 0xf57e, 0x4162, 0x82, 0xb9, 0x68, 0x31, 0x37, 0x76, 0x82, 0xd3}
ENUM! {enum MFSHUTDOWN_STATUS {
    MFSHUTDOWN_INITIATED = 0,
    MFSHUTDOWN_COMPLETED = (MFSHUTDOWN_INITIATED + 1),
}}
RIDL! {#[uuid(0x97ec2ea4, 0x0e42, 0x4937, 0x97, 0xac, 0x9d, 0x6d, 0x32, 0x88, 0x24, 0xe1)]
interface IMFShutdown(IMFShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn Shutdown() -> HRESULT,
    fn GetShutdownStatus(
        pStatus: *mut MFSHUTDOWN_STATUS,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFShutdownObject(pUnk: *mut IUnknown) -> HRESULT;
    pub fn MFCreateAudioRenderer(pAudioAttributes: *mut IMFAttributes) -> HRESULT;
    pub fn MFCreateAudioRendererActivate(ppActivate: *mut *mut IMFActivate) -> HRESULT;
}
DEFINE_GUID! {MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS,
0xede4b5e0, 0xf805, 0x4d6c, 0x99, 0xb3, 0xdb, 0x01, 0xbf, 0x95, 0xdf, 0xab}
DEFINE_GUID! {MF_AUDIO_RENDERER_ATTRIBUTE_SESSION_ID,
0xede4b5e3, 0xf805, 0x4d6c, 0x99, 0xb3, 0xdb, 0x01, 0xbf, 0x95, 0xdf, 0xab}
DEFINE_GUID! {MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ID,
0xb10aaec3, 0xef71, 0x4cc3, 0xb8, 0x73, 0x5, 0xa9, 0xa0, 0x8b, 0x9f, 0x8e}
DEFINE_GUID! {MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ROLE,
0x6ba644ff, 0x27c5, 0x4d02, 0x98, 0x87, 0xc2, 0x86, 0x19, 0xfd, 0xb9, 0x1b}
DEFINE_GUID! {MF_AUDIO_RENDERER_ATTRIBUTE_STREAM_CATEGORY,
0xa9770471, 0x92ec, 0x4df4, 0x94, 0xfe, 0x81, 0xc3, 0x6f, 0xc, 0x3a, 0x7a}
extern "system" {
    pub fn MFCreateVideoRendererActivate(
        hwndVideo: HWND,
        ppActivate: *mut *mut IMFActivate,
    ) -> HRESULT;
    pub fn MFCreateMPEG4MediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreate3GPMediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateMP3MediaSink(
        pTargetByteStream: *mut IMFByteStream,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateAC3MediaSink(
        pTargetByteStream: *mut IMFByteStream,
        pAudioMediaType: *mut IMFMediaType,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateADTSMediaSink(
        pTargetByteStream: *mut IMFByteStream,
        pAudioMediaType: *mut IMFMediaType,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateMuxSink(
        guidOutputSubType: GUID,
        pOutputAttributes: *mut IMFAttributes,
        pOutputByteStream: *mut IMFByteStream,
        ppMuxSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateFMPEG4MediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFAudioMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateAVIMediaSink(
        pIByteStream: *mut IMFByteStream,
        pVideoMediaType: *mut IMFMediaType,
        pAudioMediaType: *mut IMFMediaType,
        ppIMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
    pub fn MFCreateWAVEMediaSink(
        pTargetByteStream: *mut IMFByteStream,
        pAudioMediaType: *mut IMFMediaType,
        ppMediaSink: *mut *mut IMFMediaSink,
    ) -> HRESULT;
}
RIDL! {#[uuid(0xde9a6157, 0xf660, 0x4643, 0xb5, 0x6a, 0xdf, 0x9f, 0x79, 0x98, 0xc7, 0xcd)]
interface IMFTopoLoader(IMFTopoLoaderVtbl): IUnknown(IUnknownVtbl) {
    fn Load(
        pInputTopo: *mut IMFTopology,
        ppOutputTopo: *mut IMFTopology,
        pCurrentTopo: *mut IMFTopology,
    ) -> HRESULT,
}}
extern "system" {
    pub fn MFCreateTopoLoader(ppObj: *mut *mut IMFTopoLoader) -> HRESULT;
}
RIDL! {#[uuid(0xacf92459, 0x6a61, 0x42bd, 0xb5, 0x7c, 0xb4, 0x3e, 0x51, 0x20, 0x3c, 0xb0)]
interface IMFContentProtectionManager(IMFContentProtectionManagerVtbl): IUnknown(IUnknownVtbl) {
    fn BeginEnableContent(
        pEnablerActivate: *mut IMFActivate,
        pTopo: *mut IMFTopology,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndEnableContent(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
}}
ENUM! {enum MF_URL_TRUST_STATUS {
    MF_LICENSE_URL_UNTRUSTED = 0,
    MF_LICENSE_URL_TRUSTED = MF_LICENSE_URL_UNTRUSTED + 1,
    MF_LICENSE_URL_TAMPERED = MF_LICENSE_URL_TRUSTED + 1,
}}
RIDL! {#[uuid(0xd3c4ef59, 0x49ce, 0x4381, 0x90, 0x71, 0xd5, 0xbc, 0xd0, 0x44, 0xc7, 0x70)]
interface IMFContentEnabler(IMFContentEnablerVtbl): IUnknown(IUnknownVtbl) {
    fn GetEnableType(
        pType: *mut GUID,
    ) -> HRESULT,
    fn GetEnableURL(
        ppwszURL: *mut LPWSTR,
        pcchURL: *mut DWORD,
        pTrustStatus: *mut MF_URL_TRUST_STATUS,
    ) -> HRESULT,
    fn GetEnableData(
        ppbData: *mut *mut BYTE,
        pcbData: *mut DWORD,
    ) -> HRESULT,
    fn IsAutomaticSupported(
        pfAutomatic: *mut BOOL,
    ) -> HRESULT,
    fn AutomaticEnable() -> HRESULT,
    fn MonitorEnable() -> HRESULT,
    fn Cancel() -> HRESULT,
}}
DEFINE_GUID! {MFENABLETYPE_WMDRMV1_LicenseAcquisition,
0x4ff6eeaf, 0xb43, 0x4797, 0x9b, 0x85, 0xab, 0xf3, 0x18, 0x15, 0xe7, 0xb0}
DEFINE_GUID! {MFENABLETYPE_WMDRMV7_LicenseAcquisition,
0x3306df, 0x4a06, 0x4884,0xa0, 0x97, 0xef, 0x6d, 0x22, 0xec, 0x84, 0xa3}
DEFINE_GUID! {MFENABLETYPE_WMDRMV7_Individualization,
0xacd2c84a, 0xb303, 0x4f65, 0xbc, 0x2c, 0x2c, 0x84, 0x8d, 0x1, 0xa9, 0x89}
DEFINE_GUID! {MFENABLETYPE_MF_UpdateRevocationInformation,
0xe558b0b5, 0xb3c4, 0x44a0, 0x92, 0x4c, 0x50, 0xd1, 0x78, 0x93, 0x23, 0x85}
DEFINE_GUID! {MFENABLETYPE_MF_UpdateUntrustedComponent,
0x9879f3d6, 0xcee2, 0x48e6, 0xb5, 0x73, 0x97, 0x67, 0xab, 0x17, 0x2f, 0x16}
DEFINE_GUID! {MFENABLETYPE_MF_RebootRequired,
0x6d4d3d4b, 0x0ece, 0x4652, 0x8b, 0x3a, 0xf2, 0xd2, 0x42, 0x60, 0xd8, 0x87}
pub const MF_USER_MODE_COMPONENT_LOAD: DWORD = 0x00000001;
pub const MF_KERNEL_MODE_COMPONENT_LOAD: DWORD = 0x00000002;
pub const MF_GRL_LOAD_FAILED: DWORD = 0x00000010;
pub const MF_INVALID_GRL_SIGNATURE: DWORD = 0x00000020;
pub const MF_GRL_ABSENT: DWORD = 0x00001000;
pub const MF_COMPONENT_REVOKED: DWORD = 0x00002000;
pub const MF_COMPONENT_INVALID_EKU: DWORD = 0x00004000;
pub const MF_COMPONENT_CERT_REVOKED: DWORD = 0x00008000;
pub const MF_COMPONENT_INVALID_ROOT: DWORD = 0x00010000;
pub const MF_COMPONENT_HS_CERT_REVOKED: DWORD = 0x00020000;
pub const MF_COMPONENT_LS_CERT_REVOKED: DWORD = 0x00040000;
pub const MF_BOOT_DRIVER_VERIFICATION_FAILED: DWORD = 0x00100000;
pub const MF_TEST_SIGNED_COMPONENT_LOADING: DWORD = 0x01000000;
pub const MF_MINCRYPT_FAILURE: DWORD = 0x10000000;
pub const SHA_HASH_LEN: usize = 20;
pub const STR_HASH_LEN: usize = SHA_HASH_LEN * 2 + 3;
STRUCT! {struct MFRR_COMPONENT_HASH_INFO {
    ulReason: DWORD,
    rgHeaderHash: [WCHAR; STR_HASH_LEN],
    rgPublicKeyHash: [WCHAR; STR_HASH_LEN],
    wszName: [WCHAR; MAX_PATH],
}}
pub type PMFRR_COMPONENT_HASH_INFO = *mut MFRR_COMPONENT_HASH_INFO;
STRUCT! {struct MFRR_COMPONENTS {
    dwRRInfoVersion: DWORD,
    dwRRComponents: DWORD,
    pRRComponents: PMFRR_COMPONENT_HASH_INFO,
}}
pub type PMFRR_COMPONENTS = *mut MFRR_COMPONENTS;
STRUCT! {#[repr(packed)] struct ASF_FLAT_PICTURE {
    bPictureType: BYTE,
    dwDataLen: DWORD,
}}
STRUCT! {#[repr(packed)] struct ASF_FLAT_SYNCHRONIZED_LYRICS {
    bTimeStampFormat: BYTE,
    bContentType: BYTE,
    dwLyricsLen: DWORD,
}}
RIDL! {#[uuid(0xf88cfb8c, 0xef16, 0x4991, 0xb4, 0x50, 0xcb, 0x8c, 0x69, 0xe5, 0x17, 0x04)]
interface IMFMetadata(IMFMetadataVtbl): IUnknown(IUnknownVtbl) {
    fn SetLanguage(
        pwszRFC1766: LPCWSTR,
    ) -> HRESULT,
    fn GetLanguage(
        ppwszRFC1766: *mut LPWSTR,
    ) -> HRESULT,
    fn GetAllLanguages(
        ppvLanguages: *mut PROPVARIANT,
    ) -> HRESULT,
    fn SetProperty(
        pwszName: LPCWSTR,
        ppvValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn GetProperty(
        pwszName: LPCWSTR,
        ppvValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn DeleteProperty(
        pwszName: LPCWSTR,
    ) -> HRESULT,
    fn GetAllPropertyNames(
        ppvNames: *mut PROPVARIANT,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x56181d2d, 0xe221, 0x4adb, 0xb1, 0xc8, 0x3c, 0xee, 0x6a, 0x53, 0xf7, 0x6f)]
interface IMFMetadataProvider(IMFMetadataProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetMFMetadata(
        pPresentationDescriptor: *mut IMFPresentationDescriptor,
        dwStreamIdentifier: DWORD,
        dwFlags: DWORD,
        ppMFMetadata: *mut *mut IMFMetadata,
    ) -> HRESULT,
}}
