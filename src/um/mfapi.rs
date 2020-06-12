// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{__int64, c_void};
use shared::basetsd::{INT16, INT32, INT64, UINT32, UINT64, UINT8};
use shared::dxgiformat::DXGI_FORMAT;
use shared::guiddef::{CLSID, GUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, FLOAT, LONG, LPDWORD, UINT, ULONG};
use shared::windef::{POINT, RECT};
use um::mfobjects::{
    IMFAsyncCallback, IMFAsyncResult, IMFAsyncResultVtbl, IMFAttributes, IMFMediaBuffer, IMFSample,
};
use um::minwinbase::OVERLAPPED;
use um::propidl::PROPVARIANT;
use um::unknwnbase::{IClassFactory, IUnknown};
use um::winnt::{HANDLE, HRESULT, LONGLONG, LPCWSTR, LPWSTR, PCWSTR};
pub const MFSTARTUP_NOSOCKET: DWORD = 0x1;
pub const MFSTARTUP_LITE: DWORD = MFSTARTUP_NOSOCKET;
pub const MFSTARTUP_FULL: DWORD = 0;
extern "system" {
    pub fn MFStartup(Version: ULONG, dwFlags: DWORD) -> HRESULT;
    pub fn MFShutdown() -> HRESULT;
    pub fn MFLockPlatform() -> HRESULT;
    pub fn MFUnlockPlatform() -> HRESULT;
}
pub type MFWORKITEM_KEY = __int64;
extern "system" {
    pub fn MFPutWorkItem(
        dwQueue: DWORD,
        pCallback: *mut IMFAsyncCallback,
        pState: IUnknown,
    ) -> HRESULT;
    pub fn MFPutWorkItem2(
        dwQueue: DWORD,
        Priority: LONG,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
    ) -> HRESULT;
    pub fn MFPutWorkItemEx(dwQueue: DWORD, pResult: *mut IMFAsyncResult) -> HRESULT;
    pub fn MFPutWorkItemEx2(
        dwQueue: DWORD,
        Priority: LONG,
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT;
    pub fn MFPutWaitingWorkItem(
        hEvent: HANDLE,
        Priority: LONG,
        pResult: *mut IMFAsyncResult,
        pKey: *mut MFWORKITEM_KEY,
    ) -> HRESULT;
    pub fn MFAllocateSerialWorkQueue(dwWorkQueue: DWORD, pdwWorkQueue: *mut DWORD) -> HRESULT;
    pub fn MFScheduleWorkItemEx(
        pResult: *mut IMFAsyncResult,
        Timeout: INT64,
        pKey: *mut MFWORKITEM_KEY,
    ) -> HRESULT;
    pub fn MFScheduleWorkItem(
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
        Timeout: INT64,
        pKey: *mut MFWORKITEM_KEY,
    ) -> HRESULT;
    pub fn MFCancelWorkItem(Key: MFWORKITEM_KEY) -> HRESULT;
    pub fn MFGetTimerPeriodicity(Periodicity: *mut DWORD) -> HRESULT;
}
FN! {stdcall MFPERIODICCALLBACK(
    *mut IUnknown,
) -> ()}
extern "system" {
    pub fn MFAddPeriodicCallback(
        Callback: MFPERIODICCALLBACK,
        pContext: *mut IUnknown,
        pdwKey: *mut DWORD,
    ) -> HRESULT;
    pub fn MFRemovePeriodicCallback(dwKey: DWORD) -> HRESULT;
}
ENUM! {enum MFASYNC_WORKQUEUE_TYPE {
    MF_STANDARD_WORKQUEUE = 0,
    MF_WINDOW_WORKQUEUE = 1,
    MF_MULTITHREADED_WORKQUEUE = 2,
}}
extern "system" {
    pub fn MFAllocateWorkQueueEx(
        WorkQueueType: MFASYNC_WORKQUEUE_TYPE,
        pdwWorkQueue: *mut DWORD,
    ) -> HRESULT;
    pub fn MFAllocateWorkQueue(pdwWorkQueue: *mut DWORD) -> HRESULT;
    pub fn MFLockWorkQueue(dwWorkQueue: DWORD) -> HRESULT;
    pub fn MFUnlockWorkQueue(dwWorkQueue: DWORD) -> HRESULT;
    pub fn MFBeginRegisterWorkQueueWithMMCSS(
        dwWorkQueueId: DWORD,
        wszClass: LPCWSTR,
        dwTaskId: DWORD,
        pDoneCallback: *mut IMFAsyncCallback,
        pDoneState: *mut IUnknown,
    ) -> HRESULT;
    pub fn MFBeginRegisterWorkQueueWithMMCSSEx(
        dwWorkQueueId: DWORD,
        wszClass: LPCWSTR,
        dwTaskId: DWORD,
        lPriority: LONG,
        pDoneCallback: *mut IMFAsyncCallback,
        pDoneState: *mut IUnknown,
    ) -> HRESULT;
    pub fn MFEndRegisterWorkQueueWithMMCSS(
        pResult: *mut IMFAsyncResult,
        pdwTaskId: *mut DWORD,
    ) -> HRESULT;
    pub fn MFBeginUnregisterWorkQueueWithMMCSS(
        dwWorkQueueId: DWORD,
        pDoneCallback: *mut IMFAsyncCallback,
        pDoneState: *mut IUnknown,
    ) -> HRESULT;
    pub fn MFEndUnregisterWorkQueueWithMMCSS(pResult: *mut IMFAsyncResult) -> HRESULT;
    pub fn MFGetWorkQueueMMCSSClass(
        dwWorkQueueId: DWORD,
        pwszClass: LPWSTR,
        pcchClass: *mut DWORD,
    ) -> HRESULT;
    pub fn MFGetWorkQueueMMCSSTaskId(dwWorkQueueId: DWORD, pdwTaskId: LPDWORD) -> HRESULT;
    pub fn MFRegisterPlatformWithMMCSS(
        wszClass: PCWSTR,
        pdwTaskId: *mut DWORD,
        lPriority: LONG,
    ) -> HRESULT;
    pub fn MMUnregisterPlatformFromMMCSS() -> HRESULT;
    pub fn MFLockSharedWorkQueue(
        wszClass: PCWSTR,
        BasePriority: LONG,
        pdwTaskId: *mut DWORD,
        pID: *mut DWORD,
    ) -> HRESULT;
    pub fn MFGetWorkQueueMMCSSPriority(dwWorkQueueId: DWORD, lPriority: *mut LONG) -> HRESULT;
    pub fn MFCreateAsyncResult(
        punkObject: *mut IUnknown,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
        ppAsyncResult: *mut *mut IMFAsyncResult,
    ) -> HRESULT;
    pub fn MFInvokeCallback(pAsyncResult: *mut IMFAsyncResult) -> HRESULT;
}
STRUCT! {struct MFASYNCRESULT {
    AsyncResult: *mut IMFAsyncResultVtbl,
    overlapped: OVERLAPPED,
    pCallback: *mut IMFAsyncCallback,
    hrStatusResult: HRESULT,
    dwBytesTransferred: DWORD,
    hEvent: HANDLE,
}}
extern "system" {
    pub fn MFCreateFile(
        AccessMode: MF_FILE_ACCESSMODE,
        OpenMode: MF_FILE_OPENMODE,
        fFlags: MF_FILE_FLAGS,
        pwszFileURL: LPCWSTR,
        ppIByteStream: *mut *mut IMFByteStream,
    ) -> HRESULT;
    pub fn MFCreateTempFile(
        AccessMode: MF_FILE_ACCESSMODE,
        OpenMode: MF_FILE_OPENMODE,
        fFlags: MF_FILE_FLAGS,
        ppIByteStream: *mut *mut IMFByteStream,
    ) -> HRESULT;
    pub fn MFBeginCreateFile(
        AccessMode: MF_FILE_ACCESSMODE,
        OpenMode: MF_FILE_OPENMODE,
        fFlags: MF_FILE_FLAGS,
        pwszFilePath: LPCWSTR,
        pCallback: *mut IMFAsyncCallback,
        pState: *mut IUnknown,
        ppCancelCookie: *mut *mut IUnknown,
    ) -> HRESULT;
    pub fn MFEndCreateFile(
        pResult: *mut IMFAsyncResult,
        ppFile: *mut *mut IMFByteStream,
    ) -> HRESULT;
    pub fn MFCancelCreateFile(pCancelCookie: *mut IUnknown) -> HRESULT;
    pub fn MFCreateMemoryBuffer(cbMaxLength: DWORD, ppBuffer: *mut *mut IMFMediaBuffer) -> HRESULT;
    pub fn MFCreateMediaBufferWrapper(
        pBuffer: *mut IMFMediaBuffer,
        cbOffset: DWORD,
        dwLength: DWORD,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT;
    pub fn MFCreateLegacyMediaBufferOnMFMediaBuffer(
        pSample: *mut IMFSample,
        pMFMediaBuffer: *mut IMFMediaBuffer,
        cbOffset: DWORD,
        ppMediaBuffer: *mut *mut IMediaBuffer,
    ) -> HRESULT;
    pub fn MFMapDX9FormatToDXGIFormat(dx9: DWORD) -> DXGI_FORMAT;
    pub fn MFMapDXGIFormatToDX9Format(dx11: DXGI_FORMAT) -> DWORD;
    pub fn MFLockDXGIDeviceManager(
        pResetToken: *mut UINT,
        ppManager: *mut *mut IMFDXGIDeviceManager,
    ) -> HRESULT;
    pub fn MFUnlockDXGIDeviceManager() -> HRESULT;
    pub fn MFCreateDXSurfaceBuffer(
        riid: REFIID,
        punkSurface: *mut IUnknown,
        fBottomUpWhenLinear: BOOL,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT;
    pub fn MFCreateWICBitmapBuffer(
        riid: REFIID,
        punkSurface: *mut IUnknown,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT;
    pub fn MFCreateDXGISurfaceBuffer(
        riid: REFIID,
        punkSurface: *mut IUnknown,
        uSubresourceIndex: UINT,
        fBottomUpWhenLinear: BOOL,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT;
    pub fn MFCreateVideoSampleAllocatorEx(
        riid: REFIID,
        ppSampleAllocator: *mut *mut c_void,
    ) -> HRESULT;
    pub fn MFCreateDXGIDeviceManager(
        resetToken: *mut UINT,
        ppDeviceManager: *mut *mut IMFDXGIDeviceManager,
    ) -> HRESULT;
}
pub const MF_E_DXGI_DEVICE_NOT_INITIALIZED: HRESULT = 0x80041000;
pub const MF_E_DXGI_NEW_VIDEO_DEVICE: HRESULT = 0x80041001;
pub const MF_E_DXGI_VIDEO_DEVICE_LOCKED: HRESULT = 0x80041002;
pub const MF_1_BYTE_ALIGNMENT: DWORD = 0x00000000;
pub const MF_2_BYTE_ALIGNMENT: DWORD = 0x00000001;
pub const MF_4_BYTE_ALIGNMENT: DWORD = 0x00000003;
pub const MF_8_BYTE_ALIGNMENT: DWORD = 0x00000007;
pub const MF_16_BYTE_ALIGNMENT: DWORD = 0x0000000f;
pub const MF_32_BYTE_ALIGNMENT: DWORD = 0x0000001f;
pub const MF_64_BYTE_ALIGNMENT: DWORD = 0x0000003f;
pub const MF_128_BYTE_ALIGNMENT: DWORD = 0x0000007f;
pub const MF_256_BYTE_ALIGNMENT: DWORD = 0x000000ff;
pub const MF_512_BYTE_ALIGNMENT: DWORD = 0x000001ff;
pub const MF_1024_BYTE_ALIGNMENT: DWORD = 0x000003ff;
pub const MF_2048_BYTE_ALIGNMENT: DWORD = 0x000007ff;
pub const MF_4096_BYTE_ALIGNMENT: DWORD = 0x00000fff;
pub const MF_8192_BYTE_ALIGNMENT: DWORD = 0x00001fff;
extern "system" {
    pub fn MFCreateAlignedMemoryBuffer(
        cbMaxLength: DWORD,
        cbAlignment: DWORD,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT;
}
DEFINE_GUID! {MR_BUFFER_SERVICE,
0xa562248c, 0x9ac6, 0x4ffc, 0x9f, 0xba, 0x3a, 0xf8, 0xf8, 0xad, 0x1a, 0x4d}
extern "system" {
    pub fn MFCreateMediaEvent(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pvValue: *const PROPVARIANT,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT;
    pub fn MFCreateEventQueue(ppMediaEventQueue: *mut *mut IMFMediaEventQueue) -> HRESULT;
}
DEFINE_GUID! {MF_EVENT_SESSIONCAPS,
0x7e5ebcd0, 0x11b8, 0x4abe, 0xaf, 0xad, 0x10, 0xf6, 0x59, 0x9a, 0x7f, 0x42}
DEFINE_GUID! {MF_EVENT_SESSIONCAPS_DELTA,
0x7e5ebcd1, 0x11b8, 0x4abe, 0xaf, 0xad, 0x10, 0xf6, 0x59, 0x9a, 0x7f, 0x42}
pub const MFSESSIONCAP_START: DWORD = 00000001;
pub const MFSESSIONCAP_SEEK: DWORD = 00000002;
pub const MFSESSIONCAP_PAUSE: DWORD = 00000004;
pub const MFSESSIONCAP_RATE_FORWARD: DWORD = 00000010;
pub const MFSESSIONCAP_RATE_REVERSE: DWORD = 00000020;
pub const MFSESSIONCAP_DOES_NOT_USE_NETWORK: DWORD = 00000040;
ENUM! {enum MF_TOPOSTATUS {
    MF_TOPOSTATUS_INVALID = 0,
    MF_TOPOSTATUS_READY = 100,
    MF_TOPOSTATUS_STARTED_SOURCE = 200,
    MF_TOPOSTATUS_DYNAMIC_CHANGED = 210,
    MF_TOPOSTATUS_SINK_SWITCHED = 300,
    MF_TOPOSTATUS_ENDED = 400,
}}
DEFINE_GUID! {MF_EVENT_TOPOLOGY_STATUS,
0x30c5018d, 0x9a53, 0x454b, 0xad, 0x9e, 0x6d, 0x5f, 0x8f, 0xa7, 0xc4, 0x3b}
DEFINE_GUID! {MF_EVENT_START_PRESENTATION_TIME,
0x5ad914d0, 0x9b45, 0x4a8d, 0xa2, 0xc0, 0x81, 0xd1, 0xe5, 0xb, 0xfb, 0x7}
DEFINE_GUID! {MF_EVENT_PRESENTATION_TIME_OFFSET,
0x5ad914d1, 0x9b45, 0x4a8d, 0xa2, 0xc0, 0x81, 0xd1, 0xe5, 0xb, 0xfb, 0x7}
DEFINE_GUID! {MF_EVENT_START_PRESENTATION_TIME_AT_OUTPUT,
0x5ad914d2, 0x9b45, 0x4a8d, 0xa2, 0xc0, 0x81, 0xd1, 0xe5, 0xb, 0xfb, 0x7}
DEFINE_GUID! {MF_EVENT_SOURCE_FAKE_START,
0xa8cc55a7, 0x6b31, 0x419f, 0x84, 0x5d, 0xff, 0xb3, 0x51, 0xa2, 0x43, 0x4b}
DEFINE_GUID! {MF_EVENT_SOURCE_PROJECTSTART,
0xa8cc55a8, 0x6b31, 0x419f, 0x84, 0x5d, 0xff, 0xb3, 0x51, 0xa2, 0x43, 0x4b}
DEFINE_GUID! {MF_EVENT_SOURCE_ACTUAL_START,
0xa8cc55a9, 0x6b31, 0x419f, 0x84, 0x5d, 0xff, 0xb3, 0x51, 0xa2, 0x43, 0x4b}
DEFINE_GUID! {MF_EVENT_SOURCE_TOPOLOGY_CANCELED,
0xdb62f650, 0x9a5e, 0x4704, 0xac, 0xf3, 0x56, 0x3b, 0xc6, 0xa7, 0x33, 0x64}
DEFINE_GUID! {MF_EVENT_SOURCE_CHARACTERISTICS,
0x47db8490, 0x8b22, 0x4f52, 0xaf, 0xda, 0x9c, 0xe1, 0xb2, 0xd3, 0xcf, 0xa8}
DEFINE_GUID! {MF_EVENT_SOURCE_CHARACTERISTICS_OLD,
0x47db8491, 0x8b22, 0x4f52, 0xaf, 0xda, 0x9c, 0xe1, 0xb2, 0xd3, 0xcf, 0xa8}
DEFINE_GUID! {MF_EVENT_DO_THINNING,
0x321ea6fb, 0xdad9, 0x46e4, 0xb3, 0x1d, 0xd2, 0xea, 0xe7, 0x9, 0xe, 0x30}
DEFINE_GUID! {MF_EVENT_SCRUBSAMPLE_TIME,
0x9ac712b3, 0xdcb8, 0x44d5, 0x8d, 0xc, 0x37, 0x45, 0x5a, 0x27, 0x82, 0xe3}
DEFINE_GUID! {MF_EVENT_OUTPUT_NODE,
0x830f1a8b, 0xc060, 0x46dd, 0xa8, 0x01, 0x1c, 0x95, 0xde, 0xc9, 0xb1, 0x07}
DEFINE_GUID! {MF_EVENT_MFT_INPUT_STREAM_ID,
0xf29c2cca, 0x7ae6, 0x42d2, 0xb2, 0x84, 0xbf, 0x83, 0x7c, 0xc8, 0x74, 0xe2}
DEFINE_GUID! {MF_EVENT_MFT_CONTEXT,
0xb7cd31f1, 0x899e, 0x4b41, 0x80, 0xc9, 0x26, 0xa8, 0x96, 0xd3, 0x29, 0x77}
DEFINE_GUID! {MF_EVENT_STREAM_METADATA_KEYDATA,
0xcd59a4a1, 0x4a3b, 0x4bbd, 0x86, 0x65, 0x72, 0xa4, 0xf, 0xbe, 0xa7, 0x76}
DEFINE_GUID! {MF_EVENT_STREAM_METADATA_CONTENT_KEYIDS,
0x5063449d, 0xcc29, 0x4fc6, 0xa7, 0x5a, 0xd2, 0x47, 0xb3, 0x5a, 0xf8, 0x5c}
DEFINE_GUID! {MF_EVENT_STREAM_METADATA_SYSTEMID,
0x1ea2ef64, 0xba16, 0x4a36, 0x87, 0x19, 0xfe, 0x75, 0x60, 0xba, 0x32, 0xad}
extern "system" {
    pub fn MFCreateSample(ppIMFSample: *mut *mut IMFSample) -> HRESULT;
}
DEFINE_GUID! {MFSampleExtension_MaxDecodeFrameSize,
0xd3cc654f, 0xf9f3, 0x4a13, 0x88, 0x9f, 0xf0, 0x4e, 0xb2, 0xb5, 0xb9, 0x57}
DEFINE_GUID! {MFSampleExtension_AccumulatedNonRefPicPercent,
0x79ea74df, 0xa740, 0x445b, 0xbc, 0x98, 0xc9, 0xed, 0x1f, 0x26, 0xe, 0xee}
DEFINE_GUID! {MFSampleExtension_Encryption_ProtectionScheme,
0xd054d096, 0x28bb, 0x45da, 0x87, 0xec, 0x74, 0xf3, 0x51, 0x87, 0x14, 0x6}
ENUM! {enum MFSampleEncryptionProtectionScheme {
    MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_NONE = 0,
    MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CTR = 1,
    MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CBC = 2,
}}
DEFINE_GUID! {MFSampleExtension_Encryption_CryptByteBlock,
0x9d84289b, 0xc7f, 0x4713, 0xab, 0x95, 0x10, 0x8a, 0xb4, 0x2a, 0xd8, 0x1}
DEFINE_GUID! {MFSampleExtension_Encryption_SkipByteBlock,
0xd550548, 0x8317, 0x4ab1, 0x84, 0x5f, 0xd0, 0x63, 0x6, 0xe2, 0x93, 0xe3}
DEFINE_GUID! {MFSampleExtension_Encryption_SubSample_Mapping,
0x8444F27A, 0x69A1, 0x48DA, 0xBD, 0x08, 0x11, 0xCE, 0xF3, 0x68, 0x30, 0xD2}
DEFINE_GUID! {MFSampleExtension_Encryption_ClearSliceHeaderData,
0x5509a4f4, 0x320d, 0x4e6c, 0x8d, 0x1a, 0x94, 0xc6, 0x6d, 0xd2, 0xc, 0xb0}
DEFINE_GUID! {MFSampleExtension_Encryption_HardwareProtection_KeyInfoID,
0x8cbfcceb, 0x94a5, 0x4de1, 0x82, 0x31, 0xa8, 0x5e, 0x47, 0xcf, 0x81, 0xe7}
DEFINE_GUID! {MFSampleExtension_Encryption_HardwareProtection_KeyInfo,
0xb2372080, 0x455b, 0x4dd7, 0x99, 0x89, 0x1a, 0x95, 0x57, 0x84, 0xb7, 0x54}
DEFINE_GUID! {MFSampleExtension_Encryption_HardwareProtection_VideoDecryptorContext,
0x693470c8, 0xe837, 0x47a0, 0x88, 0xcb, 0x53, 0x5b, 0x90, 0x5e, 0x35, 0x82}
DEFINE_GUID! {MFSampleExtension_Encryption_Opaque_Data,
0x224d77e5, 0x1391, 0x4ffb, 0x9f, 0x41, 0xb4, 0x32, 0xf6, 0x8c, 0x61, 0x1d}
DEFINE_GUID! {MFSampleExtension_NALULengthInfo,
0x19124E7C, 0xAD4B, 0x465F, 0xBB, 0x18, 0x20, 0x18, 0x62, 0x87, 0xB6, 0xAF}
DEFINE_GUID! {MFSampleExtension_Encryption_ResumeVideoOutput,
0xa435aba5, 0xafde, 0x4cf5, 0xbc, 0x1c, 0xf6, 0xac, 0xaf, 0x13, 0x94, 0x9d}
DEFINE_GUID! {MFSampleExtension_Encryption_NALUTypes,
0xb0f067c7, 0x714c, 0x416c, 0x8d, 0x59, 0x5f, 0x4d, 0xdf, 0x89, 0x13, 0xb6}
DEFINE_GUID! {MFSampleExtension_Encryption_SPSPPSData,
0xaede0fa2, 0xe0c, 0x453c, 0xb7, 0xf3, 0xde, 0x86, 0x93, 0x36, 0x4d, 0x11}
DEFINE_GUID! {MFSampleExtension_Encryption_SEIData,
0x3cf0e972, 0x4542, 0x4687, 0x99, 0x99, 0x58, 0x5f, 0x56, 0x5f, 0xba, 0x7d}
DEFINE_GUID! {MFSampleExtension_Encryption_HardwareProtection,
0x9a2b2d2b, 0x8270, 0x43e3, 0x84, 0x48, 0x99, 0x4f, 0x42, 0x6e, 0x88, 0x86}
DEFINE_GUID! {MFSampleExtension_CleanPoint,
0x9cdf01d8, 0xa0f0, 0x43ba, 0xb0, 0x77, 0xea, 0xa0, 0x6c, 0xbd, 0x72, 0x8a}
DEFINE_GUID! {MFSampleExtension_Discontinuity,
0x9cdf01d9, 0xa0f0, 0x43ba, 0xb0, 0x77, 0xea, 0xa0, 0x6c, 0xbd, 0x72, 0x8a}
DEFINE_GUID! {MFSampleExtension_Token,
0x8294da66, 0xf328, 0x4805, 0xb5, 0x51, 0x00, 0xde, 0xb4, 0xc5, 0x7a, 0x61}
DEFINE_GUID! {MFSampleExtension_ClosedCaption_CEA708,
0x26f09068, 0xe744, 0x47dc, 0xaa, 0x03, 0xdb, 0xf2, 0x04, 0x03, 0xbd, 0xe6}
DEFINE_GUID! {MFSampleExtension_DecodeTimestamp,
0x73a954d4, 0x9e2, 0x4861, 0xbe, 0xfc, 0x94, 0xbd, 0x97, 0xc0, 0x8e, 0x6e}
DEFINE_GUID! {MFSampleExtension_VideoEncodeQP,
0xb2efe478, 0xf979, 0x4c66, 0xb9, 0x5e, 0xee, 0x2b, 0x82, 0xc8, 0x2f, 0x36}
DEFINE_GUID! {MFSampleExtension_VideoEncodePictureType,
0x973704e6, 0xcd14, 0x483c, 0x8f, 0x20, 0xc9, 0xfc, 0x9, 0x28, 0xba, 0xd5}
DEFINE_GUID! {MFSampleExtension_FrameCorruption,
0xb4dd4a8c, 0xbeb, 0x44c4, 0x8b, 0x75, 0xb0, 0x2b, 0x91, 0x3b, 0x4, 0xf0}
DEFINE_GUID! {MFSampleExtension_DirtyRects,
0x9ba70225, 0xb342, 0x4e97, 0x91, 0x26, 0x0b, 0x56, 0x6a, 0xb7, 0xea, 0x7e}
DEFINE_GUID! {MFSampleExtension_MoveRegions,
0xe2a6c693, 0x3a8b, 0x4b8d, 0x95, 0xd0, 0xf6, 0x02, 0x81, 0xa1, 0x2f, 0xb7}
STRUCT! {struct MOVE_RECT {
    SourcePoint: POINT,
    DestRect: RECT,
}}
STRUCT! {struct DIRTYRECT_INFO {
    FrameNumber: UINT,
    NumDirtyRects: UINT,
    DirtyRects: [RECT; 1],
}}
STRUCT! {struct MOVEREGION_INFO {
    FrameNumber: UINT,
    NumMoveRegions: UINT,
    MoveRegions: [MOVE_RECT; 1],
}}
DEFINE_GUID! {MFSampleExtension_HDCP_OptionalHeader,
0x9a2e7390, 0x121f, 0x455f, 0x83, 0x76, 0xc9, 0x74, 0x28, 0xe0, 0xb5, 0x40}
DEFINE_GUID! {MFSampleExtension_HDCP_FrameCounter,
0x9d389c60, 0xf507, 0x4aa6, 0xa4, 0xa, 0x71, 0x2, 0x7a, 0x2, 0xf3, 0xde}
DEFINE_GUID! {MFSampleExtension_HDCP_StreamID,
0x177e5d74, 0xc370, 0x4a7a, 0x95, 0xa2, 0x36, 0x83, 0x3c, 0x01, 0xd0, 0xaf}
DEFINE_GUID! {MFSampleExtension_Timestamp,
0x1e436999, 0x69be, 0x4c7a, 0x93, 0x69, 0x70, 0x06, 0x8c, 0x02, 0x60, 0xcb}
DEFINE_GUID! {MFSampleExtension_RepeatFrame,
0x88be738f, 0x711, 0x4f42, 0xb4, 0x58, 0x34, 0x4a, 0xed, 0x42, 0xec, 0x2f}
DEFINE_GUID! {MFT_ENCODER_ERROR,
0xc8d1eda4, 0x98e4, 0x41d5, 0x92, 0x97, 0x44, 0xf5, 0x38, 0x52, 0xf9, 0x0e}
DEFINE_GUID! {MFT_GFX_DRIVER_VERSION_ID_Attribute,
0xf34b9093, 0x05e0, 0x4b16, 0x99, 0x3d, 0x3e, 0x2a, 0x2c, 0xde, 0x6a, 0xd3}
DEFINE_GUID! {MFSampleExtension_DescrambleData,
0x43483be6, 0x4903, 0x4314, 0xb0, 0x32, 0x29, 0x51, 0x36, 0x59, 0x36, 0xfc}
DEFINE_GUID! {MFSampleExtension_SampleKeyID,
0x9ed713c8, 0x9b87, 0x4b26, 0x82, 0x97, 0xa9, 0x3b, 0x0c, 0x5a, 0x8a, 0xcc}
DEFINE_GUID! {MFSampleExtension_GenKeyFunc,
0x441ca1ee, 0x6b1f, 0x4501, 0x90, 0x3a, 0xde, 0x87, 0xdf, 0x42, 0xf6, 0xed}
DEFINE_GUID! {MFSampleExtension_GenKeyCtx,
0x188120cb, 0xd7da, 0x4b59, 0x9b, 0x3e, 0x92, 0x52, 0xfd, 0x37, 0x30, 0x1c}
DEFINE_GUID! {MFSampleExtension_PacketCrossOffsets,
0x2789671d, 0x389f, 0x40bb, 0x90, 0xd9, 0xc2, 0x82, 0xf7, 0x7f, 0x9a, 0xbd}
DEFINE_GUID! {MFSampleExtension_Encryption_SampleID,
0x6698b84e, 0x0afa, 0x4330, 0xae, 0xb2, 0x1c, 0x0a, 0x98, 0xd7, 0xa4, 0x4d}
DEFINE_GUID! {MFSampleExtension_Encryption_KeyID,
0x76376591, 0x795f, 0x4da1, 0x86, 0xed, 0x9d, 0x46, 0xec, 0xa1, 0x09, 0xa9}
DEFINE_GUID! {MFSampleExtension_Content_KeyID,
0xc6c7f5b0, 0xacca, 0x415b, 0x87, 0xd9, 0x10, 0x44, 0x14, 0x69, 0xef, 0xc6}
DEFINE_GUID! {MFSampleExtension_Encryption_SubSampleMappingSplit,
0xfe0254b9, 0x2aa5, 0x4edc, 0x99, 0xf7, 0x17, 0xe8, 0x9d, 0xbf, 0x91, 0x74}
DEFINE_GUID! {MFSampleExtension_Interlaced,
0xb1d5830a, 0xdeb8, 0x40e3, 0x90, 0xfa, 0x38, 0x99, 0x43, 0x71, 0x64, 0x61}
DEFINE_GUID! {MFSampleExtension_BottomFieldFirst,
0x941ce0a3, 0x6ae3, 0x4dda, 0x9a, 0x08, 0xa6, 0x42, 0x98, 0x34, 0x06, 0x17}
DEFINE_GUID! {MFSampleExtension_RepeatFirstField,
0x304d257c, 0x7493, 0x4fbd, 0xb1, 0x49, 0x92, 0x28, 0xde, 0x8d, 0x9a, 0x99}
DEFINE_GUID! {MFSampleExtension_SingleField,
0x9d85f816, 0x658b, 0x455a, 0xbd, 0xe0, 0x9f, 0xa7, 0xe1, 0x5a, 0xb8, 0xf9}
DEFINE_GUID! {MFSampleExtension_DerivedFromTopField,
0x6852465a, 0xae1c, 0x4553, 0x8e, 0x9b, 0xc3, 0x42, 0x0f, 0xcb, 0x16, 0x37}
DEFINE_GUID! {MFSampleExtension_MeanAbsoluteDifference,
0x1cdbde11, 0x08b4, 0x4311, 0xa6, 0xdd, 0x0f, 0x9f, 0x37, 0x19, 0x07, 0xaa}
DEFINE_GUID! {MFSampleExtension_LongTermReferenceFrameInfo,
0x9154733f, 0xe1bd, 0x41bf, 0x81, 0xd3, 0xfc, 0xd9, 0x18, 0xf7, 0x13, 0x32}
STRUCT! {struct ROI_AREA {
    rect: RECT,
    QPDelta: INT32,
}}
pub type PROI_AREA = *mut ROI_AREA;
DEFINE_GUID! {MFSampleExtension_ROIRectangle,
0x3414a438, 0x4998, 0x4d2c, 0xbe, 0x82, 0xbe, 0x3c, 0xa0, 0xb2, 0x4d, 0x43}
DEFINE_GUID! {MFSampleExtension_LastSlice,
0x2b5d5457, 0x5547, 0x4f07, 0xb8, 0xc8, 0xb4, 0xa3, 0xa9, 0xa1, 0xda, 0xac}
pub const MACROBLOCK_FLAG_SKIP: UINT32 = 00000001;
pub const MACROBLOCK_FLAG_DIRTY: UINT32 = 00000002;
pub const MACROBLOCK_FLAG_MOTION: UINT32 = 00000004;
pub const MACROBLOCK_FLAG_VIDEO: UINT32 = 00000008;
pub const MACROBLOCK_FLAG_HAS_MOTION_VECTOR: UINT32 = 00000010;
pub const MACROBLOCK_FLAG_HAS_QP: UINT32 = 00000020;
STRUCT! {struct MACROBLOCK_DATA {
    flags: UINT32,
    motionVectorX: INT16,
    motionVectorY: INT16,
    QPDelta: INT32,
}}
DEFINE_GUID! {MFSampleExtension_FeatureMap,
0xa032d165, 0x46fc, 0x400a, 0xb4, 0x49, 0x49, 0xde, 0x53, 0xe6, 0x2a, 0x6e}
DEFINE_GUID! {MFSampleExtension_ChromaOnly,
0x1eb9179c, 0xa01f, 0x4845, 0x8c, 0x04, 0x0e, 0x65, 0xa2, 0x6e, 0xb0, 0x4f}
DEFINE_GUID! {MFSampleExtension_PhotoThumbnail,
0x74BBC85C, 0xC8BB, 0x42DC, 0xB5, 0x86, 0xDA, 0x17, 0xFF, 0xD3, 0x5D, 0xCC}
DEFINE_GUID! {MFSampleExtension_PhotoThumbnailMediaType,
0x61AD5420, 0xEBF8, 0x4143, 0x89, 0xAF, 0x6B, 0xF2, 0x5F, 0x67, 0x2D, 0xEF}
DEFINE_GUID! {MFSampleExtension_CaptureMetadata,
0x2EBE23A8, 0xFAF5, 0x444A, 0xA6, 0xA2, 0xEB, 0x81, 0x08, 0x80, 0xAB, 0x5D}
DEFINE_GUID! {MFSampleExtension_MDLCacheCookie,
0x5F002AF9, 0xD8F9, 0x41A3, 0xB6, 0xC3, 0xA2, 0xAD, 0x43, 0xF6, 0x47, 0xAD}
DEFINE_GUID! {MF_CAPTURE_METADATA_PHOTO_FRAME_FLASH,
0x0F9DD6C6, 0x6003, 0x45D8, 0xBD, 0x59, 0xF1, 0xF5, 0x3E, 0x3D, 0x04, 0xE8}
DEFINE_GUID! {MF_CAPTURE_METADATA_FRAME_RAWSTREAM,
0x9252077B, 0x2680, 0x49B9, 0xAE, 0x02, 0xB1, 0x90, 0x75, 0x97, 0x3B, 0x70}
DEFINE_GUID! {MF_CAPTURE_METADATA_FOCUSSTATE,
0xa87ee154, 0x997f, 0x465d, 0xb9, 0x1f, 0x29, 0xd5, 0x3b, 0x98, 0x2b, 0x88}
DEFINE_GUID! {MF_CAPTURE_METADATA_REQUESTED_FRAME_SETTING_ID,
0xbb3716d9, 0x8a61, 0x47a4, 0x81, 0x97, 0x45, 0x9c, 0x7f, 0xf1, 0x74, 0xd5}
DEFINE_GUID! {MF_CAPTURE_METADATA_EXPOSURE_TIME,
0x16b9ae99, 0xcd84, 0x4063, 0x87, 0x9d, 0xa2, 0x8c, 0x76, 0x33, 0x72, 0x9e}
DEFINE_GUID! {MF_CAPTURE_METADATA_EXPOSURE_COMPENSATION,
0xd198aa75, 0x4b62, 0x4345, 0xab, 0xf3, 0x3c, 0x31, 0xfa, 0x12, 0xc2, 0x99}
DEFINE_GUID! {MF_CAPTURE_METADATA_ISO_SPEED,
0xe528a68f, 0xb2e3, 0x44fe, 0x8b, 0x65, 0x7, 0xbf, 0x4b, 0x5a, 0x13, 0xff}
DEFINE_GUID! {MF_CAPTURE_METADATA_LENS_POSITION,
0xb5fc8e86, 0x11d1, 0x4e70, 0x81, 0x9b, 0x72, 0x3a, 0x89, 0xfa, 0x45, 0x20}
DEFINE_GUID! {MF_CAPTURE_METADATA_SCENE_MODE,
0x9cc3b54d, 0x5ed3, 0x4bae, 0xb3, 0x88, 0x76, 0x70, 0xae, 0xf5, 0x9e, 0x13}
DEFINE_GUID! {MF_CAPTURE_METADATA_FLASH,
0x4a51520b, 0xfb36, 0x446c, 0x9d, 0xf2, 0x68, 0x17, 0x1b, 0x9a, 0x3, 0x89}
DEFINE_GUID! {MF_CAPTURE_METADATA_FLASH_POWER,
0x9c0e0d49, 0x205, 0x491a, 0xbc, 0x9d, 0x2d, 0x6e, 0x1f, 0x4d, 0x56, 0x84}
DEFINE_GUID! {MF_CAPTURE_METADATA_WHITEBALANCE,
0xc736fd77, 0xfb9, 0x4e2e, 0x97, 0xa2, 0xfc, 0xd4, 0x90, 0x73, 0x9e, 0xe9}
DEFINE_GUID! {MF_CAPTURE_METADATA_ZOOMFACTOR,
0xe50b0b81, 0xe501, 0x42c2, 0xab, 0xf2, 0x85, 0x7e, 0xcb, 0x13, 0xfa, 0x5c}
DEFINE_GUID! {MF_CAPTURE_METADATA_FACEROIS,
0x864f25a6, 0x349f, 0x46b1, 0xa3, 0xe, 0x54, 0xcc, 0x22, 0x92, 0x8a, 0x47}
DEFINE_GUID! {MF_CAPTURE_METADATA_FACEROITIMESTAMPS,
0xe94d50cc, 0x3da0, 0x44d4, 0xbb, 0x34, 0x83, 0x19, 0x8a, 0x74, 0x18, 0x68}
DEFINE_GUID! {MF_CAPTURE_METADATA_FACEROICHARACTERIZATIONS,
0xb927a1a8, 0x18ef, 0x46d3, 0xb3, 0xaf, 0x69, 0x37, 0x2f, 0x94, 0xd9, 0xb2}
DEFINE_GUID! {MF_CAPTURE_METADATA_ISO_GAINS,
0x5802ac9, 0xe1d, 0x41c7, 0xa8, 0xc8, 0x7e, 0x73, 0x69, 0xf8, 0x4e, 0x1e}
DEFINE_GUID! {MF_CAPTURE_METADATA_SENSORFRAMERATE,
0xdb51357e, 0x9d3d, 0x4962, 0xb0, 0x6d, 0x7, 0xce, 0x65, 0xd, 0x9a, 0xa}
DEFINE_GUID! {MF_CAPTURE_METADATA_WHITEBALANCE_GAINS,
0xe7570c8f, 0x2dcb, 0x4c7c, 0xaa, 0xce, 0x22, 0xec, 0xe7, 0xcc, 0xe6, 0x47}
DEFINE_GUID! {MF_CAPTURE_METADATA_HISTOGRAM,
0x85358432, 0x2ef6, 0x4ba9, 0xa3, 0xfb, 0x6, 0xd8, 0x29, 0x74, 0xb8, 0x95}
DEFINE_GUID! {MF_CAPTURE_METADATA_EXIF,
0x2e9575b8, 0x8c31, 0x4a02, 0x85, 0x75, 0x42, 0xb1, 0x97, 0xb7, 0x15, 0x92}
DEFINE_GUID! {MF_CAPTURE_METADATA_FRAME_ILLUMINATION,
0x6D688FFC, 0x63D3, 0x46FE, 0xBA, 0xDA, 0x5B, 0x94, 0x7D, 0xB0, 0xD0, 0x80}
DEFINE_GUID! {MF_CAPTURE_METADATA_UVC_PAYLOADHEADER,
0xf9f88a87, 0xe1dd, 0x441e, 0x95, 0xcb, 0x42, 0xe2, 0x1a, 0x64, 0xf1, 0xd9}
DEFINE_GUID! {MFSampleExtension_Depth_MinReliableDepth,
0x5f8582b2, 0xe36b, 0x47c8, 0x9b, 0x87, 0xfe, 0xe1, 0xca, 0x72, 0xc5, 0xb0}
DEFINE_GUID! {MFSampleExtension_Depth_MaxReliableDepth,
0xe45545d1, 0x1f0f, 0x4a32, 0xa8, 0xa7, 0x61, 0x1, 0xa2, 0x4e, 0xa8, 0xbe}
DEFINE_GUID! {MF_CAPTURE_METADATA_FIRST_SCANLINE_START_TIME_QPC,
0x6a2c49f1, 0xe052, 0x46b6, 0xb2, 0xd9, 0x73, 0xc1, 0x55, 0x87, 0x09, 0xaf}
DEFINE_GUID! {MF_CAPTURE_METADATA_LAST_SCANLINE_END_TIME_QPC,
0xdccadecb, 0xc4d4, 0x400d, 0xb4, 0x18, 0x10, 0xe8, 0x85, 0x25, 0xe1, 0xf6}
DEFINE_GUID! {MF_CAPTURE_METADATA_SCANLINE_TIME_QPC_ACCURACY,
0x4cd79c51, 0xf765, 0x4b09, 0xb1, 0xe1, 0x27, 0xd1, 0xf7, 0xeb, 0xea, 0x09}
DEFINE_GUID! {MF_CAPTURE_METADATA_SCANLINE_DIRECTION,
0x6496a3ba, 0x1907, 0x49e6, 0xb0, 0xc3, 0x12, 0x37, 0x95, 0xf3, 0x80, 0xa9}
pub const MFCAPTURE_METADATA_SCAN_RIGHT_LEFT: UINT32 = 00000001;
pub const MFCAPTURE_METADATA_SCAN_BOTTOM_TOP: UINT32 = 00000002;
pub const MFCAPTURE_METADATA_SCANLINE_VERTICAL: UINT32 = 00000004;
STRUCT! {struct FaceRectInfoBlobHeader {
    Size: ULONG,
    Count: ULONG,
}}
STRUCT! {struct FaceRectInfo {
    Region: RECT,
    confidenceLevel: LONG,
}}
STRUCT! {struct FaceCharacterizationBlobHeader {
    Size: ULONG,
    Count: ULONG,
}}
STRUCT! {struct FaceCharacterization {
    BlinkScoreLeft: ULONG,
    BlinkScoreRight: ULONG,
    FacialExpression: ULONG,
    FacialExpressionScore: ULONG,
}}
pub const MF_METADATAFACIALEXPRESSION_SMILE: ULONG = 0x00000001;
STRUCT! {struct CapturedMetadataExposureCompensation {
    Flags: UINT64,
    Value: INT32,
}}
STRUCT! {struct CapturedMetadataISOGains {
    AnalogGain: FLOAT,
    DigitalGain: FLOAT,
}}
STRUCT! {struct CapturedMetadataWhiteBalanceGains {
    R: FLOAT,
    G: FLOAT,
    B: FLOAT,
}}
STRUCT! {struct MetadataTimeStamps {
    Flags: ULONG,
    Device: LONGLONG,
    Presentation: LONGLONG,
}}
STRUCT! {struct HistogramGrid {
    Width: ULONG,
    Height: ULONG,
    Region: RECT,
}}
STRUCT! {struct HistogramBlobHeader {
    Size: ULONG,
    Histograms: ULONG,
}}
STRUCT! {struct HistogramDataHeader {
    Size: ULONG,
    ChannelMask: ULONG,
    Linear: ULONG,
}}
pub const MF_HISTOGRAM_CHANNEL_Y: ULONG = 00000001;
pub const MF_HISTOGRAM_CHANNEL_R: ULONG = 00000002;
pub const MF_HISTOGRAM_CHANNEL_G: ULONG = 00000004;
pub const MF_HISTOGRAM_CHANNEL_B: ULONG = 00000008;
pub const MF_HISTOGRAM_CHANNEL_Cb: ULONG = 00000010;
pub const MF_HISTOGRAM_CHANNEL_Cr: ULONG = 00000020;
extern "system" {
    pub fn MFCreateAttributes(
        ppMFAttributes: *mut *mut IMFAttributes,
        cInitialSize: UINT32,
    ) -> HRESULT;
    pub fn MFInitAttributesFromBlob(
        pAttributes: *mut IMFAttributes,
        pBuf: *const UINT8,
        cbBufSize: UINT,
    ) -> HRESULT;
    pub fn MFGetAttributesAsBlobSize(
        pAttributes: *mut IMFAttributes,
        pcbBufSize: *mut UINT32,
    ) -> HRESULT;
    pub fn MFGetAttributesAsBlob(
        pAttributes: *mut IMFAttributes,
        pBuf: *mut UINT8,
        cbBufSize: UINT,
    ) -> HRESULT;
}
DEFINE_GUID! {MFT_CATEGORY_VIDEO_DECODER,
0xd6c02d4b, 0x6833, 0x45b4, 0x97, 0x1a, 0x05, 0xa4, 0xb0, 0x4b, 0xab, 0x91}
DEFINE_GUID! {MFT_CATEGORY_VIDEO_ENCODER,
0xf79eac7d, 0xe545, 0x4387, 0xbd, 0xee, 0xd6, 0x47, 0xd7, 0xbd, 0xe4, 0x2a}
DEFINE_GUID! {MFT_CATEGORY_VIDEO_EFFECT,
0x12e17c21, 0x532c, 0x4a6e, 0x8a, 0x1c, 0x40, 0x82, 0x5a, 0x73, 0x63, 0x97}
DEFINE_GUID! {MFT_CATEGORY_MULTIPLEXER,
0x059c561e, 0x05ae, 0x4b61, 0xb6, 0x9d, 0x55, 0xb6, 0x1e, 0xe5, 0x4a, 0x7b}
DEFINE_GUID! {MFT_CATEGORY_DEMULTIPLEXER,
0xa8700a7a, 0x939b, 0x44c5, 0x99, 0xd7, 0x76, 0x22, 0x6b, 0x23, 0xb3, 0xf1}
DEFINE_GUID! {MFT_CATEGORY_AUDIO_DECODER,
0x9ea73fb4, 0xef7a, 0x4559, 0x8d, 0x5d, 0x71, 0x9d, 0x8f, 0x04, 0x26, 0xc7}
DEFINE_GUID! {MFT_CATEGORY_AUDIO_ENCODER,
0x91c64bd0, 0xf91e, 0x4d8c, 0x92, 0x76, 0xdb, 0x24, 0x82, 0x79, 0xd9, 0x75}
DEFINE_GUID! {MFT_CATEGORY_AUDIO_EFFECT,
0x11064c48, 0x3648, 0x4ed0, 0x93, 0x2e, 0x05, 0xce, 0x8a, 0xc8, 0x11, 0xb7}
DEFINE_GUID! {MFT_CATEGORY_VIDEO_PROCESSOR,
0x302ea3fc, 0xaa5f, 0x47f9, 0x9f, 0x7a, 0xc2, 0x18, 0x8b, 0xb1, 0x63, 0x2}
DEFINE_GUID! {MFT_CATEGORY_OTHER,
0x90175d57, 0xb7ea, 0x4901, 0xae, 0xb3, 0x93, 0x3a, 0x87, 0x47, 0x75, 0x6f}
DEFINE_GUID! {MFT_CATEGORY_ENCRYPTOR,
0xb0c687be, 0x01cd, 0x44b5, 0xb8, 0xb2, 0x7c, 0x1d, 0x7e, 0x05, 0x8b, 0x1f}
DEFINE_GUID! {MFT_CATEGORY_VIDEO_RENDERER_EFFECT,
0x145cd8b4, 0x92f4, 0x4b23, 0x8a, 0xe7, 0xe0, 0xdf, 0x6, 0xc2, 0xda, 0x95}
extern "system" {
    pub fn MFTRegister(
        clsidMFT: CLSID,
        guidCategory: GUID,
        pszName: LPWSTR,
        Flags: UINT32,
        cInputTypes: UINT32,
        pInputTypes: *mut MFT_REGISTER_TYPE_INFO,
        cOutputTypes: UINT32,
        pOutputTypes: *mut MFT_REGISTER_TYPE_INFO,
        pAttributes: *mut IMFAttributes,
    ) -> HRESULT;
    pub fn MFTUnregister(clsidMFT: CLSID) -> HRESULT;
    pub fn MFTRegisterLocal(
        pClassFactory: *mut IClassFactory,
        guidCategory: REFGUID,
        pszName: LPCWSTR,
        Flags: UINT32,
        cInputTypes: UINT32,
        pInputTypes: *mut MFT_REGISTER_TYPE_INFO,
        cOutputTypes: UINT32,
        pOutputTypes: *mut MFT_REGISTER_TYPE_INFO,
    ) -> HRESULT;
    pub fn MFTUnregisterLocal(pClassFactory: *mut IClassFactory) -> HRESULT;
    pub fn MFTRegisterLocalByCLSID(
        clisd /* sic */: CLSID,
        guidCategory: REFGUID,
        pszName: LPCWSTR,
        Flags: UINT32,
        cInputTypes: UINT32,
        pInputTypes: *mut MFT_REGISTER_TYPE_INFO,
        cOutputTypes: UINT32,
        pOutputTypes: *mut MFT_REGISTER_TYPE_INFO,
    ) -> HRESULT;
    pub fn MFTUnregisterLocalByCLSID(clsidMFT: CLSID) -> HRESULT;
    pub fn MFTEnum(
        guidCategory: GUID,
        Flags: UINT32,
        pInputType: *mut MFT_REGISTER_TYPE_INFO,
        pOutputType: *mut MFT_REGISTER_TYPE_INFO,
        pAttributes: *mut IMFAttributes,
        ppclsidMFT: *mut *mut CLSID,
        pcMFTs: *mut UINT32,
    ) -> HRESULT;
}
ENUM! {enum MFT_ENUM_FLAG {
    MFT_ENUM_FLAG_SYNCMFT = 0x00000001,
    MFT_ENUM_FLAG_ASYNCMFT = 0x00000002,
    MFT_ENUM_FLAG_HARDWARE = 0x00000004,
    MFT_ENUM_FLAG_FIELDOFUSE = 0x00000008,
    MFT_ENUM_FLAG_LOCALMFT = 0x00000010,
    MFT_ENUM_FLAG_TRANSCODE_ONLY = 0x00000020,
    MFT_ENUM_FLAG_SORTANDFILTER = 0x00000040,
    MFT_ENUM_FLAG_SORTANDFILTER_APPROVED_ONLY = 0x000000C0,
    MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY = 0x00000140,
    MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY_EDGEMODE = 0x00000240,
    MFT_ENUM_FLAG_UNTRUSTED_STOREMFT = 0x00000400,
    MFT_ENUM_FLAG_ALL = 0x0000003F,
}}
extern "system" {
    pub fn MFTEnumEx(
        guidCategory: GUID,
        Flags: UINT32,
        pInputType: *mut MFT_REGISTER_TYPE_INFO,
        pOutputType: *mut MFT_REGISTER_TYPE_INFO,
        pppMFTActivate: *mut *mut *mut IMFActivate,
        pnumMFTActivate: *mut UINT32,
    ) -> HRESULT;
}
DEFINE_GUID! {MFT_ENUM_VIDEO_RENDERER_EXTENSION_PROFILE,
0x62c56928, 0x9a4e, 0x443b, 0xb9, 0xdc, 0xca, 0xc8, 0x30, 0xc2, 0x41, 0x0}
DEFINE_GUID! {MFT_ENUM_ADAPTER_LUID,
0x1d39518c, 0xe220, 0x4da8, 0xa0, 0x7f, 0xba, 0x17, 0x25, 0x52, 0xd6, 0xb1}
extern "system" {
    pub fn MFTEnum2(
        guidCategory: GUID,
        Flags: UINT32,
        pInputType: *const MFT_REGISTER_TYPE_INFO,
        pOutputType: *const MFT_REGISTER_TYPE_INFO,
        pAttributes: *mut IMFAttributes,
        pppMFTActivate: *mut *mut *mut IMFActivate,
        pnumMFTActivate: *mut UINT32,
    ) -> HRESULT;
    pub fn MFTGetInfo(
        clsidMFT: CLSID,
        pszName: *mut LPWSTR,
        ppInputTypes: *mut *mut MFT_REGISTER_TYPE_INFO,
        pcInputTypes: *mut UINT32,
        ppOutputTypes: *mut *mut MFT_REGISTER_TYPE_INFO,
        pcOutputTypes: *mut UINT32,
        ppAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT;
    pub fn MFGetPluginControl(ppPluginControl: *mut *mut IMFPluginControl) -> HRESULT;
    pub fn MFGetMFTMerit(
        pMFT: *mut IUnknown,
        cbVerifier: UINT32,
        verifier: *const BYTE,
        merit: *mut DWORD,
    ) -> HRESULT;
    pub fn MFRegisterLocalSchemeHandler(szScheme: PCWSTR, pActivate: *mut IMFActivate) -> HRESULT;
    pub fn MFRegisterLocalByteStreamHandler(
        szFileExtension: PCWSTR,
        szMimeType: PCWSTR,
        pActivate: *mut IMFActivate,
    ) -> HRESULT;
    pub fn MFCreateMFByteStreamWrapper(
        pStream: *mut IMFByteStream,
        ppStreamWrapper: *mut *mut IMFByteStream,
    ) -> HRESULT;
    pub fn MFCreateMediaExtensionActivate(
        szActivatableClassId: PCWSTR,
        pConfiguration: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT;
}
DEFINE_GUID! {MFT_SUPPORT_DYNAMIC_FORMAT_CHANGE,
0x53476a11, 0x3f13, 0x49fb, 0xac, 0x42, 0xee, 0x27, 0x33, 0xc9, 0x67, 0x41}
