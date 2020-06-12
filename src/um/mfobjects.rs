// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{UINT32, UINT64};
use shared::guiddef::{GUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD};
use shared::mmreg::WAVEFORMATEX;
use shared::wtypes::{VT_CLSID, VT_LPWSTR, VT_R8, VT_UI1, VT_UI4, VT_UI8, VT_UNKNOWN, VT_VECTOR};
use um::objidlbase::IStream;
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, ULONGLONG};
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
    fract: Word,
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
