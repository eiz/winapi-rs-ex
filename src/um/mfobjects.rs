// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{__uint64, c_double, c_short, c_void};
use shared::basetsd::{UINT32, UINT64, UINT8};
use shared::guiddef::{CLSID, GUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID, UINT, ULONG, WORD};
use shared::mmreg::WAVEFORMATEX;
use shared::windef::SIZE;
use shared::wtypes::{VT_CLSID, VT_LPWSTR, VT_R8, VT_UI1, VT_UI4, VT_UI8, VT_UNKNOWN, VT_VECTOR};
use um::objidlbase::IStream;
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LONG, LONGLONG, LPCWSTR, LPWSTR, ULONGLONG};
pub type QWORD = ULONGLONG;
ENUM! {enum MF_ATTRIBUTE_TYPE {
    MF_ATTRIBUTE_UINT32 = VT_UI4,
    MF_ATTRIBUTE_UINT64 = VT_UI8,
    MF_ATTRIBUTE_DOUBLE = VT_R8,
    MF_ATTRIBUTE_GUID   = VT_CLSID,
    MF_ATTRIBUTE_STRING = VT_LPWSTR,
    MF_ATTRIBUTE_BLOB   = VT_VECTOR | VT_UI1,
    MF_ATTRIBUTE_IUNKNOWN = VT_UNKNOWN,
}}
ENUM! {enum MF_ATTRIBUTES_MATCH_TYPE {
    MF_ATTRIBUTES_MATCH_OUR_ITEMS = 0,
    MF_ATTRIBUTES_MATCH_THEIR_ITEMS = 1,
    MF_ATTRIBUTES_MATCH_ALL_ITEMS = 2,
    MF_ATTRIBUTES_MATCH_INTERSECTION = 3,
    MF_ATTRIBUTES_MATCH_SMALLER = 4,
}}
RIDL! {#[uuid(0x2cd2d921, 0xc447, 0x44a7, 0xa1, 0x3c, 0x4a, 0xda, 0xbf, 0xc2, 0x47, 0xe3)]
interface IMFAttributes(IMFAttributesVtbl): IUnknown(IUnknownVtbl) {
    fn GetItem(
        guidKey: REFGUID,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetItemType(
        guidKey: REFGUID,
        Value: REFPROPVARIANT,
        pbResult: *mut BOOL,
    ) -> HRESULT,
    fn CompareItem(
        guidKey: REFGUID,
        Value: REFPROPVARIANT,
        pbResult: *mut BOOL,
    ) -> HRESULT,
    fn Compare(
        pTheirs: *mut IMFAttributes,
        MatchType: MF_ATTRIBUTES_MATCH_TYPE,
        pbResult: *mut BOOL,
    ) -> HRESULT,
    fn GetUINT32(
        guidKey: REFGUID,
        punValue: *mut UINT32,
    ) -> HRESULT,
    fn GetUINT64(
        guidKey: REFGUID,
        punValue: *mut UINT64,
    ) -> HRESULT,
    fn GetDouble(
        guidKey: REFGUID,
        pfValue: c_double,
    ) -> HRESULT,
    fn GetGUID(
        guidKey: REFGUID,
        pguidValue: *mut GUID,
    ) -> HRESULT,
    fn GetStringLength(
        guidKey: REFGUID,
        pcchLength: *mut UINT32,
    ) -> HRESULT,
    fn GetString(
        guidKey: REFGUID,
        pwszValue: LPWSTR,
        cchBufSize: UINT32,
        pcchLength: *mut UINT32,
    ) -> HRESULT,
    fn GetAllocatedString(
        guidKey: REFGUID,
        ppwszValue: *mut LPWSTR,
        pcchLength: *mut UINT32,
    ) -> HRESULT,
    fn GetBlobSize(
        guidKey: REFGUID,
        pcbBlobSize: *mut UINT32,
    ) -> HRESULT,
    fn GetBlob(
        guidKey: REFGUID,
        pBuf: *mut UINT8,
        cbBufSize: UINT32,
        pcbBlobSize: *mut UINT32,
    ) -> HRESULT,
    fn GetAllocatedBlob(
        guidKey: REFGUID,
        ppbuf: *mut *mut UINT8,
        pcbSize: *mut UINT32,
    ) -> HRESULT,
    fn GetUnknown(
        guidKey: REFGUID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT,
    fn SetItem(
        guidKey: REFGUID,
        Value: REFPROPVARIANT,
    ) -> HRESULT,
    fn DeleteItem(
        guidKey: REFGUID,
    ) -> HRESULT,
    fn DeleteAllItems() -> HRESULT,
    fn SetUINT32(
        guidKey: REFGUID,
        unValue: UINT32,
    ) -> HRESULT,
    fn SetUINT64(
        guidKey: REFGUID,
        unValue: UINT64,
    ) -> HRESULT,
    fn SetDouble(
        guidKey: REFGUID,
        fValue: c_double,
    ) -> HRESULT,
    fn SetGUID(
        guidKey: REFGUID,
        guidValue: REFGUID,
    ) -> HRESULT,
    fn SetString(
        guidKey: REFGUID,
        wszValue: LPCWSTR,
    ) -> HRESULT,
    fn SetBlob(
        guidKey: REFGUID,
        pBuf: *const UINT8,
        cbBufSize: UINT32,
    ) -> HRESULT,
    fn SetUnknown(
        guidKey: REFGUID,
        pUnknown: *mut IUnknown,
    ) -> HRESULT,
    fn LockStore() -> HRESULT,
    fn UnlockStore() -> HRESULT,
    fn GetCount(
        pcItems: *mut UINT32,
    ) -> HRESULT,
    fn GetItemByIndex(
        unIndex: UINT32,
        pguidKey: *mut GUID,
        pValue: *mut PROPVARIANT,
    ) -> HRESULT,
    fn CopyAllItems(
        pDest: *mut IMFAttributes,
    ) -> HRESULT,
}}
ENUM! {enum MF_ATTRIBUTE_SERIALIZE_OPTIONS {
    MF_ATTRIBUTE_SERIALIZE_UNKNOWN_BYREF = 0x1,
}}
extern "system" {
    pub fn MFSerializeAttributesToStream(
        pAttr: *mut IMFAttributes,
        dwOptions: DWORD,
        pStm: *mut IStream,
    ) -> HRESULT;
    pub fn MFDeserializeAttributesFromStream(
        pAttr: *mut IMFAttributes,
        dwOptions: DWORD,
        pStm: *mut IStream,
    ) -> HRESULT;
}
RIDL! {#[uuid(0x045fa593, 0x8799, 0x42b8, 0xbc, 0x8d, 0x89, 0x68, 0xc6, 0x45, 0x35, 0x07)]
interface IMFMediaBuffer(IMFMediaBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Lock(
        ppbBuffer: *mut *mut BYTE,
        pcbMaxLength: *mut DWORD,
        pcbCurrentLength: *mut DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
    fn GetCurrentLength(
        pcbCurrentLength: *mut DWORD,
    ) -> HRESULT,
    fn SetCurrentLength(
        cbCurrentLength: DWORD,
    ) -> HRESULT,
    fn GetMaxLength(
        pcbMaxLength: *mut DWORD,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xc40a00f2, 0xb93a, 0x4d80, 0xae, 0x8c, 0x5a, 0x1c, 0x63, 0x4f, 0x58, 0xe4)]
interface IMFSample(IMFSampleVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetSampleFlags(
        pdwSampleFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetSampleFlags(
        dwSampleFlags: DWORD,
    ) -> HRESULT,
    fn GetSampleTime(
        phnsSampleTime: *mut LONGLONG,
    ) -> HRESULT,
    fn SetSampleTime(
        hnsSampleTime: LONGLONG,
    ) -> HRESULT,
    fn GetSampleDuration(
        phnsSampleDuration: *mut LONGLONG,
    ) -> HRESULT,
    fn SetSampleDuration(
        hnsSampleDuration: LONGLONG,
    ) -> HRESULT,
    fn GetBufferCount(
        pdwBufferCount: *mut DWORD,
    ) -> HRESULT,
    fn GetBufferByIndex(
        dwIndex: DWORD,
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn ConvertToContiguousBuffer(
        ppBuffer: *mut *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn AddBuffer(
        pBuffer: *mut IMFMediaBuffer,
    ) -> HRESULT,
    fn RemoveBufferByIndex(
        dwIndex: DWORD,
    ) -> HRESULT,
    fn RemoveAllBuffers() -> HRESULT,
    fn GetTotalLength(
        pcbTotalLength: *mut DWORD,
    ) -> HRESULT,
    fn CopyToBuffer(
        pBuffer: *mut IMFMediaBuffer,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x7dc9d5f9, 0x9ed9, 0x44ec, 0x9b, 0xbf, 0x06, 0x00, 0xbb, 0x58, 0x9f, 0xbb)]
interface IMF2DBuffer(IMF2DBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Lock2D(
        ppbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
    ) -> HRESULT,
    fn Unlock2D() -> HRESULT,
    fn GetScanline0AndPitch(
        pbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
    ) -> HRESULT,
    fn IsContiguousFormat(
        pfIsContiguous: *mut BOOL,
    ) -> HRESULT,
    fn GetContiguousLength(
        pcbLength: *mut DWORD,
    ) -> HRESULT,
    fn ContiguousCopyTo(
        pbDestBuffer: *mut BYTE,
        cbDestBuffer: DWORD,
    ) -> HRESULT,
    fn ContiguousCopyFrom(
        pbSrcBuffer: *mut BYTE,
        cbSrcBuffer: DWORD,
    ) -> HRESULT,
}}
ENUM! {enum MF2DBuffer_LockFlags {
    MF2DBuffer_LockFlags_LockTypeMask = 0x3,
    MF2DBuffer_LockFlags_Read = 0x1,
    MF2DBuffer_LockFlags_Write = 0x2,
    MF2DBuffer_LockFlags_ReadWrite = 0x3,
    MF2DBuffer_LockFlags_ForceDWORD = 0x7fffffff,
}}
RIDL! {#[uuid(0x33ae5ea6, 0x4316, 0x436f, 0x8d, 0xdd, 0xd7, 0x3d, 0x22, 0xf8, 0x29, 0xec)]
interface IMF2DBuffer2(IMF2DBuffer2Vtbl): IMF2DBuffer(IMF2DBufferVtbl) {
    fn Lock2DSize(
        lockFlags: MF2DBuffer_LockFlags,
        ppbScanline0: *mut *mut BYTE,
        plPitch: *mut LONG,
        ppbBufferStart: *mut *mut BYTE,
        pcbBufferLength: *mut DWORD,
    ) -> HRESULT,
    fn Copy2DTo(
        pDestBuffer: *mut IMF2DBuffer2,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xe7174cfa, 0x1c9e, 0x48b1, 0x88, 0x66, 0x62, 0x62, 0x26, 0xbf, 0xc2, 0x58)]
interface IMFDXGIBuffer(IMFDXGIBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetResource(
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn GetSubresourceIndex(
        puSubresource: *mut UINT,
    ) -> HRESULT,
    fn GetUnknown(
        guid: REFIID,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
    fn SetUnknown(
        guid: REFIID,
        pUnkData: *mut IUnknown,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x44ae0fa8, 0xea31, 0x4109, 0x8d, 0x2e, 0x4c, 0xae, 0x49, 0x97, 0xc5, 0x55)]
interface IMFMediaType(IMFMediaTypeVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetMajorType(
        pguidMajorType: *mut GUID,
    ) -> HRESULT,
    fn IsCompressedFormat(
        pfCompressed: *mut BOOL,
    ) -> HRESULT,
    fn IsEqual(
        pIMediaType: *mut IMFMediaType,
    ) -> HRESULT,
    fn GetRepresentation(
        guidRepresentation: GUID,
        ppvRepresentation: *mut LPVOID,
    ) -> HRESULT,
    fn FreeRepresentation(
        guidRepresentation: GUID,
        pvRepresentation: LPVOID,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x26a0adc3, 0xce26, 0x4672, 0x93, 0x04, 0x69, 0x55, 0x2e, 0xdd, 0x3f, 0xaf)]
interface IMFAudioMediaType(IMFAudioMediaTypeVtbl): IMFMediaType(IMFMediaTypeVtbl) {
    fn GetAudioFormat() -> *const WAVEFORMATEX,
}}
pub use um::wingdi::{BITMAPINFO, BITMAPINFOHEADER, RGBQUAD};
STRUCT! {struct MFT_REGISTER_TYPE_INFO {
    guidMajorType: GUID,
    guidSubtype: GUID,
}}
ENUM! {enum MFVideoInterlaceMode {
    MFVideoInterlace_Unknown = 0,
    MFVideoInterlace_Progressive = 2,
    MFVideoInterlace_FieldInterleavedUpperFirst = 3,
    MFVideoInterlace_FieldInterleavedLowerFirst = 4,
    MFVideoInterlace_FieldSingleUpper = 5,
    MFVideoInterlace_FieldSingleLower = 6,
    MFVideoInterlace_MixedInterlaceOrProgressive = 7,
    MFVideoInterlace_Last = MFVideoInterlace_MixedInterlaceOrProgressive + 1,
    MFVideoInterlace_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFVideoTransferFunction {
    MFVideoTransFunc_Unknown = 0,
    MFVideoTransFunc_10 = 1,
    MFVideoTransFunc_18 = 2,
    MFVideoTransFunc_20 = 3,
    MFVideoTransFunc_22 = 4,
    MFVideoTransFunc_709 = 5,
    MFVideoTransFunc_240M = 6,
    MFVideoTransFunc_sRGB = 7,
    MFVideoTransFunc_28 = 8,
    MFVideoTransFunc_Log_100 = 9,
    MFVideoTransFunc_Log_316 = 10,
    MFVideoTransFunc_709_sym = 11,
    MFVideoTransFunc_2020_const = 12,
    MFVideoTransFunc_2020 = 13,
    MFVideoTransFunc_26 = 14,
    MFVideoTransFunc_2084 = 15,
    MFVideoTransFunc_HLG  = 16,
    MFVideoTransFunc_10_rel = 17,
    MFVideoTransFunc_Last = MFVideoTransFunc_10_rel + 1,
    MFVideoTransFunc_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFVideoPrimaries {
    MFVideoPrimaries_Unknown = 0,
    MFVideoPrimaries_reserved = 1,
    MFVideoPrimaries_BT709 = 2,
    MFVideoPrimaries_BT470_2_SysM = 3,
    MFVideoPrimaries_BT470_2_SysBG = 4,
    MFVideoPrimaries_SMPTE170M = 5,
    MFVideoPrimaries_SMPTE240M = 6,
    MFVideoPrimaries_EBU3213 = 7,
    MFVideoPrimaries_SMPTE_C = 8,
    MFVideoPrimaries_BT2020 = 9,
    MFVideoPrimaries_XYZ = 10,
    MFVideoPrimaries_DCI_P3 = 11,
    MFVideoPrimaries_ACES = 12,
    MFVideoPrimaries_Last = MFVideoPrimaries_ACES + 1,
    MFVideoPrimaries_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFVideoLighting {
    MFVideoLighting_Unknown = 0,
    MFVideoLighting_bright = 1,
    MFVideoLighting_office = 2,
    MFVideoLighting_dim = 3,
    MFVideoLighting_dark = 4,
    MFVideoLighting_Last = MFVideoLighting_dark + 1,
    MFVideoLighting_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFVideoTransferMatrix {
    MFVideoTransferMatrix_Unknown = 0,
    MFVideoTransferMatrix_BT709 = 1,
    MFVideoTransferMatrix_BT601 = 2,
    MFVideoTransferMatrix_SMPTE240M = 3,
    MFVideoTransferMatrix_BT2020_10 = 4,
    MFVideoTransferMatrix_BT2020_12 = 5,
    MFVideoTransferMatrix_Last = MFVideoTransferMatrix_BT2020_12 + 1,
    MFVideoTransferMatrix_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFVideoChromaSubsampling {
    MFVideoChromaSubsampling_Unknown = 0,
    MFVideoChromaSubsampling_ProgressiveChroma = 0x8,
    MFVideoChromaSubsampling_Horizontally_Cosited = 0x4,
    MFVideoChromaSubsampling_Vertically_Cosited = 0x2,
    MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes = 0x1,
    MFVideoChromaSubsampling_MPEG2 =
        MFVideoChromaSubsampling_Horizontally_Cosited
        | MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    MFVideoChromaSubsampling_MPEG1 = MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    MFVideoChromaSubsampling_DV_PAL =
        MFVideoChromaSubsampling_Horizontally_Cosited
        | MFVideoChromaSubsampling_Vertically_Cosited,
    MFVideoChromaSubsampling_Cosited =
        MFVideoChromaSubsampling_Horizontally_Cosited
        | MFVideoChromaSubsampling_Vertically_Cosited
        | MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    MFVideoChromaSubsampling_Last = MFVideoChromaSubsampling_Cosited + 1,
    MFVideoChromaSubsampling_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFNominalRange {
    MFNominalRange_Unknown = 0,
    MFNominalRange_Normal = 1,
    MFNominalRange_Wide = 2,
    MFNominalRange_0_255 = 1,
    MFNominalRange_16_235 = 2,
    MFNominalRange_48_208 = 3,
    MFNominalRange_64_127 = 4,
    MFNominalRange_Last = MFNominalRange_64_127 + 1,
    MFNominalRange_ForceDWORD = 0x7fffffff,
}}
ENUM! {enum MFVideoFlags {
    MFVideoFlag_PAD_TO_Mask = 0x1 | 0x2 ,
    MFVideoFlag_PAD_TO_None = 0 * 0x1 ,
    MFVideoFlag_PAD_TO_4x3 = 1 * 0x1 ,
    MFVideoFlag_PAD_TO_16x9 = 2 * 0x1 ,
    MFVideoFlag_SrcContentHintMask = 0x4 | 0x8 | 0x10 ,
    MFVideoFlag_SrcContentHintNone = 0 * 0x4 ,
    MFVideoFlag_SrcContentHint16x9 = 1 * 0x4 ,
    MFVideoFlag_SrcContentHint235_1 = 2 * 0x4 ,
    MFVideoFlag_AnalogProtected = 0x20,
    MFVideoFlag_DigitallyProtected = 0x40,
    MFVideoFlag_ProgressiveContent = 0x80,
    MFVideoFlag_FieldRepeatCountMask = 0x100 | 0x200 | 0x400 ,
    MFVideoFlag_FieldRepeatCountShift = 8,
    MFVideoFlag_ProgressiveSeqReset = 0x800,
    MFVideoFlag_PanScanEnabled = 0x20000,
    MFVideoFlag_LowerFieldFirst = 0x40000,
    MFVideoFlag_BottomUpLinearRep = 0x80000,
    MFVideoFlags_DXVASurface = 0x100000,
    MFVideoFlags_RenderTargetSurface = 0x400000,
    MFVideoFlags_ForceQWORD = 0x7fffffff,
}}
STRUCT! {struct MFRatio {
    Numerator: DWORD,
    Denominator: DWORD,
}}
STRUCT! {struct MFOffset {
    fract: WORD,
    value: c_short,
}}
STRUCT! {struct MFVideoArea {
    OffsetX: MFOffset,
    OffsetY: MFOffset,
    Area: SIZE,
}}
STRUCT! {struct MFVideoInfo {
    dwWidth: DWORD,
    dwHeight: DWORD,
    PixelAspectRatio: MFRatio,
    SourceChromaSubsampling: MFVideoChromaSubsampling,
    InterlaceMode: MFVideoInterlaceMode,
    TransferFunction: MFVideoTransferFunction,
    ColorPrimaries: MFVideoPrimaries,
    TransferMatrix: MFVideoTransferMatrix,
    SourceLighting: MFVideoLighting,
    FramesPerSecond: MFRatio,
    NominalRange: MFNominalRange,
    GeometricAperture: MFVideoArea,
    MinimumDisplayAperture: MFVideoArea,
    PanScanAperture: MFVideoArea,
    VideoFlags: __uint64,
}}
STRUCT! {struct MFAYUVSample {
    bCrValue: BYTE,
    bCbValue: BYTE,
    bYValue: BYTE,
    bSampleAlpha8: BYTE,
}}
STRUCT! {struct MFARGB {
    rgbBlue: BYTE,
    rgbGreen: BYTE,
    rgbRed: BYTE,
    rgbAlpha: BYTE,
}}
STRUCT! {struct MFPaletteEntry {
    ARGB: MFARGB,
    AYCbCr: MFAYUVSample,
}}
STRUCT! {struct MFVideoSurfaceInfo {
    Format: DWORD,
    PaletteEntries: DWORD,
    Palette: [MFPaletteEntry; 1],
}}
STRUCT! {struct MFVideoCompressedInfo {
    AvgBitrate: LONGLONG,
    AvgBitErrorRate: LONGLONG,
    MaxKeyFrameSpacing: DWORD,
}}
STRUCT! {struct MFVIDEOFORMAT {
    dwSize: DWORD,
    videoInfo: MFVideoInfo,
    guidFormat: GUID,
    compressedInfo: MFVideoCompressedInfo,
    surfaceInfo: MFVideoSurfaceInfo,
}}
ENUM! {enum MFStandardVideoFormat {
    MFStdVideoFormat_reserved = 0,
    MFStdVideoFormat_NTSC = MFStdVideoFormat_reserved + 1,
    MFStdVideoFormat_PAL = MFStdVideoFormat_NTSC + 1,
    MFStdVideoFormat_DVD_NTSC = MFStdVideoFormat_PAL + 1,
    MFStdVideoFormat_DVD_PAL = MFStdVideoFormat_DVD_NTSC + 1,
    MFStdVideoFormat_DV_PAL = MFStdVideoFormat_DVD_PAL + 1,
    MFStdVideoFormat_DV_NTSC = MFStdVideoFormat_DV_PAL + 1,
    MFStdVideoFormat_ATSC_SD480i = MFStdVideoFormat_DV_NTSC + 1,
    MFStdVideoFormat_ATSC_HD1080i = MFStdVideoFormat_ATSC_SD480i + 1,
    MFStdVideoFormat_ATSC_HD720p = MFStdVideoFormat_ATSC_HD1080i + 1,
}}
RIDL! {#[uuid(0xb99f381f, 0xa8f9, 0x47a2, 0xa5, 0xaf, 0xca, 0x3a, 0x22, 0x5a, 0x38, 0x90)]
interface IMFVideoMediaType(IMFVideoMediaTypeVtbl): IMFMediaType(IMFMediaTypeVtbl) {
    fn GetVideoFormat() -> *const MFVIDEOFORMAT,
    fn GetVideoRepresentation(
        guidRepresentation: GUID,
        ppvRepresentation: *mut LPVOID,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xac6b7889, 0x0740, 0x4d51, 0x86, 0x19, 0x90, 0x59, 0x94, 0xa5, 0x5c, 0xc6)]
interface IMFAsyncResult(IMFAsyncResultVtbl): IUnknown(IUnknownVtbl) {
    fn GetState(
        ppunkState: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetStatus() -> HRESULT,
    fn SetStatus(
        hrStatus: HRESULT,
    ) -> HRESULT,
    fn GetObject(
        ppObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetStateNoAddRef() -> *mut IUnknown,
}}
RIDL! {#[uuid(0xa27003cf, 0x2354, 0x4f2a, 0x8d, 0x6a, 0xab, 0x7c, 0xff, 0x15, 0x43, 0x7e)]
interface IMFAsyncCallback(IMFAsyncCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn GetParameters(
        pdwFlags: *mut DWORD,
        pdwQueue: *mut DWORD,
    ) -> HRESULT,
    fn Invoke(
        pAsyncResult: *mut IMFAsyncResult,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xc7a4dca1, 0xf5f0, 0x47b6, 0xb9, 0x2b, 0xbf, 0x01, 0x06, 0xd2, 0x57, 0x91)]
interface IMFAsyncCallbackLogging(IMFAsyncCallbackLoggingVtbl): IUnknown(IUnknownVtbl) {
    fn GetObjectPointer() -> *mut c_void,
    fn GetObjectTag() -> DWORD,
}}
pub const MFASYNC_FAST_IO_PROCESSING_CALLBACK: DWORD = 0x00000001;
pub const MFASYNC_SIGNAL_CALLBACK: DWORD = 0x00000002;
pub const MFASYNC_BLOCKING_CALLBACK: DWORD = 0x00000004;
pub const MFASYNC_REPLY_CALLBACK: DWORD = 0x00000008;
pub const MFASYNC_LOCALIZE_REMOTE_CALLBACK: DWORD = 0x00000010;
pub const MFASYNC_CALLBACK_QUEUE_UNDEFINED: DWORD = 0x00000000;
pub const MFASYNC_CALLBACK_QUEUE_STANDARD: DWORD = 0x00000001;
pub const MFASYNC_CALLBACK_QUEUE_RT: DWORD = 0x00000002;
pub const MFASYNC_CALLBACK_QUEUE_IO: DWORD = 0x00000003;
pub const MFASYNC_CALLBACK_QUEUE_TIMER: DWORD = 0x00000004;
pub const MFASYNC_CALLBACK_QUEUE_MULTITHREADED: DWORD = 0x00000005;
pub const MFASYNC_CALLBACK_QUEUE_LONG_FUNCTION: DWORD = 0x00000007;
pub const MFASYNC_CALLBACK_QUEUE_PRIVATE_MASK: DWORD = 0xFFFF0000;
pub const MFASYNC_CALLBACK_QUEUE_ALL: DWORD = 0xFFFFFFFF;
ENUM! {enum __MIDL___MIDL_itf_mfobjects_0000_0012_0001 {
    MEUnknown = 0,
    MEError = 1,
    MEExtendedType = 2,
    MENonFatalError = 3,
    MEGenericV1Anchor = MENonFatalError,
    MESessionUnknown = 100,
    MESessionTopologySet = 101,
    MESessionTopologiesCleared = 102,
    MESessionStarted = 103,
    MESessionPaused = 104,
    MESessionStopped = 105,
    MESessionClosed = 106,
    MESessionEnded = 107,
    MESessionRateChanged = 108,
    MESessionScrubSampleComplete = 109,
    MESessionCapabilitiesChanged = 110,
    MESessionTopologyStatus = 111,
    MESessionNotifyPresentationTime = 112,
    MENewPresentation = 113,
    MELicenseAcquisitionStart = 114,
    MELicenseAcquisitionCompleted = 115,
    MEIndividualizationStart = 116,
    MEIndividualizationCompleted = 117,
    MEEnablerProgress = 118,
    MEEnablerCompleted = 119,
    MEPolicyError = 120,
    MEPolicyReport = 121,
    MEBufferingStarted = 122,
    MEBufferingStopped = 123,
    MEConnectStart = 124,
    MEConnectEnd = 125,
    MEReconnectStart = 126,
    MEReconnectEnd = 127,
    MERendererEvent = 128,
    MESessionStreamSinkFormatChanged = 129,
    MESessionV1Anchor = MESessionStreamSinkFormatChanged,
    MESourceUnknown = 200,
    MESourceStarted = 201,
    MEStreamStarted = 202,
    MESourceSeeked = 203,
    MEStreamSeeked = 204,
    MENewStream = 205,
    MEUpdatedStream = 206,
    MESourceStopped = 207,
    MEStreamStopped = 208,
    MESourcePaused = 209,
    MEStreamPaused = 210,
    MEEndOfPresentation = 211,
    MEEndOfStream = 212,
    MEMediaSample = 213,
    MEStreamTick = 214,
    MEStreamThinMode = 215,
    MEStreamFormatChanged = 216,
    MESourceRateChanged = 217,
    MEEndOfPresentationSegment = 218,
    MESourceCharacteristicsChanged = 219,
    MESourceRateChangeRequested = 220,
    MESourceMetadataChanged = 221,
    MESequencerSourceTopologyUpdated = 222,
    MESourceV1Anchor = MESequencerSourceTopologyUpdated,
    MESinkUnknown = 300,
    MEStreamSinkStarted = 301,
    MEStreamSinkStopped = 302,
    MEStreamSinkPaused = 303,
    MEStreamSinkRateChanged = 304,
    MEStreamSinkRequestSample = 305,
    MEStreamSinkMarker = 306,
    MEStreamSinkPrerolled = 307,
    MEStreamSinkScrubSampleComplete = 308,
    MEStreamSinkFormatChanged = 309,
    MEStreamSinkDeviceChanged = 310,
    MEQualityNotify = 311,
    MESinkInvalidated = 312,
    MEAudioSessionNameChanged = 313,
    MEAudioSessionVolumeChanged = 314,
    MEAudioSessionDeviceRemoved = 315,
    MEAudioSessionServerShutdown = 316,
    MEAudioSessionGroupingParamChanged = 317,
    MEAudioSessionIconChanged = 318,
    MEAudioSessionFormatChanged = 319,
    MEAudioSessionDisconnected = 320,
    MEAudioSessionExclusiveModeOverride = 321,
    MESinkV1Anchor = MEAudioSessionExclusiveModeOverride,
    MECaptureAudioSessionVolumeChanged = 322,
    MECaptureAudioSessionDeviceRemoved = 323,
    MECaptureAudioSessionFormatChanged = 324,
    MECaptureAudioSessionDisconnected = 325,
    MECaptureAudioSessionExclusiveModeOverride = 326,
    MECaptureAudioSessionServerShutdown = 327,
    MESinkV2Anchor = MECaptureAudioSessionServerShutdown,
    METrustUnknown = 400,
    MEPolicyChanged = 401,
    MEContentProtectionMessage = 402,
    MEPolicySet = 403,
    METrustV1Anchor = MEPolicySet,
    MEWMDRMLicenseBackupCompleted = 500,
    MEWMDRMLicenseBackupProgress = 501,
    MEWMDRMLicenseRestoreCompleted = 502,
    MEWMDRMLicenseRestoreProgress = 503,
    MEWMDRMLicenseAcquisitionCompleted = 506,
    MEWMDRMIndividualizationCompleted = 508,
    MEWMDRMIndividualizationProgress = 513,
    MEWMDRMProximityCompleted = 514,
    MEWMDRMLicenseStoreCleaned = 515,
    MEWMDRMRevocationDownloadCompleted = 516,
    MEWMDRMV1Anchor = MEWMDRMRevocationDownloadCompleted,
    METransformUnknown = 600,
    METransformNeedInput = METransformUnknown + 1,
    METransformHaveOutput = METransformNeedInput + 1,
    METransformDrainComplete = METransformHaveOutput + 1,
    METransformMarker = METransformDrainComplete + 1,
    METransformInputStreamStateChanged = METransformMarker + 1,
    MEByteStreamCharacteristicsChanged = 700,
    MEVideoCaptureDeviceRemoved = 800,
    MEVideoCaptureDevicePreempted = 801,
    MEStreamSinkFormatInvalidated = 802,
    MEEncodingParameters = 803,
    MEContentProtectionMetadata = 900,
    MEDeviceThermalStateChanged = 950,
    MEReservedMax = 10000,
}}
pub type MediaEventType = DWORD;
RIDL! {#[uuid(0xdf598932, 0xf10c, 0x4e39, 0xbb, 0xa2, 0xc3, 0x08, 0xf1, 0x01, 0xda, 0xa3)]
interface IMFMediaEvent(IMFMediaEventVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn GetType(
        pmet: *mut MediaEventType,
    ) -> HRESULT,
    fn GetExtendedType(
        pguidExtendedType: *mut GUID,
    ) -> HRESULT,
    fn GetStatus(
        phrStatus: *mut HRESULT,
    ) -> HRESULT,
    fn GetValue(
        pvValue: *mut PROPVARIANT,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x2cd0bd52, 0xbcd5, 0x4b89, 0xb6, 0x2c, 0xea, 0xdc, 0x0c, 0x03, 0x1e, 0x7d)]
interface IMFMediaEventGenerator(IMFMediaEventGeneratorVtbl): IUnknown(IUnknownVtbl) {
    fn GetEvent(
        dwFlags: DWORD,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn BeginGetEvent(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetEvent(
        pResult: *mut IMFAsyncResult,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEvent(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pvValue: *const PROPVARIANT,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xa27003d0, 0x2354, 0x4f2a, 0x8d, 0x6a, 0xab, 0x7c, 0xff, 0x15, 0x43, 0x7e)]
interface IMFRemoteAsyncCallback(IMFRemoteAsyncCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Invoke(
        hr: HRESULT,
        pRemoteResult: *mut IUnknown,
    ) -> HRESULT,
}}
ENUM! {enum MFBYTESTREAM_SEEK_ORIGIN {
    msoBegin = 0,
    msoCurrent = msoBegin + 1,
}}
RIDL! {#[uuid(0xad4c1b00, 0x4bf7, 0x422f, 0x91, 0x75, 0x75, 0x66, 0x93, 0xd9, 0x13, 0x0d)]
interface IMFByteStream(IMFByteStreamVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        pdwCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn GetLength(
        pqwLength: *mut QWORD,
    ) -> HRESULT,
    fn SetLength(
        qwLength: QWORD,
    ) -> HRESULT,
    fn GetCurrentPosition(
        pqwPosition: *mut QWORD,
    ) -> HRESULT,
    fn SetCurrentPosition(
        qwPosition: QWORD,
    ) -> HRESULT,
    fn IsEndOfStream(
        pfEndOfStream: *mut BOOL,
    ) -> HRESULT,
    fn Read(
        pb: *mut BYTE,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn BeginRead(
        pb: *mut BYTE,
        cb: ULONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndRead(
        pResult: *mut IMFAsyncResult,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn Write(
        pb: *mut BYTE,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn BeginWrite(
        pb: *const BYTE,
        cb: ULONG,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndWrite(
        pResult: *mut IMFAsyncResult,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn Seek(
        SeekOrigin: MFBYTESTREAM_SEEK_ORIGIN,
        llSeekOffset: LONGLONG,
        dwSeekFlags: DWORD,
        pqwCurrentPosition: *mut QWORD,
    ) -> HRESULT,
    fn Flush() -> HRESULT,
    fn Close() -> HRESULT,
}}
pub const MFBYTESTREAM_IS_READABLE: DWORD = 0x00000001;
pub const MFBYTESTREAM_IS_WRITABLE: DWORD = 0x00000002;
pub const MFBYTESTREAM_IS_SEEKABLE: DWORD = 0x00000004;
pub const MFBYTESTREAM_IS_REMOTE: DWORD = 0x00000008;
pub const MFBYTESTREAM_IS_DIRECTORY: DWORD = 0x00000080;
pub const MFBYTESTREAM_HAS_SLOW_SEEK: DWORD = 0x00000100;
pub const MFBYTESTREAM_IS_PARTIALLY_DOWNLOADED: DWORD = 0x00000200;
pub const MFBYTESTREAM_SHARE_WRITE: DWORD = 0x00000400;
pub const MFBYTESTREAM_DOES_NOT_USE_NETWORK: DWORD = 0x00000800;
pub const MFBYTESTREAM_SEEK_FLAG_CANCEL_PENDING_IO: DWORD = 0x00000001;
DEFINE_GUID! {MF_BYTESTREAM_ORIGIN_NAME,
0xfc358288, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID! {MF_BYTESTREAM_CONTENT_TYPE,
0xfc358289, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID! {MF_BYTESTREAM_DURATION,
0xfc35828a, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID! {MF_BYTESTREAM_LAST_MODIFIED_TIME,
0xfc35828b, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID! {MF_BYTESTREAM_IFO_FILE_URI,
0xfc35828c, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID! {MF_BYTESTREAM_DLNA_PROFILE_ID,
0xfc35828d, 0x3cb6, 0x460c, 0xa4, 0x24, 0xb6, 0x68, 0x12, 0x60, 0x37, 0x5a}
DEFINE_GUID! {MF_BYTESTREAM_EFFECTIVE_URL,
0x9afa0209, 0x89d1, 0x42af, 0x84, 0x56, 0x1d, 0xe6, 0xb5, 0x62, 0xd6, 0x91}
DEFINE_GUID! {MF_BYTESTREAM_TRANSCODED,
0xb6c5c282, 0x4dc9, 0x4db9, 0xab, 0x48, 0xcf, 0x3b, 0x6d, 0x8b, 0xc5, 0xe0}
DEFINE_GUID! {CLSID_MFByteStreamProxyClassFactory,
0x770e8e77, 0x4916, 0x441c, 0xa9, 0xa7, 0xb3, 0x42, 0xd0, 0xee, 0xbc, 0x71}
RIDL! {#[uuid(0xa6b43f84, 0x5c0a, 0x42e8, 0xa4, 0x4d, 0xb1, 0x85, 0x7a, 0x76, 0x99, 0x2f)]
interface IMFByteStreamProxyClassFactory(IMFByteStreamProxyClassFactoryVtbl)
    : IUnknown(IUnknownVtbl) {
    fn CreateByteStreamProxy(
        pByteStream: *mut IMFByteStream,
        pAttributes: *mut IMFAttributes,
        riid: REFIID,
        ppvObject: *mut LPVOID,
    ) -> HRESULT,
}}
ENUM! {enum __MIDL___MIDL_itf_mfobjects_0000_0017_0001 {
    MF_ACCESSMODE_READ = 1,
    MF_ACCESSMODE_WRITE = 2,
    MF_ACCESSMODE_READWRITE = 3,
}}
ENUM! {enum __MIDL___MIDL_itf_mfobjects_0000_0017_0002 {
    MF_OPENMODE_FAIL_IF_NOT_EXIST = 0,
    MF_OPENMODE_FAIL_IF_EXIST = 1,
    MF_OPENMODE_RESET_IF_EXIST = 2,
    MF_OPENMODE_APPEND_IF_EXIST = 3,
    MF_OPENMODE_DELETE_IF_EXIST = 4,
}}
ENUM! {enum __MIDL___MIDL_itf_mfobjects_0000_0017_0003 {
    MF_FILEFLAGS_NONE = 0,
    MF_FILEFLAGS_NOBUFFERING = 0x1,
    MF_FILEFLAGS_ALLOW_WRITE_SHARING = 0x2,
}}
RIDL! {#[uuid(0x8feed468, 0x6f7e, 0x440d, 0x86, 0x9a, 0x49, 0xbd, 0xd2, 0x83, 0xad, 0x0d)]
interface IMFSampleOutputStream(IMFSampleOutputStreamVtbl): IUnknown(IUnknownVtbl) {
    fn BeginWriteSample(
        pSample: *mut IMFSample,
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndWriteSample(
        pResult: *mut IMFAsyncResult,
    ) -> HRESULT,
    fn Clone() -> HRESULT,
}}
RIDL! {#[uuid(0x5bc8a76b, 0x869a, 0x46a3, 0x9b, 0x03, 0xfa, 0x21, 0x8a, 0x66, 0xae, 0xbe)]
interface IMFCollection(IMFCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetElementCount(
        pcElements: *mut DWORD,
    ) -> HRESULT,
    fn GetElement(
        dwElementIndex: DWORD,
        ppUnkElement: *mut *mut IUnknown,
    ) -> HRESULT,
    fn AddElement(
        pUnkElement: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveElement(
        dwElementIndex: DWORD,
        ppUnkElement: *mut *mut IUnknown,
    ) -> HRESULT,
    fn InsertElementAt(
        dwIndex: DWORD,
        pUnknown: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveAllElements() -> HRESULT,
}}
RIDL! {#[uuid(0x36f846fc, 0x2256, 0x48b6, 0xb5, 0x8e, 0xe2, 0xb6, 0x38, 0x31, 0x65, 0x81)]
interface IMFMediaEventQueue(IMFMediaEventQueueVtbl): IUnknown(IUnknownVtbl) {
    fn GetEvent(
        dwFlags: DWORD,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn BeginGetEvent(
        pCallback: *mut IMFAsyncCallback,
        punkState: *mut IUnknown,
    ) -> HRESULT,
    fn EndGetEvent(
        pResult: *mut IMFAsyncResult,
        ppEvent: *mut *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEvent(
        pEvent: *mut IMFMediaEvent,
    ) -> HRESULT,
    fn QueueEventParamVar(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pvValue: *const PROPVARIANT,
    ) -> HRESULT,
    fn QueueEventParamUnk(
        met: MediaEventType,
        guidExtendedType: REFGUID,
        hrStatus: HRESULT,
        pUnk: *mut IUnknown,
    ) -> HRESULT,
    fn Shutdown() -> HRESULT,
}}
RIDL! {#[uuid(0x7fee9e9a, 0x4a89, 0x47a6, 0x89, 0x9c, 0xb6, 0xa5, 0x3a, 0x70, 0xfb, 0x67)]
interface IMFActivate(IMFActivateVtbl): IMFAttributes(IMFAttributesVtbl) {
    fn ActivateObject(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn ShutdownObject() -> HRESULT,
    fn DetachObject() -> HRESULT,
}}
ENUM! {enum MF_Plugin_Type {
    MF_Plugin_Type_MFT = 0,
    MF_Plugin_Type_MediaSource = 1,
    MF_Plugin_Type_MFT_MatchOutputType = 2,
    MF_Plugin_Type_Other = -1i32 as u32,
}}
RIDL! {#[uuid(0x5c6c44bf, 0x1db6, 0x435b, 0x92, 0x49, 0xe8, 0xcd, 0x10, 0xfd, 0xec, 0x96)]
interface IMFPluginControl(IMFPluginControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetPreferredClsid(
        pluginType: DWORD,
        selector: LPCWSTR,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn GetPreferredClsidByIndex(
        pluginType: DWORD,
        index: DWORD,
        selector: *mut LPWSTR,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn SetPreferredClsid(
        pluginType: DWORD,
        selector: LPCWSTR,
        clsid: *const CLSID,
    ) -> HRESULT,
    fn IsDisabled(
        pluginType: DWORD,
        clsid: REFCLSID,
    ) -> HRESULT,
    fn GetDisabledByIndex(
        pluginType: DWORD,
        index: DWORD,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn SetDisabled(
        pluginType: DWORD,
        clsid: REFCLSID,
        disabled: BOOL,
    ) -> HRESULT,
}}
ENUM! {enum MF_PLUGIN_CONTROL_POLICY {
    MF_PLUGIN_CONTROL_POLICY_USE_ALL_PLUGINS = 0,
    MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS = 1,
    MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS = 2,
    MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE = 3,
}}
RIDL! {#[uuid(0xc6982083, 0x3ddc, 0x45cb, 0xaf, 0x5e, 0x0f, 0x7a, 0x8c, 0xe4, 0xde, 0x77)]
interface IMFPluginControl2(IMFPluginControl2Vtbl): IMFPluginControl(IMFPluginControlVtbl) {
    fn SetPolicy(
        policy: MF_PLUGIN_CONTROL_POLICY,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0xeb533d5d, 0x2db6, 0x40f8, 0x97, 0xa9, 0x49, 0x46, 0x92, 0x01, 0x4f, 0x07)]
interface IMFDXGIDeviceManager(IMFDXGIDeviceManagerVtbl): IUnknown(IUnknownVtbl) {
    fn CloseDeviceHandle(
        hDevice: HANDLE,
    ) -> HRESULT,
    fn GetVideoService(
        hDevice: HANDLE,
        riid: REFIID,
        ppService: *mut *mut c_void,
    ) -> HRESULT,
    fn LockDevice(
        hDevice: HANDLE,
        riid: REFIID,
        ppUnkDevice: *mut *mut c_void,
        fBlock: BOOL,
    ) -> HRESULT,
    fn OpenDeviceHandle(
        phDevice: *mut HANDLE,
    ) -> HRESULT,
    fn ResetDevice(
        pUnkDevice: *mut IUnknown,
        resetToken: UINT,
    ) -> HRESULT,
    fn TestDevice(
        hDevice: HANDLE,
    ) -> HRESULT,
    fn UnlockDevice(
        hDevice: HANDLE,
        fSaveState: BOOL,
    ) -> HRESULT,
}}
ENUM! {enum MF_STREAM_STATE {
    MF_STREAM_STATE_STOPPED = 0,
    MF_STREAM_STATE_PAUSED = MF_STREAM_STATE_STOPPED + 1,
    MF_STREAM_STATE_RUNNING = MF_STREAM_STATE_PAUSED + 1,
}}
RIDL! {#[uuid(0xce8bd576, 0xe440, 0x43b3, 0xbe, 0x34, 0x1e, 0x53, 0xf5, 0x65, 0xf7, 0xe8)]
interface IMFMuxStreamAttributesManager(IMFMuxStreamAttributesManagerVtbl)
    : IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pdwMuxStreamCount: *mut DWORD,
    ) -> HRESULT,
    fn GetAttributes(
        dwMuxStreamIndex: DWORD,
        ppStreamAttributes: *mut *mut IMFAttributes,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x505a2c72, 0x42f7, 0x4690, 0xae, 0xab, 0x8f, 0x51, 0x3d, 0x0f, 0xfd, 0xb8)]
interface IMFMuxStreamMediaTypeManager(IMFMuxStreamMediaTypeManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pdwMuxStreamCount: *mut DWORD,
    ) -> HRESULT,
    fn GetMediaType(
        dwMuxStreamIndex: DWORD,
        ppMediaType: *mut *mut IMFMediaType,
    ) -> HRESULT,
    fn GetStreamConfigurationCount(
        pdwCount: *mut DWORD,
    ) -> HRESULT,
    fn AddStreamConfiguration(
        ullStreamMask: ULONGLONG,
    ) -> HRESULT,
    fn RemoveStreamConfiguration(
        ullStreamMask: ULONGLONG,
    ) -> HRESULT,
    fn GetStreamConfiguration(
        ulIndex: DWORD,
        pullStreamMask: *mut ULONGLONG,
    ) -> HRESULT,
}}
RIDL! {#[uuid(0x74abbc19, 0xb1cc, 0x4e41, 0xbb, 0x8b, 0x9d, 0x9b, 0x86, 0xa8, 0xf6, 0xca)]
interface IMFMuxStreamSampleManager(IMFMuxStreamSampleManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetStreamCount(
        pdwMuxStreamCount: *mut DWORD,
    ) -> HRESULT,
    fn GetSample(
        dwMuxStreamIndex: DWORD,
        ppSample: *mut *mut IMFSample,
    ) -> HRESULT,
    fn GetStreamConfiguration() -> ULONGLONG,
}}
