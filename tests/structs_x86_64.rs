#![cfg(all(windows, target_arch = "x86_64"))]
extern crate winapi;
use std::mem::{align_of, size_of};
#[cfg(feature = "bcrypt")]
#[test]
fn shared_bcrypt() {
    use winapi::shared::bcrypt::*;
    assert_eq!(size_of::<BCRYPT_KEY_LENGTHS_STRUCT>(), 12);
    assert_eq!(align_of::<BCRYPT_KEY_LENGTHS_STRUCT>(), 4);
    assert_eq!(size_of::<BCRYPT_OID>(), 16);
    assert_eq!(align_of::<BCRYPT_OID>(), 8);
    assert_eq!(size_of::<BCRYPT_OID_LIST>(), 16);
    assert_eq!(align_of::<BCRYPT_OID_LIST>(), 8);
    assert_eq!(size_of::<BCRYPT_PKCS1_PADDING_INFO>(), 8);
    assert_eq!(align_of::<BCRYPT_PKCS1_PADDING_INFO>(), 8);
    assert_eq!(size_of::<BCRYPT_PSS_PADDING_INFO>(), 16);
    assert_eq!(align_of::<BCRYPT_PSS_PADDING_INFO>(), 8);
    assert_eq!(size_of::<BCRYPT_OAEP_PADDING_INFO>(), 24);
    assert_eq!(align_of::<BCRYPT_OAEP_PADDING_INFO>(), 8);
    assert_eq!(size_of::<BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO>(), 88);
    assert_eq!(align_of::<BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO>(), 8);
    assert_eq!(size_of::<BCryptBuffer>(), 16);
    assert_eq!(align_of::<BCryptBuffer>(), 8);
    assert_eq!(size_of::<BCryptBufferDesc>(), 16);
    assert_eq!(align_of::<BCryptBufferDesc>(), 8);
    assert_eq!(size_of::<BCRYPT_KEY_BLOB>(), 4);
    assert_eq!(align_of::<BCRYPT_KEY_BLOB>(), 4);
    assert_eq!(size_of::<BCRYPT_RSAKEY_BLOB>(), 24);
    assert_eq!(align_of::<BCRYPT_RSAKEY_BLOB>(), 4);
    assert_eq!(size_of::<BCRYPT_ECCKEY_BLOB>(), 8);
    assert_eq!(align_of::<BCRYPT_ECCKEY_BLOB>(), 4);
    assert_eq!(size_of::<SSL_ECCKEY_BLOB>(), 8);
    assert_eq!(align_of::<SSL_ECCKEY_BLOB>(), 4);
    assert_eq!(size_of::<BCRYPT_ECCFULLKEY_BLOB>(), 32);
    assert_eq!(align_of::<BCRYPT_ECCFULLKEY_BLOB>(), 4);
    assert_eq!(size_of::<BCRYPT_DH_KEY_BLOB>(), 8);
    assert_eq!(align_of::<BCRYPT_DH_KEY_BLOB>(), 4);
    assert_eq!(size_of::<BCRYPT_DH_PARAMETER_HEADER>(), 12);
    assert_eq!(align_of::<BCRYPT_DH_PARAMETER_HEADER>(), 4);
    assert_eq!(size_of::<BCRYPT_DSA_KEY_BLOB>(), 52);
    assert_eq!(align_of::<BCRYPT_DSA_KEY_BLOB>(), 4);
    assert_eq!(size_of::<BCRYPT_DSA_KEY_BLOB_V2>(), 28);
    assert_eq!(align_of::<BCRYPT_DSA_KEY_BLOB_V2>(), 4);
    assert_eq!(size_of::<BCRYPT_KEY_DATA_BLOB_HEADER>(), 12);
    assert_eq!(align_of::<BCRYPT_KEY_DATA_BLOB_HEADER>(), 4);
    assert_eq!(size_of::<BCRYPT_DSA_PARAMETER_HEADER>(), 56);
    assert_eq!(align_of::<BCRYPT_DSA_PARAMETER_HEADER>(), 4);
    assert_eq!(size_of::<BCRYPT_DSA_PARAMETER_HEADER_V2>(), 32);
    assert_eq!(align_of::<BCRYPT_DSA_PARAMETER_HEADER_V2>(), 4);
    assert_eq!(size_of::<BCRYPT_ECC_CURVE_NAMES>(), 16);
    assert_eq!(align_of::<BCRYPT_ECC_CURVE_NAMES>(), 8);
    assert_eq!(size_of::<BCRYPT_MULTI_HASH_OPERATION>(), 24);
    assert_eq!(align_of::<BCRYPT_MULTI_HASH_OPERATION>(), 8);
    assert_eq!(size_of::<BCRYPT_MULTI_OBJECT_LENGTH_STRUCT>(), 8);
    assert_eq!(align_of::<BCRYPT_MULTI_OBJECT_LENGTH_STRUCT>(), 4);
    assert_eq!(size_of::<BCRYPT_ALGORITHM_IDENTIFIER>(), 16);
    assert_eq!(align_of::<BCRYPT_ALGORITHM_IDENTIFIER>(), 8);
    assert_eq!(size_of::<BCRYPT_PROVIDER_NAME>(), 8);
    assert_eq!(align_of::<BCRYPT_PROVIDER_NAME>(), 8);
    assert_eq!(size_of::<BCRYPT_INTERFACE_VERSION>(), 4);
    assert_eq!(align_of::<BCRYPT_INTERFACE_VERSION>(), 2);
    assert_eq!(size_of::<CRYPT_INTERFACE_REG>(), 24);
    assert_eq!(align_of::<CRYPT_INTERFACE_REG>(), 8);
    assert_eq!(size_of::<CRYPT_IMAGE_REG>(), 24);
    assert_eq!(align_of::<CRYPT_IMAGE_REG>(), 8);
    assert_eq!(size_of::<CRYPT_PROVIDER_REG>(), 32);
    assert_eq!(align_of::<CRYPT_PROVIDER_REG>(), 8);
    assert_eq!(size_of::<CRYPT_PROVIDERS>(), 16);
    assert_eq!(align_of::<CRYPT_PROVIDERS>(), 8);
    assert_eq!(size_of::<CRYPT_CONTEXT_CONFIG>(), 8);
    assert_eq!(align_of::<CRYPT_CONTEXT_CONFIG>(), 4);
    assert_eq!(size_of::<CRYPT_CONTEXT_FUNCTION_CONFIG>(), 8);
    assert_eq!(align_of::<CRYPT_CONTEXT_FUNCTION_CONFIG>(), 4);
    assert_eq!(size_of::<CRYPT_CONTEXTS>(), 16);
    assert_eq!(align_of::<CRYPT_CONTEXTS>(), 8);
    assert_eq!(size_of::<CRYPT_CONTEXT_FUNCTIONS>(), 16);
    assert_eq!(align_of::<CRYPT_CONTEXT_FUNCTIONS>(), 8);
    assert_eq!(size_of::<CRYPT_CONTEXT_FUNCTION_PROVIDERS>(), 16);
    assert_eq!(align_of::<CRYPT_CONTEXT_FUNCTION_PROVIDERS>(), 8);
    assert_eq!(size_of::<CRYPT_PROPERTY_REF>(), 24);
    assert_eq!(align_of::<CRYPT_PROPERTY_REF>(), 8);
    assert_eq!(size_of::<CRYPT_IMAGE_REF>(), 16);
    assert_eq!(align_of::<CRYPT_IMAGE_REF>(), 8);
    assert_eq!(size_of::<CRYPT_PROVIDER_REF>(), 56);
    assert_eq!(align_of::<CRYPT_PROVIDER_REF>(), 8);
    assert_eq!(size_of::<CRYPT_PROVIDER_REFS>(), 16);
    assert_eq!(align_of::<CRYPT_PROVIDER_REFS>(), 8);
}
#[cfg(feature = "bthdef")]
#[test]
fn shared_bthdef() {
    use winapi::shared::bthdef::*;
    assert_eq!(size_of::<BTH_DEVICE_INFO>(), 272);
    assert_eq!(align_of::<BTH_DEVICE_INFO>(), 8);
    assert_eq!(size_of::<BTH_RADIO_IN_RANGE>(), 280);
    assert_eq!(align_of::<BTH_RADIO_IN_RANGE>(), 8);
    assert_eq!(size_of::<BTH_L2CAP_EVENT_INFO>(), 16);
    assert_eq!(align_of::<BTH_L2CAP_EVENT_INFO>(), 8);
    assert_eq!(size_of::<BTH_HCI_EVENT_INFO>(), 16);
    assert_eq!(align_of::<BTH_HCI_EVENT_INFO>(), 8);
}
#[cfg(feature = "bthioctl")]
#[test]
fn shared_bthioctl() {
    use winapi::shared::bthioctl::*;
    assert_eq!(size_of::<BTH_DEVICE_INFO_LIST>(), 276);
    assert_eq!(align_of::<BTH_DEVICE_INFO_LIST>(), 1);
    assert_eq!(size_of::<BTH_RADIO_INFO>(), 13);
    assert_eq!(align_of::<BTH_RADIO_INFO>(), 1);
    assert_eq!(size_of::<BTH_LOCAL_RADIO_INFO>(), 292);
    assert_eq!(align_of::<BTH_LOCAL_RADIO_INFO>(), 1);
    assert_eq!(size_of::<BTH_SDP_CONNECT>(), 21);
    assert_eq!(align_of::<BTH_SDP_CONNECT>(), 1);
    assert_eq!(size_of::<BTH_SDP_DISCONNECT>(), 8);
    assert_eq!(align_of::<BTH_SDP_DISCONNECT>(), 1);
    assert_eq!(size_of::<BTH_SDP_RECORD>(), 17);
    assert_eq!(align_of::<BTH_SDP_RECORD>(), 1);
    assert_eq!(size_of::<BTH_SDP_SERVICE_SEARCH_REQUEST>(), 248);
    assert_eq!(align_of::<BTH_SDP_SERVICE_SEARCH_REQUEST>(), 1);
    assert_eq!(size_of::<BTH_SDP_ATTRIBUTE_SEARCH_REQUEST>(), 20);
    assert_eq!(align_of::<BTH_SDP_ATTRIBUTE_SEARCH_REQUEST>(), 1);
    assert_eq!(size_of::<BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST>(), 256);
    assert_eq!(align_of::<BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST>(), 1);
    assert_eq!(size_of::<BTH_SDP_STREAM_RESPONSE>(), 9);
    assert_eq!(align_of::<BTH_SDP_STREAM_RESPONSE>(), 1);
    assert_eq!(size_of::<BTH_COMMAND_HEADER>(), 3);
    assert_eq!(align_of::<BTH_COMMAND_HEADER>(), 1);
    assert_eq!(size_of::<BTH_VENDOR_SPECIFIC_COMMAND>(), 10);
    assert_eq!(align_of::<BTH_VENDOR_SPECIFIC_COMMAND>(), 1);
    assert_eq!(size_of::<BTH_VENDOR_PATTERN>(), 3);
    assert_eq!(align_of::<BTH_VENDOR_PATTERN>(), 1);
    assert_eq!(size_of::<BTH_VENDOR_EVENT_INFO>(), 13);
    assert_eq!(align_of::<BTH_VENDOR_EVENT_INFO>(), 1);
    assert_eq!(size_of::<BTH_HOST_FEATURE_MASK>(), 24);
    assert_eq!(align_of::<BTH_HOST_FEATURE_MASK>(), 1);
}
#[cfg(feature = "bthsdpdef")]
#[test]
fn shared_bthsdpdef() {
    use winapi::shared::bthsdpdef::*;
    assert_eq!(size_of::<SDP_LARGE_INTEGER_16>(), 16);
    assert_eq!(align_of::<SDP_LARGE_INTEGER_16>(), 8);
    assert_eq!(size_of::<SDP_ULARGE_INTEGER_16>(), 16);
    assert_eq!(align_of::<SDP_ULARGE_INTEGER_16>(), 8);
    assert_eq!(size_of::<SdpAttributeRange>(), 4);
    assert_eq!(align_of::<SdpAttributeRange>(), 2);
    assert_eq!(size_of::<SdpQueryUuidUnion>(), 16);
    assert_eq!(align_of::<SdpQueryUuidUnion>(), 4);
    assert_eq!(size_of::<SdpQueryUuid>(), 20);
    assert_eq!(align_of::<SdpQueryUuid>(), 4);
}
#[cfg(feature = "d3d9caps")]
#[test]
fn shared_d3d9caps() {
    use winapi::shared::d3d9caps::*;
    assert_eq!(size_of::<D3DVSHADERCAPS2_0>(), 16);
    assert_eq!(align_of::<D3DVSHADERCAPS2_0>(), 4);
    assert_eq!(size_of::<D3DPSHADERCAPS2_0>(), 20);
    assert_eq!(align_of::<D3DPSHADERCAPS2_0>(), 4);
    assert_eq!(size_of::<D3DOVERLAYCAPS>(), 12);
    assert_eq!(align_of::<D3DOVERLAYCAPS>(), 4);
    assert_eq!(size_of::<D3DCONTENTPROTECTIONCAPS>(), 40);
    assert_eq!(align_of::<D3DCONTENTPROTECTIONCAPS>(), 8);
    assert_eq!(size_of::<D3DCAPS9>(), 304);
    assert_eq!(align_of::<D3DCAPS9>(), 4);
}
#[cfg(feature = "d3d9types")]
#[test]
fn shared_d3d9types() {
    use winapi::shared::d3d9types::*;
    assert_eq!(size_of::<D3DVECTOR>(), 12);
    assert_eq!(align_of::<D3DVECTOR>(), 4);
    assert_eq!(size_of::<D3DCOLORVALUE>(), 16);
    assert_eq!(align_of::<D3DCOLORVALUE>(), 4);
    assert_eq!(size_of::<D3DRECT>(), 16);
    assert_eq!(align_of::<D3DRECT>(), 4);
    assert_eq!(size_of::<D3DMATRIX>(), 64);
    assert_eq!(align_of::<D3DMATRIX>(), 4);
    assert_eq!(size_of::<D3DVIEWPORT9>(), 24);
    assert_eq!(align_of::<D3DVIEWPORT9>(), 4);
    assert_eq!(size_of::<D3DCLIPSTATUS9>(), 8);
    assert_eq!(align_of::<D3DCLIPSTATUS9>(), 4);
    assert_eq!(size_of::<D3DMATERIAL9>(), 68);
    assert_eq!(align_of::<D3DMATERIAL9>(), 4);
    assert_eq!(size_of::<D3DLIGHT9>(), 104);
    assert_eq!(align_of::<D3DLIGHT9>(), 4);
    assert_eq!(size_of::<D3DVERTEXELEMENT9>(), 8);
    assert_eq!(align_of::<D3DVERTEXELEMENT9>(), 2);
    assert_eq!(size_of::<D3DDISPLAYMODE>(), 16);
    assert_eq!(align_of::<D3DDISPLAYMODE>(), 4);
    assert_eq!(size_of::<D3DDEVICE_CREATION_PARAMETERS>(), 24);
    assert_eq!(align_of::<D3DDEVICE_CREATION_PARAMETERS>(), 8);
    assert_eq!(size_of::<D3DPRESENT_PARAMETERS>(), 64);
    assert_eq!(align_of::<D3DPRESENT_PARAMETERS>(), 8);
    assert_eq!(size_of::<D3DGAMMARAMP>(), 1536);
    assert_eq!(align_of::<D3DGAMMARAMP>(), 2);
    assert_eq!(size_of::<D3DVERTEXBUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3DVERTEXBUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3DINDEXBUFFER_DESC>(), 20);
    assert_eq!(align_of::<D3DINDEXBUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3DSURFACE_DESC>(), 32);
    assert_eq!(align_of::<D3DSURFACE_DESC>(), 4);
    assert_eq!(size_of::<D3DVOLUME_DESC>(), 28);
    assert_eq!(align_of::<D3DVOLUME_DESC>(), 4);
    assert_eq!(size_of::<D3DLOCKED_RECT>(), 16);
    assert_eq!(align_of::<D3DLOCKED_RECT>(), 8);
    assert_eq!(size_of::<D3DBOX>(), 24);
    assert_eq!(align_of::<D3DBOX>(), 4);
    assert_eq!(size_of::<D3DLOCKED_BOX>(), 16);
    assert_eq!(align_of::<D3DLOCKED_BOX>(), 8);
    assert_eq!(size_of::<D3DRANGE>(), 8);
    assert_eq!(align_of::<D3DRANGE>(), 4);
    assert_eq!(size_of::<D3DRECTPATCH_INFO>(), 28);
    assert_eq!(align_of::<D3DRECTPATCH_INFO>(), 4);
    assert_eq!(size_of::<D3DTRIPATCH_INFO>(), 16);
    assert_eq!(align_of::<D3DTRIPATCH_INFO>(), 4);
    assert_eq!(size_of::<D3DADAPTER_IDENTIFIER9>(), 1104);
    assert_eq!(align_of::<D3DADAPTER_IDENTIFIER9>(), 8);
    assert_eq!(size_of::<D3DRASTER_STATUS>(), 8);
    assert_eq!(align_of::<D3DRASTER_STATUS>(), 4);
    assert_eq!(size_of::<D3DRESOURCESTATS>(), 44);
    assert_eq!(align_of::<D3DRESOURCESTATS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_RESOURCEMANAGER>(), 352);
    assert_eq!(align_of::<D3DDEVINFO_RESOURCEMANAGER>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3DVERTEXSTATS>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3DVERTEXSTATS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_VCACHE>(), 16);
    assert_eq!(align_of::<D3DDEVINFO_VCACHE>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>(), 16);
    assert_eq!(align_of::<D3DDEVINFO_D3D9PIPELINETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>(), 20);
    assert_eq!(align_of::<D3DDEVINFO_D3D9INTERFACETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9STAGETIMINGS>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3D9STAGETIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>(), 20);
    assert_eq!(align_of::<D3DDEVINFO_D3D9BANDWIDTHTIMINGS>(), 4);
    assert_eq!(size_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>(), 8);
    assert_eq!(align_of::<D3DDEVINFO_D3D9CACHEUTILIZATION>(), 4);
    assert_eq!(size_of::<D3DMEMORYPRESSURE>(), 24);
    assert_eq!(align_of::<D3DMEMORYPRESSURE>(), 8);
    assert_eq!(size_of::<D3DCOMPOSERECTDESC>(), 8);
    assert_eq!(align_of::<D3DCOMPOSERECTDESC>(), 2);
    assert_eq!(size_of::<D3DCOMPOSERECTDESTINATION>(), 8);
    assert_eq!(align_of::<D3DCOMPOSERECTDESTINATION>(), 2);
    assert_eq!(size_of::<D3DPRESENTSTATS>(), 32);
    assert_eq!(align_of::<D3DPRESENTSTATS>(), 8);
    assert_eq!(size_of::<D3DDISPLAYMODEEX>(), 24);
    assert_eq!(align_of::<D3DDISPLAYMODEEX>(), 4);
    assert_eq!(size_of::<D3DDISPLAYMODEFILTER>(), 12);
    assert_eq!(align_of::<D3DDISPLAYMODEFILTER>(), 4);
    assert_eq!(size_of::<D3D_OMAC>(), 16);
    assert_eq!(align_of::<D3D_OMAC>(), 1);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>(), 32);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERY_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>(), 4);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS>(), 4);
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>(),
        40
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>(),
        40
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>(),
        48
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT>(),
        8
    );
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT>(), 8);
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>(),
        80
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>(),
        40
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT>(),
        8
    );
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE>(), 8);
    assert_eq!(size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>(), 56);
    assert_eq!(align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION>(), 8);
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>(),
        72
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>(),
        72
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE>(),
        8
    );
    assert_eq!(
        size_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>(),
        64
    );
    assert_eq!(
        align_of::<D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION>(),
        8
    );
    assert_eq!(size_of::<D3DENCRYPTED_BLOCK_INFO>(), 12);
    assert_eq!(align_of::<D3DENCRYPTED_BLOCK_INFO>(), 4);
    assert_eq!(size_of::<D3DAES_CTR_IV>(), 16);
    assert_eq!(align_of::<D3DAES_CTR_IV>(), 8);
}
#[cfg(feature = "dcomptypes")]
#[test]
fn shared_dcomptypes() {
    use winapi::shared::dcomptypes::*;
    assert_eq!(size_of::<DCOMPOSITION_FRAME_STATISTICS>(), 40);
    assert_eq!(align_of::<DCOMPOSITION_FRAME_STATISTICS>(), 8);
}
#[cfg(feature = "devpropdef")]
#[test]
fn shared_devpropdef() {
    use winapi::shared::devpropdef::*;
    assert_eq!(size_of::<DEVPROPKEY>(), 20);
    assert_eq!(align_of::<DEVPROPKEY>(), 4);
    assert_eq!(size_of::<DEVPROPCOMPKEY>(), 32);
    assert_eq!(align_of::<DEVPROPCOMPKEY>(), 8);
    assert_eq!(size_of::<DEVPROPERTY>(), 48);
    assert_eq!(align_of::<DEVPROPERTY>(), 8);
}
#[cfg(feature = "dxgi")]
#[test]
fn shared_dxgi() {
    use winapi::shared::dxgi::*;
    assert_eq!(size_of::<DXGI_FRAME_STATISTICS>(), 32);
    assert_eq!(align_of::<DXGI_FRAME_STATISTICS>(), 8);
    assert_eq!(size_of::<DXGI_MAPPED_RECT>(), 16);
    assert_eq!(align_of::<DXGI_MAPPED_RECT>(), 8);
    assert_eq!(size_of::<DXGI_ADAPTER_DESC>(), 304);
    assert_eq!(align_of::<DXGI_ADAPTER_DESC>(), 8);
    assert_eq!(size_of::<DXGI_OUTPUT_DESC>(), 96);
    assert_eq!(align_of::<DXGI_OUTPUT_DESC>(), 8);
    assert_eq!(size_of::<DXGI_SHARED_RESOURCE>(), 8);
    assert_eq!(align_of::<DXGI_SHARED_RESOURCE>(), 8);
    assert_eq!(size_of::<DXGI_SURFACE_DESC>(), 20);
    assert_eq!(align_of::<DXGI_SURFACE_DESC>(), 4);
    assert_eq!(size_of::<DXGI_SWAP_CHAIN_DESC>(), 72);
    assert_eq!(align_of::<DXGI_SWAP_CHAIN_DESC>(), 8);
    assert_eq!(size_of::<DXGI_ADAPTER_DESC1>(), 312);
    assert_eq!(align_of::<DXGI_ADAPTER_DESC1>(), 8);
    assert_eq!(size_of::<DXGI_DISPLAY_COLOR_SPACE>(), 192);
    assert_eq!(align_of::<DXGI_DISPLAY_COLOR_SPACE>(), 4);
}
#[cfg(feature = "dxgi1_2")]
#[test]
fn shared_dxgi1_2() {
    use winapi::shared::dxgi1_2::*;
    assert_eq!(size_of::<DXGI_ADAPTER_DESC2>(), 320);
    assert_eq!(align_of::<DXGI_ADAPTER_DESC2>(), 8);
    assert_eq!(size_of::<DXGI_MODE_DESC1>(), 32);
    assert_eq!(align_of::<DXGI_MODE_DESC1>(), 4);
    assert_eq!(size_of::<DXGI_OUTDUPL_DESC>(), 36);
    assert_eq!(align_of::<DXGI_OUTDUPL_DESC>(), 4);
    assert_eq!(size_of::<DXGI_OUTDUPL_FRAME_INFO>(), 48);
    assert_eq!(align_of::<DXGI_OUTDUPL_FRAME_INFO>(), 8);
    assert_eq!(size_of::<DXGI_OUTDUPL_MOVE_RECT>(), 24);
    assert_eq!(align_of::<DXGI_OUTDUPL_MOVE_RECT>(), 4);
    assert_eq!(size_of::<DXGI_OUTDUPL_POINTER_POSITION>(), 12);
    assert_eq!(align_of::<DXGI_OUTDUPL_POINTER_POSITION>(), 4);
    assert_eq!(size_of::<DXGI_OUTDUPL_POINTER_SHAPE_INFO>(), 24);
    assert_eq!(align_of::<DXGI_OUTDUPL_POINTER_SHAPE_INFO>(), 4);
    assert_eq!(size_of::<DXGI_PRESENT_PARAMETERS>(), 32);
    assert_eq!(align_of::<DXGI_PRESENT_PARAMETERS>(), 8);
    assert_eq!(size_of::<DXGI_SWAP_CHAIN_DESC1>(), 48);
    assert_eq!(align_of::<DXGI_SWAP_CHAIN_DESC1>(), 4);
    assert_eq!(size_of::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(), 20);
    assert_eq!(align_of::<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>(), 4);
}
#[cfg(feature = "dxgi1_3")]
#[test]
fn shared_dxgi1_3() {
    use winapi::shared::dxgi1_3::*;
    assert_eq!(size_of::<DXGI_DECODE_SWAP_CHAIN_DESC>(), 4);
    assert_eq!(align_of::<DXGI_DECODE_SWAP_CHAIN_DESC>(), 4);
    assert_eq!(size_of::<DXGI_FRAME_STATISTICS_MEDIA>(), 40);
    assert_eq!(align_of::<DXGI_FRAME_STATISTICS_MEDIA>(), 8);
    assert_eq!(size_of::<DXGI_MATRIX_3X2_F>(), 24);
    assert_eq!(align_of::<DXGI_MATRIX_3X2_F>(), 4);
}
#[cfg(feature = "dxgi1_4")]
#[test]
fn shared_dxgi1_4() {
    use winapi::shared::dxgi1_4::*;
    assert_eq!(size_of::<DXGI_QUERY_VIDEO_MEMORY_INFO>(), 32);
    assert_eq!(align_of::<DXGI_QUERY_VIDEO_MEMORY_INFO>(), 8);
}
#[cfg(feature = "dxgi1_5")]
#[test]
fn shared_dxgi1_5() {
    use winapi::shared::dxgi1_5::*;
    assert_eq!(size_of::<DXGI_HDR_METADATA_HDR10>(), 28);
    assert_eq!(align_of::<DXGI_HDR_METADATA_HDR10>(), 4);
}
#[cfg(feature = "dxgi1_6")]
#[test]
fn shared_dxgi1_6() {
    use winapi::shared::dxgi1_6::*;
    assert_eq!(size_of::<DXGI_ADAPTER_DESC3>(), 320);
    assert_eq!(align_of::<DXGI_ADAPTER_DESC3>(), 8);
    assert_eq!(size_of::<DXGI_OUTPUT_DESC1>(), 152);
    assert_eq!(align_of::<DXGI_OUTPUT_DESC1>(), 8);
}
#[cfg(feature = "dxgitype")]
#[test]
fn shared_dxgitype() {
    use winapi::shared::dxgitype::*;
    assert_eq!(size_of::<DXGI_RGB>(), 12);
    assert_eq!(align_of::<DXGI_RGB>(), 4);
    assert_eq!(size_of::<DXGI_GAMMA_CONTROL>(), 12324);
    assert_eq!(align_of::<DXGI_GAMMA_CONTROL>(), 4);
    assert_eq!(size_of::<DXGI_GAMMA_CONTROL_CAPABILITIES>(), 4116);
    assert_eq!(align_of::<DXGI_GAMMA_CONTROL_CAPABILITIES>(), 4);
    assert_eq!(size_of::<DXGI_RATIONAL>(), 8);
    assert_eq!(align_of::<DXGI_RATIONAL>(), 4);
    assert_eq!(size_of::<DXGI_MODE_DESC>(), 28);
    assert_eq!(align_of::<DXGI_MODE_DESC>(), 4);
    assert_eq!(size_of::<DXGI_SAMPLE_DESC>(), 8);
    assert_eq!(align_of::<DXGI_SAMPLE_DESC>(), 4);
    assert_eq!(size_of::<DXGI_JPEG_DC_HUFFMAN_TABLE>(), 24);
    assert_eq!(align_of::<DXGI_JPEG_DC_HUFFMAN_TABLE>(), 1);
    assert_eq!(size_of::<DXGI_JPEG_AC_HUFFMAN_TABLE>(), 178);
    assert_eq!(align_of::<DXGI_JPEG_AC_HUFFMAN_TABLE>(), 1);
    assert_eq!(size_of::<DXGI_JPEG_QUANTIZATION_TABLE>(), 64);
    assert_eq!(align_of::<DXGI_JPEG_QUANTIZATION_TABLE>(), 1);
}
#[cfg(feature = "evntprov")]
#[test]
fn shared_evntprov() {
    use winapi::shared::evntprov::*;
    assert_eq!(size_of::<EVENT_DATA_DESCRIPTOR_u_s>(), 4);
    assert_eq!(align_of::<EVENT_DATA_DESCRIPTOR_u_s>(), 2);
    assert_eq!(size_of::<EVENT_DATA_DESCRIPTOR_u>(), 4);
    assert_eq!(align_of::<EVENT_DATA_DESCRIPTOR_u>(), 4);
    assert_eq!(size_of::<EVENT_DATA_DESCRIPTOR>(), 16);
    assert_eq!(align_of::<EVENT_DATA_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<EVENT_DESCRIPTOR>(), 16);
    assert_eq!(align_of::<EVENT_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<EVENT_FILTER_DESCRIPTOR>(), 16);
    assert_eq!(align_of::<EVENT_FILTER_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<EVENT_FILTER_HEADER>(), 24);
    assert_eq!(align_of::<EVENT_FILTER_HEADER>(), 8);
    assert_eq!(size_of::<EVENT_FILTER_EVENT_ID>(), 6);
    assert_eq!(align_of::<EVENT_FILTER_EVENT_ID>(), 2);
    assert_eq!(size_of::<EVENT_FILTER_EVENT_NAME>(), 24);
    assert_eq!(align_of::<EVENT_FILTER_EVENT_NAME>(), 8);
    assert_eq!(size_of::<EVENT_FILTER_LEVEL_KW>(), 24);
    assert_eq!(align_of::<EVENT_FILTER_LEVEL_KW>(), 8);
}
#[cfg(feature = "evntrace")]
#[test]
fn shared_evntrace() {
    use winapi::shared::evntrace::*;
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u1_s>(), 2);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u1_s>(), 1);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u1>(), 2);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u1>(), 2);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u2_CLASS>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u2_CLASS>(), 2);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u2>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u2>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u3>(), 16);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u3>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u4_s1>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u4_s1>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u4_s2>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u4_s2>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_HEADER_u4>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_HEADER_u4>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_HEADER>(), 48);
    assert_eq!(align_of::<EVENT_TRACE_HEADER>(), 8);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u1_s>(), 2);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u1_s>(), 1);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u1>(), 2);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u1>(), 2);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u2_CLASS>(), 4);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u2_CLASS>(), 2);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u2>(), 4);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u2>(), 4);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u3_s1>(), 8);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u3_s1>(), 4);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u3_s2>(), 8);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u3_s2>(), 4);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER_u3>(), 8);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER_u3>(), 8);
    assert_eq!(size_of::<EVENT_INSTANCE_HEADER>(), 56);
    assert_eq!(align_of::<EVENT_INSTANCE_HEADER>(), 8);
    assert_eq!(size_of::<MOF_FIELD>(), 16);
    assert_eq!(align_of::<MOF_FIELD>(), 8);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER_u1_VERSIONDETAIL>(), 4);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER_u1_VERSIONDETAIL>(), 1);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER_u1>(), 4);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER_u1>(), 4);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER_u2_s>(), 16);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER_u2_s>(), 4);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER_u2>(), 16);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER_u2>(), 4);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER>(), 280);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER>(), 8);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER32>(), 272);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER32>(), 8);
    assert_eq!(size_of::<TRACE_LOGFILE_HEADER64>(), 280);
    assert_eq!(align_of::<TRACE_LOGFILE_HEADER64>(), 8);
    assert_eq!(size_of::<EVENT_INSTANCE_INFO>(), 16);
    assert_eq!(align_of::<EVENT_INSTANCE_INFO>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_u>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_u>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES>(), 120);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_V2_u1>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_V2_u1>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_V2_u2_s>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_V2_u2_s>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_V2_u2>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_V2_u2>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_V2_u3_s>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_V2_u3_s>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_V2_u3>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_V2_u3>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_PROPERTIES_V2>(), 144);
    assert_eq!(align_of::<EVENT_TRACE_PROPERTIES_V2>(), 8);
    assert_eq!(size_of::<TRACE_GUID_REGISTRATION>(), 16);
    assert_eq!(align_of::<TRACE_GUID_REGISTRATION>(), 8);
    assert_eq!(size_of::<TRACE_GUID_PROPERTIES>(), 36);
    assert_eq!(align_of::<TRACE_GUID_PROPERTIES>(), 4);
    assert_eq!(size_of::<ETW_BUFFER_CONTEXT_u_s>(), 2);
    assert_eq!(align_of::<ETW_BUFFER_CONTEXT_u_s>(), 1);
    assert_eq!(size_of::<ETW_BUFFER_CONTEXT_u>(), 2);
    assert_eq!(align_of::<ETW_BUFFER_CONTEXT_u>(), 2);
    assert_eq!(size_of::<ETW_BUFFER_CONTEXT>(), 4);
    assert_eq!(align_of::<ETW_BUFFER_CONTEXT>(), 2);
    assert_eq!(size_of::<TRACE_ENABLE_INFO>(), 32);
    assert_eq!(align_of::<TRACE_ENABLE_INFO>(), 8);
    assert_eq!(size_of::<TRACE_PROVIDER_INSTANCE_INFO>(), 16);
    assert_eq!(align_of::<TRACE_PROVIDER_INSTANCE_INFO>(), 4);
    assert_eq!(size_of::<TRACE_GUID_INFO>(), 8);
    assert_eq!(align_of::<TRACE_GUID_INFO>(), 4);
    assert_eq!(size_of::<PROFILE_SOURCE_INFO>(), 32);
    assert_eq!(align_of::<PROFILE_SOURCE_INFO>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_u>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_u>(), 4);
    assert_eq!(size_of::<EVENT_TRACE>(), 88);
    assert_eq!(align_of::<EVENT_TRACE>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_LOGFILE_u1>(), 4);
    assert_eq!(align_of::<EVENT_TRACE_LOGFILE_u1>(), 4);
    assert_eq!(size_of::<EVENT_TRACE_LOGFILE_u2>(), 8);
    assert_eq!(align_of::<EVENT_TRACE_LOGFILE_u2>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_LOGFILEW>(), 448);
    assert_eq!(align_of::<EVENT_TRACE_LOGFILEW>(), 8);
    assert_eq!(size_of::<EVENT_TRACE_LOGFILEA>(), 448);
    assert_eq!(align_of::<EVENT_TRACE_LOGFILEA>(), 8);
    assert_eq!(size_of::<ENABLE_TRACE_PARAMETERS_V1>(), 40);
    assert_eq!(align_of::<ENABLE_TRACE_PARAMETERS_V1>(), 8);
    assert_eq!(size_of::<ENABLE_TRACE_PARAMETERS>(), 48);
    assert_eq!(align_of::<ENABLE_TRACE_PARAMETERS>(), 8);
    assert_eq!(size_of::<CLASSIC_EVENT_ID>(), 24);
    assert_eq!(align_of::<CLASSIC_EVENT_ID>(), 4);
    assert_eq!(size_of::<TRACE_PROFILE_INTERVAL>(), 8);
    assert_eq!(align_of::<TRACE_PROFILE_INTERVAL>(), 4);
    assert_eq!(size_of::<TRACE_VERSION_INFO>(), 8);
    assert_eq!(align_of::<TRACE_VERSION_INFO>(), 4);
    assert_eq!(size_of::<TRACE_PERIODIC_CAPTURE_STATE_INFO>(), 8);
    assert_eq!(align_of::<TRACE_PERIODIC_CAPTURE_STATE_INFO>(), 4);
    assert_eq!(size_of::<ETW_TRACE_PARTITION_INFORMATION>(), 48);
    assert_eq!(align_of::<ETW_TRACE_PARTITION_INFORMATION>(), 8);
}
#[cfg(feature = "guiddef")]
#[test]
fn shared_guiddef() {
    use winapi::shared::guiddef::*;
    assert_eq!(size_of::<GUID>(), 16);
    assert_eq!(align_of::<GUID>(), 4);
}
#[cfg(feature = "hidclass")]
#[test]
fn shared_hidclass() {
    use winapi::shared::hidclass::*;
    assert_eq!(size_of::<HID_XFER_PACKET>(), 16);
    assert_eq!(align_of::<HID_XFER_PACKET>(), 8);
    assert_eq!(size_of::<HID_COLLECTION_INFORMATION>(), 12);
    assert_eq!(align_of::<HID_COLLECTION_INFORMATION>(), 4);
    assert_eq!(size_of::<HID_DRIVER_CONFIG>(), 8);
    assert_eq!(align_of::<HID_DRIVER_CONFIG>(), 4);
}
#[cfg(feature = "hidpi")]
#[test]
fn shared_hidpi() {
    use winapi::shared::hidpi::*;
    assert_eq!(size_of::<USAGE_AND_PAGE>(), 4);
    assert_eq!(align_of::<USAGE_AND_PAGE>(), 2);
    assert_eq!(size_of::<HIDP_BUTTON_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_BUTTON_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_VALUE_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_VALUE_CAPS>(), 4);
    // packed == 4
    // assert_eq!(size_of::<HIDP_LINK_COLLECTION_NODE>(), 24);
    // assert_eq!(align_of::<HIDP_LINK_COLLECTION_NODE>(), 4);
    assert_eq!(size_of::<HIDP_CAPS>(), 64);
    assert_eq!(align_of::<HIDP_CAPS>(), 2);
    assert_eq!(size_of::<HIDP_DATA>(), 8);
    assert_eq!(align_of::<HIDP_DATA>(), 4);
    assert_eq!(size_of::<HIDP_UNKNOWN_TOKEN>(), 8);
    assert_eq!(align_of::<HIDP_UNKNOWN_TOKEN>(), 4);
    // packed == 4
    // assert_eq!(size_of::<HIDP_EXTENDED_ATTRIBUTES>(), 16);
    // assert_eq!(align_of::<HIDP_EXTENDED_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
    assert_eq!(align_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
}
#[cfg(feature = "hidsdi")]
#[test]
fn shared_hidsdi() {
    use winapi::shared::hidsdi::*;
    // packed == 4
    // assert_eq!(size_of::<HIDD_CONFIGURATION>(), 16);
    // assert_eq!(align_of::<HIDD_CONFIGURATION>(), 4);
    assert_eq!(size_of::<HIDD_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDD_ATTRIBUTES>(), 4);
}
#[cfg(feature = "ifdef")]
#[test]
fn shared_ifdef() {
    use winapi::shared::ifdef::*;
    assert_eq!(size_of::<NET_LUID_LH>(), 8);
    assert_eq!(align_of::<NET_LUID_LH>(), 8);
    assert_eq!(size_of::<NET_IF_RCV_ADDRESS_LH>(), 8);
    assert_eq!(align_of::<NET_IF_RCV_ADDRESS_LH>(), 4);
    assert_eq!(size_of::<NET_IF_ALIAS_LH>(), 4);
    assert_eq!(align_of::<NET_IF_ALIAS_LH>(), 2);
    assert_eq!(size_of::<NET_PHYSICAL_LOCATION_LH>(), 12);
    assert_eq!(align_of::<NET_PHYSICAL_LOCATION_LH>(), 4);
    assert_eq!(size_of::<IF_COUNTED_STRING_LH>(), 516);
    assert_eq!(align_of::<IF_COUNTED_STRING_LH>(), 2);
    assert_eq!(size_of::<IF_PHYSICAL_ADDRESS_LH>(), 34);
    assert_eq!(align_of::<IF_PHYSICAL_ADDRESS_LH>(), 2);
    assert_eq!(size_of::<NDIS_INTERFACE_INFORMATION>(), 216);
    assert_eq!(align_of::<NDIS_INTERFACE_INFORMATION>(), 8);
}
#[cfg(feature = "ifmib")]
#[test]
fn shared_ifmib() {
    use winapi::shared::ifmib::*;
    assert_eq!(size_of::<MIB_IFNUMBER>(), 4);
    assert_eq!(align_of::<MIB_IFNUMBER>(), 4);
    assert_eq!(size_of::<MIB_IFROW>(), 860);
    assert_eq!(align_of::<MIB_IFROW>(), 4);
    assert_eq!(size_of::<MIB_IFTABLE>(), 864);
    assert_eq!(align_of::<MIB_IFTABLE>(), 4);
}
#[cfg(feature = "in6addr")]
#[test]
fn shared_in6addr() {
    use winapi::shared::in6addr::*;
    assert_eq!(size_of::<in6_addr_u>(), 16);
    assert_eq!(align_of::<in6_addr_u>(), 2);
    assert_eq!(size_of::<in6_addr>(), 16);
    assert_eq!(align_of::<in6_addr>(), 2);
}
#[cfg(feature = "inaddr")]
#[test]
fn shared_inaddr() {
    use winapi::shared::inaddr::*;
    assert_eq!(size_of::<in_addr_S_un_b>(), 4);
    assert_eq!(align_of::<in_addr_S_un_b>(), 1);
    assert_eq!(size_of::<in_addr_S_un_w>(), 4);
    assert_eq!(align_of::<in_addr_S_un_w>(), 2);
    assert_eq!(size_of::<in_addr_S_un>(), 4);
    assert_eq!(align_of::<in_addr_S_un>(), 4);
    assert_eq!(size_of::<in_addr>(), 4);
    assert_eq!(align_of::<in_addr>(), 4);
}
#[cfg(feature = "ipmib")]
#[test]
fn shared_ipmib() {
    use winapi::shared::ipmib::*;
    assert_eq!(size_of::<MIB_IPADDRROW_XP>(), 24);
    assert_eq!(align_of::<MIB_IPADDRROW_XP>(), 4);
    assert_eq!(size_of::<MIB_IPADDRROW_W2K>(), 24);
    assert_eq!(align_of::<MIB_IPADDRROW_W2K>(), 4);
    assert_eq!(size_of::<MIB_IPADDRTABLE>(), 28);
    assert_eq!(align_of::<MIB_IPADDRTABLE>(), 4);
    assert_eq!(size_of::<MIB_IPFORWARDNUMBER>(), 4);
    assert_eq!(align_of::<MIB_IPFORWARDNUMBER>(), 4);
    assert_eq!(size_of::<MIB_IPFORWARDROW>(), 56);
    assert_eq!(align_of::<MIB_IPFORWARDROW>(), 4);
    assert_eq!(size_of::<MIB_IPFORWARDTABLE>(), 60);
    assert_eq!(align_of::<MIB_IPFORWARDTABLE>(), 4);
    assert_eq!(size_of::<MIB_IPNETROW_LH>(), 24);
    assert_eq!(align_of::<MIB_IPNETROW_LH>(), 4);
    assert_eq!(size_of::<MIB_IPNETROW_W2K>(), 24);
    assert_eq!(align_of::<MIB_IPNETROW_W2K>(), 4);
    assert_eq!(size_of::<MIB_IPNETTABLE>(), 28);
    assert_eq!(align_of::<MIB_IPNETTABLE>(), 4);
    assert_eq!(size_of::<MIB_IPSTATS_LH>(), 92);
    assert_eq!(align_of::<MIB_IPSTATS_LH>(), 4);
    assert_eq!(size_of::<MIB_IPSTATS_W2K>(), 92);
    assert_eq!(align_of::<MIB_IPSTATS_W2K>(), 4);
    assert_eq!(size_of::<MIBICMPSTATS>(), 52);
    assert_eq!(align_of::<MIBICMPSTATS>(), 4);
    assert_eq!(size_of::<MIBICMPINFO>(), 104);
    assert_eq!(align_of::<MIBICMPINFO>(), 4);
    assert_eq!(size_of::<MIB_ICMP>(), 104);
    assert_eq!(align_of::<MIB_ICMP>(), 4);
    assert_eq!(size_of::<MIBICMPSTATS_EX_XPSP1>(), 1032);
    assert_eq!(align_of::<MIBICMPSTATS_EX_XPSP1>(), 4);
    assert_eq!(size_of::<MIB_ICMP_EX_XPSP1>(), 2064);
    assert_eq!(align_of::<MIB_ICMP_EX_XPSP1>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_OIF_XP>(), 16);
    assert_eq!(align_of::<MIB_IPMCAST_OIF_XP>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_OIF_W2K>(), 24);
    assert_eq!(align_of::<MIB_IPMCAST_OIF_W2K>(), 8);
    assert_eq!(size_of::<MIB_IPMCAST_MFE>(), 76);
    assert_eq!(align_of::<MIB_IPMCAST_MFE>(), 4);
    assert_eq!(size_of::<MIB_MFE_TABLE>(), 80);
    assert_eq!(align_of::<MIB_MFE_TABLE>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_OIF_STATS_LH>(), 28);
    assert_eq!(align_of::<MIB_IPMCAST_OIF_STATS_LH>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_OIF_STATS_W2K>(), 32);
    assert_eq!(align_of::<MIB_IPMCAST_OIF_STATS_W2K>(), 8);
    assert_eq!(size_of::<MIB_IPMCAST_MFE_STATS>(), 92);
    assert_eq!(align_of::<MIB_IPMCAST_MFE_STATS>(), 4);
    assert_eq!(size_of::<MIB_MFE_STATS_TABLE>(), 96);
    assert_eq!(align_of::<MIB_MFE_STATS_TABLE>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_MFE_STATS_EX_XP>(), 112);
    assert_eq!(align_of::<MIB_IPMCAST_MFE_STATS_EX_XP>(), 4);
    assert_eq!(size_of::<MIB_MFE_STATS_TABLE_EX_XP>(), 16);
    assert_eq!(align_of::<MIB_MFE_STATS_TABLE_EX_XP>(), 8);
    assert_eq!(size_of::<MIB_IPMCAST_GLOBAL>(), 4);
    assert_eq!(align_of::<MIB_IPMCAST_GLOBAL>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_IF_ENTRY>(), 24);
    assert_eq!(align_of::<MIB_IPMCAST_IF_ENTRY>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_IF_TABLE>(), 28);
    assert_eq!(align_of::<MIB_IPMCAST_IF_TABLE>(), 4);
}
#[cfg(feature = "iprtrmib")]
#[test]
fn shared_iprtrmib() {
    use winapi::shared::iprtrmib::*;
    assert_eq!(size_of::<MIB_OPAQUE_QUERY>(), 8);
    assert_eq!(align_of::<MIB_OPAQUE_QUERY>(), 4);
    assert_eq!(size_of::<TCPIP_OWNER_MODULE_BASIC_INFO>(), 16);
    assert_eq!(align_of::<TCPIP_OWNER_MODULE_BASIC_INFO>(), 8);
    assert_eq!(size_of::<MIB_IPMCAST_BOUNDARY>(), 16);
    assert_eq!(align_of::<MIB_IPMCAST_BOUNDARY>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_BOUNDARY_TABLE>(), 20);
    assert_eq!(align_of::<MIB_IPMCAST_BOUNDARY_TABLE>(), 4);
    assert_eq!(size_of::<MIB_BOUNDARYROW>(), 8);
    assert_eq!(align_of::<MIB_BOUNDARYROW>(), 4);
    assert_eq!(size_of::<MIB_MCAST_LIMIT_ROW>(), 8);
    assert_eq!(align_of::<MIB_MCAST_LIMIT_ROW>(), 4);
    assert_eq!(size_of::<MIB_IPMCAST_SCOPE>(), 524);
    assert_eq!(align_of::<MIB_IPMCAST_SCOPE>(), 4);
    assert_eq!(size_of::<MIB_IPDESTROW>(), 64);
    assert_eq!(align_of::<MIB_IPDESTROW>(), 4);
    assert_eq!(size_of::<MIB_IPDESTTABLE>(), 68);
    assert_eq!(align_of::<MIB_IPDESTTABLE>(), 4);
    assert_eq!(size_of::<MIB_BEST_IF>(), 8);
    assert_eq!(align_of::<MIB_BEST_IF>(), 4);
    assert_eq!(size_of::<MIB_PROXYARP>(), 12);
    assert_eq!(align_of::<MIB_PROXYARP>(), 4);
    assert_eq!(size_of::<MIB_IFSTATUS>(), 20);
    assert_eq!(align_of::<MIB_IFSTATUS>(), 4);
    assert_eq!(size_of::<MIB_ROUTESTATE>(), 4);
    assert_eq!(align_of::<MIB_ROUTESTATE>(), 4);
    assert_eq!(size_of::<MIB_OPAQUE_INFO>(), 16);
    assert_eq!(align_of::<MIB_OPAQUE_INFO>(), 8);
}
#[cfg(feature = "ktmtypes")]
#[test]
fn shared_ktmtypes() {
    use winapi::shared::ktmtypes::*;
    assert_eq!(size_of::<TRANSACTION_NOTIFICATION>(), 32);
    assert_eq!(align_of::<TRANSACTION_NOTIFICATION>(), 8);
    assert_eq!(size_of::<TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT>(), 32);
    assert_eq!(align_of::<TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT>(), 4);
    assert_eq!(size_of::<TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT>(), 20);
    assert_eq!(align_of::<TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT>(), 4);
    assert_eq!(size_of::<TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT>(), 4);
    assert_eq!(align_of::<TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT>(), 4);
    assert_eq!(size_of::<TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT>(), 40);
    assert_eq!(align_of::<TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT>(), 4);
    assert_eq!(size_of::<TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT>(), 20);
    assert_eq!(align_of::<TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT>(), 4);
    assert_eq!(size_of::<KCRM_MARSHAL_HEADER>(), 16);
    assert_eq!(align_of::<KCRM_MARSHAL_HEADER>(), 4);
    assert_eq!(size_of::<KCRM_TRANSACTION_BLOB>(), 172);
    assert_eq!(align_of::<KCRM_TRANSACTION_BLOB>(), 4);
    assert_eq!(size_of::<KCRM_PROTOCOL_BLOB>(), 32);
    assert_eq!(align_of::<KCRM_PROTOCOL_BLOB>(), 4);
}
#[cfg(feature = "minwindef")]
#[test]
fn shared_minwindef() {
    use winapi::shared::minwindef::*;
    assert_eq!(size_of::<FILETIME>(), 8);
    assert_eq!(align_of::<FILETIME>(), 4);
}
#[cfg(feature = "mmreg")]
#[test]
fn shared_mmreg() {
    use winapi::shared::mmreg::*;
    assert_eq!(size_of::<WAVEFORMATEX>(), 18);
    assert_eq!(align_of::<WAVEFORMATEX>(), 1);
    assert_eq!(size_of::<WAVEFORMATEXTENSIBLE>(), 40);
    assert_eq!(align_of::<WAVEFORMATEXTENSIBLE>(), 1);
}
#[cfg(feature = "mstcpip")]
#[test]
fn shared_mstcpip() {
    use winapi::shared::mstcpip::*;
    assert_eq!(size_of::<TRANSPORT_SETTING_ID>(), 16);
    assert_eq!(align_of::<TRANSPORT_SETTING_ID>(), 4);
    assert_eq!(size_of::<tcp_keepalive>(), 12);
    assert_eq!(align_of::<tcp_keepalive>(), 4);
    assert_eq!(size_of::<REAL_TIME_NOTIFICATION_SETTING_INPUT>(), 32);
    assert_eq!(align_of::<REAL_TIME_NOTIFICATION_SETTING_INPUT>(), 4);
    assert_eq!(size_of::<REAL_TIME_NOTIFICATION_SETTING_INPUT_EX>(), 36);
    assert_eq!(align_of::<REAL_TIME_NOTIFICATION_SETTING_INPUT_EX>(), 4);
    assert_eq!(size_of::<REAL_TIME_NOTIFICATION_SETTING_OUTPUT>(), 4);
    assert_eq!(align_of::<REAL_TIME_NOTIFICATION_SETTING_OUTPUT>(), 4);
    assert_eq!(size_of::<ASSOCIATE_NAMERES_CONTEXT_INPUT>(), 24);
    assert_eq!(align_of::<ASSOCIATE_NAMERES_CONTEXT_INPUT>(), 8);
    assert_eq!(size_of::<RCVALL_IF>(), 8);
    assert_eq!(align_of::<RCVALL_IF>(), 4);
    assert_eq!(size_of::<TCP_INITIAL_RTO_PARAMETERS>(), 4);
    assert_eq!(align_of::<TCP_INITIAL_RTO_PARAMETERS>(), 2);
    assert_eq!(size_of::<TCP_ICW_PARAMETERS>(), 4);
    assert_eq!(align_of::<TCP_ICW_PARAMETERS>(), 4);
    assert_eq!(size_of::<TCP_ACK_FREQUENCY_PARAMETERS>(), 1);
    assert_eq!(align_of::<TCP_ACK_FREQUENCY_PARAMETERS>(), 1);
    assert_eq!(size_of::<TCP_INFO_v0>(), 88);
    assert_eq!(align_of::<TCP_INFO_v0>(), 8);
    assert_eq!(size_of::<INET_PORT_RANGE>(), 4);
    assert_eq!(align_of::<INET_PORT_RANGE>(), 2);
    assert_eq!(size_of::<INET_PORT_RESERVATION_TOKEN>(), 8);
    assert_eq!(align_of::<INET_PORT_RESERVATION_TOKEN>(), 8);
    assert_eq!(size_of::<INET_PORT_RESERVATION_INSTANCE>(), 16);
    assert_eq!(align_of::<INET_PORT_RESERVATION_INSTANCE>(), 8);
    assert_eq!(size_of::<INET_PORT_RESERVATION_INFORMATION>(), 4);
    assert_eq!(align_of::<INET_PORT_RESERVATION_INFORMATION>(), 4);
    assert_eq!(size_of::<SOCKET_SECURITY_SETTINGS>(), 8);
    assert_eq!(align_of::<SOCKET_SECURITY_SETTINGS>(), 4);
    assert_eq!(size_of::<SOCKET_SECURITY_SETTINGS_IPSEC>(), 88);
    assert_eq!(align_of::<SOCKET_SECURITY_SETTINGS_IPSEC>(), 8);
    assert_eq!(size_of::<SOCKET_PEER_TARGET_NAME>(), 144);
    assert_eq!(align_of::<SOCKET_PEER_TARGET_NAME>(), 8);
    assert_eq!(size_of::<SOCKET_SECURITY_QUERY_TEMPLATE>(), 144);
    assert_eq!(align_of::<SOCKET_SECURITY_QUERY_TEMPLATE>(), 8);
    assert_eq!(size_of::<SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2>(), 152);
    assert_eq!(align_of::<SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2>(), 8);
    assert_eq!(size_of::<SOCKET_SECURITY_QUERY_INFO>(), 24);
    assert_eq!(align_of::<SOCKET_SECURITY_QUERY_INFO>(), 8);
    assert_eq!(size_of::<SOCKET_SECURITY_QUERY_INFO_IPSEC2>(), 64);
    assert_eq!(align_of::<SOCKET_SECURITY_QUERY_INFO_IPSEC2>(), 8);
    assert_eq!(size_of::<RSS_SCALABILITY_INFO>(), 1);
    assert_eq!(align_of::<RSS_SCALABILITY_INFO>(), 1);
}
#[cfg(feature = "netioapi")]
#[test]
fn shared_netioapi() {
    use winapi::shared::netioapi::*;
    assert_eq!(size_of::<MIB_IF_ROW2>(), 1352);
    assert_eq!(align_of::<MIB_IF_ROW2>(), 8);
    assert_eq!(size_of::<MIB_IF_TABLE2>(), 1360);
    assert_eq!(align_of::<MIB_IF_TABLE2>(), 8);
    assert_eq!(size_of::<MIB_IPINTERFACE_ROW>(), 168);
    assert_eq!(align_of::<MIB_IPINTERFACE_ROW>(), 8);
    assert_eq!(size_of::<MIB_IPINTERFACE_TABLE>(), 176);
    assert_eq!(align_of::<MIB_IPINTERFACE_TABLE>(), 8);
    assert_eq!(size_of::<MIB_IFSTACK_ROW>(), 8);
    assert_eq!(align_of::<MIB_IFSTACK_ROW>(), 4);
    assert_eq!(size_of::<MIB_INVERTEDIFSTACK_ROW>(), 8);
    assert_eq!(align_of::<MIB_INVERTEDIFSTACK_ROW>(), 4);
    assert_eq!(size_of::<MIB_IFSTACK_TABLE>(), 12);
    assert_eq!(align_of::<MIB_IFSTACK_TABLE>(), 4);
    assert_eq!(size_of::<MIB_INVERTEDIFSTACK_TABLE>(), 12);
    assert_eq!(align_of::<MIB_INVERTEDIFSTACK_TABLE>(), 4);
    assert_eq!(
        size_of::<MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES>(),
        48
    );
    assert_eq!(
        align_of::<MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES>(),
        8
    );
    assert_eq!(size_of::<MIB_UNICASTIPADDRESS_ROW>(), 80);
    assert_eq!(align_of::<MIB_UNICASTIPADDRESS_ROW>(), 8);
    assert_eq!(size_of::<MIB_UNICASTIPADDRESS_TABLE>(), 88);
    assert_eq!(align_of::<MIB_UNICASTIPADDRESS_TABLE>(), 8);
    assert_eq!(size_of::<MIB_ANYCASTIPADDRESS_ROW>(), 48);
    assert_eq!(align_of::<MIB_ANYCASTIPADDRESS_ROW>(), 8);
    assert_eq!(size_of::<MIB_ANYCASTIPADDRESS_TABLE>(), 56);
    assert_eq!(align_of::<MIB_ANYCASTIPADDRESS_TABLE>(), 8);
    assert_eq!(size_of::<MIB_MULTICASTIPADDRESS_ROW>(), 48);
    assert_eq!(align_of::<MIB_MULTICASTIPADDRESS_ROW>(), 8);
    assert_eq!(size_of::<MIB_MULTICASTIPADDRESS_TABLE>(), 56);
    assert_eq!(align_of::<MIB_MULTICASTIPADDRESS_TABLE>(), 8);
    assert_eq!(size_of::<IP_ADDRESS_PREFIX>(), 32);
    assert_eq!(align_of::<IP_ADDRESS_PREFIX>(), 4);
    assert_eq!(size_of::<MIB_IPFORWARD_ROW2>(), 104);
    assert_eq!(align_of::<MIB_IPFORWARD_ROW2>(), 8);
    assert_eq!(size_of::<MIB_IPFORWARD_TABLE2>(), 112);
    assert_eq!(align_of::<MIB_IPFORWARD_TABLE2>(), 8);
    assert_eq!(size_of::<MIB_IPPATH_ROW>(), 136);
    assert_eq!(align_of::<MIB_IPPATH_ROW>(), 8);
    assert_eq!(size_of::<MIB_IPPATH_TABLE>(), 144);
    assert_eq!(align_of::<MIB_IPPATH_TABLE>(), 8);
    assert_eq!(size_of::<MIB_IPNET_ROW2>(), 88);
    assert_eq!(align_of::<MIB_IPNET_ROW2>(), 8);
    assert_eq!(size_of::<MIB_IPNET_TABLE2>(), 96);
    assert_eq!(align_of::<MIB_IPNET_TABLE2>(), 8);
    assert_eq!(size_of::<DNS_SETTINGS>(), 40);
    assert_eq!(align_of::<DNS_SETTINGS>(), 8);
    assert_eq!(size_of::<DNS_INTERFACE_SETTINGS>(), 64);
    assert_eq!(align_of::<DNS_INTERFACE_SETTINGS>(), 8);
}
#[cfg(feature = "nldef")]
#[test]
fn shared_nldef() {
    use winapi::shared::nldef::*;
    assert_eq!(size_of::<NL_INTERFACE_OFFLOAD_ROD>(), 1);
    assert_eq!(align_of::<NL_INTERFACE_OFFLOAD_ROD>(), 1);
    assert_eq!(size_of::<NL_PATH_BANDWIDTH_ROD>(), 24);
    assert_eq!(align_of::<NL_PATH_BANDWIDTH_ROD>(), 8);
    assert_eq!(size_of::<NL_BANDWIDTH_INFORMATION>(), 24);
    assert_eq!(align_of::<NL_BANDWIDTH_INFORMATION>(), 8);
}
#[cfg(feature = "ntddndis")]
#[test]
fn shared_ntddndis() {
    use winapi::shared::ntddndis::*;
    assert_eq!(size_of::<NDIS_OBJECT_HEADER>(), 4);
    assert_eq!(align_of::<NDIS_OBJECT_HEADER>(), 2);
}
#[cfg(feature = "ntddscsi")]
#[test]
fn shared_ntddscsi() {
    use winapi::shared::ntddscsi::*;
    assert_eq!(size_of::<SCSI_PASS_THROUGH>(), 56);
    assert_eq!(align_of::<SCSI_PASS_THROUGH>(), 8);
    assert_eq!(size_of::<SCSI_PASS_THROUGH_DIRECT>(), 56);
    assert_eq!(align_of::<SCSI_PASS_THROUGH_DIRECT>(), 8);
    assert_eq!(size_of::<SCSI_PASS_THROUGH32>(), 44);
    assert_eq!(align_of::<SCSI_PASS_THROUGH32>(), 4);
    assert_eq!(size_of::<SCSI_PASS_THROUGH_DIRECT32>(), 44);
    assert_eq!(align_of::<SCSI_PASS_THROUGH_DIRECT32>(), 4);
    assert_eq!(size_of::<SCSI_PASS_THROUGH_EX>(), 64);
    assert_eq!(align_of::<SCSI_PASS_THROUGH_EX>(), 8);
    assert_eq!(size_of::<SCSI_PASS_THROUGH_DIRECT_EX>(), 64);
    assert_eq!(align_of::<SCSI_PASS_THROUGH_DIRECT_EX>(), 8);
    assert_eq!(size_of::<SCSI_PASS_THROUGH32_EX>(), 52);
    assert_eq!(align_of::<SCSI_PASS_THROUGH32_EX>(), 4);
    assert_eq!(size_of::<SCSI_PASS_THROUGH_DIRECT32_EX>(), 52);
    assert_eq!(align_of::<SCSI_PASS_THROUGH_DIRECT32_EX>(), 4);
    assert_eq!(size_of::<ATA_PASS_THROUGH_EX>(), 48);
    assert_eq!(align_of::<ATA_PASS_THROUGH_EX>(), 8);
    assert_eq!(size_of::<ATA_PASS_THROUGH_DIRECT>(), 48);
    assert_eq!(align_of::<ATA_PASS_THROUGH_DIRECT>(), 8);
    assert_eq!(size_of::<ATA_PASS_THROUGH_EX32>(), 40);
    assert_eq!(align_of::<ATA_PASS_THROUGH_EX32>(), 4);
    assert_eq!(size_of::<ATA_PASS_THROUGH_DIRECT32>(), 40);
    assert_eq!(align_of::<ATA_PASS_THROUGH_DIRECT32>(), 4);
    assert_eq!(size_of::<IDE_IO_CONTROL>(), 28);
    assert_eq!(align_of::<IDE_IO_CONTROL>(), 4);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH>(), 72);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH_DIRECT>(), 72);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH_DIRECT>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH_EX>(), 24);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH_EX>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH_DIRECT_EX>(), 24);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH_DIRECT_EX>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH32>(), 64);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH32>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH_DIRECT32>(), 64);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH_DIRECT32>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH32_EX>(), 24);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH32_EX>(), 8);
    assert_eq!(size_of::<MPIO_PASS_THROUGH_PATH_DIRECT32_EX>(), 24);
    assert_eq!(align_of::<MPIO_PASS_THROUGH_PATH_DIRECT32_EX>(), 8);
    assert_eq!(size_of::<SCSI_BUS_DATA>(), 8);
    assert_eq!(align_of::<SCSI_BUS_DATA>(), 4);
    assert_eq!(size_of::<SCSI_ADAPTER_BUS_INFO>(), 12);
    assert_eq!(align_of::<SCSI_ADAPTER_BUS_INFO>(), 4);
    assert_eq!(size_of::<SCSI_INQUIRY_DATA>(), 16);
    assert_eq!(align_of::<SCSI_INQUIRY_DATA>(), 4);
    assert_eq!(size_of::<SRB_IO_CONTROL>(), 28);
    assert_eq!(align_of::<SRB_IO_CONTROL>(), 4);
    assert_eq!(size_of::<NVCACHE_REQUEST_BLOCK>(), 48);
    assert_eq!(align_of::<NVCACHE_REQUEST_BLOCK>(), 8);
    assert_eq!(size_of::<NV_FEATURE_PARAMETER>(), 24);
    assert_eq!(align_of::<NV_FEATURE_PARAMETER>(), 4);
    assert_eq!(size_of::<NVCACHE_HINT_PAYLOAD>(), 16);
    assert_eq!(align_of::<NVCACHE_HINT_PAYLOAD>(), 1);
    assert_eq!(size_of::<NV_SEP_CACHE_PARAMETER>(), 16);
    assert_eq!(align_of::<NV_SEP_CACHE_PARAMETER>(), 4);
    assert_eq!(size_of::<NV_SEP_CACHE_PARAMETER_Flags>(), 1);
    assert_eq!(align_of::<NV_SEP_CACHE_PARAMETER_Flags>(), 1);
    assert_eq!(size_of::<NV_SEP_CACHE_PARAMETER_Flags_CacheFlags>(), 1);
    assert_eq!(align_of::<NV_SEP_CACHE_PARAMETER_Flags_CacheFlags>(), 1);
    assert_eq!(size_of::<MP_DEVICE_DATA_SET_RANGE>(), 16);
    assert_eq!(align_of::<MP_DEVICE_DATA_SET_RANGE>(), 8);
    assert_eq!(size_of::<DSM_NOTIFICATION_REQUEST_BLOCK>(), 48);
    assert_eq!(align_of::<DSM_NOTIFICATION_REQUEST_BLOCK>(), 8);
    assert_eq!(size_of::<HYBRID_REQUEST_BLOCK>(), 24);
    assert_eq!(align_of::<HYBRID_REQUEST_BLOCK>(), 4);
    assert_eq!(size_of::<NVCACHE_PRIORITY_LEVEL_DESCRIPTOR>(), 24);
    assert_eq!(align_of::<NVCACHE_PRIORITY_LEVEL_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<HYBRID_INFORMATION>(), 72);
    assert_eq!(align_of::<HYBRID_INFORMATION>(), 8);
    assert_eq!(size_of::<HYBRID_INFORMATION_Attributes>(), 4);
    assert_eq!(align_of::<HYBRID_INFORMATION_Attributes>(), 4);
    assert_eq!(size_of::<HYBRID_INFORMATION_Priorities>(), 28);
    assert_eq!(align_of::<HYBRID_INFORMATION_Priorities>(), 4);
    assert_eq!(
        size_of::<HYBRID_INFORMATION_Priorities_SupportedCommands>(),
        16
    );
    assert_eq!(
        align_of::<HYBRID_INFORMATION_Priorities_SupportedCommands>(),
        4
    );
    assert_eq!(size_of::<HYBRID_DIRTY_THRESHOLDS>(), 16);
    assert_eq!(align_of::<HYBRID_DIRTY_THRESHOLDS>(), 4);
    assert_eq!(size_of::<HYBRID_DEMOTE_BY_SIZE>(), 24);
    assert_eq!(align_of::<HYBRID_DEMOTE_BY_SIZE>(), 8);
    assert_eq!(size_of::<FIRMWARE_REQUEST_BLOCK>(), 24);
    assert_eq!(align_of::<FIRMWARE_REQUEST_BLOCK>(), 4);
    assert_eq!(size_of::<STORAGE_FIRMWARE_SLOT_INFO>(), 16);
    assert_eq!(align_of::<STORAGE_FIRMWARE_SLOT_INFO>(), 8);
    assert_eq!(size_of::<STORAGE_FIRMWARE_SLOT_INFO_Revision>(), 8);
    assert_eq!(align_of::<STORAGE_FIRMWARE_SLOT_INFO_Revision>(), 8);
    assert_eq!(size_of::<STORAGE_FIRMWARE_SLOT_INFO_V2>(), 24);
    assert_eq!(align_of::<STORAGE_FIRMWARE_SLOT_INFO_V2>(), 1);
    assert_eq!(size_of::<STORAGE_FIRMWARE_INFO>(), 16);
    assert_eq!(align_of::<STORAGE_FIRMWARE_INFO>(), 8);
    assert_eq!(size_of::<STORAGE_FIRMWARE_INFO_V2>(), 24);
    assert_eq!(align_of::<STORAGE_FIRMWARE_INFO_V2>(), 4);
    assert_eq!(size_of::<STORAGE_FIRMWARE_DOWNLOAD>(), 24);
    assert_eq!(align_of::<STORAGE_FIRMWARE_DOWNLOAD>(), 8);
    assert_eq!(size_of::<STORAGE_FIRMWARE_DOWNLOAD_V2>(), 32);
    assert_eq!(align_of::<STORAGE_FIRMWARE_DOWNLOAD_V2>(), 8);
    assert_eq!(size_of::<STORAGE_FIRMWARE_ACTIVATE>(), 12);
    assert_eq!(align_of::<STORAGE_FIRMWARE_ACTIVATE>(), 4);
    assert_eq!(size_of::<IO_SCSI_CAPABILITIES>(), 24);
    assert_eq!(align_of::<IO_SCSI_CAPABILITIES>(), 4);
    assert_eq!(size_of::<SCSI_ADDRESS>(), 8);
    assert_eq!(align_of::<SCSI_ADDRESS>(), 4);
    assert_eq!(size_of::<DUMP_POINTERS_VERSION>(), 8);
    assert_eq!(align_of::<DUMP_POINTERS_VERSION>(), 4);
    assert_eq!(size_of::<DUMP_POINTERS>(), 56);
    assert_eq!(align_of::<DUMP_POINTERS>(), 8);
    assert_eq!(size_of::<DUMP_POINTERS_EX>(), 104);
    assert_eq!(align_of::<DUMP_POINTERS_EX>(), 8);
    assert_eq!(size_of::<DUMP_DRIVER>(), 72);
    assert_eq!(align_of::<DUMP_DRIVER>(), 8);
}
#[cfg(feature = "ntdef")]
#[test]
fn shared_ntdef() {
    use winapi::shared::ntdef::*;
    assert_eq!(size_of::<QUAD>(), 8);
    assert_eq!(align_of::<QUAD>(), 8);
    assert_eq!(size_of::<PROCESSOR_NUMBER>(), 4);
    assert_eq!(align_of::<PROCESSOR_NUMBER>(), 2);
    assert_eq!(size_of::<GROUP_AFFINITY>(), 16);
    assert_eq!(align_of::<GROUP_AFFINITY>(), 8);
    assert_eq!(size_of::<FLOAT128>(), 16);
    assert_eq!(align_of::<FLOAT128>(), 8);
    assert_eq!(size_of::<LARGE_INTEGER>(), 8);
    assert_eq!(align_of::<LARGE_INTEGER>(), 8);
    assert_eq!(size_of::<LARGE_INTEGER_s>(), 8);
    assert_eq!(align_of::<LARGE_INTEGER_s>(), 4);
    assert_eq!(size_of::<LARGE_INTEGER_u>(), 8);
    assert_eq!(align_of::<LARGE_INTEGER_u>(), 4);
    assert_eq!(size_of::<ULARGE_INTEGER>(), 8);
    assert_eq!(align_of::<ULARGE_INTEGER>(), 8);
    assert_eq!(size_of::<ULARGE_INTEGER_s>(), 8);
    assert_eq!(align_of::<ULARGE_INTEGER_s>(), 4);
    assert_eq!(size_of::<ULARGE_INTEGER_u>(), 8);
    assert_eq!(align_of::<ULARGE_INTEGER_u>(), 4);
    assert_eq!(size_of::<LUID>(), 8);
    assert_eq!(align_of::<LUID>(), 4);
    assert_eq!(size_of::<STRING>(), 16);
    assert_eq!(align_of::<STRING>(), 8);
    assert_eq!(size_of::<CSTRING>(), 16);
    assert_eq!(align_of::<CSTRING>(), 8);
    assert_eq!(size_of::<UNICODE_STRING>(), 16);
    assert_eq!(align_of::<UNICODE_STRING>(), 8);
    assert_eq!(size_of::<LIST_ENTRY>(), 16);
    assert_eq!(align_of::<LIST_ENTRY>(), 8);
    assert_eq!(size_of::<SINGLE_LIST_ENTRY>(), 8);
    assert_eq!(align_of::<SINGLE_LIST_ENTRY>(), 8);
    assert_eq!(size_of::<RTL_BALANCED_NODE>(), 24);
    assert_eq!(align_of::<RTL_BALANCED_NODE>(), 8);
    assert_eq!(size_of::<RTL_BALANCED_NODE_u>(), 16);
    assert_eq!(align_of::<RTL_BALANCED_NODE_u>(), 8);
    assert_eq!(size_of::<RTL_BALANCED_NODE_s>(), 16);
    assert_eq!(align_of::<RTL_BALANCED_NODE_s>(), 8);
    assert_eq!(size_of::<LIST_ENTRY32>(), 8);
    assert_eq!(align_of::<LIST_ENTRY32>(), 4);
    assert_eq!(size_of::<LIST_ENTRY64>(), 16);
    assert_eq!(align_of::<LIST_ENTRY64>(), 8);
    assert_eq!(size_of::<SINGLE_LIST_ENTRY32>(), 4);
    assert_eq!(align_of::<SINGLE_LIST_ENTRY32>(), 4);
    assert_eq!(size_of::<WNF_STATE_NAME>(), 8);
    assert_eq!(align_of::<WNF_STATE_NAME>(), 4);
    assert_eq!(size_of::<STRING32>(), 8);
    assert_eq!(align_of::<STRING32>(), 4);
    assert_eq!(size_of::<STRING64>(), 16);
    assert_eq!(align_of::<STRING64>(), 8);
    assert_eq!(size_of::<OBJECT_ATTRIBUTES64>(), 48);
    assert_eq!(align_of::<OBJECT_ATTRIBUTES64>(), 8);
    assert_eq!(size_of::<OBJECT_ATTRIBUTES32>(), 24);
    assert_eq!(align_of::<OBJECT_ATTRIBUTES32>(), 4);
    assert_eq!(size_of::<OBJECT_ATTRIBUTES>(), 48);
    assert_eq!(align_of::<OBJECT_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<OBJECTID>(), 20);
    assert_eq!(align_of::<OBJECTID>(), 4);
}
#[cfg(feature = "qos")]
#[test]
fn shared_qos() {
    use winapi::shared::qos::*;
    assert_eq!(size_of::<FLOWSPEC>(), 32);
    assert_eq!(align_of::<FLOWSPEC>(), 4);
}
#[cfg(feature = "rpcdce")]
#[test]
fn shared_rpcdce() {
    use winapi::shared::rpcdce::*;
    assert_eq!(size_of::<RPC_BINDING_VECTOR>(), 16);
    assert_eq!(align_of::<RPC_BINDING_VECTOR>(), 8);
    assert_eq!(size_of::<UUID_VECTOR>(), 16);
    assert_eq!(align_of::<UUID_VECTOR>(), 8);
    assert_eq!(size_of::<RPC_IF_ID>(), 20);
    assert_eq!(align_of::<RPC_IF_ID>(), 4);
    assert_eq!(size_of::<RPC_PROTSEQ_VECTORA>(), 16);
    assert_eq!(align_of::<RPC_PROTSEQ_VECTORA>(), 8);
    assert_eq!(size_of::<RPC_PROTSEQ_VECTORW>(), 16);
    assert_eq!(align_of::<RPC_PROTSEQ_VECTORW>(), 8);
    assert_eq!(size_of::<RPC_POLICY>(), 12);
    assert_eq!(align_of::<RPC_POLICY>(), 4);
    assert_eq!(size_of::<RPC_STATS_VECTOR>(), 8);
    assert_eq!(align_of::<RPC_STATS_VECTOR>(), 4);
    assert_eq!(size_of::<RPC_IF_ID_VECTOR>(), 16);
    assert_eq!(align_of::<RPC_IF_ID_VECTOR>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS>(), 16);
    assert_eq!(align_of::<RPC_SECURITY_QOS>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_IDENTITY_W>(), 48);
    assert_eq!(align_of::<SEC_WINNT_AUTH_IDENTITY_W>(), 8);
    assert_eq!(size_of::<SEC_WINNT_AUTH_IDENTITY_A>(), 48);
    assert_eq!(align_of::<SEC_WINNT_AUTH_IDENTITY_A>(), 8);
    assert_eq!(size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_W>(), 40);
    assert_eq!(align_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_W>(), 8);
    assert_eq!(size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_A>(), 40);
    assert_eq!(align_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_A>(), 8);
    assert_eq!(size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W>(), 64);
    assert_eq!(align_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W>(), 8);
    assert_eq!(size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A>(), 64);
    assert_eq!(align_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A>(), 8);
    assert_eq!(size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W>(), 64);
    assert_eq!(align_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W>(), 8);
    assert_eq!(size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A>(), 64);
    assert_eq!(align_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V2_W_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V2_W_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V2_W>(), 32);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V2_W>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V2_A_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V2_A_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V2_A>(), 32);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V2_A>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V3_W_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V3_W_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V3_W>(), 40);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V3_W>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V3_A_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V3_A_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V3_A>(), 40);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V3_A>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V4_W_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V4_W_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V4_W>(), 48);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V4_W>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V4_A_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V4_A_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V4_A>(), 48);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V4_A>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V5_W_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V5_W_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V5_W>(), 56);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V5_W>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V5_A_union>(), 8);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V5_A_union>(), 8);
    assert_eq!(size_of::<RPC_SECURITY_QOS_V5_A>(), 56);
    assert_eq!(align_of::<RPC_SECURITY_QOS_V5_A>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_W_union>(), 8);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_W_union>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_W>(), 56);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_W>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_A_union>(), 8);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_A_union>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_A>(), 56);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_A>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_SECURITY_V1_W>(), 40);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_SECURITY_V1_W>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_SECURITY_V1_A>(), 40);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_SECURITY_V1_A>(), 8);
    assert_eq!(size_of::<RPC_BINDING_HANDLE_OPTIONS_V1>(), 16);
    assert_eq!(align_of::<RPC_BINDING_HANDLE_OPTIONS_V1>(), 4);
    assert_eq!(size_of::<RPC_CLIENT_INFORMATION1>(), 24);
    assert_eq!(align_of::<RPC_CLIENT_INFORMATION1>(), 8);
    assert_eq!(size_of::<RPC_ENDPOINT_TEMPLATEW>(), 40);
    assert_eq!(align_of::<RPC_ENDPOINT_TEMPLATEW>(), 8);
    assert_eq!(size_of::<RPC_ENDPOINT_TEMPLATEA>(), 40);
    assert_eq!(align_of::<RPC_ENDPOINT_TEMPLATEA>(), 8);
    assert_eq!(size_of::<RPC_INTERFACE_TEMPLATEA>(), 80);
    assert_eq!(align_of::<RPC_INTERFACE_TEMPLATEA>(), 8);
    assert_eq!(size_of::<RPC_INTERFACE_TEMPLATEW>(), 80);
    assert_eq!(align_of::<RPC_INTERFACE_TEMPLATEW>(), 8);
}
#[cfg(feature = "sspi")]
#[test]
fn shared_sspi() {
    use winapi::shared::sspi::*;
    assert_eq!(size_of::<SecHandle>(), 16);
    assert_eq!(align_of::<SecHandle>(), 8);
    assert_eq!(size_of::<SECURITY_STRING>(), 16);
    assert_eq!(align_of::<SECURITY_STRING>(), 8);
    assert_eq!(size_of::<SecPkgInfoW>(), 32);
    assert_eq!(align_of::<SecPkgInfoW>(), 8);
    assert_eq!(size_of::<SecPkgInfoA>(), 32);
    assert_eq!(align_of::<SecPkgInfoA>(), 8);
    assert_eq!(size_of::<SecBuffer>(), 16);
    assert_eq!(align_of::<SecBuffer>(), 8);
    assert_eq!(size_of::<SecBufferDesc>(), 16);
    assert_eq!(align_of::<SecBufferDesc>(), 8);
    assert_eq!(size_of::<SEC_NEGOTIATION_INFO>(), 24);
    assert_eq!(align_of::<SEC_NEGOTIATION_INFO>(), 8);
    assert_eq!(size_of::<SEC_CHANNEL_BINDINGS>(), 32);
    assert_eq!(align_of::<SEC_CHANNEL_BINDINGS>(), 4);
    assert_eq!(size_of::<SEC_APPLICATION_PROTOCOL_LIST>(), 8);
    assert_eq!(align_of::<SEC_APPLICATION_PROTOCOL_LIST>(), 4);
    assert_eq!(size_of::<SEC_APPLICATION_PROTOCOLS>(), 12);
    assert_eq!(align_of::<SEC_APPLICATION_PROTOCOLS>(), 4);
    assert_eq!(size_of::<SecPkgCredentials_NamesW>(), 8);
    assert_eq!(align_of::<SecPkgCredentials_NamesW>(), 8);
    assert_eq!(size_of::<SecPkgCredentials_NamesA>(), 8);
    assert_eq!(align_of::<SecPkgCredentials_NamesA>(), 8);
    assert_eq!(size_of::<SecPkgCredentials_SSIProviderW>(), 24);
    assert_eq!(align_of::<SecPkgCredentials_SSIProviderW>(), 8);
    assert_eq!(size_of::<SecPkgCredentials_SSIProviderA>(), 24);
    assert_eq!(align_of::<SecPkgCredentials_SSIProviderA>(), 8);
    assert_eq!(size_of::<SecPkgCredentials_KdcProxySettingsW>(), 16);
    assert_eq!(align_of::<SecPkgCredentials_KdcProxySettingsW>(), 4);
    assert_eq!(size_of::<SecPkgCredentials_Cert>(), 16);
    assert_eq!(align_of::<SecPkgCredentials_Cert>(), 8);
    assert_eq!(size_of::<SecPkgContext_SubjectAttributes>(), 8);
    assert_eq!(align_of::<SecPkgContext_SubjectAttributes>(), 8);
    assert_eq!(size_of::<SecPkgContext_CredInfo>(), 8);
    assert_eq!(align_of::<SecPkgContext_CredInfo>(), 4);
    assert_eq!(size_of::<SecPkgContext_NegoPackageInfo>(), 4);
    assert_eq!(align_of::<SecPkgContext_NegoPackageInfo>(), 4);
    assert_eq!(size_of::<SecPkgContext_NegoStatus>(), 4);
    assert_eq!(align_of::<SecPkgContext_NegoStatus>(), 4);
    assert_eq!(size_of::<SecPkgContext_Sizes>(), 16);
    assert_eq!(align_of::<SecPkgContext_Sizes>(), 4);
    assert_eq!(size_of::<SecPkgContext_StreamSizes>(), 20);
    assert_eq!(align_of::<SecPkgContext_StreamSizes>(), 4);
    assert_eq!(size_of::<SecPkgContext_NamesW>(), 8);
    assert_eq!(align_of::<SecPkgContext_NamesW>(), 8);
    assert_eq!(size_of::<SecPkgContext_LastClientTokenStatus>(), 4);
    assert_eq!(align_of::<SecPkgContext_LastClientTokenStatus>(), 4);
    assert_eq!(size_of::<SecPkgContext_NamesA>(), 8);
    assert_eq!(align_of::<SecPkgContext_NamesA>(), 8);
    assert_eq!(size_of::<SecPkgContext_Lifespan>(), 16);
    assert_eq!(align_of::<SecPkgContext_Lifespan>(), 8);
    assert_eq!(size_of::<SecPkgContext_DceInfo>(), 16);
    assert_eq!(align_of::<SecPkgContext_DceInfo>(), 8);
    assert_eq!(size_of::<SecPkgContext_KeyInfoA>(), 32);
    assert_eq!(align_of::<SecPkgContext_KeyInfoA>(), 8);
    assert_eq!(size_of::<SecPkgContext_KeyInfoW>(), 32);
    assert_eq!(align_of::<SecPkgContext_KeyInfoW>(), 8);
    assert_eq!(size_of::<SecPkgContext_AuthorityA>(), 8);
    assert_eq!(align_of::<SecPkgContext_AuthorityA>(), 8);
    assert_eq!(size_of::<SecPkgContext_AuthorityW>(), 8);
    assert_eq!(align_of::<SecPkgContext_AuthorityW>(), 8);
    assert_eq!(size_of::<SecPkgContext_ProtoInfoA>(), 16);
    assert_eq!(align_of::<SecPkgContext_ProtoInfoA>(), 8);
    assert_eq!(size_of::<SecPkgContext_ProtoInfoW>(), 16);
    assert_eq!(align_of::<SecPkgContext_ProtoInfoW>(), 8);
    assert_eq!(size_of::<SecPkgContext_PasswordExpiry>(), 8);
    assert_eq!(align_of::<SecPkgContext_PasswordExpiry>(), 8);
    assert_eq!(size_of::<SecPkgContext_LogoffTime>(), 8);
    assert_eq!(align_of::<SecPkgContext_LogoffTime>(), 8);
    assert_eq!(size_of::<SecPkgContext_SessionKey>(), 16);
    assert_eq!(align_of::<SecPkgContext_SessionKey>(), 8);
    assert_eq!(size_of::<SecPkgContext_NegoKeys>(), 32);
    assert_eq!(align_of::<SecPkgContext_NegoKeys>(), 8);
    assert_eq!(size_of::<SecPkgContext_PackageInfoW>(), 8);
    assert_eq!(align_of::<SecPkgContext_PackageInfoW>(), 8);
    assert_eq!(size_of::<SecPkgContext_PackageInfoA>(), 8);
    assert_eq!(align_of::<SecPkgContext_PackageInfoA>(), 8);
    assert_eq!(size_of::<SecPkgContext_UserFlags>(), 4);
    assert_eq!(align_of::<SecPkgContext_UserFlags>(), 4);
    assert_eq!(size_of::<SecPkgContext_Flags>(), 4);
    assert_eq!(align_of::<SecPkgContext_Flags>(), 4);
    assert_eq!(size_of::<SecPkgContext_NegotiationInfoA>(), 16);
    assert_eq!(align_of::<SecPkgContext_NegotiationInfoA>(), 8);
    assert_eq!(size_of::<SecPkgContext_NegotiationInfoW>(), 16);
    assert_eq!(align_of::<SecPkgContext_NegotiationInfoW>(), 8);
    assert_eq!(size_of::<SecPkgContext_NativeNamesW>(), 16);
    assert_eq!(align_of::<SecPkgContext_NativeNamesW>(), 8);
    assert_eq!(size_of::<SecPkgContext_NativeNamesA>(), 16);
    assert_eq!(align_of::<SecPkgContext_NativeNamesA>(), 8);
    assert_eq!(size_of::<SecPkgContext_CredentialNameW>(), 16);
    assert_eq!(align_of::<SecPkgContext_CredentialNameW>(), 8);
    assert_eq!(size_of::<SecPkgContext_CredentialNameA>(), 16);
    assert_eq!(align_of::<SecPkgContext_CredentialNameA>(), 8);
    assert_eq!(size_of::<SecPkgContext_AccessToken>(), 8);
    assert_eq!(align_of::<SecPkgContext_AccessToken>(), 8);
    assert_eq!(size_of::<SecPkgContext_TargetInformation>(), 16);
    assert_eq!(align_of::<SecPkgContext_TargetInformation>(), 8);
    assert_eq!(size_of::<SecPkgContext_AuthzID>(), 16);
    assert_eq!(align_of::<SecPkgContext_AuthzID>(), 8);
    assert_eq!(size_of::<SecPkgContext_Target>(), 16);
    assert_eq!(align_of::<SecPkgContext_Target>(), 8);
    assert_eq!(size_of::<SecPkgContext_ClientSpecifiedTarget>(), 8);
    assert_eq!(align_of::<SecPkgContext_ClientSpecifiedTarget>(), 8);
    assert_eq!(size_of::<SecPkgContext_Bindings>(), 16);
    assert_eq!(align_of::<SecPkgContext_Bindings>(), 8);
    assert_eq!(size_of::<SecPkgContext_ApplicationProtocol>(), 264);
    assert_eq!(align_of::<SecPkgContext_ApplicationProtocol>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_BYTE_VECTOR>(), 8);
    assert_eq!(align_of::<SEC_WINNT_AUTH_BYTE_VECTOR>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_DATA>(), 24);
    assert_eq!(align_of::<SEC_WINNT_AUTH_DATA>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_PACKED_CREDENTIALS>(), 28);
    assert_eq!(align_of::<SEC_WINNT_AUTH_PACKED_CREDENTIALS>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_DATA_PASSWORD>(), 8);
    assert_eq!(align_of::<SEC_WINNT_AUTH_DATA_PASSWORD>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_CERTIFICATE_DATA>(), 12);
    assert_eq!(align_of::<SEC_WINNT_AUTH_CERTIFICATE_DATA>(), 4);
    assert_eq!(size_of::<SEC_WINNT_CREDUI_CONTEXT_VECTOR>(), 8);
    assert_eq!(align_of::<SEC_WINNT_CREDUI_CONTEXT_VECTOR>(), 4);
    assert_eq!(size_of::<SEC_WINNT_AUTH_SHORT_VECTOR>(), 8);
    assert_eq!(align_of::<SEC_WINNT_AUTH_SHORT_VECTOR>(), 4);
    assert_eq!(size_of::<CREDUIWIN_MARSHALED_CONTEXT>(), 52);
    assert_eq!(align_of::<CREDUIWIN_MARSHALED_CONTEXT>(), 4);
    assert_eq!(size_of::<SEC_WINNT_CREDUI_CONTEXT>(), 48);
    assert_eq!(align_of::<SEC_WINNT_CREDUI_CONTEXT>(), 8);
}
#[cfg(feature = "tcpestats")]
#[test]
fn shared_tcpestats() {
    use winapi::shared::tcpestats::*;
    assert_eq!(size_of::<TCP_ESTATS_SYN_OPTS_ROS_v0>(), 12);
    assert_eq!(align_of::<TCP_ESTATS_SYN_OPTS_ROS_v0>(), 4);
    assert_eq!(size_of::<TCP_ESTATS_DATA_ROD_v0>(), 96);
    assert_eq!(align_of::<TCP_ESTATS_DATA_ROD_v0>(), 8);
    assert_eq!(size_of::<TCP_ESTATS_DATA_RW_v0>(), 1);
    assert_eq!(align_of::<TCP_ESTATS_DATA_RW_v0>(), 1);
    assert_eq!(size_of::<TCP_ESTATS_SND_CONG_ROD_v0>(), 88);
    assert_eq!(align_of::<TCP_ESTATS_SND_CONG_ROD_v0>(), 8);
    assert_eq!(size_of::<TCP_ESTATS_SND_CONG_ROS_v0>(), 4);
    assert_eq!(align_of::<TCP_ESTATS_SND_CONG_ROS_v0>(), 4);
    assert_eq!(size_of::<TCP_ESTATS_SND_CONG_RW_v0>(), 1);
    assert_eq!(align_of::<TCP_ESTATS_SND_CONG_RW_v0>(), 1);
    assert_eq!(size_of::<TCP_ESTATS_PATH_ROD_v0>(), 160);
    assert_eq!(align_of::<TCP_ESTATS_PATH_ROD_v0>(), 4);
    assert_eq!(size_of::<TCP_ESTATS_PATH_RW_v0>(), 1);
    assert_eq!(align_of::<TCP_ESTATS_PATH_RW_v0>(), 1);
    assert_eq!(size_of::<TCP_ESTATS_SEND_BUFF_ROD_v0>(), 32);
    assert_eq!(align_of::<TCP_ESTATS_SEND_BUFF_ROD_v0>(), 8);
    assert_eq!(size_of::<TCP_ESTATS_SEND_BUFF_RW_v0>(), 1);
    assert_eq!(align_of::<TCP_ESTATS_SEND_BUFF_RW_v0>(), 1);
    assert_eq!(size_of::<TCP_ESTATS_REC_ROD_v0>(), 72);
    assert_eq!(align_of::<TCP_ESTATS_REC_ROD_v0>(), 8);
    assert_eq!(size_of::<TCP_ESTATS_REC_RW_v0>(), 1);
    assert_eq!(align_of::<TCP_ESTATS_REC_RW_v0>(), 1);
    assert_eq!(size_of::<TCP_ESTATS_OBS_REC_ROD_v0>(), 16);
    assert_eq!(align_of::<TCP_ESTATS_OBS_REC_ROD_v0>(), 4);
    assert_eq!(size_of::<TCP_ESTATS_BANDWIDTH_RW_v0>(), 8);
    assert_eq!(align_of::<TCP_ESTATS_BANDWIDTH_RW_v0>(), 4);
    assert_eq!(size_of::<TCP_ESTATS_BANDWIDTH_ROD_v0>(), 40);
    assert_eq!(align_of::<TCP_ESTATS_BANDWIDTH_ROD_v0>(), 8);
    assert_eq!(size_of::<TCP_ESTATS_FINE_RTT_RW_v0>(), 1);
    assert_eq!(align_of::<TCP_ESTATS_FINE_RTT_RW_v0>(), 1);
    assert_eq!(size_of::<TCP_ESTATS_FINE_RTT_ROD_v0>(), 16);
    assert_eq!(align_of::<TCP_ESTATS_FINE_RTT_ROD_v0>(), 4);
}
#[cfg(feature = "tcpmib")]
#[test]
fn shared_tcpmib() {
    use winapi::shared::tcpmib::*;
    assert_eq!(size_of::<MIB_TCPROW_LH>(), 20);
    assert_eq!(align_of::<MIB_TCPROW_LH>(), 4);
    assert_eq!(size_of::<MIB_TCPROW_W2K>(), 20);
    assert_eq!(align_of::<MIB_TCPROW_W2K>(), 4);
    assert_eq!(size_of::<MIB_TCPTABLE>(), 24);
    assert_eq!(align_of::<MIB_TCPTABLE>(), 4);
    assert_eq!(size_of::<MIB_TCPROW2>(), 28);
    assert_eq!(align_of::<MIB_TCPROW2>(), 4);
    assert_eq!(size_of::<MIB_TCPTABLE2>(), 32);
    assert_eq!(align_of::<MIB_TCPTABLE2>(), 4);
    assert_eq!(size_of::<MIB_TCPROW_OWNER_PID>(), 24);
    assert_eq!(align_of::<MIB_TCPROW_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_TCPTABLE_OWNER_PID>(), 28);
    assert_eq!(align_of::<MIB_TCPTABLE_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_TCPROW_OWNER_MODULE>(), 160);
    assert_eq!(align_of::<MIB_TCPROW_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_TCPTABLE_OWNER_MODULE>(), 168);
    assert_eq!(align_of::<MIB_TCPTABLE_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_TCP6ROW>(), 52);
    assert_eq!(align_of::<MIB_TCP6ROW>(), 4);
    assert_eq!(size_of::<MIB_TCP6TABLE>(), 56);
    assert_eq!(align_of::<MIB_TCP6TABLE>(), 4);
    assert_eq!(size_of::<MIB_TCP6ROW2>(), 60);
    assert_eq!(align_of::<MIB_TCP6ROW2>(), 4);
    assert_eq!(size_of::<MIB_TCP6TABLE2>(), 64);
    assert_eq!(align_of::<MIB_TCP6TABLE2>(), 4);
    assert_eq!(size_of::<MIB_TCP6ROW_OWNER_PID>(), 56);
    assert_eq!(align_of::<MIB_TCP6ROW_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_TCP6TABLE_OWNER_PID>(), 60);
    assert_eq!(align_of::<MIB_TCP6TABLE_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_TCP6ROW_OWNER_MODULE>(), 192);
    assert_eq!(align_of::<MIB_TCP6ROW_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_TCP6TABLE_OWNER_MODULE>(), 200);
    assert_eq!(align_of::<MIB_TCP6TABLE_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_TCPSTATS_LH>(), 60);
    assert_eq!(align_of::<MIB_TCPSTATS_LH>(), 4);
    assert_eq!(size_of::<MIB_TCPSTATS_W2K>(), 60);
    assert_eq!(align_of::<MIB_TCPSTATS_W2K>(), 4);
    assert_eq!(size_of::<MIB_TCPSTATS2>(), 72);
    assert_eq!(align_of::<MIB_TCPSTATS2>(), 8);
}
#[cfg(feature = "transportsettingcommon")]
#[test]
fn shared_transportsettingcommon() {
    use winapi::shared::transportsettingcommon::*;
    assert_eq!(size_of::<TRANSPORT_SETTING_ID>(), 16);
    assert_eq!(align_of::<TRANSPORT_SETTING_ID>(), 4);
}
#[cfg(feature = "tvout")]
#[test]
fn shared_tvout() {
    use winapi::shared::tvout::*;
    assert_eq!(size_of::<VIDEOPARAMETERS>(), 356);
    assert_eq!(align_of::<VIDEOPARAMETERS>(), 4);
}
#[cfg(feature = "udpmib")]
#[test]
fn shared_udpmib() {
    use winapi::shared::udpmib::*;
    assert_eq!(size_of::<MIB_UDPROW>(), 8);
    assert_eq!(align_of::<MIB_UDPROW>(), 4);
    assert_eq!(size_of::<MIB_UDPTABLE>(), 12);
    assert_eq!(align_of::<MIB_UDPTABLE>(), 4);
    assert_eq!(size_of::<MIB_UDPROW_OWNER_PID>(), 12);
    assert_eq!(align_of::<MIB_UDPROW_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_UDPTABLE_OWNER_PID>(), 16);
    assert_eq!(align_of::<MIB_UDPTABLE_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_UDPROW_OWNER_MODULE>(), 160);
    assert_eq!(align_of::<MIB_UDPROW_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_UDPTABLE_OWNER_MODULE>(), 168);
    assert_eq!(align_of::<MIB_UDPTABLE_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_UDP6ROW>(), 24);
    assert_eq!(align_of::<MIB_UDP6ROW>(), 4);
    assert_eq!(size_of::<MIB_UDP6TABLE>(), 28);
    assert_eq!(align_of::<MIB_UDP6TABLE>(), 4);
    assert_eq!(size_of::<MIB_UDP6ROW_OWNER_PID>(), 28);
    assert_eq!(align_of::<MIB_UDP6ROW_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_UDP6TABLE_OWNER_PID>(), 32);
    assert_eq!(align_of::<MIB_UDP6TABLE_OWNER_PID>(), 4);
    assert_eq!(size_of::<MIB_UDP6ROW_OWNER_MODULE>(), 176);
    assert_eq!(align_of::<MIB_UDP6ROW_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_UDP6TABLE_OWNER_MODULE>(), 184);
    assert_eq!(align_of::<MIB_UDP6TABLE_OWNER_MODULE>(), 8);
    assert_eq!(size_of::<MIB_UDPSTATS>(), 20);
    assert_eq!(align_of::<MIB_UDPSTATS>(), 4);
    assert_eq!(size_of::<MIB_UDPSTATS2>(), 32);
    assert_eq!(align_of::<MIB_UDPSTATS2>(), 8);
}
#[cfg(feature = "usb")]
#[test]
fn shared_usb() {
    use winapi::shared::usb::*;
    assert_eq!(size_of::<USBD_VERSION_INFORMATION>(), 8);
    assert_eq!(align_of::<USBD_VERSION_INFORMATION>(), 4);
    assert_eq!(size_of::<USBD_DEVICE_INFORMATION>(), 40);
    assert_eq!(align_of::<USBD_DEVICE_INFORMATION>(), 8);
    assert_eq!(size_of::<USBD_PIPE_INFORMATION>(), 24);
    assert_eq!(align_of::<USBD_PIPE_INFORMATION>(), 8);
    assert_eq!(size_of::<USBD_INTERFACE_INFORMATION>(), 48);
    assert_eq!(align_of::<USBD_INTERFACE_INFORMATION>(), 8);
    assert_eq!(size_of::<URB_HCD_AREA>(), 64);
    assert_eq!(align_of::<URB_HCD_AREA>(), 8);
    assert_eq!(size_of::<URB_HEADER>(), 24);
    assert_eq!(align_of::<URB_HEADER>(), 8);
    assert_eq!(size_of::<URB_SELECT_INTERFACE>(), 80);
    assert_eq!(align_of::<URB_SELECT_INTERFACE>(), 8);
    assert_eq!(size_of::<URB_SELECT_CONFIGURATION>(), 88);
    assert_eq!(align_of::<URB_SELECT_CONFIGURATION>(), 8);
    assert_eq!(size_of::<URB_PIPE_REQUEST>(), 40);
    assert_eq!(align_of::<URB_PIPE_REQUEST>(), 8);
    assert_eq!(size_of::<URB_FRAME_LENGTH_CONTROL>(), 24);
    assert_eq!(align_of::<URB_FRAME_LENGTH_CONTROL>(), 8);
    assert_eq!(size_of::<URB_GET_FRAME_LENGTH>(), 32);
    assert_eq!(align_of::<URB_GET_FRAME_LENGTH>(), 8);
    assert_eq!(size_of::<URB_SET_FRAME_LENGTH>(), 32);
    assert_eq!(align_of::<URB_SET_FRAME_LENGTH>(), 8);
    assert_eq!(size_of::<URB_GET_CURRENT_FRAME_NUMBER>(), 32);
    assert_eq!(align_of::<URB_GET_CURRENT_FRAME_NUMBER>(), 8);
    assert_eq!(size_of::<URB_CONTROL_DESCRIPTOR_REQUEST>(), 136);
    assert_eq!(align_of::<URB_CONTROL_DESCRIPTOR_REQUEST>(), 8);
    assert_eq!(size_of::<URB_CONTROL_GET_STATUS_REQUEST>(), 136);
    assert_eq!(align_of::<URB_CONTROL_GET_STATUS_REQUEST>(), 8);
    assert_eq!(size_of::<URB_CONTROL_FEATURE_REQUEST>(), 136);
    assert_eq!(align_of::<URB_CONTROL_FEATURE_REQUEST>(), 8);
    assert_eq!(size_of::<URB_CONTROL_VENDOR_OR_CLASS_REQUEST>(), 136);
    assert_eq!(align_of::<URB_CONTROL_VENDOR_OR_CLASS_REQUEST>(), 8);
    assert_eq!(size_of::<URB_CONTROL_GET_INTERFACE_REQUEST>(), 136);
    assert_eq!(align_of::<URB_CONTROL_GET_INTERFACE_REQUEST>(), 8);
    assert_eq!(size_of::<URB_CONTROL_GET_CONFIGURATION_REQUEST>(), 136);
    assert_eq!(align_of::<URB_CONTROL_GET_CONFIGURATION_REQUEST>(), 8);
    assert_eq!(size_of::<OS_STRING>(), 18);
    assert_eq!(align_of::<OS_STRING>(), 2);
    assert_eq!(size_of::<URB_OS_FEATURE_DESCRIPTOR_REQUEST>(), 136);
    assert_eq!(align_of::<URB_OS_FEATURE_DESCRIPTOR_REQUEST>(), 8);
    assert_eq!(size_of::<URB_CONTROL_TRANSFER>(), 136);
    assert_eq!(align_of::<URB_CONTROL_TRANSFER>(), 8);
    assert_eq!(size_of::<URB_CONTROL_TRANSFER_EX>(), 136);
    assert_eq!(align_of::<URB_CONTROL_TRANSFER_EX>(), 8);
    assert_eq!(size_of::<URB_CONTROL_TRANSFER_EX>(), 136);
    assert_eq!(align_of::<URB_CONTROL_TRANSFER_EX>(), 8);
    assert_eq!(size_of::<URB_BULK_OR_INTERRUPT_TRANSFER>(), 128);
    assert_eq!(align_of::<URB_BULK_OR_INTERRUPT_TRANSFER>(), 8);
    assert_eq!(size_of::<USBD_ISO_PACKET_DESCRIPTOR>(), 12);
    assert_eq!(align_of::<USBD_ISO_PACKET_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<URB_ISOCH_TRANSFER>(), 152);
    assert_eq!(align_of::<URB_ISOCH_TRANSFER>(), 8);
    assert_eq!(size_of::<USBD_STREAM_INFORMATION>(), 24);
    assert_eq!(align_of::<USBD_STREAM_INFORMATION>(), 8);
    assert_eq!(size_of::<URB_OPEN_STATIC_STREAMS>(), 48);
    assert_eq!(align_of::<URB_OPEN_STATIC_STREAMS>(), 8);
    assert_eq!(size_of::<URB>(), 152);
    assert_eq!(align_of::<URB>(), 8);
}
#[cfg(feature = "usbiodef")]
#[test]
fn shared_usbiodef() {
    use winapi::shared::usbiodef::*;
    assert_eq!(size_of::<USB_IDLE_CALLBACK_INFO>(), 16);
    assert_eq!(align_of::<USB_IDLE_CALLBACK_INFO>(), 8);
}
#[cfg(feature = "usbspec")]
#[test]
fn shared_usbspec() {
    use winapi::shared::usbspec::*;
    assert_eq!(size_of::<BM_REQUEST_TYPE>(), 1);
    assert_eq!(align_of::<BM_REQUEST_TYPE>(), 1);
    assert_eq!(size_of::<USB_DEFAULT_PIPE_SETUP_PACKET_wValue>(), 2);
    assert_eq!(align_of::<USB_DEFAULT_PIPE_SETUP_PACKET_wValue>(), 1);
    assert_eq!(size_of::<USB_DEFAULT_PIPE_SETUP_PACKET_wIndex>(), 2);
    assert_eq!(align_of::<USB_DEFAULT_PIPE_SETUP_PACKET_wIndex>(), 1);
    assert_eq!(size_of::<USB_DEFAULT_PIPE_SETUP_PACKET>(), 8);
    assert_eq!(align_of::<USB_DEFAULT_PIPE_SETUP_PACKET>(), 1);
    assert_eq!(size_of::<USB_DEVICE_STATUS>(), 2);
    assert_eq!(align_of::<USB_DEVICE_STATUS>(), 1);
    assert_eq!(size_of::<USB_INTERFACE_STATUS>(), 2);
    assert_eq!(align_of::<USB_INTERFACE_STATUS>(), 1);
    assert_eq!(size_of::<USB_ENDPOINT_STATUS>(), 2);
    assert_eq!(align_of::<USB_ENDPOINT_STATUS>(), 1);
    assert_eq!(size_of::<USB_COMMON_DESCRIPTOR>(), 2);
    assert_eq!(align_of::<USB_COMMON_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_DEVICE_DESCRIPTOR>(), 18);
    assert_eq!(align_of::<USB_DEVICE_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_DEVICE_QUALIFIER_DESCRIPTOR>(), 10);
    assert_eq!(align_of::<USB_DEVICE_QUALIFIER_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_BOS_DESCRIPTOR>(), 5);
    assert_eq!(align_of::<USB_BOS_DESCRIPTOR>(), 1);
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_bmAttributes>(),
        4
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_bmAttributes>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR>(),
        7
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_bmAttributes>(),
        4
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_bmAttributes>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR>(),
        18
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_bmCapabilities>(),
        2
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_bmCapabilities>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR>(),
        24
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR>(),
        10
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR>(),
        1
    );
    assert_eq!(size_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED>(), 4);
    assert_eq!(align_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED>(), 1);
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_bmAttributes>(),
        4
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_bmAttributes>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_wFunctionalitySupport>(),
        2
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_wFunctionalitySupport>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR>(),
        16
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR>(),
        20
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR>(),
        1
    );
    assert_eq!(size_of::<USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR>(), 21);
    assert_eq!(align_of::<USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR>(), 1);
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_VconnPower>(),
        2
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_VconnPower>(),
        1
    );
    assert_eq!(
        size_of::<USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_AlternateMode>(),
        4
    );
    assert_eq!(
        align_of::<USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_AlternateMode>(),
        1
    );
    assert_eq!(size_of::<USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR>(), 48);
    assert_eq!(align_of::<USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_DEVICE_CAPABILITY_DESCRIPTOR>(), 3);
    assert_eq!(align_of::<USB_DEVICE_CAPABILITY_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_CONFIGURATION_DESCRIPTOR>(), 9);
    assert_eq!(align_of::<USB_CONFIGURATION_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_INTERFACE_ASSOCIATION_DESCRIPTOR>(), 8);
    assert_eq!(align_of::<USB_INTERFACE_ASSOCIATION_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_INTERFACE_DESCRIPTOR>(), 9);
    assert_eq!(align_of::<USB_INTERFACE_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_ENDPOINT_DESCRIPTOR>(), 7);
    assert_eq!(align_of::<USB_ENDPOINT_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_HIGH_SPEED_MAXPACKET>(), 2);
    assert_eq!(align_of::<USB_HIGH_SPEED_MAXPACKET>(), 1);
    assert_eq!(size_of::<USB_STRING_DESCRIPTOR>(), 4);
    assert_eq!(align_of::<USB_STRING_DESCRIPTOR>(), 1);
    assert_eq!(
        size_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Bulk>(),
        1
    );
    assert_eq!(
        align_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Bulk>(),
        1
    );
    assert_eq!(
        size_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Isochronous>(),
        1
    );
    assert_eq!(
        align_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Isochronous>(),
        1
    );
    assert_eq!(
        size_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes>(),
        1
    );
    assert_eq!(
        align_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes>(),
        1
    );
    assert_eq!(size_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR>(), 6);
    assert_eq!(
        align_of::<USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR>(),
        1
    );
    assert_eq!(
        size_of::<USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR>(),
        8
    );
    assert_eq!(
        align_of::<USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR>(),
        1
    );
    assert_eq!(size_of::<USB_HUB_DESCRIPTOR>(), 71);
    assert_eq!(align_of::<USB_HUB_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_30_HUB_DESCRIPTOR>(), 12);
    assert_eq!(align_of::<USB_30_HUB_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_HUB_STATUS>(), 2);
    assert_eq!(align_of::<USB_HUB_STATUS>(), 1);
    assert_eq!(size_of::<USB_HUB_CHANGE>(), 2);
    assert_eq!(align_of::<USB_HUB_CHANGE>(), 1);
    assert_eq!(size_of::<USB_HUB_STATUS_AND_CHANGE>(), 4);
    assert_eq!(align_of::<USB_HUB_STATUS_AND_CHANGE>(), 1);
    assert_eq!(size_of::<USB_20_PORT_STATUS>(), 2);
    assert_eq!(align_of::<USB_20_PORT_STATUS>(), 1);
    assert_eq!(size_of::<USB_20_PORT_CHANGE>(), 2);
    assert_eq!(align_of::<USB_20_PORT_CHANGE>(), 1);
    assert_eq!(size_of::<USB_30_PORT_STATUS>(), 2);
    assert_eq!(align_of::<USB_30_PORT_STATUS>(), 1);
    assert_eq!(size_of::<USB_30_PORT_CHANGE>(), 2);
    assert_eq!(align_of::<USB_30_PORT_CHANGE>(), 1);
    assert_eq!(size_of::<USB_PORT_STATUS>(), 2);
    assert_eq!(align_of::<USB_PORT_STATUS>(), 1);
    assert_eq!(size_of::<USB_PORT_CHANGE>(), 2);
    assert_eq!(align_of::<USB_PORT_CHANGE>(), 1);
    assert_eq!(size_of::<USB_PORT_EXT_STATUS>(), 4);
    assert_eq!(align_of::<USB_PORT_EXT_STATUS>(), 1);
    assert_eq!(size_of::<USB_PORT_STATUS_AND_CHANGE>(), 4);
    assert_eq!(align_of::<USB_PORT_STATUS_AND_CHANGE>(), 1);
    assert_eq!(size_of::<USB_PORT_EXT_STATUS_AND_CHANGE>(), 8);
    assert_eq!(align_of::<USB_PORT_EXT_STATUS_AND_CHANGE>(), 1);
    assert_eq!(size_of::<USB_HUB_30_PORT_REMOTE_WAKE_MASK>(), 1);
    assert_eq!(align_of::<USB_HUB_30_PORT_REMOTE_WAKE_MASK>(), 1);
    assert_eq!(size_of::<USB_FUNCTION_SUSPEND_OPTIONS>(), 1);
    assert_eq!(align_of::<USB_FUNCTION_SUSPEND_OPTIONS>(), 1);
    assert_eq!(size_of::<USB_CONFIGURATION_POWER_DESCRIPTOR>(), 18);
    assert_eq!(align_of::<USB_CONFIGURATION_POWER_DESCRIPTOR>(), 1);
    assert_eq!(size_of::<USB_INTERFACE_POWER_DESCRIPTOR>(), 15);
    assert_eq!(align_of::<USB_INTERFACE_POWER_DESCRIPTOR>(), 1);
}
#[cfg(feature = "windef")]
#[test]
fn shared_windef() {
    use winapi::shared::windef::*;
    assert_eq!(size_of::<RECT>(), 16);
    assert_eq!(align_of::<RECT>(), 4);
    assert_eq!(size_of::<RECTL>(), 16);
    assert_eq!(align_of::<RECTL>(), 4);
    assert_eq!(size_of::<POINT>(), 8);
    assert_eq!(align_of::<POINT>(), 4);
    assert_eq!(size_of::<POINTL>(), 8);
    assert_eq!(align_of::<POINTL>(), 4);
    assert_eq!(size_of::<SIZE>(), 8);
    assert_eq!(align_of::<SIZE>(), 4);
    assert_eq!(size_of::<POINTS>(), 4);
    assert_eq!(align_of::<POINTS>(), 2);
}
#[cfg(feature = "windot11")]
#[test]
fn shared_windot11() {
    use winapi::shared::windot11::*;
    assert_eq!(size_of::<DOT11_BSSID_LIST>(), 20);
    assert_eq!(align_of::<DOT11_BSSID_LIST>(), 4);
    assert_eq!(size_of::<DOT11_RATE_SET>(), 132);
    assert_eq!(align_of::<DOT11_RATE_SET>(), 4);
    assert_eq!(size_of::<DOT11_WFD_SESSION_INFO>(), 146);
    assert_eq!(align_of::<DOT11_WFD_SESSION_INFO>(), 2);
    assert_eq!(size_of::<DOT11_OFFLOAD_CAPABILITY>(), 28);
    assert_eq!(align_of::<DOT11_OFFLOAD_CAPABILITY>(), 4);
    assert_eq!(size_of::<DOT11_CURRENT_OFFLOAD_CAPABILITY>(), 8);
    assert_eq!(align_of::<DOT11_CURRENT_OFFLOAD_CAPABILITY>(), 4);
    assert_eq!(size_of::<DOT11_IV48_COUNTER>(), 8);
    assert_eq!(align_of::<DOT11_IV48_COUNTER>(), 4);
    assert_eq!(size_of::<DOT11_WEP_OFFLOAD>(), 224);
    assert_eq!(align_of::<DOT11_WEP_OFFLOAD>(), 8);
    assert_eq!(size_of::<DOT11_WEP_UPLOAD>(), 184);
    assert_eq!(align_of::<DOT11_WEP_UPLOAD>(), 8);
    assert_eq!(size_of::<DOT11_DEFAULT_WEP_OFFLOAD>(), 224);
    assert_eq!(align_of::<DOT11_DEFAULT_WEP_OFFLOAD>(), 8);
    assert_eq!(size_of::<DOT11_DEFAULT_WEP_UPLOAD>(), 184);
    assert_eq!(align_of::<DOT11_DEFAULT_WEP_UPLOAD>(), 8);
    assert_eq!(size_of::<DOT11_OPERATION_MODE_CAPABILITY>(), 24);
    assert_eq!(align_of::<DOT11_OPERATION_MODE_CAPABILITY>(), 4);
    assert_eq!(size_of::<DOT11_CURRENT_OPERATION_MODE>(), 8);
    assert_eq!(align_of::<DOT11_CURRENT_OPERATION_MODE>(), 4);
    assert_eq!(size_of::<DOT11_SCAN_REQUEST>(), 84);
    assert_eq!(align_of::<DOT11_SCAN_REQUEST>(), 4);
    assert_eq!(size_of::<DOT11_PHY_TYPE_INFO>(), 32);
    assert_eq!(align_of::<DOT11_PHY_TYPE_INFO>(), 4);
    assert_eq!(size_of::<DOT11_SCAN_REQUEST_V2>(), 60);
    assert_eq!(align_of::<DOT11_SCAN_REQUEST_V2>(), 4);
    assert_eq!(size_of::<DOT11_PHY_TYPE_LIST>(), 16);
    assert_eq!(align_of::<DOT11_PHY_TYPE_LIST>(), 4);
    assert_eq!(size_of::<DOT11_BSS_DESCRIPTION>(), 48);
    assert_eq!(align_of::<DOT11_BSS_DESCRIPTION>(), 8);
    assert_eq!(size_of::<DOT11_JOIN_REQUEST>(), 192);
    assert_eq!(align_of::<DOT11_JOIN_REQUEST>(), 8);
    assert_eq!(size_of::<DOT11_START_REQUEST>(), 192);
    assert_eq!(align_of::<DOT11_START_REQUEST>(), 8);
    assert_eq!(size_of::<DOT11_UPDATE_IE>(), 12);
    assert_eq!(align_of::<DOT11_UPDATE_IE>(), 4);
    assert_eq!(size_of::<DOT11_RESET_REQUEST>(), 12);
    assert_eq!(align_of::<DOT11_RESET_REQUEST>(), 4);
    assert_eq!(size_of::<DOT11_OPTIONAL_CAPABILITY>(), 8);
    assert_eq!(align_of::<DOT11_OPTIONAL_CAPABILITY>(), 4);
    assert_eq!(size_of::<DOT11_CURRENT_OPTIONAL_CAPABILITY>(), 8);
    assert_eq!(align_of::<DOT11_CURRENT_OPTIONAL_CAPABILITY>(), 4);
    assert_eq!(size_of::<DOT11_POWER_MGMT_MODE>(), 16);
    assert_eq!(align_of::<DOT11_POWER_MGMT_MODE>(), 4);
    assert_eq!(size_of::<DOT11_COUNTERS_ENTRY>(), 52);
    assert_eq!(align_of::<DOT11_COUNTERS_ENTRY>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_PHY_TYPES>(), 12);
    assert_eq!(align_of::<DOT11_SUPPORTED_PHY_TYPES>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_POWER_LEVELS>(), 36);
    assert_eq!(align_of::<DOT11_SUPPORTED_POWER_LEVELS>(), 4);
    assert_eq!(size_of::<DOT11_REG_DOMAIN_VALUE>(), 8);
    assert_eq!(align_of::<DOT11_REG_DOMAIN_VALUE>(), 4);
    assert_eq!(size_of::<DOT11_REG_DOMAINS_SUPPORT_VALUE>(), 16);
    assert_eq!(align_of::<DOT11_REG_DOMAINS_SUPPORT_VALUE>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_ANTENNA>(), 8);
    assert_eq!(align_of::<DOT11_SUPPORTED_ANTENNA>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_ANTENNA_LIST>(), 16);
    assert_eq!(align_of::<DOT11_SUPPORTED_ANTENNA_LIST>(), 4);
    assert_eq!(size_of::<DOT11_DIVERSITY_SELECTION_RX>(), 8);
    assert_eq!(align_of::<DOT11_DIVERSITY_SELECTION_RX>(), 4);
    assert_eq!(size_of::<DOT11_DIVERSITY_SELECTION_RX_LIST>(), 16);
    assert_eq!(align_of::<DOT11_DIVERSITY_SELECTION_RX_LIST>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_DATA_RATES_VALUE>(), 16);
    assert_eq!(align_of::<DOT11_SUPPORTED_DATA_RATES_VALUE>(), 1);
    assert_eq!(size_of::<DOT11_SUPPORTED_DATA_RATES_VALUE_V2>(), 510);
    assert_eq!(align_of::<DOT11_SUPPORTED_DATA_RATES_VALUE_V2>(), 1);
    assert_eq!(size_of::<DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY>(), 16);
    assert_eq!(align_of::<DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY>(), 4);
    assert_eq!(size_of::<DOT11_MD_CAPABILITY_ENTRY_LIST>(), 24);
    assert_eq!(align_of::<DOT11_MD_CAPABILITY_ENTRY_LIST>(), 4);
    assert_eq!(size_of::<DOT11_HOPPING_PATTERN_ENTRY>(), 8);
    assert_eq!(align_of::<DOT11_HOPPING_PATTERN_ENTRY>(), 4);
    assert_eq!(size_of::<DOT11_HOPPING_PATTERN_ENTRY_LIST>(), 16);
    assert_eq!(align_of::<DOT11_HOPPING_PATTERN_ENTRY_LIST>(), 4);
    assert_eq!(size_of::<DOT11_WPA_TSC>(), 24);
    assert_eq!(align_of::<DOT11_WPA_TSC>(), 8);
    assert_eq!(size_of::<DOT11_RSSI_RANGE>(), 12);
    assert_eq!(align_of::<DOT11_RSSI_RANGE>(), 4);
    assert_eq!(size_of::<DOT11_NIC_SPECIFIC_EXTENSION>(), 12);
    assert_eq!(align_of::<DOT11_NIC_SPECIFIC_EXTENSION>(), 4);
    assert_eq!(size_of::<DOT11_AP_JOIN_REQUEST>(), 192);
    assert_eq!(align_of::<DOT11_AP_JOIN_REQUEST>(), 8);
    assert_eq!(size_of::<DOT11_RECV_SENSITIVITY>(), 12);
    assert_eq!(align_of::<DOT11_RECV_SENSITIVITY>(), 4);
    assert_eq!(size_of::<DOT11_RECV_SENSITIVITY_LIST>(), 24);
    assert_eq!(align_of::<DOT11_RECV_SENSITIVITY_LIST>(), 4);
    assert_eq!(size_of::<DOT11_WME_AC_PARAMETERS>(), 6);
    assert_eq!(align_of::<DOT11_WME_AC_PARAMETERS>(), 2);
    assert_eq!(size_of::<DOT11_WME_AC_PARAMETERS_LIST>(), 16);
    assert_eq!(align_of::<DOT11_WME_AC_PARAMETERS_LIST>(), 4);
    assert_eq!(size_of::<DOT11_WME_UPDATE_IE>(), 24);
    assert_eq!(align_of::<DOT11_WME_UPDATE_IE>(), 4);
    assert_eq!(size_of::<DOT11_QOS_TX_DURATION>(), 12);
    assert_eq!(align_of::<DOT11_QOS_TX_DURATION>(), 4);
    assert_eq!(size_of::<DOT11_QOS_TX_MEDIUM_TIME>(), 12);
    assert_eq!(align_of::<DOT11_QOS_TX_MEDIUM_TIME>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_OFDM_FREQUENCY>(), 4);
    assert_eq!(align_of::<DOT11_SUPPORTED_OFDM_FREQUENCY>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_OFDM_FREQUENCY_LIST>(), 12);
    assert_eq!(align_of::<DOT11_SUPPORTED_OFDM_FREQUENCY_LIST>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_DSSS_CHANNEL>(), 4);
    assert_eq!(align_of::<DOT11_SUPPORTED_DSSS_CHANNEL>(), 4);
    assert_eq!(size_of::<DOT11_SUPPORTED_DSSS_CHANNEL_LIST>(), 12);
    assert_eq!(align_of::<DOT11_SUPPORTED_DSSS_CHANNEL_LIST>(), 4);
    assert_eq!(size_of::<DOT11_BYTE_ARRAY>(), 16);
    assert_eq!(align_of::<DOT11_BYTE_ARRAY>(), 4);
    assert_eq!(size_of::<DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO>(), 12);
    assert_eq!(align_of::<DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO>(), 4);
    assert_eq!(size_of::<DOT11_BSS_ENTRY>(), 72);
    assert_eq!(align_of::<DOT11_BSS_ENTRY>(), 8);
    assert_eq!(size_of::<DOT11_SSID_LIST>(), 48);
    assert_eq!(align_of::<DOT11_SSID_LIST>(), 4);
    assert_eq!(size_of::<DOT11_MAC_ADDRESS_LIST>(), 20);
    assert_eq!(align_of::<DOT11_MAC_ADDRESS_LIST>(), 4);
    assert_eq!(size_of::<DOT11_PMKID_ENTRY>(), 28);
    assert_eq!(align_of::<DOT11_PMKID_ENTRY>(), 4);
    assert_eq!(size_of::<DOT11_PMKID_LIST>(), 40);
    assert_eq!(align_of::<DOT11_PMKID_LIST>(), 4);
    assert_eq!(size_of::<DOT11_PHY_FRAME_STATISTICS>(), 144);
    assert_eq!(align_of::<DOT11_PHY_FRAME_STATISTICS>(), 8);
    assert_eq!(size_of::<DOT11_MAC_FRAME_STATISTICS>(), 112);
    assert_eq!(align_of::<DOT11_MAC_FRAME_STATISTICS>(), 8);
    assert_eq!(size_of::<DOT11_STATISTICS>(), 400);
    assert_eq!(align_of::<DOT11_STATISTICS>(), 8);
    assert_eq!(size_of::<DOT11_PRIVACY_EXEMPTION>(), 6);
    assert_eq!(align_of::<DOT11_PRIVACY_EXEMPTION>(), 2);
    assert_eq!(size_of::<DOT11_PRIVACY_EXEMPTION_LIST>(), 20);
    assert_eq!(align_of::<DOT11_PRIVACY_EXEMPTION_LIST>(), 4);
    assert_eq!(size_of::<DOT11_AUTH_ALGORITHM_LIST>(), 16);
    assert_eq!(align_of::<DOT11_AUTH_ALGORITHM_LIST>(), 4);
    assert_eq!(size_of::<DOT11_AUTH_CIPHER_PAIR_LIST>(), 20);
    assert_eq!(align_of::<DOT11_AUTH_CIPHER_PAIR_LIST>(), 4);
    assert_eq!(size_of::<DOT11_CIPHER_ALGORITHM_LIST>(), 16);
    assert_eq!(align_of::<DOT11_CIPHER_ALGORITHM_LIST>(), 4);
    assert_eq!(size_of::<DOT11_CIPHER_DEFAULT_KEY_VALUE>(), 24);
    assert_eq!(align_of::<DOT11_CIPHER_DEFAULT_KEY_VALUE>(), 4);
    assert_eq!(size_of::<DOT11_KEY_ALGO_TKIP_MIC>(), 20);
    assert_eq!(align_of::<DOT11_KEY_ALGO_TKIP_MIC>(), 4);
    assert_eq!(size_of::<DOT11_KEY_ALGO_CCMP>(), 16);
    assert_eq!(align_of::<DOT11_KEY_ALGO_CCMP>(), 4);
    assert_eq!(size_of::<DOT11_KEY_ALGO_GCMP>(), 16);
    assert_eq!(align_of::<DOT11_KEY_ALGO_GCMP>(), 4);
    assert_eq!(size_of::<DOT11_KEY_ALGO_BIP>(), 16);
    assert_eq!(align_of::<DOT11_KEY_ALGO_BIP>(), 4);
    assert_eq!(size_of::<DOT11_CIPHER_KEY_MAPPING_KEY_VALUE>(), 24);
    assert_eq!(align_of::<DOT11_CIPHER_KEY_MAPPING_KEY_VALUE>(), 4);
    assert_eq!(size_of::<DOT11_ASSOCIATION_INFO_EX>(), 328);
    assert_eq!(align_of::<DOT11_ASSOCIATION_INFO_EX>(), 8);
    assert_eq!(size_of::<DOT11_ASSOCIATION_INFO_LIST>(), 344);
    assert_eq!(align_of::<DOT11_ASSOCIATION_INFO_LIST>(), 8);
    assert_eq!(size_of::<DOT11_PHY_ID_LIST>(), 16);
    assert_eq!(align_of::<DOT11_PHY_ID_LIST>(), 4);
    assert_eq!(size_of::<DOT11_EXTSTA_CAPABILITY>(), 44);
    assert_eq!(align_of::<DOT11_EXTSTA_CAPABILITY>(), 4);
    assert_eq!(size_of::<DOT11_DATA_RATE_MAPPING_ENTRY>(), 4);
    assert_eq!(align_of::<DOT11_DATA_RATE_MAPPING_ENTRY>(), 2);
    assert_eq!(size_of::<DOT11_DATA_RATE_MAPPING_TABLE>(), 512);
    assert_eq!(align_of::<DOT11_DATA_RATE_MAPPING_TABLE>(), 4);
    assert_eq!(size_of::<DOT11_COUNTRY_OR_REGION_STRING_LIST>(), 16);
    assert_eq!(align_of::<DOT11_COUNTRY_OR_REGION_STRING_LIST>(), 4);
    assert_eq!(size_of::<DOT11_PORT_STATE_NOTIFICATION>(), 12);
    assert_eq!(align_of::<DOT11_PORT_STATE_NOTIFICATION>(), 2);
    assert_eq!(size_of::<DOT11_IBSS_PARAMS>(), 16);
    assert_eq!(align_of::<DOT11_IBSS_PARAMS>(), 4);
    assert_eq!(size_of::<DOT11_QOS_PARAMS>(), 6);
    assert_eq!(align_of::<DOT11_QOS_PARAMS>(), 2);
    assert_eq!(size_of::<DOT11_ASSOCIATION_PARAMS>(), 20);
    assert_eq!(align_of::<DOT11_ASSOCIATION_PARAMS>(), 4);
    assert_eq!(size_of::<DOT11_FRAGMENT_DESCRIPTOR>(), 8);
    assert_eq!(align_of::<DOT11_FRAGMENT_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<DOT11_PER_MSDU_COUNTERS>(), 20);
    assert_eq!(align_of::<DOT11_PER_MSDU_COUNTERS>(), 4);
    assert_eq!(size_of::<DOT11_PHY_ATTRIBUTES>(), 1092);
    assert_eq!(align_of::<DOT11_PHY_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<DOT11_HRDSSS_PHY_ATTRIBUTES>(), 8);
    assert_eq!(align_of::<DOT11_HRDSSS_PHY_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<DOT11_OFDM_PHY_ATTRIBUTES>(), 4);
    assert_eq!(align_of::<DOT11_OFDM_PHY_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<DOT11_ERP_PHY_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<DOT11_ERP_PHY_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<DOT11_EXTSTA_ATTRIBUTES>(), 160);
    assert_eq!(align_of::<DOT11_EXTSTA_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<DOT11_RECV_EXTENSION_INFO>(), 112);
    assert_eq!(align_of::<DOT11_RECV_EXTENSION_INFO>(), 8);
    assert_eq!(size_of::<DOT11_RECV_EXTENSION_INFO_V2>(), 104);
    assert_eq!(align_of::<DOT11_RECV_EXTENSION_INFO_V2>(), 8);
    assert_eq!(size_of::<DOT11_STATUS_INDICATION>(), 8);
    assert_eq!(align_of::<DOT11_STATUS_INDICATION>(), 4);
    assert_eq!(size_of::<DOT11_MPDU_MAX_LENGTH_INDICATION>(), 12);
    assert_eq!(align_of::<DOT11_MPDU_MAX_LENGTH_INDICATION>(), 4);
    assert_eq!(size_of::<DOT11_ASSOCIATION_START_PARAMETERS>(), 56);
    assert_eq!(align_of::<DOT11_ASSOCIATION_START_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_ENCAP_ENTRY>(), 4);
    assert_eq!(align_of::<DOT11_ENCAP_ENTRY>(), 2);
    assert_eq!(size_of::<DOT11_ASSOCIATION_COMPLETION_PARAMETERS>(), 96);
    assert_eq!(align_of::<DOT11_ASSOCIATION_COMPLETION_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_CONNECTION_START_PARAMETERS>(), 52);
    assert_eq!(align_of::<DOT11_CONNECTION_START_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_CONNECTION_COMPLETION_PARAMETERS>(), 8);
    assert_eq!(align_of::<DOT11_CONNECTION_COMPLETION_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_ROAMING_START_PARAMETERS>(), 52);
    assert_eq!(align_of::<DOT11_ROAMING_START_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_ROAMING_COMPLETION_PARAMETERS>(), 8);
    assert_eq!(align_of::<DOT11_ROAMING_COMPLETION_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_DISASSOCIATION_PARAMETERS>(), 24);
    assert_eq!(align_of::<DOT11_DISASSOCIATION_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_TKIPMIC_FAILURE_PARAMETERS>(), 20);
    assert_eq!(align_of::<DOT11_TKIPMIC_FAILURE_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_PMKID_CANDIDATE_LIST_PARAMETERS>(), 12);
    assert_eq!(align_of::<DOT11_PMKID_CANDIDATE_LIST_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_BSSID_CANDIDATE>(), 12);
    assert_eq!(align_of::<DOT11_BSSID_CANDIDATE>(), 4);
    assert_eq!(size_of::<DOT11_PHY_STATE_PARAMETERS>(), 12);
    assert_eq!(align_of::<DOT11_PHY_STATE_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_LINK_QUALITY_ENTRY>(), 7);
    assert_eq!(align_of::<DOT11_LINK_QUALITY_ENTRY>(), 1);
    assert_eq!(size_of::<DOT11_LINK_QUALITY_PARAMETERS>(), 12);
    assert_eq!(align_of::<DOT11_LINK_QUALITY_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_EXTSTA_SEND_CONTEXT>(), 32);
    assert_eq!(align_of::<DOT11_EXTSTA_SEND_CONTEXT>(), 8);
    assert_eq!(size_of::<DOT11_EXTSTA_RECV_CONTEXT>(), 48);
    assert_eq!(align_of::<DOT11_EXTSTA_RECV_CONTEXT>(), 8);
    assert_eq!(size_of::<DOT11_EXTAP_ATTRIBUTES>(), 80);
    assert_eq!(align_of::<DOT11_EXTAP_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<DOT11_INCOMING_ASSOC_STARTED_PARAMETERS>(), 10);
    assert_eq!(align_of::<DOT11_INCOMING_ASSOC_STARTED_PARAMETERS>(), 2);
    assert_eq!(
        size_of::<DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS>(),
        20
    );
    assert_eq!(
        align_of::<DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS>(),
        4
    );
    assert_eq!(size_of::<DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS>(), 64);
    assert_eq!(align_of::<DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_STOP_AP_PARAMETERS>(), 8);
    assert_eq!(align_of::<DOT11_STOP_AP_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS>(), 12);
    assert_eq!(align_of::<DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_CAN_SUSTAIN_AP_PARAMETERS>(), 8);
    assert_eq!(align_of::<DOT11_CAN_SUSTAIN_AP_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_AVAILABLE_CHANNEL_LIST>(), 16);
    assert_eq!(align_of::<DOT11_AVAILABLE_CHANNEL_LIST>(), 4);
    assert_eq!(size_of::<DOT11_AVAILABLE_FREQUENCY_LIST>(), 16);
    assert_eq!(align_of::<DOT11_AVAILABLE_FREQUENCY_LIST>(), 4);
    assert_eq!(size_of::<DOT11_DISASSOCIATE_PEER_REQUEST>(), 12);
    assert_eq!(align_of::<DOT11_DISASSOCIATE_PEER_REQUEST>(), 2);
    assert_eq!(size_of::<DOT11_INCOMING_ASSOC_DECISION>(), 24);
    assert_eq!(align_of::<DOT11_INCOMING_ASSOC_DECISION>(), 4);
    assert_eq!(size_of::<DOT11_INCOMING_ASSOC_DECISION_V2>(), 28);
    assert_eq!(align_of::<DOT11_INCOMING_ASSOC_DECISION_V2>(), 4);
    assert_eq!(size_of::<DOT11_ADDITIONAL_IE>(), 20);
    assert_eq!(align_of::<DOT11_ADDITIONAL_IE>(), 4);
    assert_eq!(size_of::<DOT11_PEER_STATISTICS>(), 48);
    assert_eq!(align_of::<DOT11_PEER_STATISTICS>(), 8);
    assert_eq!(size_of::<DOT11_PEER_INFO>(), 352);
    assert_eq!(align_of::<DOT11_PEER_INFO>(), 8);
    assert_eq!(size_of::<DOT11_PEER_INFO_LIST>(), 368);
    assert_eq!(align_of::<DOT11_PEER_INFO_LIST>(), 8);
    assert_eq!(size_of::<DOT11_VWIFI_COMBINATION>(), 16);
    assert_eq!(align_of::<DOT11_VWIFI_COMBINATION>(), 4);
    assert_eq!(size_of::<DOT11_VWIFI_COMBINATION_V2>(), 20);
    assert_eq!(align_of::<DOT11_VWIFI_COMBINATION_V2>(), 4);
    assert_eq!(size_of::<DOT11_VWIFI_COMBINATION_V3>(), 24);
    assert_eq!(align_of::<DOT11_VWIFI_COMBINATION_V3>(), 4);
    assert_eq!(size_of::<DOT11_VWIFI_ATTRIBUTES>(), 24);
    assert_eq!(align_of::<DOT11_VWIFI_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<DOT11_MAC_PARAMETERS>(), 8);
    assert_eq!(align_of::<DOT11_MAC_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_MAC_INFO>(), 16);
    assert_eq!(align_of::<DOT11_MAC_INFO>(), 4);
    assert_eq!(size_of::<DOT11_WFD_ATTRIBUTES>(), 72);
    assert_eq!(align_of::<DOT11_WFD_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<DOT11_WFD_DEVICE_TYPE>(), 8);
    assert_eq!(align_of::<DOT11_WFD_DEVICE_TYPE>(), 2);
    assert_eq!(size_of::<DOT11_WPS_DEVICE_NAME>(), 36);
    assert_eq!(align_of::<DOT11_WPS_DEVICE_NAME>(), 4);
    assert_eq!(size_of::<DOT11_WFD_CONFIGURATION_TIMEOUT>(), 2);
    assert_eq!(align_of::<DOT11_WFD_CONFIGURATION_TIMEOUT>(), 1);
    assert_eq!(size_of::<DOT11_WFD_GROUP_ID>(), 44);
    assert_eq!(align_of::<DOT11_WFD_GROUP_ID>(), 4);
    assert_eq!(size_of::<DOT11_WFD_GO_INTENT>(), 1);
    assert_eq!(align_of::<DOT11_WFD_GO_INTENT>(), 1);
    assert_eq!(size_of::<DOT11_WFD_CHANNEL>(), 5);
    assert_eq!(align_of::<DOT11_WFD_CHANNEL>(), 1);
    assert_eq!(size_of::<WFDSVC_CONNECTION_CAPABILITY>(), 3);
    assert_eq!(align_of::<WFDSVC_CONNECTION_CAPABILITY>(), 1);
    assert_eq!(size_of::<DOT11_WFD_SERVICE_HASH_LIST>(), 8);
    assert_eq!(align_of::<DOT11_WFD_SERVICE_HASH_LIST>(), 2);
    assert_eq!(size_of::<DOT11_WFD_ADVERTISEMENT_ID>(), 12);
    assert_eq!(align_of::<DOT11_WFD_ADVERTISEMENT_ID>(), 4);
    assert_eq!(size_of::<DOT11_WFD_SESSION_ID>(), 12);
    assert_eq!(align_of::<DOT11_WFD_SESSION_ID>(), 4);
    assert_eq!(size_of::<DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR>(), 264);
    assert_eq!(align_of::<DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<DOT11_WFD_ADVERTISED_SERVICE_LIST>(), 268);
    assert_eq!(align_of::<DOT11_WFD_ADVERTISED_SERVICE_LIST>(), 4);
    assert_eq!(size_of::<DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS>(), 24);
    assert_eq!(align_of::<DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS>(), 4);
    assert_eq!(
        size_of::<DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS>(),
        24
    );
    assert_eq!(
        align_of::<DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS>(),
        32
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS>(),
        8
    );
    assert_eq!(
        size_of::<DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS>(),
        24
    );
    assert_eq!(
        align_of::<DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS>(),
        32
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS>(),
        8
    );
    assert_eq!(
        size_of::<DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS>(),
        24
    );
    assert_eq!(
        align_of::<DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS>(),
        20
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS>(),
        32
    );
    assert_eq!(
        align_of::<DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS>(),
        40
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS>(),
        8
    );
    assert_eq!(
        size_of::<DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS>(),
        24
    );
    assert_eq!(
        align_of::<DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS>(),
        28
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS>(),
        32
    );
    assert_eq!(
        align_of::<DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS>(),
        40
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS>(),
        8
    );
    assert_eq!(
        size_of::<DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS>(),
        24
    );
    assert_eq!(
        align_of::<DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS>(),
        28
    );
    assert_eq!(
        align_of::<DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS>(),
        4
    );
    assert_eq!(size_of::<DOT11_ANQP_QUERY_COMPLETE_PARAMETERS>(), 24);
    assert_eq!(align_of::<DOT11_ANQP_QUERY_COMPLETE_PARAMETERS>(), 8);
    assert_eq!(size_of::<DOT11_WFD_DEVICE_CAPABILITY_CONFIG>(), 16);
    assert_eq!(align_of::<DOT11_WFD_DEVICE_CAPABILITY_CONFIG>(), 4);
    assert_eq!(size_of::<DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG>(), 16);
    assert_eq!(align_of::<DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG>(), 4);
    assert_eq!(size_of::<DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2>(), 20);
    assert_eq!(align_of::<DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2>(), 4);
    assert_eq!(size_of::<DOT11_WFD_DEVICE_INFO>(), 56);
    assert_eq!(align_of::<DOT11_WFD_DEVICE_INFO>(), 4);
    assert_eq!(size_of::<DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST>(), 20);
    assert_eq!(align_of::<DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST>(), 4);
    assert_eq!(size_of::<DOT11_WFD_DISCOVER_DEVICE_FILTER>(), 44);
    assert_eq!(align_of::<DOT11_WFD_DISCOVER_DEVICE_FILTER>(), 4);
    assert_eq!(size_of::<DOT11_WFD_DISCOVER_REQUEST>(), 36);
    assert_eq!(align_of::<DOT11_WFD_DISCOVER_REQUEST>(), 4);
    assert_eq!(size_of::<DOT11_WFD_DEVICE_ENTRY>(), 96);
    assert_eq!(align_of::<DOT11_WFD_DEVICE_ENTRY>(), 8);
    assert_eq!(size_of::<DOT11_WFD_ADDITIONAL_IE>(), 28);
    assert_eq!(align_of::<DOT11_WFD_ADDITIONAL_IE>(), 4);
    assert_eq!(
        size_of::<DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS>(),
        36
    );
    assert_eq!(
        align_of::<DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS>(),
        96
    );
    assert_eq!(
        align_of::<DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS>(),
        8
    );
    assert_eq!(
        size_of::<DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS>(),
        88
    );
    assert_eq!(
        align_of::<DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS>(),
        8
    );
    assert_eq!(size_of::<DOT11_WFD_INVITATION_FLAGS>(), 1);
    assert_eq!(align_of::<DOT11_WFD_INVITATION_FLAGS>(), 1);
    assert_eq!(size_of::<DOT11_SEND_INVITATION_REQUEST_PARAMETERS>(), 88);
    assert_eq!(align_of::<DOT11_SEND_INVITATION_REQUEST_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_SEND_INVITATION_RESPONSE_PARAMETERS>(), 56);
    assert_eq!(align_of::<DOT11_SEND_INVITATION_RESPONSE_PARAMETERS>(), 8);
    assert_eq!(
        size_of::<DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS>(),
        76
    );
    assert_eq!(
        align_of::<DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS>(),
        4
    );
    assert_eq!(
        size_of::<DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS>(),
        40
    );
    assert_eq!(
        align_of::<DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS>(),
        8
    );
    assert_eq!(size_of::<DOT11_WFD_DEVICE_LISTEN_CHANNEL>(), 6);
    assert_eq!(align_of::<DOT11_WFD_DEVICE_LISTEN_CHANNEL>(), 2);
    assert_eq!(size_of::<DOT11_WFD_GROUP_START_PARAMETERS>(), 10);
    assert_eq!(align_of::<DOT11_WFD_GROUP_START_PARAMETERS>(), 2);
    assert_eq!(size_of::<DOT11_WFD_GROUP_JOIN_PARAMETERS>(), 20);
    assert_eq!(align_of::<DOT11_WFD_GROUP_JOIN_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO>(), 6);
    assert_eq!(align_of::<DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO>(), 2);
    assert_eq!(size_of::<DOT11_POWER_MGMT_MODE_STATUS_INFO>(), 16);
    assert_eq!(align_of::<DOT11_POWER_MGMT_MODE_STATUS_INFO>(), 4);
    assert_eq!(size_of::<DOT11_CHANNEL_HINT>(), 8);
    assert_eq!(align_of::<DOT11_CHANNEL_HINT>(), 4);
    assert_eq!(size_of::<DOT11_OFFLOAD_NETWORK>(), 76);
    assert_eq!(align_of::<DOT11_OFFLOAD_NETWORK>(), 4);
    assert_eq!(size_of::<DOT11_OFFLOAD_NETWORK_LIST_INFO>(), 100);
    assert_eq!(align_of::<DOT11_OFFLOAD_NETWORK_LIST_INFO>(), 4);
    assert_eq!(size_of::<DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS>(), 8);
    assert_eq!(align_of::<DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS>(), 4);
    assert_eq!(size_of::<DOT11_MANUFACTURING_TEST>(), 12);
    assert_eq!(align_of::<DOT11_MANUFACTURING_TEST>(), 4);
    assert_eq!(size_of::<DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS>(), 32);
    assert_eq!(align_of::<DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS>(), 8);
    assert_eq!(size_of::<DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS>(), 32);
    assert_eq!(align_of::<DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS>(), 8);
    assert_eq!(size_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX>(), 16);
    assert_eq!(align_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX>(), 4);
    assert_eq!(size_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX>(), 20);
    assert_eq!(align_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX>(), 4);
    assert_eq!(
        size_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC>(),
        12
    );
    assert_eq!(
        align_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC>(),
        4
    );
    assert_eq!(size_of::<DOT11_MANUFACTURING_TEST_SET_DATA>(), 16);
    assert_eq!(align_of::<DOT11_MANUFACTURING_TEST_SET_DATA>(), 4);
    assert_eq!(size_of::<DOT11_MANUFACTURING_TEST_QUERY_DATA>(), 20);
    assert_eq!(align_of::<DOT11_MANUFACTURING_TEST_QUERY_DATA>(), 4);
    assert_eq!(size_of::<DOT11_MANUFACTURING_TEST_SLEEP>(), 16);
    assert_eq!(align_of::<DOT11_MANUFACTURING_TEST_SLEEP>(), 8);
    assert_eq!(size_of::<DOT11_MANUFACTURING_CALLBACK_PARAMETERS>(), 24);
    assert_eq!(align_of::<DOT11_MANUFACTURING_CALLBACK_PARAMETERS>(), 8);
}
#[cfg(feature = "winusbio")]
#[test]
fn shared_winusbio() {
    use winapi::shared::winusbio::*;
    assert_eq!(size_of::<WINUSB_PIPE_INFORMATION>(), 12);
    assert_eq!(align_of::<WINUSB_PIPE_INFORMATION>(), 4);
    assert_eq!(size_of::<WINUSB_PIPE_INFORMATION_EX>(), 16);
    assert_eq!(align_of::<WINUSB_PIPE_INFORMATION_EX>(), 4);
}
#[cfg(feature = "wlantypes")]
#[test]
fn shared_wlantypes() {
    use winapi::shared::wlantypes::*;
    assert_eq!(size_of::<DOT11_SSID>(), 36);
    assert_eq!(align_of::<DOT11_SSID>(), 4);
    assert_eq!(size_of::<DOT11_AUTH_CIPHER_PAIR>(), 8);
    assert_eq!(align_of::<DOT11_AUTH_CIPHER_PAIR>(), 4);
    assert_eq!(size_of::<DOT11_OI>(), 8);
    assert_eq!(align_of::<DOT11_OI>(), 2);
    assert_eq!(size_of::<DOT11_ACCESSNETWORKOPTIONS>(), 5);
    assert_eq!(align_of::<DOT11_ACCESSNETWORKOPTIONS>(), 1);
    assert_eq!(size_of::<DOT11_VENUEINFO>(), 2);
    assert_eq!(align_of::<DOT11_VENUEINFO>(), 1);
}
#[cfg(feature = "wmistr")]
#[test]
fn shared_wmistr() {
    use winapi::shared::wmistr::*;
    assert_eq!(size_of::<WNODE_HEADER_u1_s>(), 8);
    assert_eq!(align_of::<WNODE_HEADER_u1_s>(), 4);
    assert_eq!(size_of::<WNODE_HEADER_u1>(), 8);
    assert_eq!(align_of::<WNODE_HEADER_u1>(), 8);
    assert_eq!(size_of::<WNODE_HEADER_u2>(), 8);
    assert_eq!(align_of::<WNODE_HEADER_u2>(), 8);
    assert_eq!(size_of::<WNODE_HEADER>(), 48);
    assert_eq!(align_of::<WNODE_HEADER>(), 8);
    assert_eq!(size_of::<OFFSETINSTANCEDATAANDLENGTH>(), 8);
    assert_eq!(align_of::<OFFSETINSTANCEDATAANDLENGTH>(), 4);
    assert_eq!(size_of::<WNODE_ALL_DATA_u>(), 8);
    assert_eq!(align_of::<WNODE_ALL_DATA_u>(), 4);
    assert_eq!(size_of::<WNODE_ALL_DATA>(), 72);
    assert_eq!(align_of::<WNODE_ALL_DATA>(), 8);
    assert_eq!(size_of::<WNODE_SINGLE_INSTANCE>(), 64);
    assert_eq!(align_of::<WNODE_SINGLE_INSTANCE>(), 8);
    assert_eq!(size_of::<WNODE_SINGLE_ITEM>(), 72);
    assert_eq!(align_of::<WNODE_SINGLE_ITEM>(), 8);
    assert_eq!(size_of::<WNODE_METHOD_ITEM>(), 72);
    assert_eq!(align_of::<WNODE_METHOD_ITEM>(), 8);
    assert_eq!(size_of::<WNODE_EVENT_ITEM>(), 48);
    assert_eq!(align_of::<WNODE_EVENT_ITEM>(), 8);
    assert_eq!(size_of::<WNODE_EVENT_REFERENCE_u>(), 4);
    assert_eq!(align_of::<WNODE_EVENT_REFERENCE_u>(), 4);
    assert_eq!(size_of::<WNODE_EVENT_REFERENCE>(), 72);
    assert_eq!(align_of::<WNODE_EVENT_REFERENCE>(), 8);
    assert_eq!(size_of::<WNODE_TOO_SMALL>(), 56);
    assert_eq!(align_of::<WNODE_TOO_SMALL>(), 8);
    assert_eq!(size_of::<WMIREGGUIDW_u>(), 8);
    assert_eq!(align_of::<WMIREGGUIDW_u>(), 8);
    assert_eq!(size_of::<WMIREGGUIDW>(), 32);
    assert_eq!(align_of::<WMIREGGUIDW>(), 8);
    assert_eq!(size_of::<WMIREGINFOW>(), 24);
    assert_eq!(align_of::<WMIREGINFOW>(), 8);
}
#[cfg(feature = "ws2def")]
#[test]
fn shared_ws2def() {
    use winapi::shared::ws2def::*;
    assert_eq!(size_of::<SOCKADDR>(), 16);
    assert_eq!(align_of::<SOCKADDR>(), 2);
    assert_eq!(size_of::<SOCKET_ADDRESS>(), 16);
    assert_eq!(align_of::<SOCKET_ADDRESS>(), 8);
    assert_eq!(size_of::<SOCKET_ADDRESS_LIST>(), 24);
    assert_eq!(align_of::<SOCKET_ADDRESS_LIST>(), 8);
    assert_eq!(size_of::<CSADDR_INFO>(), 40);
    assert_eq!(align_of::<CSADDR_INFO>(), 8);
    assert_eq!(size_of::<SOCKADDR_STORAGE_LH>(), 128);
    assert_eq!(align_of::<SOCKADDR_STORAGE_LH>(), 8);
    assert_eq!(size_of::<SOCKADDR_STORAGE_XP>(), 128);
    assert_eq!(align_of::<SOCKADDR_STORAGE_XP>(), 8);
    assert_eq!(size_of::<SOCKET_PROCESSOR_AFFINITY>(), 8);
    assert_eq!(align_of::<SOCKET_PROCESSOR_AFFINITY>(), 2);
    assert_eq!(size_of::<SCOPE_ID>(), 4);
    assert_eq!(align_of::<SCOPE_ID>(), 4);
    assert_eq!(size_of::<SOCKADDR_IN>(), 16);
    assert_eq!(align_of::<SOCKADDR_IN>(), 4);
    assert_eq!(size_of::<SOCKADDR_DL>(), 14);
    assert_eq!(align_of::<SOCKADDR_DL>(), 2);
    assert_eq!(size_of::<WSABUF>(), 16);
    assert_eq!(align_of::<WSABUF>(), 8);
    assert_eq!(size_of::<WSAMSG>(), 56);
    assert_eq!(align_of::<WSAMSG>(), 8);
    assert_eq!(size_of::<WSACMSGHDR>(), 16);
    assert_eq!(align_of::<WSACMSGHDR>(), 8);
    assert_eq!(size_of::<ADDRINFOA>(), 48);
    assert_eq!(align_of::<ADDRINFOA>(), 8);
    assert_eq!(size_of::<ADDRINFOW>(), 48);
    assert_eq!(align_of::<ADDRINFOW>(), 8);
    assert_eq!(size_of::<ADDRINFOEXA>(), 72);
    assert_eq!(align_of::<ADDRINFOEXA>(), 8);
    assert_eq!(size_of::<ADDRINFOEXW>(), 72);
    assert_eq!(align_of::<ADDRINFOEXW>(), 8);
    assert_eq!(size_of::<ADDRINFOEX2A>(), 88);
    assert_eq!(align_of::<ADDRINFOEX2A>(), 8);
    assert_eq!(size_of::<ADDRINFOEX2W>(), 88);
    assert_eq!(align_of::<ADDRINFOEX2W>(), 8);
    assert_eq!(size_of::<ADDRINFOEX4>(), 104);
    assert_eq!(align_of::<ADDRINFOEX4>(), 8);
}
#[cfg(feature = "ws2ipdef")]
#[test]
fn shared_ws2ipdef() {
    use winapi::shared::ws2ipdef::*;
    assert_eq!(size_of::<SOCKADDR_IN6_LH>(), 28);
    assert_eq!(align_of::<SOCKADDR_IN6_LH>(), 4);
    assert_eq!(size_of::<SOCKADDR_IN6_PAIR>(), 16);
    assert_eq!(align_of::<SOCKADDR_IN6_PAIR>(), 8);
    assert_eq!(size_of::<SOCKADDR_INET>(), 28);
    assert_eq!(align_of::<SOCKADDR_INET>(), 4);
    assert_eq!(size_of::<IP_MREQ>(), 8);
    assert_eq!(align_of::<IP_MREQ>(), 4);
    assert_eq!(size_of::<IP_MREQ_SOURCE>(), 12);
    assert_eq!(align_of::<IP_MREQ_SOURCE>(), 4);
    assert_eq!(size_of::<IPV6_MREQ>(), 20);
    assert_eq!(align_of::<IPV6_MREQ>(), 4);
    assert_eq!(size_of::<IN_PKTINFO>(), 8);
    assert_eq!(align_of::<IN_PKTINFO>(), 4);
    assert_eq!(size_of::<IN6_PKTINFO>(), 20);
    assert_eq!(align_of::<IN6_PKTINFO>(), 4);
}
#[cfg(feature = "wtypes")]
#[test]
fn shared_wtypes() {
    use winapi::shared::wtypes::*;
    assert_eq!(size_of::<RemHGLOBAL>(), 12);
    assert_eq!(align_of::<RemHGLOBAL>(), 4);
    assert_eq!(size_of::<RemHMETAFILEPICT>(), 20);
    assert_eq!(align_of::<RemHMETAFILEPICT>(), 4);
    assert_eq!(size_of::<RemHENHMETAFILE>(), 8);
    assert_eq!(align_of::<RemHENHMETAFILE>(), 4);
    assert_eq!(size_of::<RemHBITMAP>(), 8);
    assert_eq!(align_of::<RemHBITMAP>(), 4);
    assert_eq!(size_of::<RemHPALETTE>(), 8);
    assert_eq!(align_of::<RemHPALETTE>(), 4);
    assert_eq!(size_of::<RemHBRUSH>(), 8);
    assert_eq!(align_of::<RemHBRUSH>(), 4);
    assert_eq!(size_of::<userCLIPFORMAT_u>(), 8);
    assert_eq!(align_of::<userCLIPFORMAT_u>(), 8);
    assert_eq!(size_of::<userCLIPFORMAT>(), 16);
    assert_eq!(align_of::<userCLIPFORMAT>(), 8);
    assert_eq!(size_of::<GDI_NONREMOTE_u>(), 8);
    assert_eq!(align_of::<GDI_NONREMOTE_u>(), 8);
    assert_eq!(size_of::<GDI_NONREMOTE>(), 16);
    assert_eq!(align_of::<GDI_NONREMOTE>(), 8);
    assert_eq!(size_of::<userHGLOBAL_u>(), 8);
    assert_eq!(align_of::<userHGLOBAL_u>(), 8);
    assert_eq!(size_of::<userHGLOBAL>(), 16);
    assert_eq!(align_of::<userHGLOBAL>(), 8);
    assert_eq!(size_of::<userHMETAFILE_u>(), 8);
    assert_eq!(align_of::<userHMETAFILE_u>(), 8);
    assert_eq!(size_of::<userHMETAFILE>(), 16);
    assert_eq!(align_of::<userHMETAFILE>(), 8);
    assert_eq!(size_of::<remoteMETAFILEPICT>(), 24);
    assert_eq!(align_of::<remoteMETAFILEPICT>(), 8);
    assert_eq!(size_of::<userHMETAFILEPICT_u>(), 8);
    assert_eq!(align_of::<userHMETAFILEPICT_u>(), 8);
    assert_eq!(size_of::<userHMETAFILEPICT>(), 16);
    assert_eq!(align_of::<userHMETAFILEPICT>(), 8);
    assert_eq!(size_of::<userHENHMETAFILE_u>(), 8);
    assert_eq!(align_of::<userHENHMETAFILE_u>(), 8);
    assert_eq!(size_of::<userHENHMETAFILE>(), 16);
    assert_eq!(align_of::<userHENHMETAFILE>(), 8);
    assert_eq!(size_of::<userBITMAP>(), 28);
    assert_eq!(align_of::<userBITMAP>(), 4);
    assert_eq!(size_of::<userHBITMAP_u>(), 8);
    assert_eq!(align_of::<userHBITMAP_u>(), 8);
    assert_eq!(size_of::<userHBITMAP>(), 16);
    assert_eq!(align_of::<userHBITMAP>(), 8);
    assert_eq!(size_of::<userHPALETTE_u>(), 8);
    assert_eq!(align_of::<userHPALETTE_u>(), 8);
    assert_eq!(size_of::<userHPALETTE>(), 16);
    assert_eq!(align_of::<userHPALETTE>(), 8);
    assert_eq!(size_of::<RemotableHandle_u>(), 4);
    assert_eq!(align_of::<RemotableHandle_u>(), 4);
    assert_eq!(size_of::<RemotableHandle>(), 8);
    assert_eq!(align_of::<RemotableHandle>(), 4);
    assert_eq!(size_of::<CY>(), 8);
    assert_eq!(align_of::<CY>(), 8);
    assert_eq!(size_of::<DECIMAL>(), 16);
    assert_eq!(align_of::<DECIMAL>(), 8);
    assert_eq!(size_of::<BSTRBLOB>(), 16);
    assert_eq!(align_of::<BSTRBLOB>(), 8);
    assert_eq!(size_of::<CLIPDATA>(), 16);
    assert_eq!(align_of::<CLIPDATA>(), 8);
    assert_eq!(size_of::<PROPERTYKEY>(), 20);
    assert_eq!(align_of::<PROPERTYKEY>(), 4);
    assert_eq!(size_of::<CSPLATFORM>(), 16);
    assert_eq!(align_of::<CSPLATFORM>(), 4);
    assert_eq!(size_of::<QUERYCONTEXT>(), 32);
    assert_eq!(align_of::<QUERYCONTEXT>(), 4);
    assert_eq!(size_of::<uCLSSPEC_ByName>(), 24);
    assert_eq!(align_of::<uCLSSPEC_ByName>(), 8);
    assert_eq!(size_of::<uCLSSPEC_ByObjectId>(), 32);
    assert_eq!(align_of::<uCLSSPEC_ByObjectId>(), 4);
    assert_eq!(size_of::<uCLSSPEC_u>(), 32);
    assert_eq!(align_of::<uCLSSPEC_u>(), 8);
    assert_eq!(size_of::<uCLSSPEC>(), 40);
    assert_eq!(align_of::<uCLSSPEC>(), 8);
}
#[cfg(feature = "wtypesbase")]
#[test]
fn shared_wtypesbase() {
    use winapi::shared::wtypesbase::*;
    assert_eq!(size_of::<COAUTHIDENTITY>(), 48);
    assert_eq!(align_of::<COAUTHIDENTITY>(), 8);
    assert_eq!(size_of::<COAUTHINFO>(), 40);
    assert_eq!(align_of::<COAUTHINFO>(), 8);
    assert_eq!(size_of::<BYTE_BLOB>(), 8);
    assert_eq!(align_of::<BYTE_BLOB>(), 4);
    assert_eq!(size_of::<WORD_BLOB>(), 8);
    assert_eq!(align_of::<WORD_BLOB>(), 4);
    assert_eq!(size_of::<DWORD_BLOB>(), 8);
    assert_eq!(align_of::<DWORD_BLOB>(), 4);
    assert_eq!(size_of::<FLAGGED_BYTE_BLOB>(), 12);
    assert_eq!(align_of::<FLAGGED_BYTE_BLOB>(), 4);
    assert_eq!(size_of::<FLAGGED_WORD_BLOB>(), 12);
    assert_eq!(align_of::<FLAGGED_WORD_BLOB>(), 4);
    assert_eq!(size_of::<BYTE_SIZEDARR>(), 16);
    assert_eq!(align_of::<BYTE_SIZEDARR>(), 8);
    assert_eq!(size_of::<WORD_SIZEDARR>(), 16);
    assert_eq!(align_of::<WORD_SIZEDARR>(), 8);
    assert_eq!(size_of::<DWORD_SIZEDARR>(), 16);
    assert_eq!(align_of::<DWORD_SIZEDARR>(), 8);
    assert_eq!(size_of::<HYPER_SIZEDARR>(), 16);
    assert_eq!(align_of::<HYPER_SIZEDARR>(), 8);
    assert_eq!(size_of::<BLOB>(), 16);
    assert_eq!(align_of::<BLOB>(), 8);
}
#[cfg(feature = "accctrl")]
#[test]
fn um_accctrl() {
    use winapi::um::accctrl::*;
    assert_eq!(size_of::<OBJECTS_AND_SID>(), 48);
    assert_eq!(align_of::<OBJECTS_AND_SID>(), 8);
    assert_eq!(size_of::<OBJECTS_AND_NAME_A>(), 32);
    assert_eq!(align_of::<OBJECTS_AND_NAME_A>(), 8);
    assert_eq!(size_of::<OBJECTS_AND_NAME_W>(), 32);
    assert_eq!(align_of::<OBJECTS_AND_NAME_W>(), 8);
    assert_eq!(size_of::<TRUSTEE_A>(), 32);
    assert_eq!(align_of::<TRUSTEE_A>(), 8);
    assert_eq!(size_of::<TRUSTEE_W>(), 32);
    assert_eq!(align_of::<TRUSTEE_W>(), 8);
    assert_eq!(size_of::<EXPLICIT_ACCESS_A>(), 48);
    assert_eq!(align_of::<EXPLICIT_ACCESS_A>(), 8);
    assert_eq!(size_of::<EXPLICIT_ACCESS_W>(), 48);
    assert_eq!(align_of::<EXPLICIT_ACCESS_W>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESS_ENTRYA>(), 56);
    assert_eq!(align_of::<ACTRL_ACCESS_ENTRYA>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESS_ENTRYW>(), 56);
    assert_eq!(align_of::<ACTRL_ACCESS_ENTRYW>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESS_ENTRY_LISTA>(), 16);
    assert_eq!(align_of::<ACTRL_ACCESS_ENTRY_LISTA>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESS_ENTRY_LISTW>(), 16);
    assert_eq!(align_of::<ACTRL_ACCESS_ENTRY_LISTW>(), 8);
    assert_eq!(size_of::<ACTRL_PROPERTY_ENTRYA>(), 24);
    assert_eq!(align_of::<ACTRL_PROPERTY_ENTRYA>(), 8);
    assert_eq!(size_of::<ACTRL_PROPERTY_ENTRYW>(), 24);
    assert_eq!(align_of::<ACTRL_PROPERTY_ENTRYW>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESSA>(), 16);
    assert_eq!(align_of::<ACTRL_ACCESSA>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESSW>(), 16);
    assert_eq!(align_of::<ACTRL_ACCESSW>(), 8);
    assert_eq!(size_of::<TRUSTEE_ACCESSA>(), 24);
    assert_eq!(align_of::<TRUSTEE_ACCESSA>(), 8);
    assert_eq!(size_of::<TRUSTEE_ACCESSW>(), 24);
    assert_eq!(align_of::<TRUSTEE_ACCESSW>(), 8);
    assert_eq!(size_of::<ACTRL_OVERLAPPED_u>(), 8);
    assert_eq!(align_of::<ACTRL_OVERLAPPED_u>(), 8);
    assert_eq!(size_of::<ACTRL_OVERLAPPED>(), 24);
    assert_eq!(align_of::<ACTRL_OVERLAPPED>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESS_INFOA>(), 16);
    assert_eq!(align_of::<ACTRL_ACCESS_INFOA>(), 8);
    assert_eq!(size_of::<ACTRL_ACCESS_INFOW>(), 16);
    assert_eq!(align_of::<ACTRL_ACCESS_INFOW>(), 8);
    assert_eq!(size_of::<ACTRL_CONTROL_INFOA>(), 16);
    assert_eq!(align_of::<ACTRL_CONTROL_INFOA>(), 8);
    assert_eq!(size_of::<ACTRL_CONTROL_INFOW>(), 16);
    assert_eq!(align_of::<ACTRL_CONTROL_INFOW>(), 8);
    assert_eq!(size_of::<FN_OBJECT_MGR_FUNCTS>(), 4);
    assert_eq!(align_of::<FN_OBJECT_MGR_FUNCTS>(), 4);
    assert_eq!(size_of::<INHERITED_FROMA>(), 16);
    assert_eq!(align_of::<INHERITED_FROMA>(), 8);
    assert_eq!(size_of::<INHERITED_FROMW>(), 16);
    assert_eq!(align_of::<INHERITED_FROMW>(), 8);
}
#[cfg(feature = "appmgmt")]
#[test]
fn um_appmgmt() {
    use winapi::um::appmgmt::*;
    assert_eq!(size_of::<INSTALLSPEC_APPNAME>(), 24);
    assert_eq!(align_of::<INSTALLSPEC_APPNAME>(), 8);
    assert_eq!(size_of::<INSTALLSPEC_COMCLASS>(), 20);
    assert_eq!(align_of::<INSTALLSPEC_COMCLASS>(), 4);
    assert_eq!(size_of::<INSTALLSPEC>(), 24);
    assert_eq!(align_of::<INSTALLSPEC>(), 8);
    assert_eq!(size_of::<INSTALLDATA>(), 32);
    assert_eq!(align_of::<INSTALLDATA>(), 8);
    assert_eq!(size_of::<LOCALMANAGEDAPPLICATION>(), 32);
    assert_eq!(align_of::<LOCALMANAGEDAPPLICATION>(), 8);
    assert_eq!(size_of::<MANAGEDAPPLICATION>(), 128);
    assert_eq!(align_of::<MANAGEDAPPLICATION>(), 8);
    assert_eq!(size_of::<APPCATEGORYINFO>(), 32);
    assert_eq!(align_of::<APPCATEGORYINFO>(), 8);
    assert_eq!(size_of::<APPCATEGORYINFOLIST>(), 16);
    assert_eq!(align_of::<APPCATEGORYINFOLIST>(), 8);
}
#[cfg(feature = "bits")]
#[test]
fn um_bits() {
    use winapi::um::bits::*;
    assert_eq!(size_of::<BG_FILE_PROGRESS>(), 24);
    assert_eq!(align_of::<BG_FILE_PROGRESS>(), 8);
    assert_eq!(size_of::<BG_FILE_INFO>(), 16);
    assert_eq!(align_of::<BG_FILE_INFO>(), 8);
    assert_eq!(size_of::<BG_JOB_PROGRESS>(), 24);
    assert_eq!(align_of::<BG_JOB_PROGRESS>(), 8);
    assert_eq!(size_of::<BG_JOB_TIMES>(), 24);
    assert_eq!(align_of::<BG_JOB_TIMES>(), 4);
}
#[cfg(feature = "bits1_5")]
#[test]
fn um_bits1_5() {
    use winapi::um::bits1_5::*;
    assert_eq!(size_of::<BG_JOB_REPLY_PROGRESS>(), 16);
    assert_eq!(align_of::<BG_JOB_REPLY_PROGRESS>(), 8);
    assert_eq!(size_of::<BG_BASIC_CREDENTIALS>(), 16);
    assert_eq!(align_of::<BG_BASIC_CREDENTIALS>(), 8);
    assert_eq!(size_of::<BG_AUTH_CREDENTIALS_UNION>(), 16);
    assert_eq!(align_of::<BG_AUTH_CREDENTIALS_UNION>(), 8);
    assert_eq!(size_of::<BG_AUTH_CREDENTIALS>(), 24);
    assert_eq!(align_of::<BG_AUTH_CREDENTIALS>(), 8);
}
#[cfg(feature = "bits2_0")]
#[test]
fn um_bits2_0() {
    use winapi::um::bits2_0::*;
    assert_eq!(size_of::<BG_FILE_RANGE>(), 16);
    assert_eq!(align_of::<BG_FILE_RANGE>(), 8);
}
#[cfg(feature = "bits5_0")]
#[test]
fn um_bits5_0() {
    use winapi::um::bits5_0::*;
    assert_eq!(size_of::<BITS_JOB_PROPERTY_VALUE>(), 16);
    assert_eq!(align_of::<BITS_JOB_PROPERTY_VALUE>(), 8);
    assert_eq!(size_of::<BITS_FILE_PROPERTY_VALUE>(), 8);
    assert_eq!(align_of::<BITS_FILE_PROPERTY_VALUE>(), 8);
}
#[cfg(feature = "bluetoothapis")]
#[test]
fn um_bluetoothapis() {
    use winapi::um::bluetoothapis::*;
    assert_eq!(size_of::<BLUETOOTH_LOCAL_SERVICE_INFO>(), 1040);
    assert_eq!(align_of::<BLUETOOTH_LOCAL_SERVICE_INFO>(), 8);
    assert_eq!(size_of::<BLUETOOTH_FIND_RADIO_PARAMS>(), 4);
    assert_eq!(align_of::<BLUETOOTH_FIND_RADIO_PARAMS>(), 4);
    assert_eq!(size_of::<BLUETOOTH_RADIO_INFO>(), 520);
    assert_eq!(align_of::<BLUETOOTH_RADIO_INFO>(), 8);
    assert_eq!(size_of::<BLUETOOTH_DEVICE_INFO>(), 560);
    assert_eq!(align_of::<BLUETOOTH_DEVICE_INFO>(), 8);
    assert_eq!(size_of::<BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS>(), 576);
    assert_eq!(align_of::<BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>(), 40);
    assert_eq!(align_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_COD_PAIRS>(), 16);
    assert_eq!(align_of::<BLUETOOTH_COD_PAIRS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>(), 88);
    assert_eq!(align_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>(), 8);
    assert_eq!(size_of::<BLUETOOTH_PIN_INFO>(), 17);
    assert_eq!(align_of::<BLUETOOTH_PIN_INFO>(), 1);
    assert_eq!(size_of::<BLUETOOTH_OOB_DATA_INFO>(), 32);
    assert_eq!(align_of::<BLUETOOTH_OOB_DATA_INFO>(), 1);
    assert_eq!(size_of::<BLUETOOTH_NUMERIC_COMPARISON_INFO>(), 4);
    assert_eq!(align_of::<BLUETOOTH_NUMERIC_COMPARISON_INFO>(), 4);
    assert_eq!(size_of::<BLUETOOTH_PASSKEY_INFO>(), 4);
    assert_eq!(align_of::<BLUETOOTH_PASSKEY_INFO>(), 4);
    assert_eq!(size_of::<BLUETOOTH_AUTHENTICATE_RESPONSE>(), 48);
    assert_eq!(align_of::<BLUETOOTH_AUTHENTICATE_RESPONSE>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_string>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_string>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_url>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_url>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_sequence>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_sequence>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data_alternative>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data_alternative>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA_data>(), 16);
    assert_eq!(align_of::<SDP_ELEMENT_DATA_data>(), 8);
    assert_eq!(size_of::<SDP_ELEMENT_DATA>(), 24);
    assert_eq!(align_of::<SDP_ELEMENT_DATA>(), 8);
    assert_eq!(size_of::<SDP_STRING_TYPE_DATA>(), 6);
    assert_eq!(align_of::<SDP_STRING_TYPE_DATA>(), 2);
}
#[cfg(feature = "bthledef")]
#[test]
fn um_bthledef() {
    use winapi::um::bthledef::*;
    assert_eq!(size_of::<BTH_LE_UUID>(), 20);
    assert_eq!(align_of::<BTH_LE_UUID>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_SERVICE>(), 24);
    assert_eq!(align_of::<BTH_LE_GATT_SERVICE>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_CHARACTERISTIC>(), 36);
    assert_eq!(align_of::<BTH_LE_GATT_CHARACTERISTIC>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_CHARACTERISTIC_VALUE>(), 8);
    assert_eq!(align_of::<BTH_LE_GATT_CHARACTERISTIC_VALUE>(), 4);
    assert_eq!(size_of::<BTH_LE_GATT_DESCRIPTOR>(), 32);
    assert_eq!(align_of::<BTH_LE_GATT_DESCRIPTOR>(), 4);
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicExtendedProperties>(),
        2
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicExtendedProperties>(),
        1
    );
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ClientCharacteristicConfiguration>(),
        2
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ClientCharacteristicConfiguration>(),
        1
    );
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ServerCharacteristicConfiguration>(),
        1
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_ServerCharacteristicConfiguration>(),
        1
    );
    assert_eq!(
        size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicFormat>(),
        48
    );
    assert_eq!(
        align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE_u_CharacteristicFormat>(),
        4
    );
    assert_eq!(size_of::<BTH_LE_GATT_DESCRIPTOR_VALUE>(), 80);
    assert_eq!(align_of::<BTH_LE_GATT_DESCRIPTOR_VALUE>(), 4);
    assert_eq!(
        size_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT_REGISTRATION>(),
        40
    );
    assert_eq!(
        align_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT_REGISTRATION>(),
        4
    );
    assert_eq!(size_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT>(), 24);
    assert_eq!(align_of::<BLUETOOTH_GATT_VALUE_CHANGED_EVENT>(), 8);
}
#[cfg(feature = "cfgmgr32")]
#[test]
fn um_cfgmgr32() {
    use winapi::um::cfgmgr32::*;
    assert_eq!(size_of::<CONFLICT_DETAILS_A>(), 288);
    assert_eq!(align_of::<CONFLICT_DETAILS_A>(), 8);
    assert_eq!(size_of::<CONFLICT_DETAILS_W>(), 552);
    assert_eq!(align_of::<CONFLICT_DETAILS_W>(), 8);
    assert_eq!(size_of::<MEM_RANGE>(), 36);
    assert_eq!(align_of::<MEM_RANGE>(), 1);
    assert_eq!(size_of::<MEM_DES>(), 32);
    assert_eq!(align_of::<MEM_DES>(), 1);
    assert_eq!(size_of::<MEM_RESOURCE>(), 68);
    assert_eq!(align_of::<MEM_RESOURCE>(), 1);
    assert_eq!(size_of::<MEM_LARGE_RANGE>(), 40);
    assert_eq!(align_of::<MEM_LARGE_RANGE>(), 1);
    assert_eq!(size_of::<MEM_LARGE_DES>(), 32);
    assert_eq!(align_of::<MEM_LARGE_DES>(), 1);
    assert_eq!(size_of::<MEM_LARGE_RESOURCE>(), 72);
    assert_eq!(align_of::<MEM_LARGE_RESOURCE>(), 1);
    assert_eq!(size_of::<IO_RANGE>(), 40);
    assert_eq!(align_of::<IO_RANGE>(), 1);
    assert_eq!(size_of::<IO_DES>(), 28);
    assert_eq!(align_of::<IO_DES>(), 1);
    assert_eq!(size_of::<IO_RESOURCE>(), 68);
    assert_eq!(align_of::<IO_RESOURCE>(), 1);
    assert_eq!(size_of::<DMA_RANGE>(), 12);
    assert_eq!(align_of::<DMA_RANGE>(), 1);
    assert_eq!(size_of::<DMA_DES>(), 16);
    assert_eq!(align_of::<DMA_DES>(), 1);
    assert_eq!(size_of::<DMA_RESOURCE>(), 28);
    assert_eq!(align_of::<DMA_RESOURCE>(), 1);
    assert_eq!(size_of::<IRQ_RANGE>(), 12);
    assert_eq!(align_of::<IRQ_RANGE>(), 1);
    assert_eq!(size_of::<IRQ_DES_32>(), 20);
    assert_eq!(align_of::<IRQ_DES_32>(), 1);
    assert_eq!(size_of::<IRQ_DES_64>(), 24);
    assert_eq!(align_of::<IRQ_DES_64>(), 1);
    assert_eq!(size_of::<IRQ_RESOURCE_32>(), 32);
    assert_eq!(align_of::<IRQ_RESOURCE_32>(), 1);
    assert_eq!(size_of::<IRQ_RESOURCE_64>(), 36);
    assert_eq!(align_of::<IRQ_RESOURCE_64>(), 1);
    assert_eq!(size_of::<DEVPRIVATE_RANGE>(), 12);
    assert_eq!(align_of::<DEVPRIVATE_RANGE>(), 1);
    assert_eq!(size_of::<DEVPRIVATE_DES>(), 24);
    assert_eq!(align_of::<DEVPRIVATE_DES>(), 1);
    assert_eq!(size_of::<DEVPRIVATE_RESOURCE>(), 36);
    assert_eq!(align_of::<DEVPRIVATE_RESOURCE>(), 1);
    assert_eq!(size_of::<CS_DES>(), 33);
    assert_eq!(align_of::<CS_DES>(), 1);
    assert_eq!(size_of::<CS_RESOURCE>(), 33);
    assert_eq!(align_of::<CS_RESOURCE>(), 1);
    assert_eq!(size_of::<PCCARD_DES>(), 38);
    assert_eq!(align_of::<PCCARD_DES>(), 1);
    assert_eq!(size_of::<PCCARD_RESOURCE>(), 38);
    assert_eq!(align_of::<PCCARD_RESOURCE>(), 1);
    assert_eq!(size_of::<MFCARD_DES>(), 20);
    assert_eq!(align_of::<MFCARD_DES>(), 1);
    assert_eq!(size_of::<MFCARD_RESOURCE>(), 20);
    assert_eq!(align_of::<MFCARD_RESOURCE>(), 1);
    assert_eq!(size_of::<BUSNUMBER_RANGE>(), 16);
    assert_eq!(align_of::<BUSNUMBER_RANGE>(), 1);
    assert_eq!(size_of::<BUSNUMBER_DES>(), 20);
    assert_eq!(align_of::<BUSNUMBER_DES>(), 1);
    assert_eq!(size_of::<BUSNUMBER_RESOURCE>(), 36);
    assert_eq!(align_of::<BUSNUMBER_RESOURCE>(), 1);
    assert_eq!(size_of::<CONNECTION_DES>(), 20);
    assert_eq!(align_of::<CONNECTION_DES>(), 1);
    assert_eq!(size_of::<CONNECTION_RESOURCE>(), 20);
    assert_eq!(align_of::<CONNECTION_RESOURCE>(), 1);
    assert_eq!(size_of::<HWPROFILEINFO_A>(), 88);
    assert_eq!(align_of::<HWPROFILEINFO_A>(), 1);
    assert_eq!(size_of::<HWPROFILEINFO_W>(), 168);
    assert_eq!(align_of::<HWPROFILEINFO_W>(), 1);
    assert_eq!(size_of::<CM_NOTIFY_FILTER_DeviceInterface>(), 16);
    assert_eq!(align_of::<CM_NOTIFY_FILTER_DeviceInterface>(), 4);
    assert_eq!(size_of::<CM_NOTIFY_FILTER_DeviceHandle>(), 8);
    assert_eq!(align_of::<CM_NOTIFY_FILTER_DeviceHandle>(), 8);
    assert_eq!(size_of::<CM_NOTIFY_FILTER_DeviceInstance>(), 400);
    assert_eq!(align_of::<CM_NOTIFY_FILTER_DeviceInstance>(), 2);
    assert_eq!(size_of::<CM_NOTIFY_FILTER_u>(), 400);
    assert_eq!(align_of::<CM_NOTIFY_FILTER_u>(), 8);
    assert_eq!(size_of::<CM_NOTIFY_FILTER>(), 416);
    assert_eq!(align_of::<CM_NOTIFY_FILTER>(), 8);
    assert_eq!(size_of::<CM_NOTIFY_EVENT_DATA_DeviceInterface>(), 20);
    assert_eq!(align_of::<CM_NOTIFY_EVENT_DATA_DeviceInterface>(), 4);
    assert_eq!(size_of::<CM_NOTIFY_EVENT_DATA_DeviceHandle>(), 28);
    assert_eq!(align_of::<CM_NOTIFY_EVENT_DATA_DeviceHandle>(), 4);
    assert_eq!(size_of::<CM_NOTIFY_EVENT_DATA_DeviceInstance>(), 2);
    assert_eq!(align_of::<CM_NOTIFY_EVENT_DATA_DeviceInstance>(), 2);
    assert_eq!(size_of::<CM_NOTIFY_EVENT_DATA_u>(), 28);
    assert_eq!(align_of::<CM_NOTIFY_EVENT_DATA_u>(), 4);
    assert_eq!(size_of::<CM_NOTIFY_EVENT_DATA>(), 36);
    assert_eq!(align_of::<CM_NOTIFY_EVENT_DATA>(), 4);
}
#[cfg(feature = "combaseapi")]
#[test]
fn um_combaseapi() {
    use winapi::um::combaseapi::*;
    assert_eq!(size_of::<ServerInformation>(), 16);
    assert_eq!(align_of::<ServerInformation>(), 8);
}
#[cfg(feature = "commctrl")]
#[test]
fn um_commctrl() {
    use winapi::um::commctrl::*;
    assert_eq!(size_of::<INITCOMMONCONTROLSEX>(), 8);
    assert_eq!(align_of::<INITCOMMONCONTROLSEX>(), 4);
    assert_eq!(size_of::<COLORSCHEME>(), 12);
    assert_eq!(align_of::<COLORSCHEME>(), 4);
    assert_eq!(size_of::<NMTOOLTIPSCREATED>(), 32);
    assert_eq!(align_of::<NMTOOLTIPSCREATED>(), 8);
    assert_eq!(size_of::<NMMOUSE>(), 56);
    assert_eq!(align_of::<NMMOUSE>(), 8);
    assert_eq!(size_of::<NMOBJECTNOTIFY>(), 56);
    assert_eq!(align_of::<NMOBJECTNOTIFY>(), 8);
    assert_eq!(size_of::<NMKEY>(), 32);
    assert_eq!(align_of::<NMKEY>(), 8);
    assert_eq!(size_of::<NMCHAR>(), 40);
    assert_eq!(align_of::<NMCHAR>(), 8);
    assert_eq!(size_of::<NMCUSTOMTEXT>(), 64);
    assert_eq!(align_of::<NMCUSTOMTEXT>(), 8);
    assert_eq!(size_of::<NMCUSTOMDRAW>(), 80);
    assert_eq!(align_of::<NMCUSTOMDRAW>(), 8);
    assert_eq!(size_of::<NMTTCUSTOMDRAW>(), 88);
    assert_eq!(align_of::<NMTTCUSTOMDRAW>(), 8);
    assert_eq!(size_of::<NMCUSTOMSPLITRECTINFO>(), 72);
    assert_eq!(align_of::<NMCUSTOMSPLITRECTINFO>(), 8);
    assert_eq!(size_of::<IMAGELISTDRAWPARAMS>(), 88);
    assert_eq!(align_of::<IMAGELISTDRAWPARAMS>(), 8);
    assert_eq!(size_of::<IMAGEINFO>(), 40);
    assert_eq!(align_of::<IMAGEINFO>(), 8);
    assert_eq!(size_of::<HD_TEXTFILTERA>(), 16);
    assert_eq!(align_of::<HD_TEXTFILTERA>(), 8);
    assert_eq!(size_of::<HD_TEXTFILTERW>(), 16);
    assert_eq!(align_of::<HD_TEXTFILTERW>(), 8);
    assert_eq!(size_of::<HDITEMA>(), 72);
    assert_eq!(align_of::<HDITEMA>(), 8);
    assert_eq!(size_of::<HDITEMW>(), 72);
    assert_eq!(align_of::<HDITEMW>(), 8);
    assert_eq!(size_of::<HDLAYOUT>(), 16);
    assert_eq!(align_of::<HDLAYOUT>(), 8);
    assert_eq!(size_of::<HDHITTESTINFO>(), 16);
    assert_eq!(align_of::<HDHITTESTINFO>(), 4);
    assert_eq!(size_of::<NMHEADERA>(), 40);
    assert_eq!(align_of::<NMHEADERA>(), 8);
    assert_eq!(size_of::<NMHEADERW>(), 40);
    assert_eq!(align_of::<NMHEADERW>(), 8);
    assert_eq!(size_of::<NMHDDISPINFOW>(), 56);
    assert_eq!(align_of::<NMHDDISPINFOW>(), 8);
    assert_eq!(size_of::<NMHDDISPINFOA>(), 56);
    assert_eq!(align_of::<NMHDDISPINFOA>(), 8);
    assert_eq!(size_of::<NMHDFILTERBTNCLICK>(), 48);
    assert_eq!(align_of::<NMHDFILTERBTNCLICK>(), 8);
    assert_eq!(size_of::<TBBUTTON>(), 32);
    assert_eq!(align_of::<TBBUTTON>(), 8);
    assert_eq!(size_of::<TBBUTTON>(), 32);
    assert_eq!(align_of::<TBBUTTON>(), 8);
    assert_eq!(size_of::<COLORMAP>(), 8);
    assert_eq!(align_of::<COLORMAP>(), 4);
    assert_eq!(size_of::<NMTBCUSTOMDRAW>(), 160);
    assert_eq!(align_of::<NMTBCUSTOMDRAW>(), 8);
    assert_eq!(size_of::<TBADDBITMAP>(), 16);
    assert_eq!(align_of::<TBADDBITMAP>(), 8);
    assert_eq!(size_of::<TBSAVEPARAMSA>(), 24);
    assert_eq!(align_of::<TBSAVEPARAMSA>(), 8);
    assert_eq!(size_of::<TBSAVEPARAMSW>(), 24);
    assert_eq!(align_of::<TBSAVEPARAMSW>(), 8);
    assert_eq!(size_of::<TBINSERTMARK>(), 8);
    assert_eq!(align_of::<TBINSERTMARK>(), 4);
    assert_eq!(size_of::<TBREPLACEBITMAP>(), 40);
    assert_eq!(align_of::<TBREPLACEBITMAP>(), 8);
    assert_eq!(size_of::<TBBUTTONINFOA>(), 48);
    assert_eq!(align_of::<TBBUTTONINFOA>(), 8);
    assert_eq!(size_of::<TBBUTTONINFOW>(), 48);
    assert_eq!(align_of::<TBBUTTONINFOW>(), 8);
    assert_eq!(size_of::<TBMETRICS>(), 32);
    assert_eq!(align_of::<TBMETRICS>(), 4);
    assert_eq!(size_of::<NMTBHOTITEM>(), 40);
    assert_eq!(align_of::<NMTBHOTITEM>(), 8);
    assert_eq!(size_of::<NMTBSAVE>(), 88);
    assert_eq!(align_of::<NMTBSAVE>(), 8);
    assert_eq!(size_of::<NMTBRESTORE>(), 88);
    assert_eq!(align_of::<NMTBRESTORE>(), 8);
    assert_eq!(size_of::<NMTBGETINFOTIPA>(), 48);
    assert_eq!(align_of::<NMTBGETINFOTIPA>(), 8);
    assert_eq!(size_of::<NMTBGETINFOTIPW>(), 48);
    assert_eq!(align_of::<NMTBGETINFOTIPW>(), 8);
    assert_eq!(size_of::<NMTBDISPINFOA>(), 64);
    assert_eq!(align_of::<NMTBDISPINFOA>(), 8);
    assert_eq!(size_of::<NMTBDISPINFOW>(), 64);
    assert_eq!(align_of::<NMTBDISPINFOW>(), 8);
    assert_eq!(size_of::<NMTOOLBARA>(), 96);
    assert_eq!(align_of::<NMTOOLBARA>(), 8);
    assert_eq!(size_of::<NMTOOLBARW>(), 96);
    assert_eq!(align_of::<NMTOOLBARW>(), 8);
    assert_eq!(size_of::<REBARINFO>(), 16);
    assert_eq!(align_of::<REBARINFO>(), 8);
    assert_eq!(size_of::<REBARBANDINFOA>(), 128);
    assert_eq!(align_of::<REBARBANDINFOA>(), 8);
    assert_eq!(size_of::<REBARBANDINFOW>(), 128);
    assert_eq!(align_of::<REBARBANDINFOW>(), 8);
    assert_eq!(size_of::<NMREBARCHILDSIZE>(), 64);
    assert_eq!(align_of::<NMREBARCHILDSIZE>(), 8);
    assert_eq!(size_of::<NMREBAR>(), 48);
    assert_eq!(align_of::<NMREBAR>(), 8);
    assert_eq!(size_of::<NMRBAUTOSIZE>(), 64);
    assert_eq!(align_of::<NMRBAUTOSIZE>(), 8);
    assert_eq!(size_of::<NMREBARCHEVRON>(), 64);
    assert_eq!(align_of::<NMREBARCHEVRON>(), 8);
    assert_eq!(size_of::<NMREBARSPLITTER>(), 40);
    assert_eq!(align_of::<NMREBARSPLITTER>(), 8);
    assert_eq!(size_of::<NMREBARAUTOBREAK>(), 56);
    assert_eq!(align_of::<NMREBARAUTOBREAK>(), 8);
    assert_eq!(size_of::<RBHITTESTINFO>(), 16);
    assert_eq!(align_of::<RBHITTESTINFO>(), 4);
    assert_eq!(size_of::<TTTOOLINFOA>(), 72);
    assert_eq!(align_of::<TTTOOLINFOA>(), 8);
    assert_eq!(size_of::<TTTOOLINFOW>(), 72);
    assert_eq!(align_of::<TTTOOLINFOW>(), 8);
    assert_eq!(size_of::<TTGETTITLE>(), 24);
    assert_eq!(align_of::<TTGETTITLE>(), 8);
    assert_eq!(size_of::<TTHITTESTINFOA>(), 88);
    assert_eq!(align_of::<TTHITTESTINFOA>(), 8);
    assert_eq!(size_of::<TTHITTESTINFOW>(), 88);
    assert_eq!(align_of::<TTHITTESTINFOW>(), 8);
    assert_eq!(size_of::<NMTTDISPINFOA>(), 136);
    assert_eq!(align_of::<NMTTDISPINFOA>(), 8);
    assert_eq!(size_of::<NMTTDISPINFOW>(), 216);
    assert_eq!(align_of::<NMTTDISPINFOW>(), 8);
    assert_eq!(size_of::<NMTRBTHUMBPOSCHANGING>(), 32);
    assert_eq!(align_of::<NMTRBTHUMBPOSCHANGING>(), 8);
    assert_eq!(size_of::<DRAGLISTINFO>(), 24);
    assert_eq!(align_of::<DRAGLISTINFO>(), 8);
    assert_eq!(size_of::<UDACCEL>(), 8);
    assert_eq!(align_of::<UDACCEL>(), 4);
    assert_eq!(size_of::<NMUPDOWN>(), 32);
    assert_eq!(align_of::<NMUPDOWN>(), 8);
    assert_eq!(size_of::<PBRANGE>(), 8);
    assert_eq!(align_of::<PBRANGE>(), 4);
    assert_eq!(size_of::<LITEM>(), 4280);
    assert_eq!(align_of::<LITEM>(), 4);
    assert_eq!(size_of::<LHITTESTINFO>(), 4288);
    assert_eq!(align_of::<LHITTESTINFO>(), 4);
    assert_eq!(size_of::<NMLINK>(), 4304);
    assert_eq!(align_of::<NMLINK>(), 8);
    assert_eq!(size_of::<LVITEMA>(), 88);
    assert_eq!(align_of::<LVITEMA>(), 8);
    assert_eq!(size_of::<LVITEMW>(), 88);
    assert_eq!(align_of::<LVITEMW>(), 8);
    assert_eq!(size_of::<LVFINDINFOA>(), 40);
    assert_eq!(align_of::<LVFINDINFOA>(), 8);
    assert_eq!(size_of::<LVFINDINFOW>(), 40);
    assert_eq!(align_of::<LVFINDINFOW>(), 8);
    assert_eq!(size_of::<LVHITTESTINFO>(), 24);
    assert_eq!(align_of::<LVHITTESTINFO>(), 4);
    assert_eq!(size_of::<LVCOLUMNA>(), 56);
    assert_eq!(align_of::<LVCOLUMNA>(), 8);
    assert_eq!(size_of::<LVCOLUMNW>(), 56);
    assert_eq!(align_of::<LVCOLUMNW>(), 8);
    assert_eq!(size_of::<LVBKIMAGEA>(), 40);
    assert_eq!(align_of::<LVBKIMAGEA>(), 8);
    assert_eq!(size_of::<LVBKIMAGEW>(), 40);
    assert_eq!(align_of::<LVBKIMAGEW>(), 8);
    assert_eq!(size_of::<LVGROUP>(), 152);
    assert_eq!(align_of::<LVGROUP>(), 8);
    assert_eq!(size_of::<LVGROUPMETRICS>(), 48);
    assert_eq!(align_of::<LVGROUPMETRICS>(), 4);
    assert_eq!(size_of::<LVINSERTGROUPSORTED>(), 168);
    assert_eq!(align_of::<LVINSERTGROUPSORTED>(), 8);
    assert_eq!(size_of::<LVTILEVIEWINFO>(), 40);
    assert_eq!(align_of::<LVTILEVIEWINFO>(), 4);
    assert_eq!(size_of::<LVTILEINFO>(), 32);
    assert_eq!(align_of::<LVTILEINFO>(), 8);
    assert_eq!(size_of::<LVINSERTMARK>(), 16);
    assert_eq!(align_of::<LVINSERTMARK>(), 4);
    assert_eq!(size_of::<LVSETINFOTIP>(), 24);
    assert_eq!(align_of::<LVSETINFOTIP>(), 8);
    assert_eq!(size_of::<LVFOOTERINFO>(), 24);
    assert_eq!(align_of::<LVFOOTERINFO>(), 8);
    assert_eq!(size_of::<LVFOOTERITEM>(), 32);
    assert_eq!(align_of::<LVFOOTERITEM>(), 8);
    assert_eq!(size_of::<LVITEMINDEX>(), 8);
    assert_eq!(align_of::<LVITEMINDEX>(), 4);
    assert_eq!(size_of::<NMLISTVIEW>(), 64);
    assert_eq!(align_of::<NMLISTVIEW>(), 8);
    assert_eq!(size_of::<NMITEMACTIVATE>(), 72);
    assert_eq!(align_of::<NMITEMACTIVATE>(), 8);
    assert_eq!(size_of::<NMLVCUSTOMDRAW>(), 136);
    assert_eq!(align_of::<NMLVCUSTOMDRAW>(), 8);
    assert_eq!(size_of::<NMLVCACHEHINT>(), 32);
    assert_eq!(align_of::<NMLVCACHEHINT>(), 8);
    assert_eq!(size_of::<NMLVFINDITEMA>(), 72);
    assert_eq!(align_of::<NMLVFINDITEMA>(), 8);
    assert_eq!(size_of::<NMLVFINDITEMW>(), 72);
    assert_eq!(align_of::<NMLVFINDITEMW>(), 8);
    assert_eq!(size_of::<NMLVODSTATECHANGE>(), 40);
    assert_eq!(align_of::<NMLVODSTATECHANGE>(), 8);
    assert_eq!(size_of::<NMLVDISPINFOA>(), 112);
    assert_eq!(align_of::<NMLVDISPINFOA>(), 8);
    assert_eq!(size_of::<NMLVDISPINFOW>(), 112);
    assert_eq!(align_of::<NMLVDISPINFOW>(), 8);
    assert_eq!(size_of::<NMLVKEYDOWN>(), 30);
    assert_eq!(align_of::<NMLVKEYDOWN>(), 1);
    assert_eq!(size_of::<NMLVLINK>(), 4312);
    assert_eq!(align_of::<NMLVLINK>(), 8);
    assert_eq!(size_of::<NMLVGETINFOTIPA>(), 64);
    assert_eq!(align_of::<NMLVGETINFOTIPA>(), 8);
    assert_eq!(size_of::<NMLVGETINFOTIPW>(), 64);
    assert_eq!(align_of::<NMLVGETINFOTIPW>(), 8);
    assert_eq!(size_of::<NMLVSCROLL>(), 32);
    assert_eq!(align_of::<NMLVSCROLL>(), 8);
    assert_eq!(size_of::<NMLVEMPTYMARKUP>(), 4200);
    assert_eq!(align_of::<NMLVEMPTYMARKUP>(), 8);
    assert_eq!(size_of::<NMTVSTATEIMAGECHANGING>(), 40);
    assert_eq!(align_of::<NMTVSTATEIMAGECHANGING>(), 8);
    assert_eq!(size_of::<TVITEMA>(), 56);
    assert_eq!(align_of::<TVITEMA>(), 8);
    assert_eq!(size_of::<TVITEMW>(), 56);
    assert_eq!(align_of::<TVITEMW>(), 8);
    assert_eq!(size_of::<TVITEMEXA>(), 80);
    assert_eq!(align_of::<TVITEMEXA>(), 8);
    assert_eq!(size_of::<TVITEMEXW>(), 80);
    assert_eq!(align_of::<TVITEMEXW>(), 8);
    assert_eq!(size_of::<TVINSERTSTRUCTA_u>(), 80);
    assert_eq!(align_of::<TVINSERTSTRUCTA_u>(), 8);
    assert_eq!(size_of::<TVINSERTSTRUCTA>(), 96);
    assert_eq!(align_of::<TVINSERTSTRUCTA>(), 8);
    assert_eq!(size_of::<TVINSERTSTRUCTW_u>(), 80);
    assert_eq!(align_of::<TVINSERTSTRUCTW_u>(), 8);
    assert_eq!(size_of::<TVINSERTSTRUCTW>(), 96);
    assert_eq!(align_of::<TVINSERTSTRUCTW>(), 8);
    assert_eq!(size_of::<TVHITTESTINFO>(), 24);
    assert_eq!(align_of::<TVHITTESTINFO>(), 8);
    assert_eq!(size_of::<TVGETITEMPARTRECTINFO>(), 24);
    assert_eq!(align_of::<TVGETITEMPARTRECTINFO>(), 8);
    assert_eq!(size_of::<TVSORTCB>(), 24);
    assert_eq!(align_of::<TVSORTCB>(), 8);
    assert_eq!(size_of::<NMTREEVIEWA>(), 152);
    assert_eq!(align_of::<NMTREEVIEWA>(), 8);
    assert_eq!(size_of::<NMTREEVIEWW>(), 152);
    assert_eq!(align_of::<NMTREEVIEWW>(), 8);
    assert_eq!(size_of::<NMTVDISPINFOA>(), 80);
    assert_eq!(align_of::<NMTVDISPINFOA>(), 8);
    assert_eq!(size_of::<NMTVDISPINFOW>(), 80);
    assert_eq!(align_of::<NMTVDISPINFOW>(), 8);
    assert_eq!(size_of::<NMTVDISPINFOEXA>(), 104);
    assert_eq!(align_of::<NMTVDISPINFOEXA>(), 8);
    assert_eq!(size_of::<NMTVDISPINFOEXW>(), 104);
    assert_eq!(align_of::<NMTVDISPINFOEXW>(), 8);
    assert_eq!(size_of::<NMTVKEYDOWN>(), 30);
    assert_eq!(align_of::<NMTVKEYDOWN>(), 1);
    assert_eq!(size_of::<NMTVCUSTOMDRAW>(), 96);
    assert_eq!(align_of::<NMTVCUSTOMDRAW>(), 8);
    assert_eq!(size_of::<NMTVGETINFOTIPA>(), 56);
    assert_eq!(align_of::<NMTVGETINFOTIPA>(), 8);
    assert_eq!(size_of::<NMTVGETINFOTIPW>(), 56);
    assert_eq!(align_of::<NMTVGETINFOTIPW>(), 8);
    assert_eq!(size_of::<NMTVITEMCHANGE>(), 56);
    assert_eq!(align_of::<NMTVITEMCHANGE>(), 8);
    assert_eq!(size_of::<NMTVASYNCDRAW>(), 64);
    assert_eq!(align_of::<NMTVASYNCDRAW>(), 8);
    assert_eq!(size_of::<COMBOBOXEXITEMA>(), 56);
    assert_eq!(align_of::<COMBOBOXEXITEMA>(), 8);
    assert_eq!(size_of::<COMBOBOXEXITEMW>(), 56);
    assert_eq!(align_of::<COMBOBOXEXITEMW>(), 8);
    assert_eq!(size_of::<NMCOMBOBOXEXA>(), 80);
    assert_eq!(align_of::<NMCOMBOBOXEXA>(), 8);
    assert_eq!(size_of::<NMCOMBOBOXEXW>(), 80);
    assert_eq!(align_of::<NMCOMBOBOXEXW>(), 8);
    assert_eq!(size_of::<NMCBEDRAGBEGINW>(), 552);
    assert_eq!(align_of::<NMCBEDRAGBEGINW>(), 8);
    assert_eq!(size_of::<NMCBEDRAGBEGINA>(), 288);
    assert_eq!(align_of::<NMCBEDRAGBEGINA>(), 8);
    assert_eq!(size_of::<NMCBEENDEDITW>(), 560);
    assert_eq!(align_of::<NMCBEENDEDITW>(), 8);
    assert_eq!(size_of::<NMCBEENDEDITA>(), 296);
    assert_eq!(align_of::<NMCBEENDEDITA>(), 8);
    assert_eq!(size_of::<TCITEMHEADERA>(), 32);
    assert_eq!(align_of::<TCITEMHEADERA>(), 8);
    assert_eq!(size_of::<TCITEMHEADERW>(), 32);
    assert_eq!(align_of::<TCITEMHEADERW>(), 8);
    assert_eq!(size_of::<TCITEMA>(), 40);
    assert_eq!(align_of::<TCITEMA>(), 8);
    assert_eq!(size_of::<TCITEMW>(), 40);
    assert_eq!(align_of::<TCITEMW>(), 8);
    assert_eq!(size_of::<TCHITTESTINFO>(), 12);
    assert_eq!(align_of::<TCHITTESTINFO>(), 4);
    assert_eq!(size_of::<NMTCKEYDOWN>(), 30);
    assert_eq!(align_of::<NMTCKEYDOWN>(), 1);
    assert_eq!(size_of::<MCHITTESTINFO>(), 60);
    assert_eq!(align_of::<MCHITTESTINFO>(), 4);
    assert_eq!(size_of::<MCGRIDINFO>(), 96);
    assert_eq!(align_of::<MCGRIDINFO>(), 8);
    assert_eq!(size_of::<NMSELCHANGE>(), 56);
    assert_eq!(align_of::<NMSELCHANGE>(), 8);
    assert_eq!(size_of::<NMDAYSTATE>(), 56);
    assert_eq!(align_of::<NMDAYSTATE>(), 8);
    assert_eq!(size_of::<NMVIEWCHANGE>(), 32);
    assert_eq!(align_of::<NMVIEWCHANGE>(), 8);
    assert_eq!(size_of::<DATETIMEPICKERINFO>(), 72);
    assert_eq!(align_of::<DATETIMEPICKERINFO>(), 8);
    assert_eq!(size_of::<NMDATETIMECHANGE>(), 48);
    assert_eq!(align_of::<NMDATETIMECHANGE>(), 8);
    assert_eq!(size_of::<NMDATETIMESTRINGA>(), 56);
    assert_eq!(align_of::<NMDATETIMESTRINGA>(), 8);
    assert_eq!(size_of::<NMDATETIMESTRINGW>(), 56);
    assert_eq!(align_of::<NMDATETIMESTRINGW>(), 8);
    assert_eq!(size_of::<NMDATETIMEWMKEYDOWNA>(), 56);
    assert_eq!(align_of::<NMDATETIMEWMKEYDOWNA>(), 8);
    assert_eq!(size_of::<NMDATETIMEWMKEYDOWNW>(), 56);
    assert_eq!(align_of::<NMDATETIMEWMKEYDOWNW>(), 8);
    assert_eq!(size_of::<NMDATETIMEFORMATA>(), 120);
    assert_eq!(align_of::<NMDATETIMEFORMATA>(), 8);
    assert_eq!(size_of::<NMDATETIMEFORMATW>(), 184);
    assert_eq!(align_of::<NMDATETIMEFORMATW>(), 8);
    assert_eq!(size_of::<NMDATETIMEFORMATQUERYA>(), 40);
    assert_eq!(align_of::<NMDATETIMEFORMATQUERYA>(), 8);
    assert_eq!(size_of::<NMDATETIMEFORMATQUERYW>(), 40);
    assert_eq!(align_of::<NMDATETIMEFORMATQUERYW>(), 8);
    assert_eq!(size_of::<NMIPADDRESS>(), 32);
    assert_eq!(align_of::<NMIPADDRESS>(), 8);
    assert_eq!(size_of::<NMPGSCROLL>(), 58);
    assert_eq!(align_of::<NMPGSCROLL>(), 1);
    assert_eq!(size_of::<NMPGCALCSIZE>(), 40);
    assert_eq!(align_of::<NMPGCALCSIZE>(), 8);
    assert_eq!(size_of::<NMPGHOTITEM>(), 40);
    assert_eq!(align_of::<NMPGHOTITEM>(), 8);
    assert_eq!(size_of::<BUTTON_IMAGELIST>(), 32);
    assert_eq!(align_of::<BUTTON_IMAGELIST>(), 8);
    assert_eq!(size_of::<NMBCHOTITEM>(), 32);
    assert_eq!(align_of::<NMBCHOTITEM>(), 8);
    assert_eq!(size_of::<BUTTON_SPLITINFO>(), 32);
    assert_eq!(align_of::<BUTTON_SPLITINFO>(), 8);
    assert_eq!(size_of::<NMBCDROPDOWN>(), 40);
    assert_eq!(align_of::<NMBCDROPDOWN>(), 8);
    assert_eq!(size_of::<EDITBALLOONTIP>(), 32);
    assert_eq!(align_of::<EDITBALLOONTIP>(), 8);
    assert_eq!(size_of::<TASKDIALOG_BUTTON>(), 12);
    assert_eq!(align_of::<TASKDIALOG_BUTTON>(), 1);
    assert_eq!(size_of::<TASKDIALOGCONFIG_u1>(), 8);
    assert_eq!(align_of::<TASKDIALOGCONFIG_u1>(), 1);
    assert_eq!(size_of::<TASKDIALOGCONFIG_u2>(), 8);
    assert_eq!(align_of::<TASKDIALOGCONFIG_u2>(), 1);
    assert_eq!(size_of::<TASKDIALOGCONFIG>(), 160);
    assert_eq!(align_of::<TASKDIALOGCONFIG>(), 1);
}
#[cfg(feature = "commdlg")]
#[test]
fn um_commdlg() {
    use winapi::um::commdlg::*;
    assert_eq!(size_of::<OPENFILENAME_NT4A>(), 136);
    assert_eq!(align_of::<OPENFILENAME_NT4A>(), 8);
    assert_eq!(size_of::<OPENFILENAME_NT4W>(), 136);
    assert_eq!(align_of::<OPENFILENAME_NT4W>(), 8);
    assert_eq!(size_of::<OPENFILENAMEA>(), 152);
    assert_eq!(align_of::<OPENFILENAMEA>(), 8);
    assert_eq!(size_of::<OPENFILENAMEW>(), 152);
    assert_eq!(align_of::<OPENFILENAMEW>(), 8);
    assert_eq!(size_of::<OFNOTIFYA>(), 40);
    assert_eq!(align_of::<OFNOTIFYA>(), 8);
    assert_eq!(size_of::<OFNOTIFYW>(), 40);
    assert_eq!(align_of::<OFNOTIFYW>(), 8);
    assert_eq!(size_of::<OFNOTIFYEXA>(), 48);
    assert_eq!(align_of::<OFNOTIFYEXA>(), 8);
    assert_eq!(size_of::<OFNOTIFYEXW>(), 48);
    assert_eq!(align_of::<OFNOTIFYEXW>(), 8);
    assert_eq!(size_of::<CHOOSECOLORA>(), 72);
    assert_eq!(align_of::<CHOOSECOLORA>(), 8);
    assert_eq!(size_of::<CHOOSECOLORW>(), 72);
    assert_eq!(align_of::<CHOOSECOLORW>(), 8);
    assert_eq!(size_of::<FINDREPLACEA>(), 80);
    assert_eq!(align_of::<FINDREPLACEA>(), 8);
    assert_eq!(size_of::<FINDREPLACEW>(), 80);
    assert_eq!(align_of::<FINDREPLACEW>(), 8);
    assert_eq!(size_of::<CHOOSEFONTA>(), 104);
    assert_eq!(align_of::<CHOOSEFONTA>(), 8);
    assert_eq!(size_of::<CHOOSEFONTW>(), 104);
    assert_eq!(align_of::<CHOOSEFONTW>(), 8);
    assert_eq!(size_of::<PRINTDLGA>(), 120);
    assert_eq!(align_of::<PRINTDLGA>(), 8);
    assert_eq!(size_of::<PRINTDLGW>(), 120);
    assert_eq!(align_of::<PRINTDLGW>(), 8);
    assert_eq!(size_of::<PRINTPAGERANGE>(), 8);
    assert_eq!(align_of::<PRINTPAGERANGE>(), 4);
    assert_eq!(size_of::<PRINTDLGEXA>(), 136);
    assert_eq!(align_of::<PRINTDLGEXA>(), 8);
    assert_eq!(size_of::<PRINTDLGEXW>(), 136);
    assert_eq!(align_of::<PRINTDLGEXW>(), 8);
    assert_eq!(size_of::<DEVNAMES>(), 8);
    assert_eq!(align_of::<DEVNAMES>(), 2);
    assert_eq!(size_of::<PAGESETUPDLGA>(), 128);
    assert_eq!(align_of::<PAGESETUPDLGA>(), 8);
    assert_eq!(size_of::<PAGESETUPDLGW>(), 128);
    assert_eq!(align_of::<PAGESETUPDLGW>(), 8);
}
#[cfg(feature = "commoncontrols")]
#[test]
fn um_commoncontrols() {
    use winapi::um::commoncontrols::*;
    assert_eq!(size_of::<IMAGELISTSTATS>(), 16);
    assert_eq!(align_of::<IMAGELISTSTATS>(), 4);
}
#[cfg(feature = "d2d1")]
#[test]
fn um_d2d1() {
    use winapi::um::d2d1::*;
    assert_eq!(size_of::<D2D1_BITMAP_PROPERTIES>(), 16);
    assert_eq!(align_of::<D2D1_BITMAP_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_GRADIENT_STOP>(), 20);
    assert_eq!(align_of::<D2D1_GRADIENT_STOP>(), 4);
    assert_eq!(size_of::<D2D1_BRUSH_PROPERTIES>(), 28);
    assert_eq!(align_of::<D2D1_BRUSH_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_BITMAP_BRUSH_PROPERTIES>(), 12);
    assert_eq!(align_of::<D2D1_BITMAP_BRUSH_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES>(), 16);
    assert_eq!(align_of::<D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES>(), 24);
    assert_eq!(align_of::<D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_BEZIER_SEGMENT>(), 24);
    assert_eq!(align_of::<D2D1_BEZIER_SEGMENT>(), 4);
    assert_eq!(size_of::<D2D1_TRIANGLE>(), 24);
    assert_eq!(align_of::<D2D1_TRIANGLE>(), 4);
    assert_eq!(size_of::<D2D1_ARC_SEGMENT>(), 28);
    assert_eq!(align_of::<D2D1_ARC_SEGMENT>(), 4);
    assert_eq!(size_of::<D2D1_QUADRATIC_BEZIER_SEGMENT>(), 16);
    assert_eq!(align_of::<D2D1_QUADRATIC_BEZIER_SEGMENT>(), 4);
    assert_eq!(size_of::<D2D1_ELLIPSE>(), 16);
    assert_eq!(align_of::<D2D1_ELLIPSE>(), 4);
    assert_eq!(size_of::<D2D1_ROUNDED_RECT>(), 24);
    assert_eq!(align_of::<D2D1_ROUNDED_RECT>(), 4);
    assert_eq!(size_of::<D2D1_STROKE_STYLE_PROPERTIES>(), 28);
    assert_eq!(align_of::<D2D1_STROKE_STYLE_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_LAYER_PARAMETERS>(), 72);
    assert_eq!(align_of::<D2D1_LAYER_PARAMETERS>(), 8);
    assert_eq!(size_of::<D2D1_RENDER_TARGET_PROPERTIES>(), 28);
    assert_eq!(align_of::<D2D1_RENDER_TARGET_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_HWND_RENDER_TARGET_PROPERTIES>(), 24);
    assert_eq!(align_of::<D2D1_HWND_RENDER_TARGET_PROPERTIES>(), 8);
    assert_eq!(size_of::<D2D1_DRAWING_STATE_DESCRIPTION>(), 48);
    assert_eq!(align_of::<D2D1_DRAWING_STATE_DESCRIPTION>(), 8);
    assert_eq!(size_of::<D2D1_FACTORY_OPTIONS>(), 4);
    assert_eq!(align_of::<D2D1_FACTORY_OPTIONS>(), 4);
}
#[cfg(feature = "d2d1effectauthor")]
#[test]
fn um_d2d1effectauthor() {
    use winapi::um::d2d1effectauthor::*;
    assert_eq!(size_of::<D2D1_PROPERTY_BINDING>(), 24);
    assert_eq!(align_of::<D2D1_PROPERTY_BINDING>(), 8);
    assert_eq!(size_of::<D2D1_RESOURCE_TEXTURE_PROPERTIES>(), 32);
    assert_eq!(align_of::<D2D1_RESOURCE_TEXTURE_PROPERTIES>(), 8);
    assert_eq!(size_of::<D2D1_INPUT_ELEMENT_DESC>(), 24);
    assert_eq!(align_of::<D2D1_INPUT_ELEMENT_DESC>(), 8);
    assert_eq!(size_of::<D2D1_VERTEX_BUFFER_PROPERTIES>(), 24);
    assert_eq!(align_of::<D2D1_VERTEX_BUFFER_PROPERTIES>(), 8);
    assert_eq!(size_of::<D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES>(), 32);
    assert_eq!(align_of::<D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES>(), 8);
    assert_eq!(size_of::<D2D1_VERTEX_RANGE>(), 8);
    assert_eq!(align_of::<D2D1_VERTEX_RANGE>(), 4);
    assert_eq!(size_of::<D2D1_BLEND_DESCRIPTION>(), 40);
    assert_eq!(align_of::<D2D1_BLEND_DESCRIPTION>(), 4);
    assert_eq!(size_of::<D2D1_INPUT_DESCRIPTION>(), 8);
    assert_eq!(align_of::<D2D1_INPUT_DESCRIPTION>(), 4);
    assert_eq!(size_of::<D2D1_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(align_of::<D2D1_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(size_of::<D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(align_of::<D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
}
#[cfg(feature = "d2d1svg")]
#[test]
fn um_d2d1svg() {
    use winapi::um::d2d1svg::*;
    assert_eq!(size_of::<D2D1_SVG_LENGTH>(), 8);
    assert_eq!(align_of::<D2D1_SVG_LENGTH>(), 4);
    assert_eq!(size_of::<D2D1_SVG_PRESERVE_ASPECT_RATIO>(), 12);
    assert_eq!(align_of::<D2D1_SVG_PRESERVE_ASPECT_RATIO>(), 4);
    assert_eq!(size_of::<D2D1_SVG_VIEWBOX>(), 16);
    assert_eq!(align_of::<D2D1_SVG_VIEWBOX>(), 4);
}
#[cfg(feature = "d2d1_1")]
#[test]
fn um_d2d1_1() {
    use winapi::um::d2d1_1::*;
    assert_eq!(size_of::<D2D1_BITMAP_PROPERTIES1>(), 32);
    assert_eq!(align_of::<D2D1_BITMAP_PROPERTIES1>(), 8);
    assert_eq!(size_of::<D2D1_MAPPED_RECT>(), 16);
    assert_eq!(align_of::<D2D1_MAPPED_RECT>(), 8);
    assert_eq!(size_of::<D2D1_RENDERING_CONTROLS>(), 12);
    assert_eq!(align_of::<D2D1_RENDERING_CONTROLS>(), 4);
    assert_eq!(size_of::<D2D1_EFFECT_INPUT_DESCRIPTION>(), 32);
    assert_eq!(align_of::<D2D1_EFFECT_INPUT_DESCRIPTION>(), 8);
    assert_eq!(size_of::<D2D1_POINT_DESCRIPTION>(), 28);
    assert_eq!(align_of::<D2D1_POINT_DESCRIPTION>(), 4);
    assert_eq!(size_of::<D2D1_IMAGE_BRUSH_PROPERTIES>(), 28);
    assert_eq!(align_of::<D2D1_IMAGE_BRUSH_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_BITMAP_BRUSH_PROPERTIES1>(), 12);
    assert_eq!(align_of::<D2D1_BITMAP_BRUSH_PROPERTIES1>(), 4);
    assert_eq!(size_of::<D2D1_STROKE_STYLE_PROPERTIES1>(), 32);
    assert_eq!(align_of::<D2D1_STROKE_STYLE_PROPERTIES1>(), 4);
    assert_eq!(size_of::<D2D1_LAYER_PARAMETERS1>(), 72);
    assert_eq!(align_of::<D2D1_LAYER_PARAMETERS1>(), 8);
    assert_eq!(size_of::<D2D1_DRAWING_STATE_DESCRIPTION1>(), 56);
    assert_eq!(align_of::<D2D1_DRAWING_STATE_DESCRIPTION1>(), 8);
    assert_eq!(size_of::<D2D1_PRINT_CONTROL_PROPERTIES>(), 12);
    assert_eq!(align_of::<D2D1_PRINT_CONTROL_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_CREATION_PROPERTIES>(), 12);
    assert_eq!(align_of::<D2D1_CREATION_PROPERTIES>(), 4);
}
#[cfg(feature = "d2d1_3")]
#[test]
fn um_d2d1_3() {
    use winapi::um::d2d1_3::*;
    assert_eq!(size_of::<D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES>(), 20);
    assert_eq!(align_of::<D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_INK_POINT>(), 12);
    assert_eq!(align_of::<D2D1_INK_POINT>(), 4);
    assert_eq!(size_of::<D2D1_INK_BEZIER_SEGMENT>(), 36);
    assert_eq!(align_of::<D2D1_INK_BEZIER_SEGMENT>(), 4);
    assert_eq!(size_of::<D2D1_INK_STYLE_PROPERTIES>(), 28);
    assert_eq!(align_of::<D2D1_INK_STYLE_PROPERTIES>(), 4);
    assert_eq!(size_of::<D2D1_GRADIENT_MESH_PATCH>(), 208);
    assert_eq!(align_of::<D2D1_GRADIENT_MESH_PATCH>(), 4);
    assert_eq!(size_of::<D2D1_SIMPLE_COLOR_PROFILE>(), 36);
    assert_eq!(align_of::<D2D1_SIMPLE_COLOR_PROFILE>(), 4);
}
#[cfg(feature = "d3d10shader")]
#[test]
fn um_d3d10shader() {
    use winapi::um::d3d10shader::*;
    assert_eq!(size_of::<D3D10_SHADER_DESC>(), 120);
    assert_eq!(align_of::<D3D10_SHADER_DESC>(), 8);
    assert_eq!(size_of::<D3D10_SHADER_BUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3D10_SHADER_BUFFER_DESC>(), 8);
    assert_eq!(size_of::<D3D10_SHADER_VARIABLE_DESC>(), 32);
    assert_eq!(align_of::<D3D10_SHADER_VARIABLE_DESC>(), 8);
    assert_eq!(size_of::<D3D10_SHADER_TYPE_DESC>(), 28);
    assert_eq!(align_of::<D3D10_SHADER_TYPE_DESC>(), 4);
    assert_eq!(size_of::<D3D10_SHADER_INPUT_BIND_DESC>(), 40);
    assert_eq!(align_of::<D3D10_SHADER_INPUT_BIND_DESC>(), 8);
    assert_eq!(size_of::<D3D10_SIGNATURE_PARAMETER_DESC>(), 32);
    assert_eq!(align_of::<D3D10_SIGNATURE_PARAMETER_DESC>(), 8);
}
#[cfg(feature = "d3d11")]
#[test]
fn um_d3d11() {
    use winapi::um::d3d11::*;
    assert_eq!(size_of::<D3D11_INPUT_ELEMENT_DESC>(), 32);
    assert_eq!(align_of::<D3D11_INPUT_ELEMENT_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SO_DECLARATION_ENTRY>(), 24);
    assert_eq!(align_of::<D3D11_SO_DECLARATION_ENTRY>(), 8);
    assert_eq!(size_of::<D3D11_VIEWPORT>(), 24);
    assert_eq!(align_of::<D3D11_VIEWPORT>(), 4);
    assert_eq!(size_of::<D3D11_DRAW_INSTANCED_INDIRECT_ARGS>(), 16);
    assert_eq!(align_of::<D3D11_DRAW_INSTANCED_INDIRECT_ARGS>(), 4);
    assert_eq!(size_of::<D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS>(), 20);
    assert_eq!(align_of::<D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS>(), 4);
    assert_eq!(size_of::<D3D11_BOX>(), 24);
    assert_eq!(align_of::<D3D11_BOX>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCILOP_DESC>(), 16);
    assert_eq!(align_of::<D3D11_DEPTH_STENCILOP_DESC>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCIL_DESC>(), 52);
    assert_eq!(align_of::<D3D11_DEPTH_STENCIL_DESC>(), 4);
    assert_eq!(size_of::<D3D11_RENDER_TARGET_BLEND_DESC>(), 32);
    assert_eq!(align_of::<D3D11_RENDER_TARGET_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BLEND_DESC>(), 264);
    assert_eq!(align_of::<D3D11_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D11_RASTERIZER_DESC>(), 40);
    assert_eq!(align_of::<D3D11_RASTERIZER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SUBRESOURCE_DATA>(), 16);
    assert_eq!(align_of::<D3D11_SUBRESOURCE_DATA>(), 8);
    assert_eq!(size_of::<D3D11_MAPPED_SUBRESOURCE>(), 16);
    assert_eq!(align_of::<D3D11_MAPPED_SUBRESOURCE>(), 8);
    assert_eq!(size_of::<D3D11_BUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3D11_BUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE1D_DESC>(), 32);
    assert_eq!(align_of::<D3D11_TEXTURE1D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE2D_DESC>(), 44);
    assert_eq!(align_of::<D3D11_TEXTURE2D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE3D_DESC>(), 36);
    assert_eq!(align_of::<D3D11_TEXTURE3D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_SRV>(), 8);
    assert_eq!(align_of::<D3D11_BUFFER_SRV>(), 4);
    assert_eq!(size_of::<D3D11_BUFFEREX_SRV>(), 12);
    assert_eq!(align_of::<D3D11_BUFFEREX_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX1D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX3D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEXCUBE_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEXCUBE_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEXCUBE_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEXCUBE_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_SRV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_RTV>(), 8);
    assert_eq!(align_of::<D3D11_BUFFER_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_RTV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX3D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_RENDER_TARGET_VIEW_DESC>(), 20);
    assert_eq!(align_of::<D3D11_RENDER_TARGET_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_DSV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_UAV>(), 12);
    assert_eq!(align_of::<D3D11_BUFFER_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_UAV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_UAV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX3D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>(), 20);
    assert_eq!(align_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SAMPLER_DESC>(), 52);
    assert_eq!(align_of::<D3D11_SAMPLER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_QUERY_DESC>(), 8);
    assert_eq!(align_of::<D3D11_QUERY_DESC>(), 4);
    assert_eq!(size_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>(), 16);
    assert_eq!(align_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>(), 8);
    assert_eq!(size_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>(), 88);
    assert_eq!(align_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D11_QUERY_DATA_SO_STATISTICS>(), 16);
    assert_eq!(align_of::<D3D11_QUERY_DATA_SO_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D11_COUNTER_DESC>(), 8);
    assert_eq!(align_of::<D3D11_COUNTER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_COUNTER_INFO>(), 12);
    assert_eq!(align_of::<D3D11_COUNTER_INFO>(), 4);
    assert_eq!(size_of::<D3D11_CLASS_INSTANCE_DESC>(), 32);
    assert_eq!(align_of::<D3D11_CLASS_INSTANCE_DESC>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_THREADING>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_THREADING>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT2>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT2>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS>(), 56);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_ARCHITECTURE_INFO>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_ARCHITECTURE_INFO>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT>(), 4);
    assert_eq!(
        size_of::<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT>(),
        8
    );
    assert_eq!(
        align_of::<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT>(),
        4
    );
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS1>(), 16);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS1>(), 4);
    assert_eq!(
        size_of::<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT>(),
        4
    );
    assert_eq!(
        align_of::<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT>(),
        4
    );
    assert_eq!(size_of::<D3D11_FEATURE_DATA_MARKER_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_MARKER_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS1>(), 16);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS1>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>(), 32);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS3>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS3>(), 4);
    assert_eq!(
        size_of::<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(),
        8
    );
    assert_eq!(
        align_of::<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(),
        4
    );
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_DESC>(), 28);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_CONFIG>(), 100);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_CONFIG>(), 4);
    assert_eq!(size_of::<D3D11_AES_CTR_IV>(), 16);
    assert_eq!(align_of::<D3D11_AES_CTR_IV>(), 8);
    assert_eq!(size_of::<D3D11_ENCRYPTED_BLOCK_INFO>(), 12);
    assert_eq!(align_of::<D3D11_ENCRYPTED_BLOCK_INFO>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_BUFFER_DESC>(), 72);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_BUFFER_DESC>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_EXTENSION>(), 48);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_EXTENSION>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CAPS>(), 36);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CAPS>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>(), 20);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>(), 24);
    assert_eq!(align_of::<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>(), 20);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>(), 40);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR_RGBA>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR_RGBA>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR_YCbCrA>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR_YCbCrA>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_COLOR_SPACE>(), 4);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_COLOR_SPACE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_STREAM>(), 72);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_STREAM>(), 8);
    assert_eq!(size_of::<D3D11_OMAC>(), 16);
    assert_eq!(align_of::<D3D11_OMAC>(), 1);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_INPUT>(), 32);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_PROTECTION_FLAGS>(), 4);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_PROTECTION_FLAGS>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT>(), 8);
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT>(),
        40
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT>(),
        40
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT>(),
        48
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT>(),
        8
    );
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT>(), 80);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT>(), 8);
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_ACESSIBILITY_OUTPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_ACESSIBILITY_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT>(),
        40
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT>(),
        8
    );
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_INPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>(), 8);
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT>(),
        56
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT>(),
        72
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT>(),
        8
    );
    assert_eq!(
        size_of::<D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT>(),
        64
    );
    assert_eq!(
        align_of::<D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT>(),
        8
    );
    assert_eq!(size_of::<D3D11_TEX2D_VDOV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_VDOV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VPIV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2D_VPIV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VPOV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_VPOV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_VPOV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_VPOV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC>(), 4);
}
#[cfg(feature = "d3d11on12")]
#[test]
fn um_d3d11on12() {
    use winapi::um::d3d11on12::*;
    assert_eq!(size_of::<D3D11_RESOURCE_FLAGS>(), 16);
    assert_eq!(align_of::<D3D11_RESOURCE_FLAGS>(), 4);
}
#[cfg(feature = "d3d11sdklayers")]
#[test]
fn um_d3d11sdklayers() {
    use winapi::um::d3d11sdklayers::*;
    assert_eq!(size_of::<D3D11_MESSAGE>(), 32);
    assert_eq!(align_of::<D3D11_MESSAGE>(), 8);
    assert_eq!(size_of::<D3D11_INFO_QUEUE_FILTER_DESC>(), 48);
    assert_eq!(align_of::<D3D11_INFO_QUEUE_FILTER_DESC>(), 8);
    assert_eq!(size_of::<D3D11_INFO_QUEUE_FILTER>(), 96);
    assert_eq!(align_of::<D3D11_INFO_QUEUE_FILTER>(), 8);
}
#[cfg(feature = "d3d11shader")]
#[test]
fn um_d3d11shader() {
    use winapi::um::d3d11shader::*;
    assert_eq!(size_of::<D3D11_SIGNATURE_PARAMETER_DESC>(), 40);
    assert_eq!(align_of::<D3D11_SIGNATURE_PARAMETER_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SHADER_BUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3D11_SHADER_BUFFER_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SHADER_VARIABLE_DESC>(), 48);
    assert_eq!(align_of::<D3D11_SHADER_VARIABLE_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SHADER_TYPE_DESC>(), 40);
    assert_eq!(align_of::<D3D11_SHADER_TYPE_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SHADER_DESC>(), 160);
    assert_eq!(align_of::<D3D11_SHADER_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SHADER_INPUT_BIND_DESC>(), 40);
    assert_eq!(align_of::<D3D11_SHADER_INPUT_BIND_DESC>(), 8);
    assert_eq!(size_of::<D3D11_LIBRARY_DESC>(), 16);
    assert_eq!(align_of::<D3D11_LIBRARY_DESC>(), 8);
    assert_eq!(size_of::<D3D11_FUNCTION_DESC>(), 152);
    assert_eq!(align_of::<D3D11_FUNCTION_DESC>(), 8);
    assert_eq!(size_of::<D3D11_PARAMETER_DESC>(), 56);
    assert_eq!(align_of::<D3D11_PARAMETER_DESC>(), 8);
}
#[cfg(feature = "d3d12")]
#[test]
fn um_d3d12() {
    use winapi::um::d3d12::*;
    assert_eq!(size_of::<D3D12_COMMAND_QUEUE_DESC>(), 16);
    assert_eq!(align_of::<D3D12_COMMAND_QUEUE_DESC>(), 4);
    assert_eq!(size_of::<D3D12_INPUT_ELEMENT_DESC>(), 32);
    assert_eq!(align_of::<D3D12_INPUT_ELEMENT_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SO_DECLARATION_ENTRY>(), 24);
    assert_eq!(align_of::<D3D12_SO_DECLARATION_ENTRY>(), 8);
    assert_eq!(size_of::<D3D12_VIEWPORT>(), 24);
    assert_eq!(align_of::<D3D12_VIEWPORT>(), 4);
    assert_eq!(size_of::<D3D12_BOX>(), 24);
    assert_eq!(align_of::<D3D12_BOX>(), 4);
    assert_eq!(size_of::<D3D12_DEPTH_STENCILOP_DESC>(), 16);
    assert_eq!(align_of::<D3D12_DEPTH_STENCILOP_DESC>(), 4);
    assert_eq!(size_of::<D3D12_DEPTH_STENCIL_DESC>(), 52);
    assert_eq!(align_of::<D3D12_DEPTH_STENCIL_DESC>(), 4);
    assert_eq!(size_of::<D3D12_DEPTH_STENCIL_DESC1>(), 56);
    assert_eq!(align_of::<D3D12_DEPTH_STENCIL_DESC1>(), 4);
    assert_eq!(size_of::<D3D12_RENDER_TARGET_BLEND_DESC>(), 40);
    assert_eq!(align_of::<D3D12_RENDER_TARGET_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D12_BLEND_DESC>(), 328);
    assert_eq!(align_of::<D3D12_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D12_RASTERIZER_DESC>(), 44);
    assert_eq!(align_of::<D3D12_RASTERIZER_DESC>(), 4);
    assert_eq!(size_of::<D3D12_SHADER_BYTECODE>(), 16);
    assert_eq!(align_of::<D3D12_SHADER_BYTECODE>(), 8);
    assert_eq!(size_of::<D3D12_STREAM_OUTPUT_DESC>(), 32);
    assert_eq!(align_of::<D3D12_STREAM_OUTPUT_DESC>(), 8);
    assert_eq!(size_of::<D3D12_INPUT_LAYOUT_DESC>(), 16);
    assert_eq!(align_of::<D3D12_INPUT_LAYOUT_DESC>(), 8);
    assert_eq!(size_of::<D3D12_CACHED_PIPELINE_STATE>(), 16);
    assert_eq!(align_of::<D3D12_CACHED_PIPELINE_STATE>(), 8);
    assert_eq!(size_of::<D3D12_GRAPHICS_PIPELINE_STATE_DESC>(), 656);
    assert_eq!(align_of::<D3D12_GRAPHICS_PIPELINE_STATE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_COMPUTE_PIPELINE_STATE_DESC>(), 56);
    assert_eq!(align_of::<D3D12_COMPUTE_PIPELINE_STATE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_RT_FORMAT_ARRAY>(), 36);
    assert_eq!(align_of::<D3D12_RT_FORMAT_ARRAY>(), 4);
    assert_eq!(size_of::<D3D12_PIPELINE_STATE_STREAM_DESC>(), 16);
    assert_eq!(align_of::<D3D12_PIPELINE_STATE_STREAM_DESC>(), 8);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS>(), 60);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS1>(), 24);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS1>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS2>(), 8);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS2>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_ROOT_SIGNATURE>(), 4);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_ROOT_SIGNATURE>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_ARCHITECTURE>(), 16);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_ARCHITECTURE>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_ARCHITECTURE1>(), 20);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_ARCHITECTURE1>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_FEATURE_LEVELS>(), 24);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_FEATURE_LEVELS>(), 8);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_SHADER_MODEL>(), 4);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_SHADER_MODEL>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_FORMAT_SUPPORT>(), 12);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_FORMAT_SUPPORT>(), 4);
    assert_eq!(
        size_of::<D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS>(),
        16
    );
    assert_eq!(
        align_of::<D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS>(),
        4
    );
    assert_eq!(size_of::<D3D12_FEATURE_DATA_FORMAT_INFO>(), 8);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_FORMAT_INFO>(), 4);
    assert_eq!(
        size_of::<D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(),
        8
    );
    assert_eq!(
        align_of::<D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(),
        4
    );
    assert_eq!(size_of::<D3D12_FEATURE_DATA_SHADER_CACHE>(), 4);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_SHADER_CACHE>(), 4);
    assert_eq!(size_of::<D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY>(), 12);
    assert_eq!(align_of::<D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY>(), 4);
    assert_eq!(size_of::<D3D12_RESOURCE_ALLOCATION_INFO>(), 16);
    assert_eq!(align_of::<D3D12_RESOURCE_ALLOCATION_INFO>(), 8);
    assert_eq!(size_of::<D3D12_HEAP_PROPERTIES>(), 20);
    assert_eq!(align_of::<D3D12_HEAP_PROPERTIES>(), 4);
    assert_eq!(size_of::<D3D12_HEAP_DESC>(), 48);
    assert_eq!(align_of::<D3D12_HEAP_DESC>(), 8);
    assert_eq!(size_of::<D3D12_RESOURCE_DESC>(), 56);
    assert_eq!(align_of::<D3D12_RESOURCE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_DEPTH_STENCIL_VALUE>(), 8);
    assert_eq!(align_of::<D3D12_DEPTH_STENCIL_VALUE>(), 4);
    assert_eq!(size_of::<D3D12_CLEAR_VALUE>(), 20);
    assert_eq!(align_of::<D3D12_CLEAR_VALUE>(), 4);
    assert_eq!(size_of::<D3D12_RANGE>(), 16);
    assert_eq!(align_of::<D3D12_RANGE>(), 8);
    assert_eq!(size_of::<D3D12_RANGE_UINT64>(), 16);
    assert_eq!(align_of::<D3D12_RANGE_UINT64>(), 8);
    assert_eq!(size_of::<D3D12_SUBRESOURCE_RANGE_UINT64>(), 24);
    assert_eq!(align_of::<D3D12_SUBRESOURCE_RANGE_UINT64>(), 8);
    assert_eq!(size_of::<D3D12_SUBRESOURCE_INFO>(), 16);
    assert_eq!(align_of::<D3D12_SUBRESOURCE_INFO>(), 8);
    assert_eq!(size_of::<D3D12_TILED_RESOURCE_COORDINATE>(), 16);
    assert_eq!(align_of::<D3D12_TILED_RESOURCE_COORDINATE>(), 4);
    assert_eq!(size_of::<D3D12_TILE_REGION_SIZE>(), 16);
    assert_eq!(align_of::<D3D12_TILE_REGION_SIZE>(), 4);
    assert_eq!(size_of::<D3D12_SUBRESOURCE_TILING>(), 12);
    assert_eq!(align_of::<D3D12_SUBRESOURCE_TILING>(), 4);
    assert_eq!(size_of::<D3D12_TILE_SHAPE>(), 12);
    assert_eq!(align_of::<D3D12_TILE_SHAPE>(), 4);
    assert_eq!(size_of::<D3D12_PACKED_MIP_INFO>(), 12);
    assert_eq!(align_of::<D3D12_PACKED_MIP_INFO>(), 4);
    assert_eq!(size_of::<D3D12_RESOURCE_TRANSITION_BARRIER>(), 24);
    assert_eq!(align_of::<D3D12_RESOURCE_TRANSITION_BARRIER>(), 8);
    assert_eq!(size_of::<D3D12_RESOURCE_ALIASING_BARRIER>(), 16);
    assert_eq!(align_of::<D3D12_RESOURCE_ALIASING_BARRIER>(), 8);
    assert_eq!(size_of::<D3D12_RESOURCE_UAV_BARRIER>(), 8);
    assert_eq!(align_of::<D3D12_RESOURCE_UAV_BARRIER>(), 8);
    assert_eq!(size_of::<D3D12_RESOURCE_BARRIER>(), 32);
    assert_eq!(align_of::<D3D12_RESOURCE_BARRIER>(), 8);
    assert_eq!(size_of::<D3D12_SUBRESOURCE_FOOTPRINT>(), 20);
    assert_eq!(align_of::<D3D12_SUBRESOURCE_FOOTPRINT>(), 4);
    assert_eq!(size_of::<D3D12_PLACED_SUBRESOURCE_FOOTPRINT>(), 32);
    assert_eq!(align_of::<D3D12_PLACED_SUBRESOURCE_FOOTPRINT>(), 8);
    assert_eq!(size_of::<D3D12_TEXTURE_COPY_LOCATION>(), 48);
    assert_eq!(align_of::<D3D12_TEXTURE_COPY_LOCATION>(), 8);
    assert_eq!(size_of::<D3D12_SAMPLE_POSITION>(), 2);
    assert_eq!(align_of::<D3D12_SAMPLE_POSITION>(), 1);
    assert_eq!(size_of::<D3D12_BUFFER_SRV>(), 24);
    assert_eq!(align_of::<D3D12_BUFFER_SRV>(), 8);
    assert_eq!(size_of::<D3D12_TEX1D_SRV>(), 12);
    assert_eq!(align_of::<D3D12_TEX1D_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEX1D_ARRAY_SRV>(), 20);
    assert_eq!(align_of::<D3D12_TEX1D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_SRV>(), 16);
    assert_eq!(align_of::<D3D12_TEX2D_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_ARRAY_SRV>(), 24);
    assert_eq!(align_of::<D3D12_TEX2D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEX3D_SRV>(), 12);
    assert_eq!(align_of::<D3D12_TEX3D_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEXCUBE_SRV>(), 12);
    assert_eq!(align_of::<D3D12_TEXCUBE_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEXCUBE_ARRAY_SRV>(), 20);
    assert_eq!(align_of::<D3D12_TEXCUBE_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2DMS_SRV>(), 4);
    assert_eq!(align_of::<D3D12_TEX2DMS_SRV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2DMS_ARRAY_SRV>(), 8);
    assert_eq!(align_of::<D3D12_TEX2DMS_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D12_SHADER_RESOURCE_VIEW_DESC>(), 40);
    assert_eq!(align_of::<D3D12_SHADER_RESOURCE_VIEW_DESC>(), 8);
    assert_eq!(size_of::<D3D12_CONSTANT_BUFFER_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D12_CONSTANT_BUFFER_VIEW_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SAMPLER_DESC>(), 52);
    assert_eq!(align_of::<D3D12_SAMPLER_DESC>(), 4);
    assert_eq!(size_of::<D3D12_BUFFER_UAV>(), 32);
    assert_eq!(align_of::<D3D12_BUFFER_UAV>(), 8);
    assert_eq!(size_of::<D3D12_TEX1D_UAV>(), 4);
    assert_eq!(align_of::<D3D12_TEX1D_UAV>(), 4);
    assert_eq!(size_of::<D3D12_TEX1D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D12_TEX1D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_UAV>(), 8);
    assert_eq!(align_of::<D3D12_TEX2D_UAV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_ARRAY_UAV>(), 16);
    assert_eq!(align_of::<D3D12_TEX2D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D12_TEX3D_UAV>(), 12);
    assert_eq!(align_of::<D3D12_TEX3D_UAV>(), 4);
    assert_eq!(size_of::<D3D12_UNORDERED_ACCESS_VIEW_DESC>(), 40);
    assert_eq!(align_of::<D3D12_UNORDERED_ACCESS_VIEW_DESC>(), 8);
    assert_eq!(size_of::<D3D12_BUFFER_RTV>(), 16);
    assert_eq!(align_of::<D3D12_BUFFER_RTV>(), 8);
    assert_eq!(size_of::<D3D12_TEX1D_RTV>(), 4);
    assert_eq!(align_of::<D3D12_TEX1D_RTV>(), 4);
    assert_eq!(size_of::<D3D12_TEX1D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D12_TEX1D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_RTV>(), 8);
    assert_eq!(align_of::<D3D12_TEX2D_RTV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2DMS_RTV>(), 4);
    assert_eq!(align_of::<D3D12_TEX2DMS_RTV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_ARRAY_RTV>(), 16);
    assert_eq!(align_of::<D3D12_TEX2D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2DMS_ARRAY_RTV>(), 8);
    assert_eq!(align_of::<D3D12_TEX2DMS_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D12_TEX3D_RTV>(), 12);
    assert_eq!(align_of::<D3D12_TEX3D_RTV>(), 4);
    assert_eq!(size_of::<D3D12_RENDER_TARGET_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D12_RENDER_TARGET_VIEW_DESC>(), 8);
    assert_eq!(size_of::<D3D12_TEX1D_DSV>(), 4);
    assert_eq!(align_of::<D3D12_TEX1D_DSV>(), 4);
    assert_eq!(size_of::<D3D12_TEX1D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D12_TEX1D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_DSV>(), 4);
    assert_eq!(align_of::<D3D12_TEX2D_DSV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D12_TEX2D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2DMS_DSV>(), 4);
    assert_eq!(align_of::<D3D12_TEX2DMS_DSV>(), 4);
    assert_eq!(size_of::<D3D12_TEX2DMS_ARRAY_DSV>(), 8);
    assert_eq!(align_of::<D3D12_TEX2DMS_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D12_DEPTH_STENCIL_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D12_DEPTH_STENCIL_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D12_DESCRIPTOR_HEAP_DESC>(), 16);
    assert_eq!(align_of::<D3D12_DESCRIPTOR_HEAP_DESC>(), 4);
    assert_eq!(size_of::<D3D12_DESCRIPTOR_RANGE>(), 20);
    assert_eq!(align_of::<D3D12_DESCRIPTOR_RANGE>(), 4);
    assert_eq!(size_of::<D3D12_ROOT_DESCRIPTOR_TABLE>(), 16);
    assert_eq!(align_of::<D3D12_ROOT_DESCRIPTOR_TABLE>(), 8);
    assert_eq!(size_of::<D3D12_ROOT_CONSTANTS>(), 12);
    assert_eq!(align_of::<D3D12_ROOT_CONSTANTS>(), 4);
    assert_eq!(size_of::<D3D12_ROOT_DESCRIPTOR>(), 8);
    assert_eq!(align_of::<D3D12_ROOT_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<D3D12_ROOT_PARAMETER>(), 32);
    assert_eq!(align_of::<D3D12_ROOT_PARAMETER>(), 8);
    assert_eq!(size_of::<D3D12_STATIC_SAMPLER_DESC>(), 52);
    assert_eq!(align_of::<D3D12_STATIC_SAMPLER_DESC>(), 4);
    assert_eq!(size_of::<D3D12_ROOT_SIGNATURE_DESC>(), 40);
    assert_eq!(align_of::<D3D12_ROOT_SIGNATURE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_DESCRIPTOR_RANGE1>(), 24);
    assert_eq!(align_of::<D3D12_DESCRIPTOR_RANGE1>(), 4);
    assert_eq!(size_of::<D3D12_ROOT_DESCRIPTOR_TABLE1>(), 16);
    assert_eq!(align_of::<D3D12_ROOT_DESCRIPTOR_TABLE1>(), 8);
    assert_eq!(size_of::<D3D12_ROOT_DESCRIPTOR1>(), 12);
    assert_eq!(align_of::<D3D12_ROOT_DESCRIPTOR1>(), 4);
    assert_eq!(size_of::<D3D12_ROOT_PARAMETER1>(), 32);
    assert_eq!(align_of::<D3D12_ROOT_PARAMETER1>(), 8);
    assert_eq!(size_of::<D3D12_ROOT_SIGNATURE_DESC1>(), 40);
    assert_eq!(align_of::<D3D12_ROOT_SIGNATURE_DESC1>(), 8);
    assert_eq!(size_of::<D3D12_VERSIONED_ROOT_SIGNATURE_DESC>(), 48);
    assert_eq!(align_of::<D3D12_VERSIONED_ROOT_SIGNATURE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_CPU_DESCRIPTOR_HANDLE>(), 8);
    assert_eq!(align_of::<D3D12_CPU_DESCRIPTOR_HANDLE>(), 8);
    assert_eq!(size_of::<D3D12_GPU_DESCRIPTOR_HANDLE>(), 8);
    assert_eq!(align_of::<D3D12_GPU_DESCRIPTOR_HANDLE>(), 8);
    assert_eq!(size_of::<D3D12_DISCARD_REGION>(), 24);
    assert_eq!(align_of::<D3D12_DISCARD_REGION>(), 8);
    assert_eq!(size_of::<D3D12_QUERY_HEAP_DESC>(), 12);
    assert_eq!(align_of::<D3D12_QUERY_HEAP_DESC>(), 4);
    assert_eq!(size_of::<D3D12_QUERY_DATA_PIPELINE_STATISTICS>(), 88);
    assert_eq!(align_of::<D3D12_QUERY_DATA_PIPELINE_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D12_QUERY_DATA_SO_STATISTICS>(), 16);
    assert_eq!(align_of::<D3D12_QUERY_DATA_SO_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D12_STREAM_OUTPUT_BUFFER_VIEW>(), 24);
    assert_eq!(align_of::<D3D12_STREAM_OUTPUT_BUFFER_VIEW>(), 8);
    assert_eq!(size_of::<D3D12_DRAW_ARGUMENTS>(), 16);
    assert_eq!(align_of::<D3D12_DRAW_ARGUMENTS>(), 4);
    assert_eq!(size_of::<D3D12_DRAW_INDEXED_ARGUMENTS>(), 20);
    assert_eq!(align_of::<D3D12_DRAW_INDEXED_ARGUMENTS>(), 4);
    assert_eq!(size_of::<D3D12_DISPATCH_ARGUMENTS>(), 12);
    assert_eq!(align_of::<D3D12_DISPATCH_ARGUMENTS>(), 4);
    assert_eq!(size_of::<D3D12_VERTEX_BUFFER_VIEW>(), 16);
    assert_eq!(align_of::<D3D12_VERTEX_BUFFER_VIEW>(), 8);
    assert_eq!(size_of::<D3D12_INDEX_BUFFER_VIEW>(), 16);
    assert_eq!(align_of::<D3D12_INDEX_BUFFER_VIEW>(), 8);
    assert_eq!(size_of::<D3D12_INDIRECT_ARGUMENT_DESC_VertexBuffer>(), 4);
    assert_eq!(align_of::<D3D12_INDIRECT_ARGUMENT_DESC_VertexBuffer>(), 4);
    assert_eq!(size_of::<D3D12_INDIRECT_ARGUMENT_DESC_Constant>(), 12);
    assert_eq!(align_of::<D3D12_INDIRECT_ARGUMENT_DESC_Constant>(), 4);
    assert_eq!(
        size_of::<D3D12_INDIRECT_ARGUMENT_DESC_ConstantBufferView>(),
        4
    );
    assert_eq!(
        align_of::<D3D12_INDIRECT_ARGUMENT_DESC_ConstantBufferView>(),
        4
    );
    assert_eq!(
        size_of::<D3D12_INDIRECT_ARGUMENT_DESC_ShaderResourceView>(),
        4
    );
    assert_eq!(
        align_of::<D3D12_INDIRECT_ARGUMENT_DESC_ShaderResourceView>(),
        4
    );
    assert_eq!(
        size_of::<D3D12_INDIRECT_ARGUMENT_DESC_UnorderedAccessView>(),
        4
    );
    assert_eq!(
        align_of::<D3D12_INDIRECT_ARGUMENT_DESC_UnorderedAccessView>(),
        4
    );
    assert_eq!(size_of::<D3D12_INDIRECT_ARGUMENT_DESC>(), 16);
    assert_eq!(align_of::<D3D12_INDIRECT_ARGUMENT_DESC>(), 4);
    assert_eq!(size_of::<D3D12_COMMAND_SIGNATURE_DESC>(), 24);
    assert_eq!(align_of::<D3D12_COMMAND_SIGNATURE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SUBRESOURCE_DATA>(), 24);
    assert_eq!(align_of::<D3D12_SUBRESOURCE_DATA>(), 8);
    assert_eq!(size_of::<D3D12_MEMCPY_DEST>(), 24);
    assert_eq!(align_of::<D3D12_MEMCPY_DEST>(), 8);
}
#[cfg(feature = "d3d12sdklayers")]
#[test]
fn um_d3d12sdklayers() {
    use winapi::um::d3d12sdklayers::*;
    assert_eq!(
        size_of::<D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS>(),
        12
    );
    assert_eq!(
        align_of::<D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS>(),
        4
    );
    assert_eq!(
        size_of::<D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR>(),
        4
    );
    assert_eq!(
        align_of::<D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR>(),
        4
    );
    assert_eq!(
        size_of::<D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS>(),
        4
    );
    assert_eq!(
        align_of::<D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS>(),
        4
    );
    assert_eq!(size_of::<D3D12_MESSAGE>(), 32);
    assert_eq!(align_of::<D3D12_MESSAGE>(), 8);
    assert_eq!(size_of::<D3D12_INFO_QUEUE_FILTER_DESC>(), 48);
    assert_eq!(align_of::<D3D12_INFO_QUEUE_FILTER_DESC>(), 8);
    assert_eq!(size_of::<D3D12_INFO_QUEUE_FILTER>(), 96);
    assert_eq!(align_of::<D3D12_INFO_QUEUE_FILTER>(), 8);
}
#[cfg(feature = "d3d12shader")]
#[test]
fn um_d3d12shader() {
    use winapi::um::d3d12shader::*;
    assert_eq!(size_of::<D3D12_FUNCTION_DESC>(), 152);
    assert_eq!(align_of::<D3D12_FUNCTION_DESC>(), 8);
    assert_eq!(size_of::<D3D12_LIBRARY_DESC>(), 16);
    assert_eq!(align_of::<D3D12_LIBRARY_DESC>(), 8);
    assert_eq!(size_of::<D3D12_PARAMETER_DESC>(), 56);
    assert_eq!(align_of::<D3D12_PARAMETER_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SHADER_BUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3D12_SHADER_BUFFER_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SHADER_DESC>(), 160);
    assert_eq!(align_of::<D3D12_SHADER_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SHADER_INPUT_BIND_DESC>(), 48);
    assert_eq!(align_of::<D3D12_SHADER_INPUT_BIND_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SHADER_TYPE_DESC>(), 40);
    assert_eq!(align_of::<D3D12_SHADER_TYPE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SHADER_VARIABLE_DESC>(), 48);
    assert_eq!(align_of::<D3D12_SHADER_VARIABLE_DESC>(), 8);
    assert_eq!(size_of::<D3D12_SIGNATURE_PARAMETER_DESC>(), 40);
    assert_eq!(align_of::<D3D12_SIGNATURE_PARAMETER_DESC>(), 8);
}
#[cfg(feature = "d3dcommon")]
#[test]
fn um_d3dcommon() {
    use winapi::um::d3dcommon::*;
    assert_eq!(size_of::<D3D_SHADER_MACRO>(), 16);
    assert_eq!(align_of::<D3D_SHADER_MACRO>(), 8);
}
#[cfg(feature = "d3dcompiler")]
#[test]
fn um_d3dcompiler() {
    use winapi::um::d3dcompiler::*;
    assert_eq!(size_of::<D3D_SHADER_DATA>(), 16);
    assert_eq!(align_of::<D3D_SHADER_DATA>(), 8);
}
#[cfg(feature = "davclnt")]
#[test]
fn um_davclnt() {
    use winapi::um::davclnt::*;
    assert_eq!(size_of::<DAV_CALLBACK_AUTH_BLOB>(), 16);
    assert_eq!(align_of::<DAV_CALLBACK_AUTH_BLOB>(), 8);
    assert_eq!(size_of::<DAV_CALLBACK_AUTH_UNP>(), 32);
    assert_eq!(align_of::<DAV_CALLBACK_AUTH_UNP>(), 8);
    assert_eq!(size_of::<DAV_CALLBACK_CRED>(), 56);
    assert_eq!(align_of::<DAV_CALLBACK_CRED>(), 8);
}
#[cfg(feature = "dcommon")]
#[test]
fn um_dcommon() {
    use winapi::um::dcommon::*;
    assert_eq!(size_of::<D2D1_PIXEL_FORMAT>(), 8);
    assert_eq!(align_of::<D2D1_PIXEL_FORMAT>(), 4);
    assert_eq!(size_of::<D2D_POINT_2U>(), 8);
    assert_eq!(align_of::<D2D_POINT_2U>(), 4);
    assert_eq!(size_of::<D2D_POINT_2F>(), 8);
    assert_eq!(align_of::<D2D_POINT_2F>(), 4);
    assert_eq!(size_of::<D2D_VECTOR_2F>(), 8);
    assert_eq!(align_of::<D2D_VECTOR_2F>(), 4);
    assert_eq!(size_of::<D2D_VECTOR_3F>(), 12);
    assert_eq!(align_of::<D2D_VECTOR_3F>(), 4);
    assert_eq!(size_of::<D2D_VECTOR_4F>(), 16);
    assert_eq!(align_of::<D2D_VECTOR_4F>(), 4);
    assert_eq!(size_of::<D2D_RECT_F>(), 16);
    assert_eq!(align_of::<D2D_RECT_F>(), 4);
    assert_eq!(size_of::<D2D_RECT_U>(), 16);
    assert_eq!(align_of::<D2D_RECT_U>(), 4);
    assert_eq!(size_of::<D2D_SIZE_F>(), 8);
    assert_eq!(align_of::<D2D_SIZE_F>(), 4);
    assert_eq!(size_of::<D2D_SIZE_U>(), 8);
    assert_eq!(align_of::<D2D_SIZE_U>(), 4);
    assert_eq!(size_of::<D2D_MATRIX_3X2_F>(), 24);
    assert_eq!(align_of::<D2D_MATRIX_3X2_F>(), 4);
    assert_eq!(size_of::<D2D_MATRIX_4X3_F>(), 48);
    assert_eq!(align_of::<D2D_MATRIX_4X3_F>(), 4);
    assert_eq!(size_of::<D2D_MATRIX_4X4_F>(), 64);
    assert_eq!(align_of::<D2D_MATRIX_4X4_F>(), 4);
    assert_eq!(size_of::<D2D_MATRIX_5X4_F>(), 80);
    assert_eq!(align_of::<D2D_MATRIX_5X4_F>(), 4);
}
#[cfg(feature = "dbghelp")]
#[test]
fn um_dbghelp() {
    use winapi::um::dbghelp::*;
    assert_eq!(size_of::<LOADED_IMAGE>(), 88);
    assert_eq!(align_of::<LOADED_IMAGE>(), 8);
    assert_eq!(size_of::<LOADED_IMAGE>(), 88);
    assert_eq!(align_of::<LOADED_IMAGE>(), 8);
    assert_eq!(size_of::<MODLOAD_DATA>(), 24);
    assert_eq!(align_of::<MODLOAD_DATA>(), 8);
    assert_eq!(size_of::<MODLOAD_CVMISC>(), 40);
    assert_eq!(align_of::<MODLOAD_CVMISC>(), 8);
    assert_eq!(size_of::<MODLOAD_PDBGUID_PDBAGE>(), 20);
    assert_eq!(align_of::<MODLOAD_PDBGUID_PDBAGE>(), 4);
    assert_eq!(size_of::<ADDRESS64>(), 16);
    assert_eq!(align_of::<ADDRESS64>(), 8);
    assert_eq!(size_of::<ADDRESS>(), 16);
    assert_eq!(align_of::<ADDRESS>(), 8);
    assert_eq!(size_of::<KDHELP64>(), 112);
    assert_eq!(align_of::<KDHELP64>(), 8);
    assert_eq!(size_of::<KDHELP>(), 112);
    assert_eq!(align_of::<KDHELP>(), 8);
    assert_eq!(size_of::<STACKFRAME64>(), 264);
    assert_eq!(align_of::<STACKFRAME64>(), 8);
    assert_eq!(size_of::<STACKFRAME_EX>(), 272);
    assert_eq!(align_of::<STACKFRAME_EX>(), 8);
    assert_eq!(size_of::<STACKFRAME>(), 264);
    assert_eq!(align_of::<STACKFRAME>(), 8);
    assert_eq!(size_of::<API_VERSION>(), 8);
    assert_eq!(align_of::<API_VERSION>(), 2);
    assert_eq!(size_of::<SYMBOL_INFOW>(), 88);
    assert_eq!(align_of::<SYMBOL_INFOW>(), 8);
    assert_eq!(size_of::<IMAGEHLP_SYMBOL64>(), 32);
    assert_eq!(align_of::<IMAGEHLP_SYMBOL64>(), 8);
    assert_eq!(size_of::<IMAGEHLP_LINEW64>(), 40);
    assert_eq!(align_of::<IMAGEHLP_LINEW64>(), 8);
}
#[cfg(feature = "dbt")]
#[test]
fn um_dbt() {
    use winapi::um::dbt::*;
    assert_eq!(size_of::<DEV_BROADCAST_HDR>(), 12);
    assert_eq!(align_of::<DEV_BROADCAST_HDR>(), 4);
    assert_eq!(size_of::<VolLockBroadcast>(), 20);
    assert_eq!(align_of::<VolLockBroadcast>(), 4);
    assert_eq!(size_of::<_DEV_BROADCAST_HEADER>(), 12);
    assert_eq!(align_of::<_DEV_BROADCAST_HEADER>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_OEM>(), 20);
    assert_eq!(align_of::<DEV_BROADCAST_OEM>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_DEVNODE>(), 16);
    assert_eq!(align_of::<DEV_BROADCAST_DEVNODE>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_VOLUME>(), 20);
    assert_eq!(align_of::<DEV_BROADCAST_VOLUME>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_PORT_A>(), 16);
    assert_eq!(align_of::<DEV_BROADCAST_PORT_A>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_PORT_W>(), 16);
    assert_eq!(align_of::<DEV_BROADCAST_PORT_W>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_NET>(), 20);
    assert_eq!(align_of::<DEV_BROADCAST_NET>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_DEVICEINTERFACE_A>(), 32);
    assert_eq!(align_of::<DEV_BROADCAST_DEVICEINTERFACE_A>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_DEVICEINTERFACE_W>(), 32);
    assert_eq!(align_of::<DEV_BROADCAST_DEVICEINTERFACE_W>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_HANDLE>(), 56);
    assert_eq!(align_of::<DEV_BROADCAST_HANDLE>(), 8);
    assert_eq!(size_of::<DEV_BROADCAST_HANDLE32>(), 44);
    assert_eq!(align_of::<DEV_BROADCAST_HANDLE32>(), 4);
    assert_eq!(size_of::<DEV_BROADCAST_HANDLE64>(), 56);
    assert_eq!(align_of::<DEV_BROADCAST_HANDLE64>(), 8);
    assert_eq!(size_of::<_DEV_BROADCAST_USERDEFINED>(), 16);
    assert_eq!(align_of::<_DEV_BROADCAST_USERDEFINED>(), 4);
}
#[cfg(feature = "devicetopology")]
#[test]
fn um_devicetopology() {
    use winapi::um::devicetopology::*;
    assert_eq!(size_of::<KSDATAFORMAT>(), 64);
    assert_eq!(align_of::<KSDATAFORMAT>(), 4);
    assert_eq!(size_of::<KSIDENTIFIER_s>(), 24);
    assert_eq!(align_of::<KSIDENTIFIER_s>(), 4);
    assert_eq!(size_of::<KSIDENTIFIER>(), 24);
    assert_eq!(align_of::<KSIDENTIFIER>(), 8);
    assert_eq!(size_of::<LUID>(), 8);
    assert_eq!(align_of::<LUID>(), 4);
    assert_eq!(size_of::<KSJACK_SINK_INFORMATION>(), 96);
    assert_eq!(align_of::<KSJACK_SINK_INFORMATION>(), 4);
    assert_eq!(size_of::<KSJACK_DESCRIPTION2>(), 8);
    assert_eq!(align_of::<KSJACK_DESCRIPTION2>(), 4);
}
#[cfg(feature = "docobj")]
#[test]
fn um_docobj() {
    use winapi::um::docobj::*;
    assert_eq!(size_of::<OLECMD>(), 8);
    assert_eq!(align_of::<OLECMD>(), 4);
    assert_eq!(size_of::<OLECMDTEXT>(), 16);
    assert_eq!(align_of::<OLECMDTEXT>(), 4);
}
#[cfg(feature = "dot1x")]
#[test]
fn um_dot1x() {
    use winapi::um::dot1x::*;
    assert_eq!(size_of::<ONEX_VARIABLE_BLOB>(), 8);
    assert_eq!(align_of::<ONEX_VARIABLE_BLOB>(), 4);
    assert_eq!(size_of::<ONEX_AUTH_PARAMS>(), 72);
    assert_eq!(align_of::<ONEX_AUTH_PARAMS>(), 8);
    assert_eq!(size_of::<ONEX_EAP_ERROR>(), 92);
    assert_eq!(align_of::<ONEX_EAP_ERROR>(), 4);
    assert_eq!(size_of::<ONEX_STATUS>(), 12);
    assert_eq!(align_of::<ONEX_STATUS>(), 4);
    assert_eq!(size_of::<ONEX_RESULT_UPDATE_DATA>(), 40);
    assert_eq!(align_of::<ONEX_RESULT_UPDATE_DATA>(), 4);
    assert_eq!(size_of::<ONEX_USER_INFO>(), 24);
    assert_eq!(align_of::<ONEX_USER_INFO>(), 4);
}
#[cfg(feature = "dpapi")]
#[test]
fn um_dpapi() {
    use winapi::um::dpapi::*;
    assert_eq!(size_of::<CRYPTPROTECT_PROMPTSTRUCT>(), 24);
    assert_eq!(align_of::<CRYPTPROTECT_PROMPTSTRUCT>(), 8);
}
#[cfg(feature = "dpa_dsa")]
#[test]
fn um_dpa_dsa() {
    use winapi::um::dpa_dsa::*;
    assert_eq!(size_of::<DPASTREAMINFO>(), 16);
    assert_eq!(align_of::<DPASTREAMINFO>(), 8);
}
#[cfg(feature = "dsgetdc")]
#[test]
fn um_dsgetdc() {
    use winapi::um::dsgetdc::*;
    assert_eq!(size_of::<DOMAIN_CONTROLLER_INFOA>(), 80);
    assert_eq!(align_of::<DOMAIN_CONTROLLER_INFOA>(), 8);
    assert_eq!(size_of::<DOMAIN_CONTROLLER_INFOW>(), 80);
    assert_eq!(align_of::<DOMAIN_CONTROLLER_INFOW>(), 8);
    assert_eq!(size_of::<DS_DOMAIN_TRUSTSW>(), 56);
    assert_eq!(align_of::<DS_DOMAIN_TRUSTSW>(), 8);
    assert_eq!(size_of::<DS_DOMAIN_TRUSTSA>(), 56);
    assert_eq!(align_of::<DS_DOMAIN_TRUSTSA>(), 8);
}
#[cfg(feature = "dsound")]
#[test]
fn um_dsound() {
    use winapi::um::dsound::*;
    assert_eq!(size_of::<DSCAPS>(), 96);
    assert_eq!(align_of::<DSCAPS>(), 4);
    assert_eq!(size_of::<DSBCAPS>(), 20);
    assert_eq!(align_of::<DSBCAPS>(), 4);
    assert_eq!(size_of::<DSBUFFERDESC>(), 40);
    assert_eq!(align_of::<DSBUFFERDESC>(), 8);
}
#[cfg(feature = "dsrole")]
#[test]
fn um_dsrole() {
    use winapi::um::dsrole::*;
    assert_eq!(size_of::<DSROLE_PRIMARY_DOMAIN_INFO_BASIC>(), 48);
    assert_eq!(align_of::<DSROLE_PRIMARY_DOMAIN_INFO_BASIC>(), 8);
    assert_eq!(size_of::<DSROLE_UPGRADE_STATUS_INFO>(), 8);
    assert_eq!(align_of::<DSROLE_UPGRADE_STATUS_INFO>(), 4);
    assert_eq!(size_of::<DSROLE_OPERATION_STATE_INFO>(), 4);
    assert_eq!(align_of::<DSROLE_OPERATION_STATE_INFO>(), 4);
}
#[cfg(feature = "dwmapi")]
#[test]
fn um_dwmapi() {
    use winapi::um::dwmapi::*;
    assert_eq!(size_of::<DWM_BLURBEHIND>(), 20);
    assert_eq!(align_of::<DWM_BLURBEHIND>(), 1);
    assert_eq!(size_of::<DWM_THUMBNAIL_PROPERTIES>(), 45);
    assert_eq!(align_of::<DWM_THUMBNAIL_PROPERTIES>(), 1);
    assert_eq!(size_of::<UNSIGNED_RATIO>(), 8);
    assert_eq!(align_of::<UNSIGNED_RATIO>(), 1);
    assert_eq!(size_of::<DWM_TIMING_INFO>(), 292);
    assert_eq!(align_of::<DWM_TIMING_INFO>(), 1);
    assert_eq!(size_of::<DWM_PRESENT_PARAMETERS>(), 40);
    assert_eq!(align_of::<DWM_PRESENT_PARAMETERS>(), 1);
}
#[cfg(feature = "dwrite")]
#[test]
fn um_dwrite() {
    use winapi::um::dwrite::*;
    assert_eq!(size_of::<DWRITE_FONT_METRICS>(), 20);
    assert_eq!(align_of::<DWRITE_FONT_METRICS>(), 2);
    assert_eq!(size_of::<DWRITE_GLYPH_METRICS>(), 28);
    assert_eq!(align_of::<DWRITE_GLYPH_METRICS>(), 4);
    assert_eq!(size_of::<DWRITE_GLYPH_OFFSET>(), 8);
    assert_eq!(align_of::<DWRITE_GLYPH_OFFSET>(), 4);
    assert_eq!(size_of::<DWRITE_FONT_METRICS1>(), 48);
    assert_eq!(align_of::<DWRITE_FONT_METRICS1>(), 4);
    assert_eq!(size_of::<DWRITE_UNICODE_RANGE>(), 8);
    assert_eq!(align_of::<DWRITE_UNICODE_RANGE>(), 4);
    assert_eq!(size_of::<DWRITE_CARET_METRICS>(), 6);
    assert_eq!(align_of::<DWRITE_CARET_METRICS>(), 2);
    assert_eq!(size_of::<DWRITE_MATRIX>(), 24);
    assert_eq!(align_of::<DWRITE_MATRIX>(), 4);
    assert_eq!(size_of::<DWRITE_TEXT_RANGE>(), 8);
    assert_eq!(align_of::<DWRITE_TEXT_RANGE>(), 4);
    assert_eq!(size_of::<DWRITE_FONT_FEATURE>(), 8);
    assert_eq!(align_of::<DWRITE_FONT_FEATURE>(), 4);
    assert_eq!(size_of::<DWRITE_TYPOGRAPHIC_FEATURES>(), 16);
    assert_eq!(align_of::<DWRITE_TYPOGRAPHIC_FEATURES>(), 8);
    assert_eq!(size_of::<DWRITE_TRIMMING>(), 12);
    assert_eq!(align_of::<DWRITE_TRIMMING>(), 4);
    assert_eq!(size_of::<DWRITE_SCRIPT_ANALYSIS>(), 8);
    assert_eq!(align_of::<DWRITE_SCRIPT_ANALYSIS>(), 4);
    assert_eq!(size_of::<DWRITE_LINE_BREAKPOINT>(), 1);
    assert_eq!(align_of::<DWRITE_LINE_BREAKPOINT>(), 1);
    assert_eq!(size_of::<DWRITE_SHAPING_TEXT_PROPERTIES>(), 2);
    assert_eq!(align_of::<DWRITE_SHAPING_TEXT_PROPERTIES>(), 2);
    assert_eq!(size_of::<DWRITE_SHAPING_GLYPH_PROPERTIES>(), 2);
    assert_eq!(align_of::<DWRITE_SHAPING_GLYPH_PROPERTIES>(), 2);
    assert_eq!(size_of::<DWRITE_GLYPH_RUN>(), 48);
    assert_eq!(align_of::<DWRITE_GLYPH_RUN>(), 8);
    assert_eq!(size_of::<DWRITE_GLYPH_RUN_DESCRIPTION>(), 40);
    assert_eq!(align_of::<DWRITE_GLYPH_RUN_DESCRIPTION>(), 8);
    assert_eq!(size_of::<DWRITE_UNDERLINE>(), 40);
    assert_eq!(align_of::<DWRITE_UNDERLINE>(), 8);
    assert_eq!(size_of::<DWRITE_STRIKETHROUGH>(), 40);
    assert_eq!(align_of::<DWRITE_STRIKETHROUGH>(), 8);
    assert_eq!(size_of::<DWRITE_LINE_METRICS>(), 24);
    assert_eq!(align_of::<DWRITE_LINE_METRICS>(), 4);
    assert_eq!(size_of::<DWRITE_CLUSTER_METRICS>(), 8);
    assert_eq!(align_of::<DWRITE_CLUSTER_METRICS>(), 4);
    assert_eq!(size_of::<DWRITE_TEXT_METRICS>(), 36);
    assert_eq!(align_of::<DWRITE_TEXT_METRICS>(), 4);
    assert_eq!(size_of::<DWRITE_INLINE_OBJECT_METRICS>(), 16);
    assert_eq!(align_of::<DWRITE_INLINE_OBJECT_METRICS>(), 4);
    assert_eq!(size_of::<DWRITE_OVERHANG_METRICS>(), 16);
    assert_eq!(align_of::<DWRITE_OVERHANG_METRICS>(), 4);
    assert_eq!(size_of::<DWRITE_HIT_TEST_METRICS>(), 36);
    assert_eq!(align_of::<DWRITE_HIT_TEST_METRICS>(), 4);
}
#[cfg(feature = "dwrite_1")]
#[test]
fn um_dwrite_1() {
    use winapi::um::dwrite_1::*;
    assert_eq!(size_of::<DWRITE_FONT_METRICS1>(), 48);
    assert_eq!(align_of::<DWRITE_FONT_METRICS1>(), 4);
    assert_eq!(size_of::<DWRITE_CARET_METRICS>(), 6);
    assert_eq!(align_of::<DWRITE_CARET_METRICS>(), 2);
    assert_eq!(size_of::<DWRITE_PANOSE_text>(), 10);
    assert_eq!(align_of::<DWRITE_PANOSE_text>(), 1);
    assert_eq!(size_of::<DWRITE_PANOSE_script>(), 10);
    assert_eq!(align_of::<DWRITE_PANOSE_script>(), 1);
    assert_eq!(size_of::<DWRITE_PANOSE_decorative>(), 10);
    assert_eq!(align_of::<DWRITE_PANOSE_decorative>(), 1);
    assert_eq!(size_of::<DWRITE_PANOSE_symbol>(), 10);
    assert_eq!(align_of::<DWRITE_PANOSE_symbol>(), 1);
    assert_eq!(size_of::<DWRITE_PANOSE>(), 10);
    assert_eq!(align_of::<DWRITE_PANOSE>(), 1);
    assert_eq!(size_of::<DWRITE_UNICODE_RANGE>(), 8);
    assert_eq!(align_of::<DWRITE_UNICODE_RANGE>(), 4);
    assert_eq!(size_of::<DWRITE_SCRIPT_PROPERTIES>(), 20);
    assert_eq!(align_of::<DWRITE_SCRIPT_PROPERTIES>(), 4);
    assert_eq!(size_of::<DWRITE_JUSTIFICATION_OPPORTUNITY>(), 16);
    assert_eq!(align_of::<DWRITE_JUSTIFICATION_OPPORTUNITY>(), 4);
}
#[cfg(feature = "dwrite_2")]
#[test]
fn um_dwrite_2() {
    use winapi::um::dwrite_2::*;
    assert_eq!(size_of::<DWRITE_TEXT_METRICS1>(), 40);
    assert_eq!(align_of::<DWRITE_TEXT_METRICS1>(), 4);
    assert_eq!(size_of::<DWRITE_COLOR_GLYPH_RUN>(), 88);
    assert_eq!(align_of::<DWRITE_COLOR_GLYPH_RUN>(), 8);
}
#[cfg(feature = "dwrite_3")]
#[test]
fn um_dwrite_3() {
    use winapi::um::dwrite_3::*;
    assert_eq!(size_of::<DWRITE_FONT_PROPERTY>(), 24);
    assert_eq!(align_of::<DWRITE_FONT_PROPERTY>(), 8);
    assert_eq!(size_of::<DWRITE_LINE_METRICS1>(), 32);
    assert_eq!(align_of::<DWRITE_LINE_METRICS1>(), 4);
    assert_eq!(size_of::<DWRITE_LINE_SPACING>(), 20);
    assert_eq!(align_of::<DWRITE_LINE_SPACING>(), 4);
}
#[cfg(feature = "dxgidebug")]
#[test]
fn um_dxgidebug() {
    use winapi::um::dxgidebug::*;
    assert_eq!(size_of::<DXGI_INFO_QUEUE_MESSAGE>(), 48);
    assert_eq!(align_of::<DXGI_INFO_QUEUE_MESSAGE>(), 8);
    assert_eq!(size_of::<DXGI_INFO_QUEUE_FILTER_DESC>(), 48);
    assert_eq!(align_of::<DXGI_INFO_QUEUE_FILTER_DESC>(), 8);
    assert_eq!(size_of::<DXGI_INFO_QUEUE_FILTER>(), 96);
    assert_eq!(align_of::<DXGI_INFO_QUEUE_FILTER>(), 8);
}
#[cfg(feature = "dxva2api")]
#[test]
fn um_dxva2api() {
    use winapi::um::dxva2api::*;
    assert_eq!(size_of::<DXVA2_ExtendedFormat>(), 4);
    assert_eq!(align_of::<DXVA2_ExtendedFormat>(), 4);
    assert_eq!(size_of::<DXVA2_Frequency>(), 8);
    assert_eq!(align_of::<DXVA2_Frequency>(), 4);
    assert_eq!(size_of::<DXVA2_VideoDesc>(), 40);
    assert_eq!(align_of::<DXVA2_VideoDesc>(), 4);
    assert_eq!(size_of::<DXVA2_VideoProcessorCaps>(), 40);
    assert_eq!(align_of::<DXVA2_VideoProcessorCaps>(), 4);
    assert_eq!(size_of::<DXVA2_Fixed32>(), 4);
    assert_eq!(align_of::<DXVA2_Fixed32>(), 4);
    assert_eq!(size_of::<DXVA2_AYUVSample8>(), 4);
    assert_eq!(align_of::<DXVA2_AYUVSample8>(), 1);
    assert_eq!(size_of::<DXVA2_AYUVSample16>(), 8);
    assert_eq!(align_of::<DXVA2_AYUVSample16>(), 2);
    assert_eq!(size_of::<DXVA2_VideoSample>(), 136);
    assert_eq!(align_of::<DXVA2_VideoSample>(), 8);
    assert_eq!(size_of::<DXVA2_ValueRange>(), 16);
    assert_eq!(align_of::<DXVA2_ValueRange>(), 4);
    assert_eq!(size_of::<DXVA2_ProcAmpValues>(), 16);
    assert_eq!(align_of::<DXVA2_ProcAmpValues>(), 4);
    assert_eq!(size_of::<DXVA2_FilterValues>(), 12);
    assert_eq!(align_of::<DXVA2_FilterValues>(), 4);
    assert_eq!(size_of::<DXVA2_VideoProcessBltParams>(), 120);
    assert_eq!(align_of::<DXVA2_VideoProcessBltParams>(), 8);
    assert_eq!(size_of::<DXVA2_ConfigPictureDecode>(), 100);
    assert_eq!(align_of::<DXVA2_ConfigPictureDecode>(), 4);
    assert_eq!(size_of::<DXVA2_DecodeBufferDesc>(), 48);
    assert_eq!(align_of::<DXVA2_DecodeBufferDesc>(), 8);
    assert_eq!(size_of::<DXVA2_AES_CTR_IV>(), 16);
    assert_eq!(align_of::<DXVA2_AES_CTR_IV>(), 8);
    assert_eq!(size_of::<DXVA2_DecodeExtensionData>(), 40);
    assert_eq!(align_of::<DXVA2_DecodeExtensionData>(), 8);
    assert_eq!(size_of::<DXVA2_DecodeExecuteParams>(), 24);
    assert_eq!(align_of::<DXVA2_DecodeExecuteParams>(), 8);
}
#[cfg(feature = "dxvahd")]
#[test]
fn um_dxvahd() {
    use winapi::um::dxvahd::*;
    assert_eq!(size_of::<DXVAHD_RATIONAL>(), 8);
    assert_eq!(align_of::<DXVAHD_RATIONAL>(), 4);
    assert_eq!(size_of::<DXVAHD_COLOR_RGBA>(), 16);
    assert_eq!(align_of::<DXVAHD_COLOR_RGBA>(), 4);
    assert_eq!(size_of::<DXVAHD_COLOR_YCbCrA>(), 16);
    assert_eq!(align_of::<DXVAHD_COLOR_YCbCrA>(), 4);
    assert_eq!(size_of::<DXVAHD_COLOR>(), 16);
    assert_eq!(align_of::<DXVAHD_COLOR>(), 4);
    assert_eq!(size_of::<DXVAHD_CONTENT_DESC>(), 36);
    assert_eq!(align_of::<DXVAHD_CONTENT_DESC>(), 4);
    assert_eq!(size_of::<DXVAHD_VPDEVCAPS>(), 44);
    assert_eq!(align_of::<DXVAHD_VPDEVCAPS>(), 4);
    assert_eq!(size_of::<DXVAHD_VPCAPS>(), 36);
    assert_eq!(align_of::<DXVAHD_VPCAPS>(), 4);
    assert_eq!(size_of::<DXVAHD_CUSTOM_RATE_DATA>(), 20);
    assert_eq!(align_of::<DXVAHD_CUSTOM_RATE_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_FILTER_RANGE_DATA>(), 16);
    assert_eq!(align_of::<DXVAHD_FILTER_RANGE_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_BLT_STATE_TARGET_RECT_DATA>(), 20);
    assert_eq!(align_of::<DXVAHD_BLT_STATE_TARGET_RECT_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA>(), 20);
    assert_eq!(align_of::<DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA>(), 4);
    assert_eq!(align_of::<DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_BLT_STATE_ALPHA_FILL_DATA>(), 8);
    assert_eq!(align_of::<DXVAHD_BLT_STATE_ALPHA_FILL_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_BLT_STATE_CONSTRICTION_DATA>(), 12);
    assert_eq!(align_of::<DXVAHD_BLT_STATE_CONSTRICTION_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_BLT_STATE_PRIVATE_DATA>(), 32);
    assert_eq!(align_of::<DXVAHD_BLT_STATE_PRIVATE_DATA>(), 8);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_D3DFORMAT_DATA>(), 4);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_D3DFORMAT_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA>(), 4);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA>(), 4);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA>(), 16);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_SOURCE_RECT_DATA>(), 20);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_SOURCE_RECT_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA>(), 20);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_ALPHA_DATA>(), 8);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_ALPHA_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_PALETTE_DATA>(), 16);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_PALETTE_DATA>(), 8);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_LUMA_KEY_DATA>(), 12);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_LUMA_KEY_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA>(), 20);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_FILTER_DATA>(), 8);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_FILTER_DATA>(), 4);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_PRIVATE_DATA>(), 32);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_PRIVATE_DATA>(), 8);
    assert_eq!(size_of::<DXVAHD_STREAM_DATA>(), 48);
    assert_eq!(align_of::<DXVAHD_STREAM_DATA>(), 8);
    assert_eq!(size_of::<DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA>(), 16);
    assert_eq!(align_of::<DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA>(), 4);
    assert_eq!(size_of::<DXVAHDSW_CALLBACKS>(), 128);
    assert_eq!(align_of::<DXVAHDSW_CALLBACKS>(), 8);
    assert_eq!(size_of::<DXVAHDETW_CREATEVIDEOPROCESSOR>(), 32);
    assert_eq!(align_of::<DXVAHDETW_CREATEVIDEOPROCESSOR>(), 8);
    assert_eq!(size_of::<DXVAHDETW_VIDEOPROCESSBLTSTATE>(), 24);
    assert_eq!(align_of::<DXVAHDETW_VIDEOPROCESSBLTSTATE>(), 8);
    assert_eq!(size_of::<DXVAHDETW_VIDEOPROCESSSTREAMSTATE>(), 24);
    assert_eq!(align_of::<DXVAHDETW_VIDEOPROCESSSTREAMSTATE>(), 8);
    assert_eq!(size_of::<DXVAHDETW_VIDEOPROCESSBLTHD>(), 56);
    assert_eq!(align_of::<DXVAHDETW_VIDEOPROCESSBLTHD>(), 8);
    assert_eq!(size_of::<DXVAHDETW_VIDEOPROCESSBLTHD_STREAM>(), 80);
    assert_eq!(align_of::<DXVAHDETW_VIDEOPROCESSBLTHD_STREAM>(), 8);
    assert_eq!(size_of::<DXVAHDETW_DESTROYVIDEOPROCESSOR>(), 8);
    assert_eq!(align_of::<DXVAHDETW_DESTROYVIDEOPROCESSOR>(), 8);
}
#[cfg(feature = "eaptypes")]
#[test]
fn um_eaptypes() {
    use winapi::um::eaptypes::*;
    assert_eq!(size_of::<EAP_TYPE>(), 12);
    assert_eq!(align_of::<EAP_TYPE>(), 4);
    assert_eq!(size_of::<EAP_METHOD_TYPE>(), 16);
    assert_eq!(align_of::<EAP_METHOD_TYPE>(), 4);
    assert_eq!(size_of::<EAP_METHOD_INFO>(), 48);
    assert_eq!(align_of::<EAP_METHOD_INFO>(), 8);
    assert_eq!(size_of::<EAP_METHOD_INFO_ARRAY_EX>(), 16);
    assert_eq!(align_of::<EAP_METHOD_INFO_ARRAY_EX>(), 8);
    assert_eq!(size_of::<EAP_METHOD_INFO_EX>(), 48);
    assert_eq!(align_of::<EAP_METHOD_INFO_EX>(), 8);
    assert_eq!(size_of::<EAP_METHOD_INFO_ARRAY>(), 16);
    assert_eq!(align_of::<EAP_METHOD_INFO_ARRAY>(), 8);
    assert_eq!(size_of::<EAP_ERROR>(), 88);
    assert_eq!(align_of::<EAP_ERROR>(), 8);
    assert_eq!(size_of::<EAP_ATTRIBUTE>(), 16);
    assert_eq!(align_of::<EAP_ATTRIBUTE>(), 8);
    assert_eq!(size_of::<EAP_ATTRIBUTES>(), 16);
    assert_eq!(align_of::<EAP_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<EAP_CONFIG_INPUT_FIELD_DATA>(), 40);
    assert_eq!(align_of::<EAP_CONFIG_INPUT_FIELD_DATA>(), 8);
    assert_eq!(size_of::<EAP_CONFIG_INPUT_FIELD_ARRAY>(), 16);
    assert_eq!(align_of::<EAP_CONFIG_INPUT_FIELD_ARRAY>(), 8);
    assert_eq!(size_of::<EAP_CRED_EXPIRY_REQ>(), 32);
    assert_eq!(align_of::<EAP_CRED_EXPIRY_REQ>(), 8);
    assert_eq!(size_of::<EAP_UI_DATA_FORMAT>(), 8);
    assert_eq!(align_of::<EAP_UI_DATA_FORMAT>(), 8);
    assert_eq!(size_of::<EAP_INTERACTIVE_UI_DATA>(), 24);
    assert_eq!(align_of::<EAP_INTERACTIVE_UI_DATA>(), 8);
    assert_eq!(size_of::<EAP_METHOD_PROPERTY_VALUE_BOOL>(), 8);
    assert_eq!(align_of::<EAP_METHOD_PROPERTY_VALUE_BOOL>(), 4);
    assert_eq!(size_of::<EAP_METHOD_PROPERTY_VALUE_DWORD>(), 8);
    assert_eq!(align_of::<EAP_METHOD_PROPERTY_VALUE_DWORD>(), 4);
    assert_eq!(size_of::<EAP_METHOD_PROPERTY_VALUE_STRING>(), 16);
    assert_eq!(align_of::<EAP_METHOD_PROPERTY_VALUE_STRING>(), 8);
    assert_eq!(size_of::<EAP_METHOD_PROPERTY_VALUE>(), 16);
    assert_eq!(align_of::<EAP_METHOD_PROPERTY_VALUE>(), 8);
    assert_eq!(size_of::<EAP_METHOD_PROPERTY>(), 24);
    assert_eq!(align_of::<EAP_METHOD_PROPERTY>(), 8);
    assert_eq!(size_of::<EAP_METHOD_PROPERTY_ARRAY>(), 16);
    assert_eq!(align_of::<EAP_METHOD_PROPERTY_ARRAY>(), 8);
    assert_eq!(size_of::<EAPHOST_IDENTITY_UI_PARAMS>(), 88);
    assert_eq!(align_of::<EAPHOST_IDENTITY_UI_PARAMS>(), 8);
    assert_eq!(size_of::<EAPHOST_INTERACTIVE_UI_PARAMS>(), 48);
    assert_eq!(align_of::<EAPHOST_INTERACTIVE_UI_PARAMS>(), 8);
    assert_eq!(size_of::<EapUsernamePasswordCredential>(), 16);
    assert_eq!(align_of::<EapUsernamePasswordCredential>(), 8);
    assert_eq!(size_of::<EapCertificateCredential>(), 32);
    assert_eq!(align_of::<EapCertificateCredential>(), 8);
    assert_eq!(size_of::<EapSimCredential>(), 8);
    assert_eq!(align_of::<EapSimCredential>(), 8);
    assert_eq!(size_of::<EapCredentialTypeData>(), 32);
    assert_eq!(align_of::<EapCredentialTypeData>(), 8);
    assert_eq!(size_of::<EapCredential>(), 40);
    assert_eq!(align_of::<EapCredential>(), 8);
}
#[cfg(feature = "endpointvolume")]
#[test]
fn um_endpointvolume() {
    use winapi::um::endpointvolume::*;
    assert_eq!(size_of::<AUDIO_VOLUME_NOTIFICATION_DATA>(), 32);
    assert_eq!(align_of::<AUDIO_VOLUME_NOTIFICATION_DATA>(), 4);
}
#[cfg(feature = "evntcons")]
#[test]
fn um_evntcons() {
    use winapi::um::evntcons::*;
    assert_eq!(size_of::<EVENT_HEADER_EXTENDED_DATA_ITEM>(), 16);
    assert_eq!(align_of::<EVENT_HEADER_EXTENDED_DATA_ITEM>(), 8);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_INSTANCE>(), 24);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_INSTANCE>(), 4);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID>(), 16);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID>(), 4);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_TS_ID>(), 4);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_TS_ID>(), 4);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_STACK_TRACE32>(), 16);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_STACK_TRACE32>(), 8);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_STACK_TRACE64>(), 16);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_STACK_TRACE64>(), 8);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_PEBS_INDEX>(), 8);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_PEBS_INDEX>(), 8);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_PMC_COUNTERS>(), 8);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_PMC_COUNTERS>(), 8);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_PROCESS_START_KEY>(), 8);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_PROCESS_START_KEY>(), 8);
    assert_eq!(size_of::<EVENT_EXTENDED_ITEM_EVENT_KEY>(), 8);
    assert_eq!(align_of::<EVENT_EXTENDED_ITEM_EVENT_KEY>(), 8);
    assert_eq!(size_of::<EVENT_HEADER_u_s>(), 8);
    assert_eq!(align_of::<EVENT_HEADER_u_s>(), 4);
    assert_eq!(size_of::<EVENT_HEADER_u>(), 8);
    assert_eq!(align_of::<EVENT_HEADER_u>(), 8);
    assert_eq!(size_of::<EVENT_HEADER>(), 80);
    assert_eq!(align_of::<EVENT_HEADER>(), 8);
    assert_eq!(size_of::<EVENT_RECORD>(), 112);
    assert_eq!(align_of::<EVENT_RECORD>(), 8);
}
#[cfg(feature = "fileapi")]
#[test]
fn um_fileapi() {
    use winapi::um::fileapi::*;
    assert_eq!(size_of::<WIN32_FILE_ATTRIBUTE_DATA>(), 36);
    assert_eq!(align_of::<WIN32_FILE_ATTRIBUTE_DATA>(), 4);
    assert_eq!(size_of::<BY_HANDLE_FILE_INFORMATION>(), 52);
    assert_eq!(align_of::<BY_HANDLE_FILE_INFORMATION>(), 4);
    assert_eq!(size_of::<CREATEFILE2_EXTENDED_PARAMETERS>(), 32);
    assert_eq!(align_of::<CREATEFILE2_EXTENDED_PARAMETERS>(), 8);
    assert_eq!(size_of::<FILE_BASIC_INFO>(), 40);
    assert_eq!(align_of::<FILE_BASIC_INFO>(), 8);
    assert_eq!(size_of::<FILE_STANDARD_INFO>(), 24);
    assert_eq!(align_of::<FILE_STANDARD_INFO>(), 8);
    assert_eq!(size_of::<FILE_NAME_INFO>(), 8);
    assert_eq!(align_of::<FILE_NAME_INFO>(), 4);
    assert_eq!(size_of::<FILE_RENAME_INFO>(), 24);
    assert_eq!(align_of::<FILE_RENAME_INFO>(), 8);
    assert_eq!(size_of::<FILE_DISPOSITION_INFO>(), 1);
    assert_eq!(align_of::<FILE_DISPOSITION_INFO>(), 1);
    assert_eq!(size_of::<FILE_ALLOCATION_INFO>(), 8);
    assert_eq!(align_of::<FILE_ALLOCATION_INFO>(), 8);
    assert_eq!(size_of::<FILE_END_OF_FILE_INFO>(), 8);
    assert_eq!(align_of::<FILE_END_OF_FILE_INFO>(), 8);
    assert_eq!(size_of::<FILE_STREAM_INFO>(), 32);
    assert_eq!(align_of::<FILE_STREAM_INFO>(), 8);
    assert_eq!(size_of::<FILE_COMPRESSION_INFO>(), 16);
    assert_eq!(align_of::<FILE_COMPRESSION_INFO>(), 8);
    assert_eq!(size_of::<FILE_ATTRIBUTE_TAG_INFO>(), 8);
    assert_eq!(align_of::<FILE_ATTRIBUTE_TAG_INFO>(), 4);
    assert_eq!(size_of::<FILE_ID_BOTH_DIR_INFO>(), 112);
    assert_eq!(align_of::<FILE_ID_BOTH_DIR_INFO>(), 8);
    assert_eq!(size_of::<FILE_IO_PRIORITY_HINT_INFO>(), 4);
    assert_eq!(align_of::<FILE_IO_PRIORITY_HINT_INFO>(), 4);
    assert_eq!(size_of::<FILE_FULL_DIR_INFO>(), 72);
    assert_eq!(align_of::<FILE_FULL_DIR_INFO>(), 8);
    assert_eq!(size_of::<FILE_STORAGE_INFO>(), 28);
    assert_eq!(align_of::<FILE_STORAGE_INFO>(), 4);
    assert_eq!(size_of::<FILE_ALIGNMENT_INFO>(), 4);
    assert_eq!(align_of::<FILE_ALIGNMENT_INFO>(), 4);
    assert_eq!(size_of::<FILE_ID_INFO>(), 24);
    assert_eq!(align_of::<FILE_ID_INFO>(), 8);
}
#[cfg(feature = "heapapi")]
#[test]
fn um_heapapi() {
    use winapi::um::heapapi::*;
    assert_eq!(size_of::<HEAP_SUMMARY>(), 40);
    assert_eq!(align_of::<HEAP_SUMMARY>(), 8);
}
#[cfg(feature = "http")]
#[test]
fn um_http() {
    use winapi::um::http::*;
    assert_eq!(size_of::<HTTP_PROPERTY_FLAGS>(), 4);
    assert_eq!(align_of::<HTTP_PROPERTY_FLAGS>(), 4);
    assert_eq!(size_of::<HTTP_STATE_INFO>(), 8);
    assert_eq!(align_of::<HTTP_STATE_INFO>(), 4);
    assert_eq!(size_of::<HTTP_QOS_SETTING_INFO>(), 16);
    assert_eq!(align_of::<HTTP_QOS_SETTING_INFO>(), 8);
    assert_eq!(size_of::<HTTP_CONNECTION_LIMIT_INFO>(), 8);
    assert_eq!(align_of::<HTTP_CONNECTION_LIMIT_INFO>(), 4);
    assert_eq!(size_of::<HTTP_BANDWIDTH_LIMIT_INFO>(), 8);
    assert_eq!(align_of::<HTTP_BANDWIDTH_LIMIT_INFO>(), 4);
    assert_eq!(size_of::<HTTP_FLOWRATE_INFO>(), 16);
    assert_eq!(align_of::<HTTP_FLOWRATE_INFO>(), 4);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_TIMEOUT_SET>(), 8);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_TIMEOUT_SET>(), 4);
    assert_eq!(size_of::<HTTP_TIMEOUT_LIMIT_INFO>(), 20);
    assert_eq!(align_of::<HTTP_TIMEOUT_LIMIT_INFO>(), 4);
    assert_eq!(size_of::<HTTP_LISTEN_ENDPOINT_INFO>(), 8);
    assert_eq!(align_of::<HTTP_LISTEN_ENDPOINT_INFO>(), 4);
    assert_eq!(size_of::<HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS>(), 32);
    assert_eq!(align_of::<HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS>(), 8);
    assert_eq!(size_of::<HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS>(), 16);
    assert_eq!(align_of::<HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS>(), 8);
    assert_eq!(size_of::<HTTP_SERVER_AUTHENTICATION_INFO>(), 64);
    assert_eq!(align_of::<HTTP_SERVER_AUTHENTICATION_INFO>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_BINDING_BASE>(), 4);
    assert_eq!(align_of::<HTTP_SERVICE_BINDING_BASE>(), 4);
    assert_eq!(size_of::<HTTP_SERVICE_BINDING_A>(), 24);
    assert_eq!(align_of::<HTTP_SERVICE_BINDING_A>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_BINDING_W>(), 24);
    assert_eq!(align_of::<HTTP_SERVICE_BINDING_W>(), 8);
    assert_eq!(size_of::<HTTP_CHANNEL_BIND_INFO>(), 24);
    assert_eq!(align_of::<HTTP_CHANNEL_BIND_INFO>(), 8);
    assert_eq!(size_of::<HTTP_REQUEST_CHANNEL_BIND_STATUS>(), 24);
    assert_eq!(align_of::<HTTP_REQUEST_CHANNEL_BIND_STATUS>(), 8);
    assert_eq!(size_of::<HTTP_LOGGING_INFO>(), 72);
    assert_eq!(align_of::<HTTP_LOGGING_INFO>(), 8);
    assert_eq!(size_of::<HTTP_BINDING_INFO>(), 16);
    assert_eq!(align_of::<HTTP_BINDING_INFO>(), 8);
    assert_eq!(size_of::<HTTP_PROTECTION_LEVEL_INFO>(), 8);
    assert_eq!(align_of::<HTTP_PROTECTION_LEVEL_INFO>(), 4);
    assert_eq!(size_of::<HTTP_BYTE_RANGE>(), 16);
    assert_eq!(align_of::<HTTP_BYTE_RANGE>(), 8);
    assert_eq!(size_of::<HTTP_VERSION>(), 4);
    assert_eq!(align_of::<HTTP_VERSION>(), 2);
    assert_eq!(size_of::<HTTP_KNOWN_HEADER>(), 16);
    assert_eq!(align_of::<HTTP_KNOWN_HEADER>(), 8);
    assert_eq!(size_of::<HTTP_UNKNOWN_HEADER>(), 24);
    assert_eq!(align_of::<HTTP_UNKNOWN_HEADER>(), 8);
    assert_eq!(size_of::<HTTP_LOG_DATA>(), 4);
    assert_eq!(align_of::<HTTP_LOG_DATA>(), 4);
    assert_eq!(size_of::<HTTP_LOG_FIELDS_DATA>(), 144);
    assert_eq!(align_of::<HTTP_LOG_FIELDS_DATA>(), 8);
    assert_eq!(size_of::<HTTP_DATA_CHUNK_FromMemory>(), 16);
    assert_eq!(align_of::<HTTP_DATA_CHUNK_FromMemory>(), 8);
    assert_eq!(size_of::<HTTP_DATA_CHUNK_FromFileHandle>(), 24);
    assert_eq!(align_of::<HTTP_DATA_CHUNK_FromFileHandle>(), 8);
    assert_eq!(size_of::<HTTP_DATA_CHUNK_FromFragmentCache>(), 16);
    assert_eq!(align_of::<HTTP_DATA_CHUNK_FromFragmentCache>(), 8);
    assert_eq!(size_of::<HTTP_DATA_CHUNK_FromFragmentCacheEx>(), 24);
    assert_eq!(align_of::<HTTP_DATA_CHUNK_FromFragmentCacheEx>(), 8);
    assert_eq!(size_of::<HTTP_DATA_CHUNK>(), 32);
    assert_eq!(align_of::<HTTP_DATA_CHUNK>(), 8);
    assert_eq!(size_of::<HTTP_REQUEST_HEADERS>(), 688);
    assert_eq!(align_of::<HTTP_REQUEST_HEADERS>(), 8);
    assert_eq!(size_of::<HTTP_RESPONSE_HEADERS>(), 512);
    assert_eq!(align_of::<HTTP_RESPONSE_HEADERS>(), 8);
    assert_eq!(size_of::<HTTP_TRANSPORT_ADDRESS>(), 16);
    assert_eq!(align_of::<HTTP_TRANSPORT_ADDRESS>(), 8);
    assert_eq!(size_of::<HTTP_COOKED_URL>(), 40);
    assert_eq!(align_of::<HTTP_COOKED_URL>(), 8);
    assert_eq!(size_of::<HTTP_SSL_CLIENT_CERT_INFO>(), 32);
    assert_eq!(align_of::<HTTP_SSL_CLIENT_CERT_INFO>(), 8);
    assert_eq!(size_of::<HTTP_SSL_INFO>(), 48);
    assert_eq!(align_of::<HTTP_SSL_INFO>(), 8);
    assert_eq!(size_of::<HTTP_REQUEST_INFO>(), 16);
    assert_eq!(align_of::<HTTP_REQUEST_INFO>(), 8);
    assert_eq!(size_of::<HTTP_REQUEST_AUTH_INFO>(), 80);
    assert_eq!(align_of::<HTTP_REQUEST_AUTH_INFO>(), 8);
    assert_eq!(size_of::<HTTP_REQUEST_V1>(), 848);
    assert_eq!(align_of::<HTTP_REQUEST_V1>(), 8);
    assert_eq!(size_of::<HTTP_REQUEST_V2>(), 864);
    assert_eq!(align_of::<HTTP_REQUEST_V2>(), 8);
    assert_eq!(size_of::<HTTP_RESPONSE_V1>(), 552);
    assert_eq!(align_of::<HTTP_RESPONSE_V1>(), 8);
    assert_eq!(size_of::<HTTP_RESPONSE_INFO>(), 16);
    assert_eq!(align_of::<HTTP_RESPONSE_INFO>(), 8);
    assert_eq!(size_of::<HTTP_MULTIPLE_KNOWN_HEADERS>(), 24);
    assert_eq!(align_of::<HTTP_MULTIPLE_KNOWN_HEADERS>(), 8);
    assert_eq!(size_of::<HTTP_RESPONSE_V2>(), 568);
    assert_eq!(align_of::<HTTP_RESPONSE_V2>(), 8);
    assert_eq!(size_of::<HTTPAPI_VERSION>(), 4);
    assert_eq!(align_of::<HTTPAPI_VERSION>(), 2);
    assert_eq!(size_of::<HTTP_CACHE_POLICY>(), 8);
    assert_eq!(align_of::<HTTP_CACHE_POLICY>(), 4);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_KEY>(), 8);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_KEY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_KEY>(), 136);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_SNI_KEY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_KEY>(), 128);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_CCS_KEY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_PARAM>(), 80);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_PARAM>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_SET>(), 88);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_SET>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_SET>(), 216);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_SNI_SET>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_SET>(), 208);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_CCS_SET>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_QUERY>(), 24);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_QUERY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_SNI_QUERY>(), 152);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_SNI_QUERY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_SSL_CCS_QUERY>(), 144);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_SSL_CCS_QUERY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM>(), 16);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY>(), 136);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_URLACL_KEY>(), 8);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_URLACL_KEY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_URLACL_PARAM>(), 8);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_URLACL_PARAM>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_URLACL_SET>(), 16);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_URLACL_SET>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_URLACL_QUERY>(), 24);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_URLACL_QUERY>(), 8);
    assert_eq!(size_of::<HTTP_SERVICE_CONFIG_CACHE_SET>(), 8);
    assert_eq!(align_of::<HTTP_SERVICE_CONFIG_CACHE_SET>(), 4);
}
#[cfg(feature = "ipexport")]
#[test]
fn um_ipexport() {
    use winapi::um::ipexport::*;
    assert_eq!(size_of::<IP_OPTION_INFORMATION>(), 16);
    assert_eq!(align_of::<IP_OPTION_INFORMATION>(), 8);
    assert_eq!(size_of::<IP_OPTION_INFORMATION32>(), 8);
    assert_eq!(align_of::<IP_OPTION_INFORMATION32>(), 4);
    assert_eq!(size_of::<ICMP_ECHO_REPLY>(), 40);
    assert_eq!(align_of::<ICMP_ECHO_REPLY>(), 8);
    assert_eq!(size_of::<ICMP_ECHO_REPLY32>(), 28);
    assert_eq!(align_of::<ICMP_ECHO_REPLY32>(), 4);
    assert_eq!(size_of::<IPV6_ADDRESS_EX>(), 26);
    assert_eq!(align_of::<IPV6_ADDRESS_EX>(), 1);
    assert_eq!(size_of::<ICMPV6_ECHO_REPLY_LH>(), 36);
    assert_eq!(align_of::<ICMPV6_ECHO_REPLY_LH>(), 4);
    assert_eq!(size_of::<ARP_SEND_REPLY>(), 8);
    assert_eq!(align_of::<ARP_SEND_REPLY>(), 4);
    assert_eq!(size_of::<TCP_RESERVE_PORT_RANGE>(), 4);
    assert_eq!(align_of::<TCP_RESERVE_PORT_RANGE>(), 2);
    assert_eq!(size_of::<IP_ADAPTER_INDEX_MAP>(), 260);
    assert_eq!(align_of::<IP_ADAPTER_INDEX_MAP>(), 4);
    assert_eq!(size_of::<IP_INTERFACE_INFO>(), 264);
    assert_eq!(align_of::<IP_INTERFACE_INFO>(), 4);
    assert_eq!(size_of::<IP_UNIDIRECTIONAL_ADAPTER_ADDRESS>(), 8);
    assert_eq!(align_of::<IP_UNIDIRECTIONAL_ADAPTER_ADDRESS>(), 4);
    assert_eq!(size_of::<IP_ADAPTER_ORDER_MAP>(), 8);
    assert_eq!(align_of::<IP_ADAPTER_ORDER_MAP>(), 4);
    assert_eq!(size_of::<IP_MCAST_COUNTER_INFO>(), 32);
    assert_eq!(align_of::<IP_MCAST_COUNTER_INFO>(), 8);
}
#[cfg(feature = "iphlpapi")]
#[test]
fn um_iphlpapi() {
    use winapi::um::iphlpapi::*;
    assert_eq!(size_of::<INTERFACE_TIMESTAMP_CAPABILITY_FLAGS>(), 14);
    assert_eq!(align_of::<INTERFACE_TIMESTAMP_CAPABILITY_FLAGS>(), 1);
    assert_eq!(size_of::<INTERFACE_TIMESTAMP_CAPABILITIES>(), 56);
    assert_eq!(align_of::<INTERFACE_TIMESTAMP_CAPABILITIES>(), 8);
    assert_eq!(size_of::<INTERFACE_HARDWARE_CROSSTIMESTAMP>(), 32);
    assert_eq!(align_of::<INTERFACE_HARDWARE_CROSSTIMESTAMP>(), 8);
    assert_eq!(size_of::<NET_ADDRESS_INFO>(), 528);
    assert_eq!(align_of::<NET_ADDRESS_INFO>(), 4);
}
#[cfg(feature = "iptypes")]
#[test]
fn um_iptypes() {
    use winapi::um::iptypes::*;
    assert_eq!(size_of::<IP_ADDRESS_STRING>(), 16);
    assert_eq!(align_of::<IP_ADDRESS_STRING>(), 1);
    assert_eq!(size_of::<IP_ADDR_STRING>(), 48);
    assert_eq!(align_of::<IP_ADDR_STRING>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_INFO>(), 704);
    assert_eq!(align_of::<IP_ADAPTER_INFO>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_UNICAST_ADDRESS_LH>(), 64);
    assert_eq!(align_of::<IP_ADAPTER_UNICAST_ADDRESS_LH>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_UNICAST_ADDRESS_XP>(), 56);
    assert_eq!(align_of::<IP_ADAPTER_UNICAST_ADDRESS_XP>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_ANYCAST_ADDRESS_XP>(), 32);
    assert_eq!(align_of::<IP_ADAPTER_ANYCAST_ADDRESS_XP>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_MULTICAST_ADDRESS_XP>(), 32);
    assert_eq!(align_of::<IP_ADAPTER_MULTICAST_ADDRESS_XP>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_DNS_SERVER_ADDRESS_XP>(), 32);
    assert_eq!(align_of::<IP_ADAPTER_DNS_SERVER_ADDRESS_XP>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_WINS_SERVER_ADDRESS_LH>(), 32);
    assert_eq!(align_of::<IP_ADAPTER_WINS_SERVER_ADDRESS_LH>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_GATEWAY_ADDRESS_LH>(), 32);
    assert_eq!(align_of::<IP_ADAPTER_GATEWAY_ADDRESS_LH>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_PREFIX_XP>(), 40);
    assert_eq!(align_of::<IP_ADAPTER_PREFIX_XP>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_DNS_SUFFIX>(), 520);
    assert_eq!(align_of::<IP_ADAPTER_DNS_SUFFIX>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_ADDRESSES_LH>(), 448);
    assert_eq!(align_of::<IP_ADAPTER_ADDRESSES_LH>(), 8);
    assert_eq!(size_of::<IP_ADAPTER_ADDRESSES_XP>(), 184);
    assert_eq!(align_of::<IP_ADAPTER_ADDRESSES_XP>(), 8);
    assert_eq!(size_of::<IP_PER_ADAPTER_INFO_W2KSP1>(), 64);
    assert_eq!(align_of::<IP_PER_ADAPTER_INFO_W2KSP1>(), 8);
    assert_eq!(size_of::<FIXED_INFO_W2KSP1>(), 600);
    assert_eq!(align_of::<FIXED_INFO_W2KSP1>(), 8);
    assert_eq!(size_of::<IP_INTERFACE_NAME_INFO_W2KSP1>(), 44);
    assert_eq!(align_of::<IP_INTERFACE_NAME_INFO_W2KSP1>(), 4);
}
#[cfg(feature = "jobapi2")]
#[test]
fn um_jobapi2() {
    use winapi::um::jobapi2::*;
    assert_eq!(size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION>(), 40);
    assert_eq!(align_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION>(), 8);
}
#[cfg(feature = "imm")]
#[test]
fn um_imm() {
    use winapi::um::imm::*;
    assert_eq!(size_of::<COMPOSITIONFORM>(), 28);
    assert_eq!(align_of::<COMPOSITIONFORM>(), 4);
}
#[cfg(feature = "l2cmn")]
#[test]
fn um_l2cmn() {
    use winapi::um::l2cmn::*;
    assert_eq!(size_of::<L2_NOTIFICATION_DATA>(), 40);
    assert_eq!(align_of::<L2_NOTIFICATION_DATA>(), 8);
}
#[cfg(feature = "lmaccess")]
#[test]
fn um_lmaccess() {
    use winapi::um::lmaccess::*;
    assert_eq!(size_of::<USER_INFO_0>(), 8);
    assert_eq!(align_of::<USER_INFO_0>(), 8);
    assert_eq!(size_of::<USER_INFO_1>(), 56);
    assert_eq!(align_of::<USER_INFO_1>(), 8);
    assert_eq!(size_of::<USER_INFO_2>(), 152);
    assert_eq!(align_of::<USER_INFO_2>(), 8);
    assert_eq!(size_of::<USER_INFO_3>(), 184);
    assert_eq!(align_of::<USER_INFO_3>(), 8);
    assert_eq!(size_of::<USER_INFO_4>(), 192);
    assert_eq!(align_of::<USER_INFO_4>(), 8);
    assert_eq!(size_of::<USER_INFO_10>(), 32);
    assert_eq!(align_of::<USER_INFO_10>(), 8);
    assert_eq!(size_of::<USER_INFO_11>(), 128);
    assert_eq!(align_of::<USER_INFO_11>(), 8);
    assert_eq!(size_of::<USER_INFO_20>(), 32);
    assert_eq!(align_of::<USER_INFO_20>(), 8);
    assert_eq!(size_of::<USER_INFO_21>(), 16);
    assert_eq!(align_of::<USER_INFO_21>(), 1);
    assert_eq!(size_of::<USER_INFO_22>(), 160);
    assert_eq!(align_of::<USER_INFO_22>(), 8);
    assert_eq!(size_of::<USER_INFO_23>(), 40);
    assert_eq!(align_of::<USER_INFO_23>(), 8);
    assert_eq!(size_of::<USER_INFO_24>(), 32);
    assert_eq!(align_of::<USER_INFO_24>(), 8);
    assert_eq!(size_of::<USER_INFO_1003>(), 8);
    assert_eq!(align_of::<USER_INFO_1003>(), 8);
    assert_eq!(size_of::<USER_INFO_1005>(), 4);
    assert_eq!(align_of::<USER_INFO_1005>(), 4);
    assert_eq!(size_of::<USER_INFO_1006>(), 8);
    assert_eq!(align_of::<USER_INFO_1006>(), 8);
    assert_eq!(size_of::<USER_INFO_1007>(), 8);
    assert_eq!(align_of::<USER_INFO_1007>(), 8);
    assert_eq!(size_of::<USER_INFO_1008>(), 4);
    assert_eq!(align_of::<USER_INFO_1008>(), 4);
    assert_eq!(size_of::<USER_INFO_1009>(), 8);
    assert_eq!(align_of::<USER_INFO_1009>(), 8);
    assert_eq!(size_of::<USER_INFO_1010>(), 4);
    assert_eq!(align_of::<USER_INFO_1010>(), 4);
    assert_eq!(size_of::<USER_INFO_1011>(), 8);
    assert_eq!(align_of::<USER_INFO_1011>(), 8);
    assert_eq!(size_of::<USER_INFO_1012>(), 8);
    assert_eq!(align_of::<USER_INFO_1012>(), 8);
    assert_eq!(size_of::<USER_INFO_1013>(), 8);
    assert_eq!(align_of::<USER_INFO_1013>(), 8);
    assert_eq!(size_of::<USER_INFO_1014>(), 8);
    assert_eq!(align_of::<USER_INFO_1014>(), 8);
    assert_eq!(size_of::<USER_INFO_1017>(), 4);
    assert_eq!(align_of::<USER_INFO_1017>(), 4);
    assert_eq!(size_of::<USER_INFO_1018>(), 4);
    assert_eq!(align_of::<USER_INFO_1018>(), 4);
    assert_eq!(size_of::<USER_INFO_1020>(), 16);
    assert_eq!(align_of::<USER_INFO_1020>(), 8);
    assert_eq!(size_of::<USER_INFO_1023>(), 8);
    assert_eq!(align_of::<USER_INFO_1023>(), 8);
    assert_eq!(size_of::<USER_INFO_1024>(), 4);
    assert_eq!(align_of::<USER_INFO_1024>(), 4);
    assert_eq!(size_of::<USER_INFO_1025>(), 4);
    assert_eq!(align_of::<USER_INFO_1025>(), 4);
    assert_eq!(size_of::<USER_INFO_1051>(), 4);
    assert_eq!(align_of::<USER_INFO_1051>(), 4);
    assert_eq!(size_of::<USER_INFO_1052>(), 8);
    assert_eq!(align_of::<USER_INFO_1052>(), 8);
    assert_eq!(size_of::<USER_INFO_1053>(), 8);
    assert_eq!(align_of::<USER_INFO_1053>(), 8);
    assert_eq!(size_of::<USER_MODALS_INFO_0>(), 20);
    assert_eq!(align_of::<USER_MODALS_INFO_0>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1>(), 16);
    assert_eq!(align_of::<USER_MODALS_INFO_1>(), 8);
    assert_eq!(size_of::<USER_MODALS_INFO_2>(), 16);
    assert_eq!(align_of::<USER_MODALS_INFO_2>(), 8);
    assert_eq!(size_of::<USER_MODALS_INFO_3>(), 12);
    assert_eq!(align_of::<USER_MODALS_INFO_3>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1001>(), 4);
    assert_eq!(align_of::<USER_MODALS_INFO_1001>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1002>(), 4);
    assert_eq!(align_of::<USER_MODALS_INFO_1002>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1003>(), 4);
    assert_eq!(align_of::<USER_MODALS_INFO_1003>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1004>(), 4);
    assert_eq!(align_of::<USER_MODALS_INFO_1004>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1005>(), 4);
    assert_eq!(align_of::<USER_MODALS_INFO_1005>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1006>(), 4);
    assert_eq!(align_of::<USER_MODALS_INFO_1006>(), 4);
    assert_eq!(size_of::<USER_MODALS_INFO_1007>(), 8);
    assert_eq!(align_of::<USER_MODALS_INFO_1007>(), 8);
    assert_eq!(size_of::<GROUP_INFO_0>(), 8);
    assert_eq!(align_of::<GROUP_INFO_0>(), 8);
    assert_eq!(size_of::<GROUP_INFO_1>(), 16);
    assert_eq!(align_of::<GROUP_INFO_1>(), 8);
    assert_eq!(size_of::<GROUP_INFO_2>(), 24);
    assert_eq!(align_of::<GROUP_INFO_2>(), 8);
    assert_eq!(size_of::<GROUP_INFO_3>(), 32);
    assert_eq!(align_of::<GROUP_INFO_3>(), 8);
    assert_eq!(size_of::<GROUP_INFO_1002>(), 8);
    assert_eq!(align_of::<GROUP_INFO_1002>(), 8);
    assert_eq!(size_of::<GROUP_INFO_1005>(), 4);
    assert_eq!(align_of::<GROUP_INFO_1005>(), 4);
    assert_eq!(size_of::<GROUP_USERS_INFO_0>(), 8);
    assert_eq!(align_of::<GROUP_USERS_INFO_0>(), 8);
    assert_eq!(size_of::<GROUP_USERS_INFO_1>(), 16);
    assert_eq!(align_of::<GROUP_USERS_INFO_1>(), 8);
    assert_eq!(size_of::<LOCALGROUP_INFO_0>(), 8);
    assert_eq!(align_of::<LOCALGROUP_INFO_0>(), 8);
    assert_eq!(size_of::<LOCALGROUP_INFO_1>(), 16);
    assert_eq!(align_of::<LOCALGROUP_INFO_1>(), 8);
    assert_eq!(size_of::<LOCALGROUP_INFO_1002>(), 8);
    assert_eq!(align_of::<LOCALGROUP_INFO_1002>(), 8);
    assert_eq!(size_of::<LOCALGROUP_MEMBERS_INFO_0>(), 8);
    assert_eq!(align_of::<LOCALGROUP_MEMBERS_INFO_0>(), 8);
    assert_eq!(size_of::<LOCALGROUP_MEMBERS_INFO_1>(), 24);
    assert_eq!(align_of::<LOCALGROUP_MEMBERS_INFO_1>(), 8);
    assert_eq!(size_of::<LOCALGROUP_MEMBERS_INFO_2>(), 24);
    assert_eq!(align_of::<LOCALGROUP_MEMBERS_INFO_2>(), 8);
    assert_eq!(size_of::<LOCALGROUP_MEMBERS_INFO_3>(), 8);
    assert_eq!(align_of::<LOCALGROUP_MEMBERS_INFO_3>(), 8);
    assert_eq!(size_of::<LOCALGROUP_USERS_INFO_0>(), 8);
    assert_eq!(align_of::<LOCALGROUP_USERS_INFO_0>(), 8);
    assert_eq!(size_of::<NET_DISPLAY_USER>(), 40);
    assert_eq!(align_of::<NET_DISPLAY_USER>(), 8);
    assert_eq!(size_of::<NET_DISPLAY_MACHINE>(), 32);
    assert_eq!(align_of::<NET_DISPLAY_MACHINE>(), 8);
    assert_eq!(size_of::<NET_DISPLAY_GROUP>(), 32);
    assert_eq!(align_of::<NET_DISPLAY_GROUP>(), 8);
    assert_eq!(size_of::<ACCESS_INFO_0>(), 8);
    assert_eq!(align_of::<ACCESS_INFO_0>(), 8);
    assert_eq!(size_of::<ACCESS_INFO_1>(), 16);
    assert_eq!(align_of::<ACCESS_INFO_1>(), 8);
    assert_eq!(size_of::<ACCESS_INFO_1002>(), 4);
    assert_eq!(align_of::<ACCESS_INFO_1002>(), 4);
    assert_eq!(size_of::<ACCESS_LIST>(), 16);
    assert_eq!(align_of::<ACCESS_LIST>(), 8);
    assert_eq!(size_of::<NET_VALIDATE_PASSWORD_HASH>(), 16);
    assert_eq!(align_of::<NET_VALIDATE_PASSWORD_HASH>(), 8);
    assert_eq!(size_of::<NET_VALIDATE_PERSISTED_FIELDS>(), 48);
    assert_eq!(align_of::<NET_VALIDATE_PERSISTED_FIELDS>(), 8);
    assert_eq!(size_of::<NET_VALIDATE_OUTPUT_ARG>(), 56);
    assert_eq!(align_of::<NET_VALIDATE_OUTPUT_ARG>(), 8);
    assert_eq!(size_of::<NET_VALIDATE_AUTHENTICATION_INPUT_ARG>(), 56);
    assert_eq!(align_of::<NET_VALIDATE_AUTHENTICATION_INPUT_ARG>(), 8);
    assert_eq!(size_of::<NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG>(), 88);
    assert_eq!(align_of::<NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG>(), 8);
    assert_eq!(size_of::<NET_VALIDATE_PASSWORD_RESET_INPUT_ARG>(), 88);
    assert_eq!(align_of::<NET_VALIDATE_PASSWORD_RESET_INPUT_ARG>(), 8);
    assert_eq!(size_of::<NETLOGON_INFO_1>(), 8);
    assert_eq!(align_of::<NETLOGON_INFO_1>(), 4);
    assert_eq!(size_of::<NETLOGON_INFO_2>(), 24);
    assert_eq!(align_of::<NETLOGON_INFO_2>(), 8);
    assert_eq!(size_of::<NETLOGON_INFO_3>(), 28);
    assert_eq!(align_of::<NETLOGON_INFO_3>(), 4);
    assert_eq!(size_of::<NETLOGON_INFO_4>(), 16);
    assert_eq!(align_of::<NETLOGON_INFO_4>(), 8);
    assert_eq!(size_of::<MSA_INFO_0>(), 4);
    assert_eq!(align_of::<MSA_INFO_0>(), 4);
}
#[cfg(feature = "lmalert")]
#[test]
fn um_lmalert() {
    use winapi::um::lmalert::*;
    assert_eq!(size_of::<STD_ALERT>(), 200);
    assert_eq!(align_of::<STD_ALERT>(), 4);
    assert_eq!(size_of::<ADMIN_OTHER_INFO>(), 8);
    assert_eq!(align_of::<ADMIN_OTHER_INFO>(), 4);
    assert_eq!(size_of::<ERRLOG_OTHER_INFO>(), 8);
    assert_eq!(align_of::<ERRLOG_OTHER_INFO>(), 4);
    assert_eq!(size_of::<PRINT_OTHER_INFO>(), 16);
    assert_eq!(align_of::<PRINT_OTHER_INFO>(), 4);
    assert_eq!(size_of::<USER_OTHER_INFO>(), 8);
    assert_eq!(align_of::<USER_OTHER_INFO>(), 4);
}
#[cfg(feature = "lmat")]
#[test]
fn um_lmat() {
    use winapi::um::lmat::*;
    assert_eq!(size_of::<AT_INFO>(), 24);
    assert_eq!(align_of::<AT_INFO>(), 8);
    assert_eq!(size_of::<AT_ENUM>(), 32);
    assert_eq!(align_of::<AT_ENUM>(), 8);
}
#[cfg(feature = "lmdfs")]
#[test]
fn um_lmdfs() {
    use winapi::um::lmdfs::*;
    assert_eq!(size_of::<DFS_TARGET_PRIORITY>(), 8);
    assert_eq!(align_of::<DFS_TARGET_PRIORITY>(), 4);
    assert_eq!(size_of::<DFS_INFO_1>(), 8);
    assert_eq!(align_of::<DFS_INFO_1>(), 8);
    assert_eq!(size_of::<DFS_INFO_1_32>(), 4);
    assert_eq!(align_of::<DFS_INFO_1_32>(), 4);
    assert_eq!(size_of::<DFS_INFO_2>(), 24);
    assert_eq!(align_of::<DFS_INFO_2>(), 8);
    assert_eq!(size_of::<DFS_INFO_2_32>(), 16);
    assert_eq!(align_of::<DFS_INFO_2_32>(), 4);
    assert_eq!(size_of::<DFS_STORAGE_INFO>(), 24);
    assert_eq!(align_of::<DFS_STORAGE_INFO>(), 8);
    assert_eq!(size_of::<DFS_STORAGE_INFO_0_32>(), 12);
    assert_eq!(align_of::<DFS_STORAGE_INFO_0_32>(), 4);
    assert_eq!(size_of::<DFS_STORAGE_INFO_1>(), 32);
    assert_eq!(align_of::<DFS_STORAGE_INFO_1>(), 8);
    assert_eq!(size_of::<DFS_INFO_3>(), 32);
    assert_eq!(align_of::<DFS_INFO_3>(), 8);
    assert_eq!(size_of::<DFS_INFO_3_32>(), 20);
    assert_eq!(align_of::<DFS_INFO_3_32>(), 4);
    assert_eq!(size_of::<DFS_INFO_4>(), 56);
    assert_eq!(align_of::<DFS_INFO_4>(), 8);
    assert_eq!(size_of::<DFS_INFO_4_32>(), 40);
    assert_eq!(align_of::<DFS_INFO_4_32>(), 4);
    assert_eq!(size_of::<DFS_INFO_5>(), 56);
    assert_eq!(align_of::<DFS_INFO_5>(), 8);
    assert_eq!(size_of::<DFS_INFO_6>(), 64);
    assert_eq!(align_of::<DFS_INFO_6>(), 8);
    assert_eq!(size_of::<DFS_INFO_7>(), 16);
    assert_eq!(align_of::<DFS_INFO_7>(), 4);
    assert_eq!(size_of::<DFS_INFO_8>(), 72);
    assert_eq!(align_of::<DFS_INFO_8>(), 8);
    assert_eq!(size_of::<DFS_INFO_9>(), 80);
    assert_eq!(align_of::<DFS_INFO_9>(), 8);
    assert_eq!(size_of::<DFS_INFO_50>(), 16);
    assert_eq!(align_of::<DFS_INFO_50>(), 8);
    assert_eq!(size_of::<DFS_INFO_100>(), 8);
    assert_eq!(align_of::<DFS_INFO_100>(), 8);
    assert_eq!(size_of::<DFS_INFO_101>(), 4);
    assert_eq!(align_of::<DFS_INFO_101>(), 4);
    assert_eq!(size_of::<DFS_INFO_102>(), 4);
    assert_eq!(align_of::<DFS_INFO_102>(), 4);
    assert_eq!(size_of::<DFS_INFO_103>(), 8);
    assert_eq!(align_of::<DFS_INFO_103>(), 4);
    assert_eq!(size_of::<DFS_INFO_104>(), 8);
    assert_eq!(align_of::<DFS_INFO_104>(), 4);
    assert_eq!(size_of::<DFS_INFO_105>(), 24);
    assert_eq!(align_of::<DFS_INFO_105>(), 8);
    assert_eq!(size_of::<DFS_INFO_106>(), 12);
    assert_eq!(align_of::<DFS_INFO_106>(), 4);
    assert_eq!(size_of::<DFS_INFO_107>(), 40);
    assert_eq!(align_of::<DFS_INFO_107>(), 8);
    assert_eq!(size_of::<DFS_INFO_150>(), 16);
    assert_eq!(align_of::<DFS_INFO_150>(), 8);
    assert_eq!(size_of::<DFS_INFO_200>(), 8);
    assert_eq!(align_of::<DFS_INFO_200>(), 8);
    assert_eq!(size_of::<DFS_INFO_300>(), 16);
    assert_eq!(align_of::<DFS_INFO_300>(), 8);
    assert_eq!(size_of::<DFS_SITENAME_INFO>(), 16);
    assert_eq!(align_of::<DFS_SITENAME_INFO>(), 8);
    assert_eq!(size_of::<DFS_SITELIST_INFO>(), 24);
    assert_eq!(align_of::<DFS_SITELIST_INFO>(), 8);
    assert_eq!(size_of::<DFS_SUPPORTED_NAMESPACE_VERSION_INFO>(), 32);
    assert_eq!(align_of::<DFS_SUPPORTED_NAMESPACE_VERSION_INFO>(), 8);
    assert_eq!(size_of::<DFS_GET_PKT_ENTRY_STATE_ARG>(), 16);
    assert_eq!(align_of::<DFS_GET_PKT_ENTRY_STATE_ARG>(), 4);
}
#[cfg(feature = "lmerrlog")]
#[test]
fn um_lmerrlog() {
    use winapi::um::lmerrlog::*;
    assert_eq!(size_of::<ERROR_LOG>(), 48);
    assert_eq!(align_of::<ERROR_LOG>(), 8);
    assert_eq!(size_of::<HLOG>(), 16);
    assert_eq!(align_of::<HLOG>(), 4);
}
#[cfg(feature = "lmjoin")]
#[test]
fn um_lmjoin() {
    use winapi::um::lmjoin::*;
    assert_eq!(size_of::<DSREG_USER_INFO>(), 24);
    assert_eq!(align_of::<DSREG_USER_INFO>(), 8);
    assert_eq!(size_of::<DSREG_JOIN_INFO>(), 96);
    assert_eq!(align_of::<DSREG_JOIN_INFO>(), 8);
    assert_eq!(size_of::<NETSETUP_PROVISIONING_PARAMS>(), 120);
    assert_eq!(align_of::<NETSETUP_PROVISIONING_PARAMS>(), 8);
}
#[cfg(feature = "lmmsg")]
#[test]
fn um_lmmsg() {
    use winapi::um::lmmsg::*;
    assert_eq!(size_of::<MSG_INFO_0>(), 8);
    assert_eq!(align_of::<MSG_INFO_0>(), 8);
    assert_eq!(size_of::<MSG_INFO_1>(), 24);
    assert_eq!(align_of::<MSG_INFO_1>(), 8);
}
#[cfg(feature = "lmremutl")]
#[test]
fn um_lmremutl() {
    use winapi::um::lmremutl::*;
    assert_eq!(size_of::<TIME_OF_DAY_INFO>(), 48);
    assert_eq!(align_of::<TIME_OF_DAY_INFO>(), 4);
}
#[cfg(feature = "lmrepl")]
#[test]
fn um_lmrepl() {
    use winapi::um::lmrepl::*;
    assert_eq!(size_of::<REPL_INFO_0>(), 64);
    assert_eq!(align_of::<REPL_INFO_0>(), 8);
    assert_eq!(size_of::<REPL_INFO_1000>(), 4);
    assert_eq!(align_of::<REPL_INFO_1000>(), 4);
    assert_eq!(size_of::<REPL_INFO_1001>(), 4);
    assert_eq!(align_of::<REPL_INFO_1001>(), 4);
    assert_eq!(size_of::<REPL_INFO_1002>(), 4);
    assert_eq!(align_of::<REPL_INFO_1002>(), 4);
    assert_eq!(size_of::<REPL_INFO_1003>(), 4);
    assert_eq!(align_of::<REPL_INFO_1003>(), 4);
    assert_eq!(size_of::<REPL_EDIR_INFO_0>(), 8);
    assert_eq!(align_of::<REPL_EDIR_INFO_0>(), 8);
    assert_eq!(size_of::<REPL_EDIR_INFO_1>(), 16);
    assert_eq!(align_of::<REPL_EDIR_INFO_1>(), 8);
    assert_eq!(size_of::<REPL_EDIR_INFO_2>(), 24);
    assert_eq!(align_of::<REPL_EDIR_INFO_2>(), 8);
    assert_eq!(size_of::<REPL_EDIR_INFO_1000>(), 4);
    assert_eq!(align_of::<REPL_EDIR_INFO_1000>(), 4);
    assert_eq!(size_of::<REPL_EDIR_INFO_1001>(), 4);
    assert_eq!(align_of::<REPL_EDIR_INFO_1001>(), 4);
    assert_eq!(size_of::<REPL_IDIR_INFO_0>(), 8);
    assert_eq!(align_of::<REPL_IDIR_INFO_0>(), 8);
    assert_eq!(size_of::<REPL_IDIR_INFO_1>(), 40);
    assert_eq!(align_of::<REPL_IDIR_INFO_1>(), 8);
}
#[cfg(feature = "lmserver")]
#[test]
fn um_lmserver() {
    use winapi::um::lmserver::*;
    assert_eq!(size_of::<SERVER_INFO_100>(), 16);
    assert_eq!(align_of::<SERVER_INFO_100>(), 8);
    assert_eq!(size_of::<SERVER_INFO_101>(), 40);
    assert_eq!(align_of::<SERVER_INFO_101>(), 8);
    assert_eq!(size_of::<SERVER_INFO_102>(), 72);
    assert_eq!(align_of::<SERVER_INFO_102>(), 8);
    assert_eq!(size_of::<SERVER_INFO_103>(), 80);
    assert_eq!(align_of::<SERVER_INFO_103>(), 8);
    assert_eq!(size_of::<SERVER_INFO_402>(), 144);
    assert_eq!(align_of::<SERVER_INFO_402>(), 8);
    assert_eq!(size_of::<SERVER_INFO_403>(), 160);
    assert_eq!(align_of::<SERVER_INFO_403>(), 8);
    assert_eq!(size_of::<SERVER_INFO_502>(), 72);
    assert_eq!(align_of::<SERVER_INFO_502>(), 4);
    assert_eq!(size_of::<SERVER_INFO_503>(), 176);
    assert_eq!(align_of::<SERVER_INFO_503>(), 8);
    assert_eq!(size_of::<SERVER_INFO_599>(), 232);
    assert_eq!(align_of::<SERVER_INFO_599>(), 8);
    assert_eq!(size_of::<SERVER_INFO_598>(), 184);
    assert_eq!(align_of::<SERVER_INFO_598>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1005>(), 8);
    assert_eq!(align_of::<SERVER_INFO_1005>(), 8);
    assert_eq!(size_of::<SERVER_INFO_1107>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1107>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1010>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1010>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1016>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1016>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1017>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1017>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1018>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1018>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1501>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1501>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1502>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1502>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1503>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1503>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1506>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1506>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1509>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1509>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1510>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1510>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1511>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1511>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1512>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1512>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1513>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1513>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1514>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1514>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1515>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1515>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1516>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1516>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1518>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1518>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1520>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1520>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1521>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1521>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1522>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1522>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1523>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1523>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1524>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1524>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1525>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1525>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1528>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1528>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1529>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1529>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1530>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1530>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1533>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1533>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1534>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1534>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1535>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1535>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1536>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1536>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1537>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1537>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1538>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1538>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1539>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1539>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1540>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1540>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1541>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1541>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1542>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1542>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1543>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1543>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1544>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1544>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1545>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1545>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1546>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1546>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1547>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1547>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1548>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1548>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1549>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1549>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1550>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1550>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1552>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1552>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1553>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1553>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1554>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1554>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1555>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1555>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1556>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1556>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1557>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1557>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1560>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1560>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1561>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1561>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1562>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1562>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1563>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1563>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1564>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1564>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1565>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1565>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1566>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1566>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1567>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1567>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1568>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1568>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1569>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1569>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1570>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1570>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1571>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1571>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1572>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1572>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1573>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1573>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1574>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1574>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1575>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1575>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1576>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1576>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1577>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1577>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1578>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1578>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1579>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1579>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1580>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1580>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1581>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1581>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1582>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1582>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1583>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1583>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1584>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1584>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1585>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1585>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1586>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1586>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1587>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1587>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1588>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1588>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1590>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1590>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1591>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1591>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1592>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1592>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1593>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1593>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1594>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1594>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1595>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1595>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1596>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1596>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1597>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1597>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1598>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1598>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1599>(), 1);
    assert_eq!(align_of::<SERVER_INFO_1599>(), 1);
    assert_eq!(size_of::<SERVER_INFO_1600>(), 1);
    assert_eq!(align_of::<SERVER_INFO_1600>(), 1);
    assert_eq!(size_of::<SERVER_INFO_1601>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1601>(), 4);
    assert_eq!(size_of::<SERVER_INFO_1602>(), 4);
    assert_eq!(align_of::<SERVER_INFO_1602>(), 4);
    assert_eq!(size_of::<SERVER_TRANSPORT_INFO_0>(), 40);
    assert_eq!(align_of::<SERVER_TRANSPORT_INFO_0>(), 8);
    assert_eq!(size_of::<SERVER_TRANSPORT_INFO_1>(), 48);
    assert_eq!(align_of::<SERVER_TRANSPORT_INFO_1>(), 8);
    assert_eq!(size_of::<SERVER_TRANSPORT_INFO_2>(), 56);
    assert_eq!(align_of::<SERVER_TRANSPORT_INFO_2>(), 8);
    assert_eq!(size_of::<SERVER_TRANSPORT_INFO_3>(), 312);
    assert_eq!(align_of::<SERVER_TRANSPORT_INFO_3>(), 8);
}
#[cfg(feature = "lmshare")]
#[test]
fn um_lmshare() {
    use winapi::um::lmshare::*;
    assert_eq!(size_of::<SHARE_INFO_0>(), 8);
    assert_eq!(align_of::<SHARE_INFO_0>(), 8);
    assert_eq!(size_of::<SHARE_INFO_1>(), 24);
    assert_eq!(align_of::<SHARE_INFO_1>(), 8);
    assert_eq!(size_of::<SHARE_INFO_2>(), 56);
    assert_eq!(align_of::<SHARE_INFO_2>(), 8);
    assert_eq!(size_of::<SHARE_INFO_501>(), 32);
    assert_eq!(align_of::<SHARE_INFO_501>(), 8);
    assert_eq!(size_of::<SHARE_INFO_502>(), 72);
    assert_eq!(align_of::<SHARE_INFO_502>(), 8);
    assert_eq!(size_of::<SHARE_INFO_503>(), 80);
    assert_eq!(align_of::<SHARE_INFO_503>(), 8);
    assert_eq!(size_of::<SHARE_INFO_1004>(), 8);
    assert_eq!(align_of::<SHARE_INFO_1004>(), 8);
    assert_eq!(size_of::<SHARE_INFO_1005>(), 4);
    assert_eq!(align_of::<SHARE_INFO_1005>(), 4);
    assert_eq!(size_of::<SHARE_INFO_1006>(), 4);
    assert_eq!(align_of::<SHARE_INFO_1006>(), 4);
    assert_eq!(size_of::<SHARE_INFO_1501>(), 16);
    assert_eq!(align_of::<SHARE_INFO_1501>(), 8);
    assert_eq!(size_of::<SHARE_INFO_1503>(), 16);
    assert_eq!(align_of::<SHARE_INFO_1503>(), 4);
    assert_eq!(size_of::<SERVER_ALIAS_INFO_0>(), 24);
    assert_eq!(align_of::<SERVER_ALIAS_INFO_0>(), 8);
    assert_eq!(size_of::<SESSION_INFO_0>(), 8);
    assert_eq!(align_of::<SESSION_INFO_0>(), 8);
    assert_eq!(size_of::<SESSION_INFO_1>(), 32);
    assert_eq!(align_of::<SESSION_INFO_1>(), 8);
    assert_eq!(size_of::<SESSION_INFO_2>(), 40);
    assert_eq!(align_of::<SESSION_INFO_2>(), 8);
    assert_eq!(size_of::<SESSION_INFO_10>(), 24);
    assert_eq!(align_of::<SESSION_INFO_10>(), 8);
    assert_eq!(size_of::<SESSION_INFO_502>(), 48);
    assert_eq!(align_of::<SESSION_INFO_502>(), 8);
    assert_eq!(size_of::<CONNECTION_INFO_0>(), 4);
    assert_eq!(align_of::<CONNECTION_INFO_0>(), 4);
    assert_eq!(size_of::<CONNECTION_INFO_1>(), 40);
    assert_eq!(align_of::<CONNECTION_INFO_1>(), 8);
    assert_eq!(size_of::<FILE_INFO_2>(), 4);
    assert_eq!(align_of::<FILE_INFO_2>(), 4);
    assert_eq!(size_of::<FILE_INFO_3>(), 32);
    assert_eq!(align_of::<FILE_INFO_3>(), 8);
}
#[cfg(feature = "lmstats")]
#[test]
fn um_lmstats() {
    use winapi::um::lmstats::*;
    assert_eq!(size_of::<STAT_WORKSTATION_0>(), 216);
    assert_eq!(align_of::<STAT_WORKSTATION_0>(), 8);
    assert_eq!(size_of::<STAT_SERVER_0>(), 68);
    assert_eq!(align_of::<STAT_SERVER_0>(), 4);
}
#[cfg(feature = "lmsvc")]
#[test]
fn um_lmsvc() {
    use winapi::um::lmsvc::*;
    assert_eq!(size_of::<SERVICE_INFO_0>(), 8);
    assert_eq!(align_of::<SERVICE_INFO_0>(), 8);
    assert_eq!(size_of::<SERVICE_INFO_1>(), 24);
    assert_eq!(align_of::<SERVICE_INFO_1>(), 8);
    assert_eq!(size_of::<SERVICE_INFO_2>(), 48);
    assert_eq!(align_of::<SERVICE_INFO_2>(), 8);
}
#[cfg(feature = "lmuse")]
#[test]
fn um_lmuse() {
    use winapi::um::lmuse::*;
    assert_eq!(size_of::<USE_INFO_0>(), 16);
    assert_eq!(align_of::<USE_INFO_0>(), 8);
    assert_eq!(size_of::<USE_INFO_1>(), 40);
    assert_eq!(align_of::<USE_INFO_1>(), 8);
    assert_eq!(size_of::<USE_INFO_2>(), 56);
    assert_eq!(align_of::<USE_INFO_2>(), 8);
    assert_eq!(size_of::<USE_INFO_3>(), 64);
    assert_eq!(align_of::<USE_INFO_3>(), 8);
    assert_eq!(size_of::<USE_INFO_4>(), 80);
    assert_eq!(align_of::<USE_INFO_4>(), 8);
}
#[cfg(feature = "lmwksta")]
#[test]
fn um_lmwksta() {
    use winapi::um::lmwksta::*;
    assert_eq!(size_of::<WKSTA_INFO_100>(), 32);
    assert_eq!(align_of::<WKSTA_INFO_100>(), 8);
    assert_eq!(size_of::<WKSTA_INFO_101>(), 40);
    assert_eq!(align_of::<WKSTA_INFO_101>(), 8);
    assert_eq!(size_of::<WKSTA_INFO_102>(), 48);
    assert_eq!(align_of::<WKSTA_INFO_102>(), 8);
    assert_eq!(size_of::<WKSTA_INFO_302>(), 88);
    assert_eq!(align_of::<WKSTA_INFO_302>(), 8);
    assert_eq!(size_of::<WKSTA_INFO_402>(), 96);
    assert_eq!(align_of::<WKSTA_INFO_402>(), 8);
    assert_eq!(size_of::<WKSTA_INFO_502>(), 140);
    assert_eq!(align_of::<WKSTA_INFO_502>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1010>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1010>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1011>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1011>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1012>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1012>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1027>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1027>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1028>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1028>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1032>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1032>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1013>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1013>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1018>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1018>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1023>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1023>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1033>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1033>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1041>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1041>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1042>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1042>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1043>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1043>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1044>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1044>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1045>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1045>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1046>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1046>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1047>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1047>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1048>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1048>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1049>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1049>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1050>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1050>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1051>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1051>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1052>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1052>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1053>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1053>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1054>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1054>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1055>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1055>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1056>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1056>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1057>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1057>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1058>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1058>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1059>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1059>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1060>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1060>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1061>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1061>(), 4);
    assert_eq!(size_of::<WKSTA_INFO_1062>(), 4);
    assert_eq!(align_of::<WKSTA_INFO_1062>(), 4);
    assert_eq!(size_of::<WKSTA_USER_INFO_0>(), 8);
    assert_eq!(align_of::<WKSTA_USER_INFO_0>(), 8);
    assert_eq!(size_of::<WKSTA_USER_INFO_1>(), 32);
    assert_eq!(align_of::<WKSTA_USER_INFO_1>(), 8);
    assert_eq!(size_of::<WKSTA_USER_INFO_1101>(), 8);
    assert_eq!(align_of::<WKSTA_USER_INFO_1101>(), 8);
    assert_eq!(size_of::<WKSTA_TRANSPORT_INFO_0>(), 32);
    assert_eq!(align_of::<WKSTA_TRANSPORT_INFO_0>(), 8);
}
#[cfg(feature = "lowlevelmonitorconfigurationapi")]
#[test]
fn um_lowlevelmonitorconfigurationapi() {
    use winapi::um::lowlevelmonitorconfigurationapi::*;
    assert_eq!(size_of::<MC_TIMING_REPORT>(), 9);
    assert_eq!(align_of::<MC_TIMING_REPORT>(), 1);
}
#[cfg(feature = "lsalookup")]
#[test]
fn um_lsalookup() {
    use winapi::um::lsalookup::*;
    assert_eq!(size_of::<LSA_UNICODE_STRING>(), 16);
    assert_eq!(align_of::<LSA_UNICODE_STRING>(), 8);
    assert_eq!(size_of::<LSA_STRING>(), 16);
    assert_eq!(align_of::<LSA_STRING>(), 8);
    assert_eq!(size_of::<LSA_OBJECT_ATTRIBUTES>(), 48);
    assert_eq!(align_of::<LSA_OBJECT_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<LSA_TRUST_INFORMATION>(), 24);
    assert_eq!(align_of::<LSA_TRUST_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_REFERENCED_DOMAIN_LIST>(), 16);
    assert_eq!(align_of::<LSA_REFERENCED_DOMAIN_LIST>(), 8);
    assert_eq!(size_of::<LSA_TRANSLATED_SID2>(), 24);
    assert_eq!(align_of::<LSA_TRANSLATED_SID2>(), 8);
    assert_eq!(size_of::<LSA_TRANSLATED_NAME>(), 32);
    assert_eq!(align_of::<LSA_TRANSLATED_NAME>(), 8);
    assert_eq!(size_of::<POLICY_ACCOUNT_DOMAIN_INFO>(), 24);
    assert_eq!(align_of::<POLICY_ACCOUNT_DOMAIN_INFO>(), 8);
    assert_eq!(size_of::<POLICY_DNS_DOMAIN_INFO>(), 72);
    assert_eq!(align_of::<POLICY_DNS_DOMAIN_INFO>(), 8);
}
#[cfg(feature = "memoryapi")]
#[test]
fn um_memoryapi() {
    use winapi::um::memoryapi::*;
    assert_eq!(size_of::<WIN32_MEMORY_RANGE_ENTRY>(), 16);
    assert_eq!(align_of::<WIN32_MEMORY_RANGE_ENTRY>(), 8);
    assert_eq!(size_of::<WIN32_MEMORY_REGION_INFORMATION>(), 32);
    assert_eq!(align_of::<WIN32_MEMORY_REGION_INFORMATION>(), 8);
    assert_eq!(size_of::<WIN32_MEMORY_REGION_INFORMATION_u>(), 4);
    assert_eq!(align_of::<WIN32_MEMORY_REGION_INFORMATION_u>(), 4);
    assert_eq!(size_of::<WIN32_MEMORY_REGION_INFORMATION_u_s>(), 4);
    assert_eq!(align_of::<WIN32_MEMORY_REGION_INFORMATION_u_s>(), 4);
}
#[cfg(feature = "minschannel")]
#[test]
fn um_minschannel() {
    use winapi::um::minschannel::*;
    assert_eq!(size_of::<SecPkgCred_SupportedAlgs>(), 16);
    assert_eq!(align_of::<SecPkgCred_SupportedAlgs>(), 8);
    assert_eq!(size_of::<SecPkgCred_CipherStrengths>(), 8);
    assert_eq!(align_of::<SecPkgCred_CipherStrengths>(), 4);
    assert_eq!(size_of::<SecPkgCred_SupportedProtocols>(), 4);
    assert_eq!(align_of::<SecPkgCred_SupportedProtocols>(), 4);
    assert_eq!(size_of::<SecPkgCred_ClientCertPolicy>(), 56);
    assert_eq!(align_of::<SecPkgCred_ClientCertPolicy>(), 8);
}
#[cfg(feature = "minwinbase")]
#[test]
fn um_minwinbase() {
    use winapi::um::minwinbase::*;
    assert_eq!(size_of::<SECURITY_ATTRIBUTES>(), 24);
    assert_eq!(align_of::<SECURITY_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<OVERLAPPED_u_s>(), 8);
    assert_eq!(align_of::<OVERLAPPED_u_s>(), 4);
    assert_eq!(size_of::<OVERLAPPED_u>(), 8);
    assert_eq!(align_of::<OVERLAPPED_u>(), 8);
    assert_eq!(size_of::<OVERLAPPED>(), 32);
    assert_eq!(align_of::<OVERLAPPED>(), 8);
    assert_eq!(size_of::<OVERLAPPED_ENTRY>(), 32);
    assert_eq!(align_of::<OVERLAPPED_ENTRY>(), 8);
    assert_eq!(size_of::<SYSTEMTIME>(), 16);
    assert_eq!(align_of::<SYSTEMTIME>(), 2);
    assert_eq!(size_of::<WIN32_FIND_DATAA>(), 320);
    assert_eq!(align_of::<WIN32_FIND_DATAA>(), 4);
    assert_eq!(size_of::<WIN32_FIND_DATAW>(), 592);
    assert_eq!(align_of::<WIN32_FIND_DATAW>(), 4);
    assert_eq!(size_of::<PROCESS_HEAP_ENTRY_Block>(), 24);
    assert_eq!(align_of::<PROCESS_HEAP_ENTRY_Block>(), 8);
    assert_eq!(size_of::<PROCESS_HEAP_ENTRY_Region>(), 24);
    assert_eq!(align_of::<PROCESS_HEAP_ENTRY_Region>(), 8);
    assert_eq!(size_of::<PROCESS_HEAP_ENTRY_u>(), 24);
    assert_eq!(align_of::<PROCESS_HEAP_ENTRY_u>(), 8);
    assert_eq!(size_of::<PROCESS_HEAP_ENTRY>(), 40);
    assert_eq!(align_of::<PROCESS_HEAP_ENTRY>(), 8);
    assert_eq!(size_of::<REASON_CONTEXT_Detailed>(), 24);
    assert_eq!(align_of::<REASON_CONTEXT_Detailed>(), 8);
    assert_eq!(size_of::<REASON_CONTEXT_Reason>(), 24);
    assert_eq!(align_of::<REASON_CONTEXT_Reason>(), 8);
    assert_eq!(size_of::<REASON_CONTEXT>(), 32);
    assert_eq!(align_of::<REASON_CONTEXT>(), 8);
    assert_eq!(size_of::<EXCEPTION_DEBUG_INFO>(), 160);
    assert_eq!(align_of::<EXCEPTION_DEBUG_INFO>(), 8);
    assert_eq!(size_of::<CREATE_THREAD_DEBUG_INFO>(), 24);
    assert_eq!(align_of::<CREATE_THREAD_DEBUG_INFO>(), 8);
    assert_eq!(size_of::<CREATE_PROCESS_DEBUG_INFO>(), 72);
    assert_eq!(align_of::<CREATE_PROCESS_DEBUG_INFO>(), 8);
    assert_eq!(size_of::<EXIT_THREAD_DEBUG_INFO>(), 4);
    assert_eq!(align_of::<EXIT_THREAD_DEBUG_INFO>(), 4);
    assert_eq!(size_of::<EXIT_PROCESS_DEBUG_INFO>(), 4);
    assert_eq!(align_of::<EXIT_PROCESS_DEBUG_INFO>(), 4);
    assert_eq!(size_of::<LOAD_DLL_DEBUG_INFO>(), 40);
    assert_eq!(align_of::<LOAD_DLL_DEBUG_INFO>(), 8);
    assert_eq!(size_of::<UNLOAD_DLL_DEBUG_INFO>(), 8);
    assert_eq!(align_of::<UNLOAD_DLL_DEBUG_INFO>(), 8);
    assert_eq!(size_of::<OUTPUT_DEBUG_STRING_INFO>(), 16);
    assert_eq!(align_of::<OUTPUT_DEBUG_STRING_INFO>(), 8);
    assert_eq!(size_of::<RIP_INFO>(), 8);
    assert_eq!(align_of::<RIP_INFO>(), 4);
    assert_eq!(size_of::<DEBUG_EVENT_u>(), 160);
    assert_eq!(align_of::<DEBUG_EVENT_u>(), 8);
    assert_eq!(size_of::<DEBUG_EVENT>(), 176);
    assert_eq!(align_of::<DEBUG_EVENT>(), 8);
}
#[cfg(feature = "mmdeviceapi")]
#[test]
fn um_mmdeviceapi() {
    use winapi::um::mmdeviceapi::*;
    assert_eq!(size_of::<DIRECTX_AUDIO_ACTIVATION_PARAMS>(), 24);
    assert_eq!(align_of::<DIRECTX_AUDIO_ACTIVATION_PARAMS>(), 4);
    assert_eq!(size_of::<AudioExtensionParams>(), 32);
    assert_eq!(align_of::<AudioExtensionParams>(), 8);
}
#[cfg(feature = "mmsystem")]
#[test]
fn um_mmsystem() {
    use winapi::um::mmsystem::*;
    assert_eq!(size_of::<MMTIME_smpte>(), 8);
    assert_eq!(align_of::<MMTIME_smpte>(), 1);
    assert_eq!(size_of::<MMTIME_midi>(), 4);
    assert_eq!(align_of::<MMTIME_midi>(), 1);
    assert_eq!(size_of::<MMTIME_u>(), 8);
    assert_eq!(align_of::<MMTIME_u>(), 1);
    assert_eq!(size_of::<MMTIME>(), 12);
    assert_eq!(align_of::<MMTIME>(), 1);
    assert_eq!(size_of::<WAVEHDR>(), 48);
    assert_eq!(align_of::<WAVEHDR>(), 1);
    assert_eq!(size_of::<WAVEOUTCAPSW>(), 84);
    assert_eq!(align_of::<WAVEOUTCAPSW>(), 1);
    assert_eq!(size_of::<WAVEINCAPSW>(), 80);
    assert_eq!(align_of::<WAVEINCAPSW>(), 1);
    assert_eq!(size_of::<TIMECAPS>(), 8);
    assert_eq!(align_of::<TIMECAPS>(), 1);
    assert_eq!(size_of::<MIDIHDR>(), 112);
    assert_eq!(align_of::<MIDIHDR>(), 1);
    assert_eq!(size_of::<MIDIINCAPSW>(), 76);
    assert_eq!(align_of::<MIDIINCAPSW>(), 1);
    assert_eq!(size_of::<MIDIOUTCAPSW>(), 84);
    assert_eq!(align_of::<MIDIOUTCAPSW>(), 1);
}
#[cfg(feature = "mscat")]
#[test]
fn um_mscat() {
    use winapi::um::mscat::*;
    assert_eq!(size_of::<CRYPTCATSTORE>(), 64);
    assert_eq!(align_of::<CRYPTCATSTORE>(), 8);
    assert_eq!(size_of::<CRYPTCATMEMBER>(), 104);
    assert_eq!(align_of::<CRYPTCATMEMBER>(), 8);
}
#[cfg(feature = "mschapp")]
#[test]
fn um_mschapp() {
    use winapi::um::mschapp::*;
    assert_eq!(size_of::<CYPHER_BLOCK>(), 8);
    assert_eq!(align_of::<CYPHER_BLOCK>(), 1);
    assert_eq!(size_of::<LM_OWF_PASSWORD>(), 16);
    assert_eq!(align_of::<LM_OWF_PASSWORD>(), 1);
    assert_eq!(size_of::<SAMPR_ENCRYPTED_USER_PASSWORD>(), 516);
    assert_eq!(align_of::<SAMPR_ENCRYPTED_USER_PASSWORD>(), 1);
    assert_eq!(size_of::<ENCRYPTED_LM_OWF_PASSWORD>(), 16);
    assert_eq!(align_of::<ENCRYPTED_LM_OWF_PASSWORD>(), 1);
}
#[cfg(feature = "mssip")]
#[test]
fn um_mssip() {
    use winapi::um::mssip::*;
    assert_eq!(size_of::<SIP_SUBJECTINFO>(), 128);
    assert_eq!(align_of::<SIP_SUBJECTINFO>(), 8);
    assert_eq!(size_of::<MS_ADDINFO_FLAT>(), 16);
    assert_eq!(align_of::<MS_ADDINFO_FLAT>(), 8);
    assert_eq!(size_of::<MS_ADDINFO_CATALOGMEMBER>(), 24);
    assert_eq!(align_of::<MS_ADDINFO_CATALOGMEMBER>(), 8);
    assert_eq!(size_of::<MS_ADDINFO_BLOB>(), 32);
    assert_eq!(align_of::<MS_ADDINFO_BLOB>(), 8);
    assert_eq!(size_of::<SIP_CAP_SET_V2>(), 16);
    assert_eq!(align_of::<SIP_CAP_SET_V2>(), 4);
    assert_eq!(size_of::<SIP_CAP_SET_V3>(), 16);
    assert_eq!(align_of::<SIP_CAP_SET_V3>(), 4);
    assert_eq!(size_of::<SIP_INDIRECT_DATA>(), 64);
    assert_eq!(align_of::<SIP_INDIRECT_DATA>(), 8);
    assert_eq!(size_of::<SIP_DISPATCH_INFO>(), 56);
    assert_eq!(align_of::<SIP_DISPATCH_INFO>(), 8);
    assert_eq!(size_of::<SIP_ADD_NEWPROVIDER>(), 96);
    assert_eq!(align_of::<SIP_ADD_NEWPROVIDER>(), 8);
}
#[cfg(feature = "nb30")]
#[test]
fn um_nb30() {
    use winapi::um::nb30::*;
    assert_eq!(size_of::<NCB>(), 96);
    assert_eq!(align_of::<NCB>(), 8);
    assert_eq!(size_of::<NCB>(), 96);
    assert_eq!(align_of::<NCB>(), 8);
    assert_eq!(size_of::<ADAPTER_STATUS>(), 60);
    assert_eq!(align_of::<ADAPTER_STATUS>(), 4);
    assert_eq!(size_of::<NAME_BUFFER>(), 18);
    assert_eq!(align_of::<NAME_BUFFER>(), 1);
    assert_eq!(size_of::<SESSION_HEADER>(), 4);
    assert_eq!(align_of::<SESSION_HEADER>(), 1);
    assert_eq!(size_of::<SESSION_BUFFER>(), 36);
    assert_eq!(align_of::<SESSION_BUFFER>(), 1);
    assert_eq!(size_of::<LANA_ENUM>(), 256);
    assert_eq!(align_of::<LANA_ENUM>(), 1);
    assert_eq!(size_of::<FIND_NAME_HEADER>(), 4);
    assert_eq!(align_of::<FIND_NAME_HEADER>(), 2);
    assert_eq!(size_of::<FIND_NAME_BUFFER>(), 33);
    assert_eq!(align_of::<FIND_NAME_BUFFER>(), 1);
    assert_eq!(size_of::<ACTION_HEADER>(), 8);
    assert_eq!(align_of::<ACTION_HEADER>(), 4);
}
#[cfg(feature = "ntlsa")]
#[test]
fn um_ntlsa() {
    use winapi::um::ntlsa::*;
    assert_eq!(size_of::<SE_ADT_OBJECT_TYPE>(), 24);
    assert_eq!(align_of::<SE_ADT_OBJECT_TYPE>(), 4);
    assert_eq!(size_of::<SE_ADT_PARAMETER_ARRAY_ENTRY>(), 32);
    assert_eq!(align_of::<SE_ADT_PARAMETER_ARRAY_ENTRY>(), 8);
    assert_eq!(size_of::<SE_ADT_ACCESS_REASON>(), 152);
    assert_eq!(align_of::<SE_ADT_ACCESS_REASON>(), 8);
    assert_eq!(size_of::<SE_ADT_CLAIMS>(), 16);
    assert_eq!(align_of::<SE_ADT_CLAIMS>(), 8);
    assert_eq!(size_of::<SE_ADT_PARAMETER_ARRAY>(), 1048);
    assert_eq!(align_of::<SE_ADT_PARAMETER_ARRAY>(), 8);
    assert_eq!(size_of::<SE_ADT_PARAMETER_ARRAY_EX>(), 1056);
    assert_eq!(align_of::<SE_ADT_PARAMETER_ARRAY_EX>(), 8);
    assert_eq!(size_of::<LSA_ADT_STRING_LIST_ENTRY>(), 24);
    assert_eq!(align_of::<LSA_ADT_STRING_LIST_ENTRY>(), 8);
    assert_eq!(size_of::<LSA_ADT_STRING_LIST>(), 16);
    assert_eq!(align_of::<LSA_ADT_STRING_LIST>(), 8);
    assert_eq!(size_of::<LSA_ADT_SID_LIST_ENTRY>(), 16);
    assert_eq!(align_of::<LSA_ADT_SID_LIST_ENTRY>(), 8);
    assert_eq!(size_of::<LSA_ADT_SID_LIST>(), 16);
    assert_eq!(align_of::<LSA_ADT_SID_LIST>(), 8);
    assert_eq!(size_of::<LSA_AUTH_CALLBACKS>(), 48);
    assert_eq!(align_of::<LSA_AUTH_CALLBACKS>(), 8);
    assert_eq!(size_of::<LSA_TOKEN_INFORMATION_NULL>(), 16);
    assert_eq!(align_of::<LSA_TOKEN_INFORMATION_NULL>(), 8);
    assert_eq!(size_of::<LSA_TOKEN_INFORMATION_V1>(), 64);
    assert_eq!(align_of::<LSA_TOKEN_INFORMATION_V1>(), 8);
    assert_eq!(size_of::<LSA_TOKEN_INFORMATION_V3>(), 88);
    assert_eq!(align_of::<LSA_TOKEN_INFORMATION_V3>(), 8);
    assert_eq!(size_of::<LSA_DISPATCH_TABLE>(), 88);
    assert_eq!(align_of::<LSA_DISPATCH_TABLE>(), 8);
    assert_eq!(size_of::<LSA_TRANSLATED_SID>(), 12);
    assert_eq!(align_of::<LSA_TRANSLATED_SID>(), 4);
    assert_eq!(size_of::<POLICY_PRIVILEGE_DEFINITION>(), 24);
    assert_eq!(align_of::<POLICY_PRIVILEGE_DEFINITION>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_LOG_INFO>(), 40);
    assert_eq!(align_of::<POLICY_AUDIT_LOG_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_EVENTS_INFO>(), 24);
    assert_eq!(align_of::<POLICY_AUDIT_EVENTS_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_SUBCATEGORIES_INFO>(), 16);
    assert_eq!(align_of::<POLICY_AUDIT_SUBCATEGORIES_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_CATEGORIES_INFO>(), 16);
    assert_eq!(align_of::<POLICY_AUDIT_CATEGORIES_INFO>(), 8);
    assert_eq!(size_of::<POLICY_PRIMARY_DOMAIN_INFO>(), 24);
    assert_eq!(align_of::<POLICY_PRIMARY_DOMAIN_INFO>(), 8);
    assert_eq!(size_of::<POLICY_PD_ACCOUNT_INFO>(), 16);
    assert_eq!(align_of::<POLICY_PD_ACCOUNT_INFO>(), 8);
    assert_eq!(size_of::<POLICY_LSA_SERVER_ROLE_INFO>(), 4);
    assert_eq!(align_of::<POLICY_LSA_SERVER_ROLE_INFO>(), 4);
    assert_eq!(size_of::<POLICY_REPLICA_SOURCE_INFO>(), 32);
    assert_eq!(align_of::<POLICY_REPLICA_SOURCE_INFO>(), 8);
    assert_eq!(size_of::<POLICY_DEFAULT_QUOTA_INFO>(), 48);
    assert_eq!(align_of::<POLICY_DEFAULT_QUOTA_INFO>(), 8);
    assert_eq!(size_of::<POLICY_MODIFICATION_INFO>(), 16);
    assert_eq!(align_of::<POLICY_MODIFICATION_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_FULL_SET_INFO>(), 1);
    assert_eq!(align_of::<POLICY_AUDIT_FULL_SET_INFO>(), 1);
    assert_eq!(size_of::<POLICY_AUDIT_FULL_QUERY_INFO>(), 2);
    assert_eq!(align_of::<POLICY_AUDIT_FULL_QUERY_INFO>(), 1);
    assert_eq!(size_of::<POLICY_DOMAIN_EFS_INFO>(), 16);
    assert_eq!(align_of::<POLICY_DOMAIN_EFS_INFO>(), 8);
    assert_eq!(size_of::<POLICY_DOMAIN_KERBEROS_TICKET_INFO>(), 48);
    assert_eq!(align_of::<POLICY_DOMAIN_KERBEROS_TICKET_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_NAME_INFO>(), 16);
    assert_eq!(align_of::<TRUSTED_DOMAIN_NAME_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_CONTROLLERS_INFO>(), 16);
    assert_eq!(align_of::<TRUSTED_CONTROLLERS_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_POSIX_OFFSET_INFO>(), 4);
    assert_eq!(align_of::<TRUSTED_POSIX_OFFSET_INFO>(), 4);
    assert_eq!(size_of::<TRUSTED_PASSWORD_INFO>(), 32);
    assert_eq!(align_of::<TRUSTED_PASSWORD_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_INFORMATION_EX>(), 56);
    assert_eq!(align_of::<TRUSTED_DOMAIN_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_INFORMATION_EX2>(), 64);
    assert_eq!(align_of::<TRUSTED_DOMAIN_INFORMATION_EX2>(), 8);
    assert_eq!(size_of::<LSA_AUTH_INFORMATION>(), 24);
    assert_eq!(align_of::<LSA_AUTH_INFORMATION>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_AUTH_INFORMATION>(), 48);
    assert_eq!(align_of::<TRUSTED_DOMAIN_AUTH_INFORMATION>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_FULL_INFORMATION>(), 112);
    assert_eq!(align_of::<TRUSTED_DOMAIN_FULL_INFORMATION>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_FULL_INFORMATION2>(), 120);
    assert_eq!(align_of::<TRUSTED_DOMAIN_FULL_INFORMATION2>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES>(), 4);
    assert_eq!(align_of::<TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES>(), 4);
    assert_eq!(size_of::<LSA_FOREST_TRUST_DOMAIN_INFO>(), 40);
    assert_eq!(align_of::<LSA_FOREST_TRUST_DOMAIN_INFO>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_BINARY_DATA>(), 16);
    assert_eq!(align_of::<LSA_FOREST_TRUST_BINARY_DATA>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_RECORD_FORESTTRUSTDATA>(), 40);
    assert_eq!(align_of::<LSA_FOREST_TRUST_RECORD_FORESTTRUSTDATA>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_RECORD>(), 56);
    assert_eq!(align_of::<LSA_FOREST_TRUST_RECORD>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_INFORMATION>(), 16);
    assert_eq!(align_of::<LSA_FOREST_TRUST_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_COLLISION_RECORD>(), 32);
    assert_eq!(align_of::<LSA_FOREST_TRUST_COLLISION_RECORD>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_COLLISION_INFORMATION>(), 16);
    assert_eq!(align_of::<LSA_FOREST_TRUST_COLLISION_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_ENUMERATION_INFORMATION>(), 8);
    assert_eq!(align_of::<LSA_ENUMERATION_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_LAST_INTER_LOGON_INFO>(), 24);
    assert_eq!(align_of::<LSA_LAST_INTER_LOGON_INFO>(), 8);
    assert_eq!(size_of::<SECURITY_LOGON_SESSION_DATA>(), 272);
    assert_eq!(align_of::<SECURITY_LOGON_SESSION_DATA>(), 8);
    assert_eq!(size_of::<CENTRAL_ACCESS_POLICY_ENTRY>(), 104);
    assert_eq!(align_of::<CENTRAL_ACCESS_POLICY_ENTRY>(), 8);
    assert_eq!(size_of::<CENTRAL_ACCESS_POLICY>(), 72);
    assert_eq!(align_of::<CENTRAL_ACCESS_POLICY>(), 8);
    assert_eq!(size_of::<NEGOTIATE_PACKAGE_PREFIX>(), 64);
    assert_eq!(align_of::<NEGOTIATE_PACKAGE_PREFIX>(), 8);
    assert_eq!(size_of::<NEGOTIATE_PACKAGE_PREFIXES>(), 16);
    assert_eq!(align_of::<NEGOTIATE_PACKAGE_PREFIXES>(), 4);
    assert_eq!(size_of::<NEGOTIATE_CALLER_NAME_REQUEST>(), 12);
    assert_eq!(align_of::<NEGOTIATE_CALLER_NAME_REQUEST>(), 4);
    assert_eq!(size_of::<NEGOTIATE_CALLER_NAME_RESPONSE>(), 16);
    assert_eq!(align_of::<NEGOTIATE_CALLER_NAME_RESPONSE>(), 8);
    assert_eq!(size_of::<NEGOTIATE_PACKAGE_NAMES>(), 24);
    assert_eq!(align_of::<NEGOTIATE_PACKAGE_NAMES>(), 8);
    assert_eq!(size_of::<NEGOTIATE_PACKAGE_PREFIX_WOW>(), 48);
    assert_eq!(align_of::<NEGOTIATE_PACKAGE_PREFIX_WOW>(), 4);
    assert_eq!(size_of::<NEGOTIATE_CALLER_NAME_RESPONSE_WOW>(), 8);
    assert_eq!(align_of::<NEGOTIATE_CALLER_NAME_RESPONSE_WOW>(), 4);
    assert_eq!(size_of::<LSA_USER_REGISTRATION_INFO>(), 72);
    assert_eq!(align_of::<LSA_USER_REGISTRATION_INFO>(), 8);
    assert_eq!(size_of::<LSA_REGISTRATION_INFO>(), 16);
    assert_eq!(align_of::<LSA_REGISTRATION_INFO>(), 8);
}
#[cfg(feature = "ntsecapi")]
#[test]
fn um_ntsecapi() {
    use winapi::um::ntsecapi::*;
    assert_eq!(size_of::<LSA_TRANSLATED_SID>(), 12);
    assert_eq!(align_of::<LSA_TRANSLATED_SID>(), 4);
    assert_eq!(size_of::<POLICY_AUDIT_LOG_INFO>(), 40);
    assert_eq!(align_of::<POLICY_AUDIT_LOG_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_EVENTS_INFO>(), 24);
    assert_eq!(align_of::<POLICY_AUDIT_EVENTS_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_SUBCATEGORIES_INFO>(), 16);
    assert_eq!(align_of::<POLICY_AUDIT_SUBCATEGORIES_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_CATEGORIES_INFO>(), 16);
    assert_eq!(align_of::<POLICY_AUDIT_CATEGORIES_INFO>(), 8);
    assert_eq!(size_of::<POLICY_PRIMARY_DOMAIN_INFO>(), 24);
    assert_eq!(align_of::<POLICY_PRIMARY_DOMAIN_INFO>(), 8);
    assert_eq!(size_of::<POLICY_PD_ACCOUNT_INFO>(), 16);
    assert_eq!(align_of::<POLICY_PD_ACCOUNT_INFO>(), 8);
    assert_eq!(size_of::<POLICY_LSA_SERVER_ROLE_INFO>(), 4);
    assert_eq!(align_of::<POLICY_LSA_SERVER_ROLE_INFO>(), 4);
    assert_eq!(size_of::<POLICY_REPLICA_SOURCE_INFO>(), 32);
    assert_eq!(align_of::<POLICY_REPLICA_SOURCE_INFO>(), 8);
    assert_eq!(size_of::<POLICY_DEFAULT_QUOTA_INFO>(), 48);
    assert_eq!(align_of::<POLICY_DEFAULT_QUOTA_INFO>(), 8);
    assert_eq!(size_of::<POLICY_MODIFICATION_INFO>(), 16);
    assert_eq!(align_of::<POLICY_MODIFICATION_INFO>(), 8);
    assert_eq!(size_of::<POLICY_AUDIT_FULL_SET_INFO>(), 1);
    assert_eq!(align_of::<POLICY_AUDIT_FULL_SET_INFO>(), 1);
    assert_eq!(size_of::<POLICY_AUDIT_FULL_QUERY_INFO>(), 2);
    assert_eq!(align_of::<POLICY_AUDIT_FULL_QUERY_INFO>(), 1);
    assert_eq!(size_of::<POLICY_DOMAIN_EFS_INFO>(), 16);
    assert_eq!(align_of::<POLICY_DOMAIN_EFS_INFO>(), 8);
    assert_eq!(size_of::<POLICY_DOMAIN_KERBEROS_TICKET_INFO>(), 48);
    assert_eq!(align_of::<POLICY_DOMAIN_KERBEROS_TICKET_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_NAME_INFO>(), 16);
    assert_eq!(align_of::<TRUSTED_DOMAIN_NAME_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_CONTROLLERS_INFO>(), 16);
    assert_eq!(align_of::<TRUSTED_CONTROLLERS_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_POSIX_OFFSET_INFO>(), 4);
    assert_eq!(align_of::<TRUSTED_POSIX_OFFSET_INFO>(), 4);
    assert_eq!(size_of::<TRUSTED_PASSWORD_INFO>(), 32);
    assert_eq!(align_of::<TRUSTED_PASSWORD_INFO>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_INFORMATION_EX>(), 56);
    assert_eq!(align_of::<TRUSTED_DOMAIN_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_INFORMATION_EX2>(), 64);
    assert_eq!(align_of::<TRUSTED_DOMAIN_INFORMATION_EX2>(), 8);
    assert_eq!(size_of::<LSA_AUTH_INFORMATION>(), 24);
    assert_eq!(align_of::<LSA_AUTH_INFORMATION>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_AUTH_INFORMATION>(), 48);
    assert_eq!(align_of::<TRUSTED_DOMAIN_AUTH_INFORMATION>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_FULL_INFORMATION>(), 112);
    assert_eq!(align_of::<TRUSTED_DOMAIN_FULL_INFORMATION>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_FULL_INFORMATION2>(), 120);
    assert_eq!(align_of::<TRUSTED_DOMAIN_FULL_INFORMATION2>(), 8);
    assert_eq!(size_of::<TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES>(), 4);
    assert_eq!(align_of::<TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES>(), 4);
    assert_eq!(size_of::<LSA_FOREST_TRUST_DOMAIN_INFO>(), 40);
    assert_eq!(align_of::<LSA_FOREST_TRUST_DOMAIN_INFO>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_BINARY_DATA>(), 16);
    assert_eq!(align_of::<LSA_FOREST_TRUST_BINARY_DATA>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_RECORD_ForestTrustData>(), 40);
    assert_eq!(align_of::<LSA_FOREST_TRUST_RECORD_ForestTrustData>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_RECORD>(), 56);
    assert_eq!(align_of::<LSA_FOREST_TRUST_RECORD>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_INFORMATION>(), 16);
    assert_eq!(align_of::<LSA_FOREST_TRUST_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_COLLISION_RECORD>(), 32);
    assert_eq!(align_of::<LSA_FOREST_TRUST_COLLISION_RECORD>(), 8);
    assert_eq!(size_of::<LSA_FOREST_TRUST_COLLISION_INFORMATION>(), 16);
    assert_eq!(align_of::<LSA_FOREST_TRUST_COLLISION_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_ENUMERATION_INFORMATION>(), 8);
    assert_eq!(align_of::<LSA_ENUMERATION_INFORMATION>(), 8);
    assert_eq!(size_of::<LSA_LAST_INTER_LOGON_INFO>(), 24);
    assert_eq!(align_of::<LSA_LAST_INTER_LOGON_INFO>(), 8);
    assert_eq!(size_of::<SECURITY_LOGON_SESSION_DATA>(), 272);
    assert_eq!(align_of::<SECURITY_LOGON_SESSION_DATA>(), 8);
    assert_eq!(size_of::<CENTRAL_ACCESS_POLICY_ENTRY>(), 104);
    assert_eq!(align_of::<CENTRAL_ACCESS_POLICY_ENTRY>(), 8);
    assert_eq!(size_of::<CENTRAL_ACCESS_POLICY>(), 72);
    assert_eq!(align_of::<CENTRAL_ACCESS_POLICY>(), 8);
    assert_eq!(size_of::<NEGOTIATE_PACKAGE_PREFIX>(), 64);
    assert_eq!(align_of::<NEGOTIATE_PACKAGE_PREFIX>(), 8);
    assert_eq!(size_of::<NEGOTIATE_PACKAGE_PREFIXES>(), 16);
    assert_eq!(align_of::<NEGOTIATE_PACKAGE_PREFIXES>(), 4);
    assert_eq!(size_of::<NEGOTIATE_CALLER_NAME_REQUEST>(), 12);
    assert_eq!(align_of::<NEGOTIATE_CALLER_NAME_REQUEST>(), 4);
    assert_eq!(size_of::<NEGOTIATE_CALLER_NAME_RESPONSE>(), 16);
    assert_eq!(align_of::<NEGOTIATE_CALLER_NAME_RESPONSE>(), 8);
    assert_eq!(size_of::<DOMAIN_PASSWORD_INFORMATION>(), 24);
    assert_eq!(align_of::<DOMAIN_PASSWORD_INFORMATION>(), 8);
    assert_eq!(size_of::<MSV1_0_INTERACTIVE_LOGON>(), 56);
    assert_eq!(align_of::<MSV1_0_INTERACTIVE_LOGON>(), 8);
    assert_eq!(size_of::<MSV1_0_INTERACTIVE_PROFILE>(), 160);
    assert_eq!(align_of::<MSV1_0_INTERACTIVE_PROFILE>(), 8);
    assert_eq!(size_of::<MSV1_0_LM20_LOGON>(), 104);
    assert_eq!(align_of::<MSV1_0_LM20_LOGON>(), 8);
    assert_eq!(size_of::<MSV1_0_SUBAUTH_LOGON>(), 104);
    assert_eq!(align_of::<MSV1_0_SUBAUTH_LOGON>(), 8);
    assert_eq!(size_of::<MSV1_0_S4U_LOGON>(), 40);
    assert_eq!(align_of::<MSV1_0_S4U_LOGON>(), 8);
    assert_eq!(size_of::<MSV1_0_LM20_LOGON_PROFILE>(), 104);
    assert_eq!(align_of::<MSV1_0_LM20_LOGON_PROFILE>(), 8);
    assert_eq!(size_of::<MSV1_0_SUPPLEMENTAL_CREDENTIAL>(), 40);
    assert_eq!(align_of::<MSV1_0_SUPPLEMENTAL_CREDENTIAL>(), 4);
    assert_eq!(size_of::<MSV1_0_NTLM3_RESPONSE>(), 48);
    assert_eq!(align_of::<MSV1_0_NTLM3_RESPONSE>(), 8);
    assert_eq!(size_of::<MSV1_0_AV_PAIR>(), 4);
    assert_eq!(align_of::<MSV1_0_AV_PAIR>(), 2);
    assert_eq!(size_of::<MSV1_0_CHANGEPASSWORD_REQUEST>(), 80);
    assert_eq!(align_of::<MSV1_0_CHANGEPASSWORD_REQUEST>(), 8);
    assert_eq!(size_of::<MSV1_0_CHANGEPASSWORD_RESPONSE>(), 32);
    assert_eq!(align_of::<MSV1_0_CHANGEPASSWORD_RESPONSE>(), 8);
    assert_eq!(size_of::<MSV1_0_PASSTHROUGH_REQUEST>(), 64);
    assert_eq!(align_of::<MSV1_0_PASSTHROUGH_REQUEST>(), 8);
    assert_eq!(size_of::<MSV1_0_PASSTHROUGH_RESPONSE>(), 24);
    assert_eq!(align_of::<MSV1_0_PASSTHROUGH_RESPONSE>(), 8);
    assert_eq!(size_of::<MSV1_0_SUBAUTH_REQUEST>(), 24);
    assert_eq!(align_of::<MSV1_0_SUBAUTH_REQUEST>(), 8);
    assert_eq!(size_of::<MSV1_0_SUBAUTH_RESPONSE>(), 16);
    assert_eq!(align_of::<MSV1_0_SUBAUTH_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_INTERACTIVE_LOGON>(), 56);
    assert_eq!(align_of::<KERB_INTERACTIVE_LOGON>(), 8);
    assert_eq!(size_of::<KERB_INTERACTIVE_UNLOCK_LOGON>(), 64);
    assert_eq!(align_of::<KERB_INTERACTIVE_UNLOCK_LOGON>(), 8);
    assert_eq!(size_of::<KERB_SMART_CARD_LOGON>(), 40);
    assert_eq!(align_of::<KERB_SMART_CARD_LOGON>(), 8);
    assert_eq!(size_of::<KERB_SMART_CARD_UNLOCK_LOGON>(), 48);
    assert_eq!(align_of::<KERB_SMART_CARD_UNLOCK_LOGON>(), 8);
    assert_eq!(size_of::<KERB_CERTIFICATE_LOGON>(), 72);
    assert_eq!(align_of::<KERB_CERTIFICATE_LOGON>(), 8);
    assert_eq!(size_of::<KERB_CERTIFICATE_UNLOCK_LOGON>(), 80);
    assert_eq!(align_of::<KERB_CERTIFICATE_UNLOCK_LOGON>(), 8);
    assert_eq!(size_of::<KERB_CERTIFICATE_S4U_LOGON>(), 56);
    assert_eq!(align_of::<KERB_CERTIFICATE_S4U_LOGON>(), 8);
    assert_eq!(size_of::<KERB_TICKET_LOGON>(), 32);
    assert_eq!(align_of::<KERB_TICKET_LOGON>(), 8);
    assert_eq!(size_of::<KERB_TICKET_UNLOCK_LOGON>(), 40);
    assert_eq!(align_of::<KERB_TICKET_UNLOCK_LOGON>(), 8);
    assert_eq!(size_of::<KERB_S4U_LOGON>(), 40);
    assert_eq!(align_of::<KERB_S4U_LOGON>(), 8);
    assert_eq!(size_of::<KERB_INTERACTIVE_PROFILE>(), 160);
    assert_eq!(align_of::<KERB_INTERACTIVE_PROFILE>(), 8);
    assert_eq!(size_of::<KERB_SMART_CARD_PROFILE>(), 176);
    assert_eq!(align_of::<KERB_SMART_CARD_PROFILE>(), 8);
    assert_eq!(size_of::<KERB_CRYPTO_KEY>(), 16);
    assert_eq!(align_of::<KERB_CRYPTO_KEY>(), 8);
    assert_eq!(size_of::<KERB_CRYPTO_KEY32>(), 12);
    assert_eq!(align_of::<KERB_CRYPTO_KEY32>(), 4);
    assert_eq!(size_of::<KERB_TICKET_PROFILE>(), 176);
    assert_eq!(align_of::<KERB_TICKET_PROFILE>(), 8);
    assert_eq!(size_of::<KERB_QUERY_TKT_CACHE_REQUEST>(), 12);
    assert_eq!(align_of::<KERB_QUERY_TKT_CACHE_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_TICKET_CACHE_INFO>(), 64);
    assert_eq!(align_of::<KERB_TICKET_CACHE_INFO>(), 8);
    assert_eq!(size_of::<KERB_TICKET_CACHE_INFO_EX>(), 96);
    assert_eq!(align_of::<KERB_TICKET_CACHE_INFO_EX>(), 8);
    assert_eq!(size_of::<KERB_TICKET_CACHE_INFO_EX2>(), 104);
    assert_eq!(align_of::<KERB_TICKET_CACHE_INFO_EX2>(), 8);
    assert_eq!(size_of::<KERB_TICKET_CACHE_INFO_EX3>(), 128);
    assert_eq!(align_of::<KERB_TICKET_CACHE_INFO_EX3>(), 8);
    assert_eq!(size_of::<KERB_QUERY_TKT_CACHE_RESPONSE>(), 72);
    assert_eq!(align_of::<KERB_QUERY_TKT_CACHE_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_QUERY_TKT_CACHE_EX_RESPONSE>(), 104);
    assert_eq!(align_of::<KERB_QUERY_TKT_CACHE_EX_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_QUERY_TKT_CACHE_EX2_RESPONSE>(), 112);
    assert_eq!(align_of::<KERB_QUERY_TKT_CACHE_EX2_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_QUERY_TKT_CACHE_EX3_RESPONSE>(), 136);
    assert_eq!(align_of::<KERB_QUERY_TKT_CACHE_EX3_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_AUTH_DATA>(), 16);
    assert_eq!(align_of::<KERB_AUTH_DATA>(), 8);
    assert_eq!(size_of::<KERB_NET_ADDRESS>(), 16);
    assert_eq!(align_of::<KERB_NET_ADDRESS>(), 8);
    assert_eq!(size_of::<KERB_NET_ADDRESSES>(), 24);
    assert_eq!(align_of::<KERB_NET_ADDRESSES>(), 8);
    assert_eq!(size_of::<KERB_EXTERNAL_NAME>(), 24);
    assert_eq!(align_of::<KERB_EXTERNAL_NAME>(), 8);
    assert_eq!(size_of::<KERB_EXTERNAL_TICKET>(), 152);
    assert_eq!(align_of::<KERB_EXTERNAL_TICKET>(), 8);
    assert_eq!(size_of::<KERB_RETRIEVE_TKT_REQUEST>(), 64);
    assert_eq!(align_of::<KERB_RETRIEVE_TKT_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_RETRIEVE_TKT_RESPONSE>(), 152);
    assert_eq!(align_of::<KERB_RETRIEVE_TKT_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_PURGE_TKT_CACHE_REQUEST>(), 48);
    assert_eq!(align_of::<KERB_PURGE_TKT_CACHE_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_PURGE_TKT_CACHE_EX_REQUEST>(), 112);
    assert_eq!(align_of::<KERB_PURGE_TKT_CACHE_EX_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_SUBMIT_TKT_REQUEST>(), 36);
    assert_eq!(align_of::<KERB_SUBMIT_TKT_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_QUERY_KDC_PROXY_CACHE_REQUEST>(), 16);
    assert_eq!(align_of::<KERB_QUERY_KDC_PROXY_CACHE_REQUEST>(), 4);
    assert_eq!(size_of::<KDC_PROXY_CACHE_ENTRY_DATA>(), 112);
    assert_eq!(align_of::<KDC_PROXY_CACHE_ENTRY_DATA>(), 8);
    assert_eq!(size_of::<KERB_QUERY_KDC_PROXY_CACHE_RESPONSE>(), 16);
    assert_eq!(align_of::<KERB_QUERY_KDC_PROXY_CACHE_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_PURGE_KDC_PROXY_CACHE_REQUEST>(), 16);
    assert_eq!(align_of::<KERB_PURGE_KDC_PROXY_CACHE_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_PURGE_KDC_PROXY_CACHE_RESPONSE>(), 8);
    assert_eq!(align_of::<KERB_PURGE_KDC_PROXY_CACHE_RESPONSE>(), 4);
    assert_eq!(size_of::<KERB_S4U2PROXY_CACHE_ENTRY_INFO>(), 32);
    assert_eq!(align_of::<KERB_S4U2PROXY_CACHE_ENTRY_INFO>(), 8);
    assert_eq!(size_of::<KERB_S4U2PROXY_CRED>(), 64);
    assert_eq!(align_of::<KERB_S4U2PROXY_CRED>(), 8);
    assert_eq!(size_of::<KERB_QUERY_S4U2PROXY_CACHE_REQUEST>(), 16);
    assert_eq!(align_of::<KERB_QUERY_S4U2PROXY_CACHE_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_QUERY_S4U2PROXY_CACHE_RESPONSE>(), 16);
    assert_eq!(align_of::<KERB_QUERY_S4U2PROXY_CACHE_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_CHANGEPASSWORD_REQUEST>(), 80);
    assert_eq!(align_of::<KERB_CHANGEPASSWORD_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_SETPASSWORD_REQUEST>(), 88);
    assert_eq!(align_of::<KERB_SETPASSWORD_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_SETPASSWORD_EX_REQUEST>(), 152);
    assert_eq!(align_of::<KERB_SETPASSWORD_EX_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_DECRYPT_REQUEST>(), 64);
    assert_eq!(align_of::<KERB_DECRYPT_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_DECRYPT_RESPONSE>(), 1);
    assert_eq!(align_of::<KERB_DECRYPT_RESPONSE>(), 1);
    assert_eq!(size_of::<KERB_ADD_BINDING_CACHE_ENTRY_REQUEST>(), 48);
    assert_eq!(align_of::<KERB_ADD_BINDING_CACHE_ENTRY_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_REFRESH_SCCRED_REQUEST>(), 40);
    assert_eq!(align_of::<KERB_REFRESH_SCCRED_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_ADD_CREDENTIALS_REQUEST>(), 72);
    assert_eq!(align_of::<KERB_ADD_CREDENTIALS_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_ADD_CREDENTIALS_REQUEST_EX>(), 96);
    assert_eq!(align_of::<KERB_ADD_CREDENTIALS_REQUEST_EX>(), 8);
    assert_eq!(size_of::<KERB_TRANSFER_CRED_REQUEST>(), 24);
    assert_eq!(align_of::<KERB_TRANSFER_CRED_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST>(), 12);
    assert_eq!(align_of::<KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_BINDING_CACHE_ENTRY_DATA>(), 72);
    assert_eq!(align_of::<KERB_BINDING_CACHE_ENTRY_DATA>(), 8);
    assert_eq!(size_of::<KERB_QUERY_BINDING_CACHE_RESPONSE>(), 16);
    assert_eq!(align_of::<KERB_QUERY_BINDING_CACHE_RESPONSE>(), 8);
    assert_eq!(size_of::<KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST>(), 48);
    assert_eq!(align_of::<KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST>(), 8);
    assert_eq!(size_of::<KERB_QUERY_BINDING_CACHE_REQUEST>(), 4);
    assert_eq!(align_of::<KERB_QUERY_BINDING_CACHE_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_PURGE_BINDING_CACHE_REQUEST>(), 4);
    assert_eq!(align_of::<KERB_PURGE_BINDING_CACHE_REQUEST>(), 4);
    assert_eq!(size_of::<KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST>(), 24);
    assert_eq!(align_of::<KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST>(), 8);
    assert_eq!(
        size_of::<KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE>(),
        16
    );
    assert_eq!(
        align_of::<KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE>(),
        4
    );
    assert_eq!(size_of::<KERB_CERTIFICATE_HASHINFO>(), 4);
    assert_eq!(align_of::<KERB_CERTIFICATE_HASHINFO>(), 2);
    assert_eq!(size_of::<KERB_CERTIFICATE_INFO>(), 8);
    assert_eq!(align_of::<KERB_CERTIFICATE_INFO>(), 4);
    assert_eq!(size_of::<POLICY_AUDIT_SID_ARRAY>(), 16);
    assert_eq!(align_of::<POLICY_AUDIT_SID_ARRAY>(), 8);
    assert_eq!(size_of::<AUDIT_POLICY_INFORMATION>(), 36);
    assert_eq!(align_of::<AUDIT_POLICY_INFORMATION>(), 4);
    assert_eq!(size_of::<PKU2U_CERT_BLOB>(), 8);
    assert_eq!(align_of::<PKU2U_CERT_BLOB>(), 4);
    assert_eq!(size_of::<PKU2U_CREDUI_CONTEXT>(), 24);
    assert_eq!(align_of::<PKU2U_CREDUI_CONTEXT>(), 8);
    assert_eq!(size_of::<PKU2U_CERTIFICATE_S4U_LOGON>(), 56);
    assert_eq!(align_of::<PKU2U_CERTIFICATE_S4U_LOGON>(), 8);
}
#[cfg(feature = "oaidl")]
#[test]
fn um_oaidl() {
    use winapi::um::oaidl::*;
    assert_eq!(size_of::<SAFEARRAYBOUND>(), 8);
    assert_eq!(align_of::<SAFEARRAYBOUND>(), 4);
    assert_eq!(size_of::<SAFEARR_BSTR>(), 16);
    assert_eq!(align_of::<SAFEARR_BSTR>(), 8);
    assert_eq!(size_of::<SAFEARR_UNKNOWN>(), 16);
    assert_eq!(align_of::<SAFEARR_UNKNOWN>(), 8);
    assert_eq!(size_of::<SAFEARR_DISPATCH>(), 16);
    assert_eq!(align_of::<SAFEARR_DISPATCH>(), 8);
    assert_eq!(size_of::<SAFEARR_VARIANT>(), 16);
    assert_eq!(align_of::<SAFEARR_VARIANT>(), 8);
    assert_eq!(size_of::<SAFEARR_BRECORD>(), 16);
    assert_eq!(align_of::<SAFEARR_BRECORD>(), 8);
    assert_eq!(size_of::<SAFEARR_HAVEIID>(), 32);
    assert_eq!(align_of::<SAFEARR_HAVEIID>(), 8);
    assert_eq!(size_of::<SAFEARRAYUNION>(), 40);
    assert_eq!(align_of::<SAFEARRAYUNION>(), 8);
    assert_eq!(size_of::<_wireSAFEARRAY>(), 64);
    assert_eq!(align_of::<_wireSAFEARRAY>(), 8);
    assert_eq!(size_of::<SAFEARRAY>(), 32);
    assert_eq!(align_of::<SAFEARRAY>(), 8);
    assert_eq!(size_of::<VARIANT_n1>(), 24);
    assert_eq!(align_of::<VARIANT_n1>(), 8);
    assert_eq!(size_of::<VARIANT>(), 24);
    assert_eq!(align_of::<VARIANT>(), 8);
    assert_eq!(size_of::<_wireBRECORD>(), 24);
    assert_eq!(align_of::<_wireBRECORD>(), 8);
    assert_eq!(size_of::<_wireVARIANT_u>(), 16);
    assert_eq!(align_of::<_wireVARIANT_u>(), 8);
    assert_eq!(size_of::<_wireVARIANT>(), 32);
    assert_eq!(align_of::<_wireVARIANT>(), 8);
    assert_eq!(size_of::<TYPEDESC_u>(), 8);
    assert_eq!(align_of::<TYPEDESC_u>(), 8);
    assert_eq!(size_of::<TYPEDESC>(), 16);
    assert_eq!(align_of::<TYPEDESC>(), 8);
    assert_eq!(size_of::<ARRAYDESC>(), 32);
    assert_eq!(align_of::<ARRAYDESC>(), 8);
    assert_eq!(size_of::<PARAMDESCEX>(), 32);
    assert_eq!(align_of::<PARAMDESCEX>(), 8);
    assert_eq!(size_of::<PARAMDESC>(), 16);
    assert_eq!(align_of::<PARAMDESC>(), 8);
    assert_eq!(size_of::<IDLDESC>(), 16);
    assert_eq!(align_of::<IDLDESC>(), 8);
    assert_eq!(size_of::<ELEMDESC_u>(), 16);
    assert_eq!(align_of::<ELEMDESC_u>(), 8);
    assert_eq!(size_of::<ELEMDESC>(), 32);
    assert_eq!(align_of::<ELEMDESC>(), 8);
    assert_eq!(size_of::<TYPEATTR>(), 96);
    assert_eq!(align_of::<TYPEATTR>(), 8);
    assert_eq!(size_of::<DISPPARAMS>(), 24);
    assert_eq!(align_of::<DISPPARAMS>(), 8);
    assert_eq!(size_of::<EXCEPINFO>(), 64);
    assert_eq!(align_of::<EXCEPINFO>(), 8);
    assert_eq!(size_of::<FUNCDESC>(), 88);
    assert_eq!(align_of::<FUNCDESC>(), 8);
    assert_eq!(size_of::<VARDESC_u>(), 8);
    assert_eq!(align_of::<VARDESC_u>(), 8);
    assert_eq!(size_of::<VARDESC>(), 64);
    assert_eq!(align_of::<VARDESC>(), 8);
    assert_eq!(size_of::<CLEANLOCALSTORAGE>(), 24);
    assert_eq!(align_of::<CLEANLOCALSTORAGE>(), 8);
    assert_eq!(size_of::<CUSTDATAITEM>(), 40);
    assert_eq!(align_of::<CUSTDATAITEM>(), 8);
    assert_eq!(size_of::<CUSTDATA>(), 16);
    assert_eq!(align_of::<CUSTDATA>(), 8);
    assert_eq!(size_of::<TLIBATTR>(), 32);
    assert_eq!(align_of::<TLIBATTR>(), 4);
    assert_eq!(size_of::<BINDPTR>(), 8);
    assert_eq!(align_of::<BINDPTR>(), 8);
}
#[cfg(feature = "objidl")]
#[test]
fn um_objidl() {
    use winapi::um::objidl::*;
    assert_eq!(size_of::<BIND_OPTS>(), 16);
    assert_eq!(align_of::<BIND_OPTS>(), 4);
    assert_eq!(size_of::<SOLE_AUTHENTICATION_SERVICE>(), 24);
    assert_eq!(align_of::<SOLE_AUTHENTICATION_SERVICE>(), 8);
}
#[cfg(feature = "objidlbase")]
#[test]
fn um_objidlbase() {
    use winapi::um::objidlbase::*;
    assert_eq!(size_of::<COSERVERINFO>(), 32);
    assert_eq!(align_of::<COSERVERINFO>(), 8);
    assert_eq!(size_of::<MULTI_QI>(), 24);
    assert_eq!(align_of::<MULTI_QI>(), 8);
    assert_eq!(size_of::<STATSTG>(), 80);
    assert_eq!(align_of::<STATSTG>(), 8);
    assert_eq!(size_of::<RPCOLEMESSAGE>(), 80);
    assert_eq!(align_of::<RPCOLEMESSAGE>(), 8);
    assert_eq!(size_of::<SChannelHookCallInfo>(), 56);
    assert_eq!(align_of::<SChannelHookCallInfo>(), 8);
    assert_eq!(size_of::<SOLE_AUTHENTICATION_SERVICE>(), 24);
    assert_eq!(align_of::<SOLE_AUTHENTICATION_SERVICE>(), 8);
    assert_eq!(size_of::<SOLE_AUTHENTICATION_INFO>(), 16);
    assert_eq!(align_of::<SOLE_AUTHENTICATION_INFO>(), 8);
    assert_eq!(size_of::<SOLE_AUTHENTICATION_LIST>(), 16);
    assert_eq!(align_of::<SOLE_AUTHENTICATION_LIST>(), 8);
    assert_eq!(size_of::<ContextProperty>(), 32);
    assert_eq!(align_of::<ContextProperty>(), 8);
}
#[cfg(feature = "ocidl")]
#[test]
fn um_ocidl() {
    use winapi::um::ocidl::*;
    assert_eq!(size_of::<PROPBAG2>(), 40);
    assert_eq!(align_of::<PROPBAG2>(), 8);
}
#[cfg(feature = "opmapi")]
#[test]
fn um_opmapi() {
    use winapi::um::opmapi::*;
    assert_eq!(size_of::<OPM_RANDOM_NUMBER>(), 16);
    assert_eq!(align_of::<OPM_RANDOM_NUMBER>(), 1);
    assert_eq!(size_of::<OPM_OMAC>(), 16);
    assert_eq!(align_of::<OPM_OMAC>(), 1);
    assert_eq!(size_of::<OPM_ENCRYPTED_INITIALIZATION_PARAMETERS>(), 256);
    assert_eq!(align_of::<OPM_ENCRYPTED_INITIALIZATION_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_GET_INFO_PARAMETERS>(), 4112);
    assert_eq!(align_of::<OPM_GET_INFO_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS>(), 4096);
    assert_eq!(align_of::<OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_HDCP_KEY_SELECTION_VECTOR>(), 5);
    assert_eq!(align_of::<OPM_HDCP_KEY_SELECTION_VECTOR>(), 1);
    assert_eq!(size_of::<OPM_CONNECTED_HDCP_DEVICE_INFORMATION>(), 72);
    assert_eq!(align_of::<OPM_CONNECTED_HDCP_DEVICE_INFORMATION>(), 1);
    assert_eq!(size_of::<OPM_REQUESTED_INFORMATION>(), 4096);
    assert_eq!(align_of::<OPM_REQUESTED_INFORMATION>(), 1);
    assert_eq!(size_of::<OPM_STANDARD_INFORMATION>(), 32);
    assert_eq!(align_of::<OPM_STANDARD_INFORMATION>(), 1);
    assert_eq!(size_of::<OPM_ACTUAL_OUTPUT_FORMAT>(), 44);
    assert_eq!(align_of::<OPM_ACTUAL_OUTPUT_FORMAT>(), 1);
    assert_eq!(size_of::<OPM_ACP_AND_CGMSA_SIGNALING>(), 88);
    assert_eq!(align_of::<OPM_ACP_AND_CGMSA_SIGNALING>(), 1);
    assert_eq!(size_of::<OPM_OUTPUT_ID_DATA>(), 28);
    assert_eq!(align_of::<OPM_OUTPUT_ID_DATA>(), 1);
    assert_eq!(size_of::<OPM_CONFIGURE_PARAMETERS>(), 4096);
    assert_eq!(align_of::<OPM_CONFIGURE_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_SET_PROTECTION_LEVEL_PARAMETERS>(), 16);
    assert_eq!(align_of::<OPM_SET_PROTECTION_LEVEL_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS>(), 64);
    assert_eq!(align_of::<OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_SET_HDCP_SRM_PARAMETERS>(), 4);
    assert_eq!(align_of::<OPM_SET_HDCP_SRM_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_GET_CODEC_INFO_PARAMETERS>(), 4056);
    assert_eq!(align_of::<OPM_GET_CODEC_INFO_PARAMETERS>(), 1);
    assert_eq!(size_of::<OPM_GET_CODEC_INFO_INFORMATION>(), 20);
    assert_eq!(align_of::<OPM_GET_CODEC_INFO_INFORMATION>(), 1);
}
#[cfg(feature = "pdh")]
#[test]
fn um_pdh() {
    use winapi::um::pdh::*;
    assert_eq!(size_of::<PDH_FMT_COUNTERVALUE>(), 16);
    assert_eq!(align_of::<PDH_FMT_COUNTERVALUE>(), 8);
    assert_eq!(size_of::<PDH_RAW_LOG_RECORD>(), 16);
    assert_eq!(align_of::<PDH_RAW_LOG_RECORD>(), 4);
    assert_eq!(size_of::<PDH_TIME_INFO>(), 24);
    assert_eq!(align_of::<PDH_TIME_INFO>(), 8);
    assert_eq!(size_of::<PDH_RAW_COUNTER>(), 40);
    assert_eq!(align_of::<PDH_RAW_COUNTER>(), 8);
    assert_eq!(size_of::<PDH_STATISTICS>(), 56);
    assert_eq!(align_of::<PDH_STATISTICS>(), 8);
    assert_eq!(size_of::<PDH_FMT_COUNTERVALUE_ITEM_A>(), 24);
    assert_eq!(align_of::<PDH_FMT_COUNTERVALUE_ITEM_A>(), 8);
    assert_eq!(size_of::<PDH_FMT_COUNTERVALUE_ITEM_W>(), 24);
    assert_eq!(align_of::<PDH_FMT_COUNTERVALUE_ITEM_W>(), 8);
    assert_eq!(size_of::<PDH_BROWSE_DLG_CONFIG_A>(), 72);
    assert_eq!(align_of::<PDH_BROWSE_DLG_CONFIG_A>(), 8);
    assert_eq!(size_of::<PDH_BROWSE_DLG_CONFIG_W>(), 72);
    assert_eq!(align_of::<PDH_BROWSE_DLG_CONFIG_W>(), 8);
    assert_eq!(size_of::<PDH_COUNTER_PATH_ELEMENTS_A>(), 48);
    assert_eq!(align_of::<PDH_COUNTER_PATH_ELEMENTS_A>(), 8);
    assert_eq!(size_of::<PDH_COUNTER_PATH_ELEMENTS_W>(), 48);
    assert_eq!(align_of::<PDH_COUNTER_PATH_ELEMENTS_W>(), 8);
    assert_eq!(size_of::<PDH_DATA_ITEM_PATH_ELEMENTS_A>(), 40);
    assert_eq!(align_of::<PDH_DATA_ITEM_PATH_ELEMENTS_A>(), 8);
    assert_eq!(size_of::<PDH_DATA_ITEM_PATH_ELEMENTS_W>(), 40);
    assert_eq!(align_of::<PDH_DATA_ITEM_PATH_ELEMENTS_W>(), 8);
    assert_eq!(size_of::<PDH_COUNTER_INFO_A>(), 112);
    assert_eq!(align_of::<PDH_COUNTER_INFO_A>(), 8);
    assert_eq!(size_of::<PDH_COUNTER_INFO_W>(), 112);
    assert_eq!(align_of::<PDH_COUNTER_INFO_W>(), 8);
    assert_eq!(size_of::<PDH_BROWSE_DLG_CONFIG_HA>(), 72);
    assert_eq!(align_of::<PDH_BROWSE_DLG_CONFIG_HA>(), 8);
    assert_eq!(size_of::<PDH_BROWSE_DLG_CONFIG_HW>(), 72);
    assert_eq!(align_of::<PDH_BROWSE_DLG_CONFIG_HW>(), 8);
}
#[cfg(feature = "perflib")]
#[test]
fn um_perflib() {
    use winapi::um::perflib::*;
    assert_eq!(size_of::<PERF_COUNTERSET_INFO>(), 40);
    assert_eq!(align_of::<PERF_COUNTERSET_INFO>(), 4);
    assert_eq!(size_of::<PERF_COUNTER_INFO>(), 32);
    assert_eq!(align_of::<PERF_COUNTER_INFO>(), 8);
    assert_eq!(size_of::<PERF_COUNTERSET_INSTANCE>(), 32);
    assert_eq!(align_of::<PERF_COUNTERSET_INSTANCE>(), 4);
    assert_eq!(size_of::<PERF_COUNTER_IDENTITY>(), 40);
    assert_eq!(align_of::<PERF_COUNTER_IDENTITY>(), 4);
    assert_eq!(size_of::<PERF_PROVIDER_CONTEXT>(), 40);
    assert_eq!(align_of::<PERF_PROVIDER_CONTEXT>(), 8);
    assert_eq!(size_of::<PERF_INSTANCE_HEADER>(), 8);
    assert_eq!(align_of::<PERF_INSTANCE_HEADER>(), 4);
    assert_eq!(size_of::<PERF_COUNTERSET_REG_INFO>(), 32);
    assert_eq!(align_of::<PERF_COUNTERSET_REG_INFO>(), 4);
    assert_eq!(size_of::<PERF_COUNTER_REG_INFO>(), 48);
    assert_eq!(align_of::<PERF_COUNTER_REG_INFO>(), 8);
    assert_eq!(size_of::<PERF_STRING_BUFFER_HEADER>(), 8);
    assert_eq!(align_of::<PERF_STRING_BUFFER_HEADER>(), 4);
    assert_eq!(size_of::<PERF_STRING_COUNTER_HEADER>(), 8);
    assert_eq!(align_of::<PERF_STRING_COUNTER_HEADER>(), 4);
    assert_eq!(size_of::<PERF_COUNTER_IDENTIFIER>(), 40);
    assert_eq!(align_of::<PERF_COUNTER_IDENTIFIER>(), 4);
    assert_eq!(size_of::<PERF_DATA_HEADER>(), 48);
    assert_eq!(align_of::<PERF_DATA_HEADER>(), 8);
    assert_eq!(size_of::<PERF_COUNTER_HEADER>(), 16);
    assert_eq!(align_of::<PERF_COUNTER_HEADER>(), 4);
    assert_eq!(size_of::<PERF_MULTI_INSTANCES>(), 8);
    assert_eq!(align_of::<PERF_MULTI_INSTANCES>(), 4);
    assert_eq!(size_of::<PERF_MULTI_COUNTERS>(), 8);
    assert_eq!(align_of::<PERF_MULTI_COUNTERS>(), 4);
    assert_eq!(size_of::<PERF_COUNTER_DATA>(), 8);
    assert_eq!(align_of::<PERF_COUNTER_DATA>(), 4);
}
#[cfg(feature = "physicalmonitorenumerationapi")]
#[test]
fn um_physicalmonitorenumerationapi() {
    use winapi::um::physicalmonitorenumerationapi::*;
    assert_eq!(size_of::<PHYSICAL_MONITOR>(), 264);
    assert_eq!(align_of::<PHYSICAL_MONITOR>(), 1);
}
#[cfg(feature = "powrprof")]
#[test]
fn um_powrprof() {
    use winapi::um::powrprof::*;
    assert_eq!(size_of::<GLOBAL_MACHINE_POWER_POLICY>(), 16);
    assert_eq!(align_of::<GLOBAL_MACHINE_POWER_POLICY>(), 4);
    assert_eq!(size_of::<GLOBAL_USER_POWER_POLICY>(), 176);
    assert_eq!(align_of::<GLOBAL_USER_POWER_POLICY>(), 4);
    assert_eq!(size_of::<GLOBAL_POWER_POLICY>(), 192);
    assert_eq!(align_of::<GLOBAL_POWER_POLICY>(), 4);
    assert_eq!(size_of::<MACHINE_POWER_POLICY>(), 64);
    assert_eq!(align_of::<MACHINE_POWER_POLICY>(), 4);
    assert_eq!(size_of::<MACHINE_PROCESSOR_POWER_POLICY>(), 156);
    assert_eq!(align_of::<MACHINE_PROCESSOR_POWER_POLICY>(), 4);
    assert_eq!(size_of::<USER_POWER_POLICY>(), 80);
    assert_eq!(align_of::<USER_POWER_POLICY>(), 4);
    assert_eq!(size_of::<POWER_POLICY>(), 144);
    assert_eq!(align_of::<POWER_POLICY>(), 4);
    assert_eq!(size_of::<DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS>(), 16);
    assert_eq!(align_of::<DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS>(), 8);
    assert_eq!(size_of::<THERMAL_EVENT>(), 32);
    assert_eq!(align_of::<THERMAL_EVENT>(), 8);
}
#[cfg(feature = "processsnapshot")]
#[test]
fn um_processsnapshot() {
    use winapi::um::processsnapshot::*;
    assert_eq!(size_of::<PSS_ALLOCATOR>(), 24);
    assert_eq!(align_of::<PSS_ALLOCATOR>(), 8);
}
#[cfg(feature = "processthreadsapi")]
#[test]
fn um_processthreadsapi() {
    use winapi::um::processthreadsapi::*;
    assert_eq!(size_of::<PROCESS_INFORMATION>(), 24);
    assert_eq!(align_of::<PROCESS_INFORMATION>(), 8);
    assert_eq!(size_of::<STARTUPINFOA>(), 104);
    assert_eq!(align_of::<STARTUPINFOA>(), 8);
    assert_eq!(size_of::<STARTUPINFOW>(), 104);
    assert_eq!(align_of::<STARTUPINFOW>(), 8);
}
#[cfg(feature = "propidl")]
#[test]
fn um_propidl() {
    // FIXME
    // assert_eq!(size_of::<PROPVARIANT>(), 24);
    // assert_eq!(align_of::<PROPVARIANT>(), 8);
}
#[cfg(feature = "prsht")]
#[test]
fn um_prsht() {
    use winapi::um::prsht::*;
    assert_eq!(size_of::<PROPSHEETPAGEA_V1_u1>(), 8);
    assert_eq!(align_of::<PROPSHEETPAGEA_V1_u1>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEA_V1_u2>(), 8);
    assert_eq!(align_of::<PROPSHEETPAGEA_V1_u2>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEA_V4_u3>(), 8);
    assert_eq!(align_of::<PROPSHEETPAGEA_V4_u3>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEA_V4>(), 104);
    assert_eq!(align_of::<PROPSHEETPAGEA_V4>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEW_V1_u1>(), 8);
    assert_eq!(align_of::<PROPSHEETPAGEW_V1_u1>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEW_V1_u2>(), 8);
    assert_eq!(align_of::<PROPSHEETPAGEW_V1_u2>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEW_V4_u3>(), 8);
    assert_eq!(align_of::<PROPSHEETPAGEW_V4_u3>(), 8);
    assert_eq!(size_of::<PROPSHEETPAGEW_V4>(), 104);
    assert_eq!(align_of::<PROPSHEETPAGEW_V4>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERA_V1_u1>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERA_V1_u1>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERA_V1_u2>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERA_V1_u2>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERA_V1_u3>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERA_V1_u3>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERA_V2_u4>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERA_V2_u4>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERA_V2_u5>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERA_V2_u5>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERA_V2>(), 96);
    assert_eq!(align_of::<PROPSHEETHEADERA_V2>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERW_V1_u1>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERW_V1_u1>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERW_V1_u2>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERW_V1_u2>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERW_V1_u3>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERW_V1_u3>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERW_V2_u4>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERW_V2_u4>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERW_V2_u5>(), 8);
    assert_eq!(align_of::<PROPSHEETHEADERW_V2_u5>(), 8);
    assert_eq!(size_of::<PROPSHEETHEADERW_V2>(), 96);
    assert_eq!(align_of::<PROPSHEETHEADERW_V2>(), 8);
    assert_eq!(size_of::<PSHNOTIFY>(), 32);
    assert_eq!(align_of::<PSHNOTIFY>(), 8);
}
#[cfg(feature = "psapi")]
#[test]
fn um_psapi() {
    use winapi::um::psapi::*;
    assert_eq!(size_of::<MODULEINFO>(), 24);
    assert_eq!(align_of::<MODULEINFO>(), 8);
    assert_eq!(size_of::<ENUM_PAGE_FILE_INFORMATION>(), 32);
    assert_eq!(align_of::<ENUM_PAGE_FILE_INFORMATION>(), 8);
    assert_eq!(size_of::<PERFORMANCE_INFORMATION>(), 104);
    assert_eq!(align_of::<PERFORMANCE_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESS_MEMORY_COUNTERS>(), 72);
    assert_eq!(align_of::<PROCESS_MEMORY_COUNTERS>(), 8);
    assert_eq!(size_of::<PROCESS_MEMORY_COUNTERS_EX>(), 80);
    assert_eq!(align_of::<PROCESS_MEMORY_COUNTERS_EX>(), 8);
    assert_eq!(size_of::<PSAPI_WORKING_SET_BLOCK>(), 8);
    assert_eq!(align_of::<PSAPI_WORKING_SET_BLOCK>(), 8);
    assert_eq!(size_of::<PSAPI_WORKING_SET_EX_BLOCK>(), 8);
    assert_eq!(align_of::<PSAPI_WORKING_SET_EX_BLOCK>(), 8);
    assert_eq!(size_of::<PSAPI_WORKING_SET_INFORMATION>(), 16);
    assert_eq!(align_of::<PSAPI_WORKING_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<PSAPI_WORKING_SET_EX_INFORMATION>(), 16);
    assert_eq!(align_of::<PSAPI_WORKING_SET_EX_INFORMATION>(), 8);
    assert_eq!(size_of::<PSAPI_WS_WATCH_INFORMATION>(), 16);
    assert_eq!(align_of::<PSAPI_WS_WATCH_INFORMATION>(), 8);
    assert_eq!(size_of::<PSAPI_WS_WATCH_INFORMATION_EX>(), 32);
    assert_eq!(align_of::<PSAPI_WS_WATCH_INFORMATION_EX>(), 8);
}
#[cfg(feature = "restartmanager")]
#[test]
fn um_restartmanager() {
    use winapi::um::restartmanager::*;
    assert_eq!(size_of::<RM_UNIQUE_PROCESS>(), 12);
    assert_eq!(align_of::<RM_UNIQUE_PROCESS>(), 4);
    assert_eq!(size_of::<RM_PROCESS_INFO>(), 668);
    assert_eq!(align_of::<RM_PROCESS_INFO>(), 4);
    assert_eq!(size_of::<RM_FILTER_INFO>(), 32);
    assert_eq!(align_of::<RM_FILTER_INFO>(), 8);
}
#[cfg(feature = "sapi")]
#[test]
fn um_sapi() {
    use winapi::um::sapi::*;
    assert_eq!(size_of::<SPPHRASE>(), 184);
    assert_eq!(align_of::<SPPHRASE>(), 8);
}
#[cfg(feature = "sapi51")]
#[test]
fn um_sapi51() {
    use winapi::um::sapi51::*;
    assert_eq!(size_of::<SPEVENT>(), 32);
    assert_eq!(align_of::<SPEVENT>(), 8);
    assert_eq!(size_of::<SPSERIALIZEDEVENT>(), 24);
    assert_eq!(align_of::<SPSERIALIZEDEVENT>(), 8);
    assert_eq!(size_of::<SPSERIALIZEDEVENT64>(), 32);
    assert_eq!(align_of::<SPSERIALIZEDEVENT64>(), 8);
    assert_eq!(size_of::<SPEVENTSOURCEINFO>(), 24);
    assert_eq!(align_of::<SPEVENTSOURCEINFO>(), 8);
    assert_eq!(size_of::<SPAUDIOSTATUS>(), 40);
    assert_eq!(align_of::<SPAUDIOSTATUS>(), 8);
    assert_eq!(size_of::<SPAUDIOBUFFERINFO>(), 12);
    assert_eq!(align_of::<SPAUDIOBUFFERINFO>(), 4);
    assert_eq!(size_of::<SPPHRASEELEMENT>(), 56);
    assert_eq!(align_of::<SPPHRASEELEMENT>(), 8);
    assert_eq!(size_of::<SPPHRASERULE>(), 48);
    assert_eq!(align_of::<SPPHRASERULE>(), 8);
    assert_eq!(size_of::<SPPHRASEPROPERTY>(), 80);
    assert_eq!(align_of::<SPPHRASEPROPERTY>(), 8);
    assert_eq!(size_of::<SPPHRASEREPLACEMENT>(), 24);
    assert_eq!(align_of::<SPPHRASEREPLACEMENT>(), 8);
    assert_eq!(size_of::<SPPHRASE>(), 160);
    assert_eq!(align_of::<SPPHRASE>(), 8);
    assert_eq!(size_of::<SPSERIALIZEDPHRASE>(), 4);
    assert_eq!(align_of::<SPSERIALIZEDPHRASE>(), 4);
    assert_eq!(size_of::<SPBINARYGRAMMAR>(), 4);
    assert_eq!(align_of::<SPBINARYGRAMMAR>(), 4);
    assert_eq!(size_of::<SPWORDPRONUNCIATION>(), 24);
    assert_eq!(align_of::<SPWORDPRONUNCIATION>(), 8);
    assert_eq!(size_of::<SPWORDPRONUNCIATIONLIST>(), 24);
    assert_eq!(align_of::<SPWORDPRONUNCIATIONLIST>(), 8);
    assert_eq!(size_of::<SPWORD>(), 32);
    assert_eq!(align_of::<SPWORD>(), 8);
    assert_eq!(size_of::<SPWORDLIST>(), 24);
    assert_eq!(align_of::<SPWORDLIST>(), 8);
    assert_eq!(size_of::<SPVPITCH>(), 8);
    assert_eq!(align_of::<SPVPITCH>(), 4);
    assert_eq!(size_of::<SPVCONTEXT>(), 24);
    assert_eq!(align_of::<SPVCONTEXT>(), 8);
    assert_eq!(size_of::<SPVSTATE>(), 72);
    assert_eq!(align_of::<SPVSTATE>(), 8);
    assert_eq!(size_of::<SPVOICESTATUS>(), 52);
    assert_eq!(align_of::<SPVOICESTATUS>(), 4);
    assert_eq!(size_of::<SPRECORESULTTIMES>(), 32);
    assert_eq!(align_of::<SPRECORESULTTIMES>(), 8);
    assert_eq!(size_of::<SPSERIALIZEDRESULT>(), 4);
    assert_eq!(align_of::<SPSERIALIZEDRESULT>(), 4);
    assert_eq!(size_of::<SPTEXTSELECTIONINFO>(), 16);
    assert_eq!(align_of::<SPTEXTSELECTIONINFO>(), 4);
    assert_eq!(size_of::<SPPROPERTYINFO>(), 48);
    assert_eq!(align_of::<SPPROPERTYINFO>(), 8);
    assert_eq!(size_of::<SPRECOCONTEXTSTATUS>(), 524);
    assert_eq!(align_of::<SPRECOCONTEXTSTATUS>(), 4);
    assert_eq!(size_of::<SPRECOGNIZERSTATUS>(), 128);
    assert_eq!(align_of::<SPRECOGNIZERSTATUS>(), 8);
}
#[cfg(feature = "sapi53")]
#[test]
fn um_sapi53() {
    use winapi::um::sapi53::*;
    assert_eq!(size_of::<SPEVENTEX>(), 40);
    assert_eq!(align_of::<SPEVENTEX>(), 8);
    assert_eq!(size_of::<SPSEMANTICERRORINFO>(), 40);
    assert_eq!(align_of::<SPSEMANTICERRORINFO>(), 8);
    assert_eq!(size_of::<SPPHRASE>(), 176);
    assert_eq!(align_of::<SPPHRASE>(), 8);
    assert_eq!(size_of::<SPRULE>(), 16);
    assert_eq!(align_of::<SPRULE>(), 8);
    assert_eq!(size_of::<SPSHORTCUTPAIR>(), 32);
    assert_eq!(align_of::<SPSHORTCUTPAIR>(), 8);
    assert_eq!(size_of::<SPSHORTCUTPAIRLIST>(), 24);
    assert_eq!(align_of::<SPSHORTCUTPAIRLIST>(), 8);
    assert_eq!(size_of::<SPNORMALIZATIONLIST>(), 16);
    assert_eq!(align_of::<SPNORMALIZATIONLIST>(), 8);
    assert_eq!(size_of::<SPDISPLAYTOKEN>(), 24);
    assert_eq!(align_of::<SPDISPLAYTOKEN>(), 8);
    assert_eq!(size_of::<SPDISPLAYPHRASE>(), 16);
    assert_eq!(align_of::<SPDISPLAYPHRASE>(), 8);
}
#[cfg(feature = "sapiddk")]
#[test]
fn um_sapiddk() {
    use winapi::um::sapiddk::*;
    assert_eq!(size_of::<SPRECORESULTINFOEX>(), 96);
    assert_eq!(align_of::<SPRECORESULTINFOEX>(), 8);
}
#[cfg(feature = "sapiddk51")]
#[test]
fn um_sapiddk51() {
    use winapi::um::sapiddk51::*;
    assert_eq!(size_of::<SPTMTHREADINFO>(), 16);
    assert_eq!(align_of::<SPTMTHREADINFO>(), 4);
    assert_eq!(size_of::<SPVTEXTFRAG>(), 96);
    assert_eq!(align_of::<SPVTEXTFRAG>(), 8);
    assert_eq!(size_of::<SPWORDENTRY>(), 48);
    assert_eq!(align_of::<SPWORDENTRY>(), 8);
    assert_eq!(size_of::<SPRULEENTRY>(), 40);
    assert_eq!(align_of::<SPRULEENTRY>(), 8);
    assert_eq!(size_of::<SPTRANSITIONENTRY>(), 56);
    assert_eq!(align_of::<SPTRANSITIONENTRY>(), 8);
    assert_eq!(size_of::<SPTRANSITIONPROPERTY>(), 48);
    assert_eq!(align_of::<SPTRANSITIONPROPERTY>(), 8);
    assert_eq!(size_of::<SPSTATEINFO>(), 32);
    assert_eq!(align_of::<SPSTATEINFO>(), 8);
    assert_eq!(size_of::<SPPATHENTRY>(), 64);
    assert_eq!(align_of::<SPPATHENTRY>(), 8);
    assert_eq!(size_of::<SPPHRASEALT>(), 40);
    assert_eq!(align_of::<SPPHRASEALT>(), 8);
    assert_eq!(size_of::<SPRECORESULTINFO>(), 80);
    assert_eq!(align_of::<SPRECORESULTINFO>(), 8);
    assert_eq!(size_of::<SPPARSEINFO>(), 80);
    assert_eq!(align_of::<SPPARSEINFO>(), 8);
    assert_eq!(size_of::<SPPHRASEALTREQUEST>(), 48);
    assert_eq!(align_of::<SPPHRASEALTREQUEST>(), 8);
}
#[cfg(feature = "schannel")]
#[test]
fn um_schannel() {
    use winapi::um::schannel::*;
    assert_eq!(size_of::<SecPkgContext_RemoteCredentialInfo>(), 32);
    assert_eq!(align_of::<SecPkgContext_RemoteCredentialInfo>(), 8);
    assert_eq!(size_of::<SecPkgContext_LocalCredentialInfo>(), 32);
    assert_eq!(align_of::<SecPkgContext_LocalCredentialInfo>(), 8);
    assert_eq!(size_of::<SecPkgContext_ClientCertPolicyResult>(), 20);
    assert_eq!(align_of::<SecPkgContext_ClientCertPolicyResult>(), 4);
    assert_eq!(size_of::<SecPkgContext_IssuerListInfoEx>(), 16);
    assert_eq!(align_of::<SecPkgContext_IssuerListInfoEx>(), 8);
    assert_eq!(size_of::<SecPkgContext_ConnectionInfo>(), 28);
    assert_eq!(align_of::<SecPkgContext_ConnectionInfo>(), 4);
    assert_eq!(size_of::<SecPkgContext_CipherInfo>(), 680);
    assert_eq!(align_of::<SecPkgContext_CipherInfo>(), 4);
    assert_eq!(size_of::<SecPkgContext_EapKeyBlock>(), 192);
    assert_eq!(align_of::<SecPkgContext_EapKeyBlock>(), 1);
    assert_eq!(size_of::<SecPkgContext_MappedCredAttr>(), 16);
    assert_eq!(align_of::<SecPkgContext_MappedCredAttr>(), 8);
    assert_eq!(size_of::<SecPkgContext_SessionInfo>(), 40);
    assert_eq!(align_of::<SecPkgContext_SessionInfo>(), 4);
    assert_eq!(size_of::<SecPkgContext_SessionAppData>(), 16);
    assert_eq!(align_of::<SecPkgContext_SessionAppData>(), 8);
    assert_eq!(size_of::<SecPkgContext_EapPrfInfo>(), 16);
    assert_eq!(align_of::<SecPkgContext_EapPrfInfo>(), 8);
    assert_eq!(size_of::<SecPkgContext_SupportedSignatures>(), 16);
    assert_eq!(align_of::<SecPkgContext_SupportedSignatures>(), 8);
    assert_eq!(size_of::<SecPkgContext_Certificates>(), 16);
    assert_eq!(align_of::<SecPkgContext_Certificates>(), 8);
    assert_eq!(size_of::<SecPkgContext_CertInfo>(), 40);
    assert_eq!(align_of::<SecPkgContext_CertInfo>(), 8);
    assert_eq!(size_of::<SecPkgContext_UiInfo>(), 8);
    assert_eq!(align_of::<SecPkgContext_UiInfo>(), 8);
    assert_eq!(size_of::<SecPkgContext_EarlyStart>(), 4);
    assert_eq!(align_of::<SecPkgContext_EarlyStart>(), 4);
    assert_eq!(size_of::<SCHANNEL_CRED>(), 80);
    assert_eq!(align_of::<SCHANNEL_CRED>(), 8);
    assert_eq!(size_of::<SCHANNEL_CERT_HASH>(), 40);
    assert_eq!(align_of::<SCHANNEL_CERT_HASH>(), 8);
    assert_eq!(size_of::<SCHANNEL_CERT_HASH_STORE>(), 296);
    assert_eq!(align_of::<SCHANNEL_CERT_HASH_STORE>(), 8);
    assert_eq!(size_of::<SCHANNEL_ALERT_TOKEN>(), 12);
    assert_eq!(align_of::<SCHANNEL_ALERT_TOKEN>(), 4);
    assert_eq!(size_of::<SCHANNEL_SESSION_TOKEN>(), 8);
    assert_eq!(align_of::<SCHANNEL_SESSION_TOKEN>(), 4);
    assert_eq!(size_of::<SCHANNEL_CLIENT_SIGNATURE>(), 68);
    assert_eq!(align_of::<SCHANNEL_CLIENT_SIGNATURE>(), 4);
}
#[cfg(feature = "setupapi")]
#[test]
fn um_setupapi() {
    use winapi::um::setupapi::*;
    assert_eq!(size_of::<INFCONTEXT>(), 24);
    assert_eq!(align_of::<INFCONTEXT>(), 8);
    assert_eq!(size_of::<SP_INF_INFORMATION>(), 12);
    assert_eq!(align_of::<SP_INF_INFORMATION>(), 4);
    assert_eq!(size_of::<SP_ALTPLATFORM_INFO_V3_u>(), 2);
    assert_eq!(align_of::<SP_ALTPLATFORM_INFO_V3_u>(), 2);
    assert_eq!(size_of::<SP_ALTPLATFORM_INFO_V3>(), 36);
    assert_eq!(align_of::<SP_ALTPLATFORM_INFO_V3>(), 4);
    assert_eq!(size_of::<SP_ALTPLATFORM_INFO_V2_u>(), 2);
    assert_eq!(align_of::<SP_ALTPLATFORM_INFO_V2_u>(), 2);
    assert_eq!(size_of::<SP_ALTPLATFORM_INFO_V2>(), 28);
    assert_eq!(align_of::<SP_ALTPLATFORM_INFO_V2>(), 4);
    assert_eq!(size_of::<SP_ALTPLATFORM_INFO_V1>(), 20);
    assert_eq!(align_of::<SP_ALTPLATFORM_INFO_V1>(), 4);
    assert_eq!(size_of::<SP_ORIGINAL_FILE_INFO_A>(), 524);
    assert_eq!(align_of::<SP_ORIGINAL_FILE_INFO_A>(), 4);
    assert_eq!(size_of::<SP_ORIGINAL_FILE_INFO_W>(), 1044);
    assert_eq!(align_of::<SP_ORIGINAL_FILE_INFO_W>(), 4);
    assert_eq!(size_of::<FILEPATHS_A>(), 24);
    assert_eq!(align_of::<FILEPATHS_A>(), 8);
    assert_eq!(size_of::<FILEPATHS_W>(), 24);
    assert_eq!(align_of::<FILEPATHS_W>(), 8);
    assert_eq!(size_of::<FILEPATHS_SIGNERINFO_A>(), 48);
    assert_eq!(align_of::<FILEPATHS_SIGNERINFO_A>(), 8);
    assert_eq!(size_of::<FILEPATHS_SIGNERINFO_W>(), 48);
    assert_eq!(align_of::<FILEPATHS_SIGNERINFO_W>(), 8);
    assert_eq!(size_of::<SOURCE_MEDIA_A>(), 48);
    assert_eq!(align_of::<SOURCE_MEDIA_A>(), 8);
    assert_eq!(size_of::<SOURCE_MEDIA_W>(), 48);
    assert_eq!(align_of::<SOURCE_MEDIA_W>(), 8);
    assert_eq!(size_of::<CABINET_INFO_A>(), 32);
    assert_eq!(align_of::<CABINET_INFO_A>(), 8);
    assert_eq!(size_of::<CABINET_INFO_W>(), 32);
    assert_eq!(align_of::<CABINET_INFO_W>(), 8);
    assert_eq!(size_of::<FILE_IN_CABINET_INFO_A>(), 288);
    assert_eq!(align_of::<FILE_IN_CABINET_INFO_A>(), 8);
    assert_eq!(size_of::<FILE_IN_CABINET_INFO_W>(), 544);
    assert_eq!(align_of::<FILE_IN_CABINET_INFO_W>(), 8);
    assert_eq!(size_of::<SP_REGISTER_CONTROL_STATUSA>(), 24);
    assert_eq!(align_of::<SP_REGISTER_CONTROL_STATUSA>(), 8);
    assert_eq!(size_of::<SP_REGISTER_CONTROL_STATUSW>(), 24);
    assert_eq!(align_of::<SP_REGISTER_CONTROL_STATUSW>(), 8);
    assert_eq!(size_of::<SP_FILE_COPY_PARAMS_A>(), 96);
    assert_eq!(align_of::<SP_FILE_COPY_PARAMS_A>(), 8);
    assert_eq!(size_of::<SP_FILE_COPY_PARAMS_W>(), 96);
    assert_eq!(align_of::<SP_FILE_COPY_PARAMS_W>(), 8);
    assert_eq!(size_of::<SP_DEVINFO_DATA>(), 32);
    assert_eq!(align_of::<SP_DEVINFO_DATA>(), 8);
    assert_eq!(size_of::<SP_DEVICE_INTERFACE_DATA>(), 32);
    assert_eq!(align_of::<SP_DEVICE_INTERFACE_DATA>(), 8);
    assert_eq!(size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>(), 8);
    assert_eq!(align_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>(), 4);
    assert_eq!(size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>(), 8);
    assert_eq!(align_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>(), 4);
    assert_eq!(size_of::<SP_DEVINFO_LIST_DETAIL_DATA_A>(), 296);
    assert_eq!(align_of::<SP_DEVINFO_LIST_DETAIL_DATA_A>(), 8);
    assert_eq!(size_of::<SP_DEVINFO_LIST_DETAIL_DATA_W>(), 560);
    assert_eq!(align_of::<SP_DEVINFO_LIST_DETAIL_DATA_W>(), 8);
    assert_eq!(size_of::<SP_DEVINSTALL_PARAMS_A>(), 320);
    assert_eq!(align_of::<SP_DEVINSTALL_PARAMS_A>(), 8);
    assert_eq!(size_of::<SP_DEVINSTALL_PARAMS_W>(), 584);
    assert_eq!(align_of::<SP_DEVINSTALL_PARAMS_W>(), 8);
    assert_eq!(size_of::<SP_CLASSINSTALL_HEADER>(), 8);
    assert_eq!(align_of::<SP_CLASSINSTALL_HEADER>(), 4);
    assert_eq!(size_of::<SP_ENABLECLASS_PARAMS>(), 28);
    assert_eq!(align_of::<SP_ENABLECLASS_PARAMS>(), 4);
    assert_eq!(size_of::<SP_PROPCHANGE_PARAMS>(), 20);
    assert_eq!(align_of::<SP_PROPCHANGE_PARAMS>(), 4);
    assert_eq!(size_of::<SP_REMOVEDEVICE_PARAMS>(), 16);
    assert_eq!(align_of::<SP_REMOVEDEVICE_PARAMS>(), 4);
    assert_eq!(size_of::<SP_UNREMOVEDEVICE_PARAMS>(), 16);
    assert_eq!(align_of::<SP_UNREMOVEDEVICE_PARAMS>(), 4);
    assert_eq!(size_of::<SP_SELECTDEVICE_PARAMS_A>(), 612);
    assert_eq!(align_of::<SP_SELECTDEVICE_PARAMS_A>(), 4);
    assert_eq!(size_of::<SP_SELECTDEVICE_PARAMS_W>(), 1212);
    assert_eq!(align_of::<SP_SELECTDEVICE_PARAMS_W>(), 4);
    assert_eq!(size_of::<SP_DETECTDEVICE_PARAMS>(), 24);
    assert_eq!(align_of::<SP_DETECTDEVICE_PARAMS>(), 8);
    assert_eq!(size_of::<SP_INSTALLWIZARD_DATA>(), 208);
    assert_eq!(align_of::<SP_INSTALLWIZARD_DATA>(), 8);
    assert_eq!(size_of::<SP_NEWDEVICEWIZARD_DATA>(), 192);
    assert_eq!(align_of::<SP_NEWDEVICEWIZARD_DATA>(), 8);
    assert_eq!(size_of::<SP_TROUBLESHOOTER_PARAMS_A>(), 528);
    assert_eq!(align_of::<SP_TROUBLESHOOTER_PARAMS_A>(), 4);
    assert_eq!(size_of::<SP_TROUBLESHOOTER_PARAMS_W>(), 1048);
    assert_eq!(align_of::<SP_TROUBLESHOOTER_PARAMS_W>(), 4);
    assert_eq!(size_of::<SP_POWERMESSAGEWAKE_PARAMS_A>(), 520);
    assert_eq!(align_of::<SP_POWERMESSAGEWAKE_PARAMS_A>(), 4);
    assert_eq!(size_of::<SP_POWERMESSAGEWAKE_PARAMS_W>(), 1032);
    assert_eq!(align_of::<SP_POWERMESSAGEWAKE_PARAMS_W>(), 4);
    assert_eq!(size_of::<SP_DRVINFO_DATA_V2_A>(), 800);
    assert_eq!(align_of::<SP_DRVINFO_DATA_V2_A>(), 8);
    assert_eq!(size_of::<SP_DRVINFO_DATA_V2_W>(), 1568);
    assert_eq!(align_of::<SP_DRVINFO_DATA_V2_W>(), 8);
    assert_eq!(size_of::<SP_DRVINFO_DATA_V1_A>(), 784);
    assert_eq!(align_of::<SP_DRVINFO_DATA_V1_A>(), 8);
    assert_eq!(size_of::<SP_DRVINFO_DATA_V1_W>(), 1552);
    assert_eq!(align_of::<SP_DRVINFO_DATA_V1_W>(), 8);
    assert_eq!(size_of::<SP_DRVINFO_DETAIL_DATA_A>(), 808);
    assert_eq!(align_of::<SP_DRVINFO_DETAIL_DATA_A>(), 8);
    assert_eq!(size_of::<SP_DRVINFO_DETAIL_DATA_W>(), 1584);
    assert_eq!(align_of::<SP_DRVINFO_DETAIL_DATA_W>(), 8);
    assert_eq!(size_of::<SP_DRVINSTALL_PARAMS>(), 32);
    assert_eq!(align_of::<SP_DRVINSTALL_PARAMS>(), 8);
    assert_eq!(size_of::<COINSTALLER_CONTEXT_DATA>(), 16);
    assert_eq!(align_of::<COINSTALLER_CONTEXT_DATA>(), 8);
    assert_eq!(size_of::<SP_CLASSIMAGELIST_DATA>(), 24);
    assert_eq!(align_of::<SP_CLASSIMAGELIST_DATA>(), 8);
    assert_eq!(size_of::<SP_PROPSHEETPAGE_REQUEST>(), 24);
    assert_eq!(align_of::<SP_PROPSHEETPAGE_REQUEST>(), 8);
    assert_eq!(size_of::<SP_BACKUP_QUEUE_PARAMS_V2_A>(), 528);
    assert_eq!(align_of::<SP_BACKUP_QUEUE_PARAMS_V2_A>(), 4);
    assert_eq!(size_of::<SP_BACKUP_QUEUE_PARAMS_V2_W>(), 1048);
    assert_eq!(align_of::<SP_BACKUP_QUEUE_PARAMS_V2_W>(), 4);
    assert_eq!(size_of::<SP_BACKUP_QUEUE_PARAMS_V1_A>(), 268);
    assert_eq!(align_of::<SP_BACKUP_QUEUE_PARAMS_V1_A>(), 4);
    assert_eq!(size_of::<SP_BACKUP_QUEUE_PARAMS_V1_W>(), 528);
    assert_eq!(align_of::<SP_BACKUP_QUEUE_PARAMS_V1_W>(), 4);
    assert_eq!(size_of::<SP_INF_SIGNER_INFO_V1_A>(), 784);
    assert_eq!(align_of::<SP_INF_SIGNER_INFO_V1_A>(), 4);
    assert_eq!(size_of::<SP_INF_SIGNER_INFO_V1_W>(), 1564);
    assert_eq!(align_of::<SP_INF_SIGNER_INFO_V1_W>(), 4);
    assert_eq!(size_of::<SP_INF_SIGNER_INFO_V2_A>(), 788);
    assert_eq!(align_of::<SP_INF_SIGNER_INFO_V2_A>(), 4);
    assert_eq!(size_of::<SP_INF_SIGNER_INFO_V2_W>(), 1568);
    assert_eq!(align_of::<SP_INF_SIGNER_INFO_V2_W>(), 4);
}
#[cfg(feature = "shellapi")]
#[test]
fn um_shellapi() {
    use winapi::um::shellapi::*;
    assert_eq!(size_of::<DRAGINFOA>(), 32);
    assert_eq!(align_of::<DRAGINFOA>(), 8);
    assert_eq!(size_of::<DRAGINFOW>(), 32);
    assert_eq!(align_of::<DRAGINFOW>(), 8);
    assert_eq!(size_of::<APPBARDATA>(), 48);
    assert_eq!(align_of::<APPBARDATA>(), 8);
    assert_eq!(size_of::<SHFILEOPSTRUCTA>(), 56);
    assert_eq!(align_of::<SHFILEOPSTRUCTA>(), 8);
    assert_eq!(size_of::<SHFILEOPSTRUCTW>(), 56);
    assert_eq!(align_of::<SHFILEOPSTRUCTW>(), 8);
    assert_eq!(size_of::<SHNAMEMAPPINGA>(), 24);
    assert_eq!(align_of::<SHNAMEMAPPINGA>(), 8);
    assert_eq!(size_of::<SHNAMEMAPPINGW>(), 24);
    assert_eq!(align_of::<SHNAMEMAPPINGW>(), 8);
    assert_eq!(size_of::<SHELLEXECUTEINFOA>(), 112);
    assert_eq!(align_of::<SHELLEXECUTEINFOA>(), 8);
    assert_eq!(size_of::<SHELLEXECUTEINFOW>(), 112);
    assert_eq!(align_of::<SHELLEXECUTEINFOW>(), 8);
    assert_eq!(size_of::<SHCREATEPROCESSINFOW>(), 88);
    assert_eq!(align_of::<SHCREATEPROCESSINFOW>(), 8);
    assert_eq!(size_of::<ASSOCIATIONELEMENT>(), 24);
    assert_eq!(align_of::<ASSOCIATIONELEMENT>(), 8);
    assert_eq!(size_of::<SHQUERYRBINFO>(), 24);
    assert_eq!(align_of::<SHQUERYRBINFO>(), 8);
    assert_eq!(size_of::<NOTIFYICONDATAA_u>(), 4);
    assert_eq!(align_of::<NOTIFYICONDATAA_u>(), 4);
    assert_eq!(size_of::<NOTIFYICONDATAA>(), 528);
    assert_eq!(align_of::<NOTIFYICONDATAA>(), 8);
    assert_eq!(size_of::<NOTIFYICONDATAW_u>(), 4);
    assert_eq!(align_of::<NOTIFYICONDATAW_u>(), 4);
    assert_eq!(size_of::<NOTIFYICONDATAW>(), 976);
    assert_eq!(align_of::<NOTIFYICONDATAW>(), 8);
    assert_eq!(size_of::<NOTIFYICONIDENTIFIER>(), 40);
    assert_eq!(align_of::<NOTIFYICONIDENTIFIER>(), 8);
    assert_eq!(size_of::<SHFILEINFOA>(), 360);
    assert_eq!(align_of::<SHFILEINFOA>(), 8);
    assert_eq!(size_of::<SHFILEINFOW>(), 696);
    assert_eq!(align_of::<SHFILEINFOW>(), 8);
    assert_eq!(size_of::<SHSTOCKICONINFO>(), 544);
    assert_eq!(align_of::<SHSTOCKICONINFO>(), 8);
    assert_eq!(size_of::<OPEN_PRINTER_PROPS_INFOA>(), 32);
    assert_eq!(align_of::<OPEN_PRINTER_PROPS_INFOA>(), 8);
    assert_eq!(size_of::<OPEN_PRINTER_PROPS_INFOW>(), 32);
    assert_eq!(align_of::<OPEN_PRINTER_PROPS_INFOW>(), 8);
}
#[cfg(feature = "shobjidl_core")]
#[test]
fn um_shobjidl_core() {
    use winapi::um::shobjidl_core::*;
    assert_eq!(size_of::<THUMBBUTTON>(), 552);
    assert_eq!(align_of::<THUMBBUTTON>(), 8);
}
#[cfg(feature = "shtypes")]
#[test]
fn um_shtypes() {
    use winapi::um::shtypes::*;
    assert_eq!(size_of::<SHITEMID>(), 3);
    assert_eq!(align_of::<SHITEMID>(), 1);
    assert_eq!(size_of::<ITEMIDLIST>(), 3);
    assert_eq!(align_of::<ITEMIDLIST>(), 1);
    assert_eq!(size_of::<COMDLG_FILTERSPEC>(), 16);
    assert_eq!(align_of::<COMDLG_FILTERSPEC>(), 8);
}
#[cfg(feature = "sqltypes")]
#[test]
fn um_sqltypes() {
    use winapi::um::sqltypes::*;
    assert_eq!(size_of::<DATE_STRUCT>(), 6);
    assert_eq!(align_of::<DATE_STRUCT>(), 2);
    assert_eq!(size_of::<TIME_STRUCT>(), 6);
    assert_eq!(align_of::<TIME_STRUCT>(), 2);
    assert_eq!(size_of::<TIMESTAMP_STRUCT>(), 16);
    assert_eq!(align_of::<TIMESTAMP_STRUCT>(), 4);
    assert_eq!(size_of::<SQL_YEAR_MONTH_STRUCT>(), 8);
    assert_eq!(align_of::<SQL_YEAR_MONTH_STRUCT>(), 4);
    assert_eq!(size_of::<SQL_DAY_SECOND_STRUCT>(), 20);
    assert_eq!(align_of::<SQL_DAY_SECOND_STRUCT>(), 4);
    assert_eq!(size_of::<SQL_INTERVAL_STRUCT_intval>(), 20);
    assert_eq!(align_of::<SQL_INTERVAL_STRUCT_intval>(), 4);
    assert_eq!(size_of::<SQL_INTERVAL_STRUCT>(), 28);
    assert_eq!(align_of::<SQL_INTERVAL_STRUCT>(), 4);
    assert_eq!(size_of::<SQL_NUMERIC_STRUCT>(), 19);
    assert_eq!(align_of::<SQL_NUMERIC_STRUCT>(), 1);
}
#[cfg(feature = "subauth")]
#[test]
fn um_subauth() {
    use winapi::um::subauth::*;
    assert_eq!(size_of::<UNICODE_STRING>(), 16);
    assert_eq!(align_of::<UNICODE_STRING>(), 8);
    assert_eq!(size_of::<STRING>(), 16);
    assert_eq!(align_of::<STRING>(), 8);
    assert_eq!(size_of::<OLD_LARGE_INTEGER>(), 8);
    assert_eq!(align_of::<OLD_LARGE_INTEGER>(), 4);
    assert_eq!(size_of::<LOGON_HOURS>(), 16);
    assert_eq!(align_of::<LOGON_HOURS>(), 8);
    assert_eq!(size_of::<SR_SECURITY_DESCRIPTOR>(), 16);
    assert_eq!(align_of::<SR_SECURITY_DESCRIPTOR>(), 8);
    // packed = 4
    // assert_eq!(size_of::<USER_ALL_INFORMATION>(), 316);
    // assert_eq!(align_of::<USER_ALL_INFORMATION>(), 4);
    assert_eq!(size_of::<CLEAR_BLOCK>(), 8);
    assert_eq!(align_of::<CLEAR_BLOCK>(), 1);
    assert_eq!(size_of::<CYPHER_BLOCK>(), 8);
    assert_eq!(align_of::<CYPHER_BLOCK>(), 1);
    assert_eq!(size_of::<LM_OWF_PASSWORD>(), 16);
    assert_eq!(align_of::<LM_OWF_PASSWORD>(), 1);
    assert_eq!(size_of::<USER_SESSION_KEY>(), 16);
    assert_eq!(align_of::<USER_SESSION_KEY>(), 1);
    assert_eq!(size_of::<NETLOGON_LOGON_IDENTITY_INFO>(), 64);
    assert_eq!(align_of::<NETLOGON_LOGON_IDENTITY_INFO>(), 8);
    assert_eq!(size_of::<NETLOGON_INTERACTIVE_INFO>(), 96);
    assert_eq!(align_of::<NETLOGON_INTERACTIVE_INFO>(), 8);
    assert_eq!(size_of::<NETLOGON_SERVICE_INFO>(), 96);
    assert_eq!(align_of::<NETLOGON_SERVICE_INFO>(), 8);
    assert_eq!(size_of::<NETLOGON_NETWORK_INFO>(), 104);
    assert_eq!(align_of::<NETLOGON_NETWORK_INFO>(), 8);
    assert_eq!(size_of::<NETLOGON_GENERIC_INFO>(), 96);
    assert_eq!(align_of::<NETLOGON_GENERIC_INFO>(), 8);
    assert_eq!(size_of::<MSV1_0_VALIDATION_INFO>(), 80);
    assert_eq!(align_of::<MSV1_0_VALIDATION_INFO>(), 8);
}
#[cfg(feature = "sysinfoapi")]
#[test]
fn um_sysinfoapi() {
    use winapi::um::sysinfoapi::*;
    assert_eq!(size_of::<SYSTEM_INFO_u_s>(), 4);
    assert_eq!(align_of::<SYSTEM_INFO_u_s>(), 2);
    assert_eq!(size_of::<SYSTEM_INFO_u>(), 4);
    assert_eq!(align_of::<SYSTEM_INFO_u>(), 4);
    assert_eq!(size_of::<SYSTEM_INFO>(), 48);
    assert_eq!(align_of::<SYSTEM_INFO>(), 8);
    assert_eq!(size_of::<MEMORYSTATUSEX>(), 64);
    assert_eq!(align_of::<MEMORYSTATUSEX>(), 8);
}
#[cfg(feature = "timezoneapi")]
#[test]
fn um_timezoneapi() {
    use winapi::um::timezoneapi::*;
    assert_eq!(size_of::<TIME_ZONE_INFORMATION>(), 172);
    assert_eq!(align_of::<TIME_ZONE_INFORMATION>(), 4);
    assert_eq!(size_of::<DYNAMIC_TIME_ZONE_INFORMATION>(), 432);
    assert_eq!(align_of::<DYNAMIC_TIME_ZONE_INFORMATION>(), 4);
}
#[cfg(feature = "tlhelp32")]
#[test]
fn um_tlhelp32() {
    use winapi::um::tlhelp32::*;
    assert_eq!(size_of::<HEAPLIST32>(), 32);
    assert_eq!(align_of::<HEAPLIST32>(), 8);
    assert_eq!(size_of::<HEAPENTRY32>(), 56);
    assert_eq!(align_of::<HEAPENTRY32>(), 8);
    assert_eq!(size_of::<PROCESSENTRY32W>(), 568);
    assert_eq!(align_of::<PROCESSENTRY32W>(), 8);
    assert_eq!(size_of::<PROCESSENTRY32>(), 304);
    assert_eq!(align_of::<PROCESSENTRY32>(), 8);
    assert_eq!(size_of::<THREADENTRY32>(), 28);
    assert_eq!(align_of::<THREADENTRY32>(), 4);
    assert_eq!(size_of::<MODULEENTRY32W>(), 1080);
    assert_eq!(align_of::<MODULEENTRY32W>(), 8);
    assert_eq!(size_of::<MODULEENTRY32>(), 568);
    assert_eq!(align_of::<MODULEENTRY32>(), 8);
}
#[cfg(feature = "urlhist")]
#[test]
fn um_urlhist() {
    use winapi::um::urlhist::*;
    assert_eq!(size_of::<STATURL>(), 56);
    assert_eq!(align_of::<STATURL>(), 8);
}
#[cfg(feature = "usp10")]
#[test]
fn um_usp10() {
    use winapi::um::usp10::*;
    assert_eq!(size_of::<SCRIPT_CONTROL>(), 4);
    assert_eq!(align_of::<SCRIPT_CONTROL>(), 4);
    assert_eq!(size_of::<SCRIPT_STATE>(), 2);
    assert_eq!(align_of::<SCRIPT_STATE>(), 2);
    assert_eq!(size_of::<SCRIPT_ANALYSIS>(), 4);
    assert_eq!(align_of::<SCRIPT_ANALYSIS>(), 2);
    assert_eq!(size_of::<SCRIPT_ITEM>(), 8);
    assert_eq!(align_of::<SCRIPT_ITEM>(), 4);
    assert_eq!(size_of::<SCRIPT_VISATTR>(), 2);
    assert_eq!(align_of::<SCRIPT_VISATTR>(), 2);
    assert_eq!(size_of::<GOFFSET>(), 8);
    assert_eq!(align_of::<GOFFSET>(), 4);
    assert_eq!(size_of::<SCRIPT_LOGATTR>(), 1);
    assert_eq!(align_of::<SCRIPT_LOGATTR>(), 1);
    assert_eq!(size_of::<SCRIPT_PROPERTIES>(), 8);
    assert_eq!(align_of::<SCRIPT_PROPERTIES>(), 4);
    assert_eq!(size_of::<SCRIPT_FONTPROPERTIES>(), 16);
    assert_eq!(align_of::<SCRIPT_FONTPROPERTIES>(), 4);
    assert_eq!(size_of::<SCRIPT_TABDEF>(), 24);
    assert_eq!(align_of::<SCRIPT_TABDEF>(), 8);
    assert_eq!(size_of::<SCRIPT_DIGITSUBSTITUTE>(), 12);
    assert_eq!(align_of::<SCRIPT_DIGITSUBSTITUTE>(), 4);
    assert_eq!(size_of::<OPENTYPE_FEATURE_RECORD>(), 8);
    assert_eq!(align_of::<OPENTYPE_FEATURE_RECORD>(), 4);
    assert_eq!(size_of::<TEXTRANGE_PROPERTIES>(), 16);
    assert_eq!(align_of::<TEXTRANGE_PROPERTIES>(), 8);
    assert_eq!(size_of::<SCRIPT_CHARPROP>(), 2);
    assert_eq!(align_of::<SCRIPT_CHARPROP>(), 2);
    assert_eq!(size_of::<SCRIPT_GLYPHPROP>(), 4);
    assert_eq!(align_of::<SCRIPT_GLYPHPROP>(), 2);
}
#[cfg(feature = "uxtheme")]
#[test]
fn um_uxtheme() {
    use winapi::um::uxtheme::*;
    assert_eq!(size_of::<TA_TRANSFORM>(), 20);
    assert_eq!(align_of::<TA_TRANSFORM>(), 4);
    assert_eq!(size_of::<TA_TRANSFORM_2D>(), 44);
    assert_eq!(align_of::<TA_TRANSFORM_2D>(), 4);
    assert_eq!(size_of::<TA_TRANSFORM_OPACITY>(), 28);
    assert_eq!(align_of::<TA_TRANSFORM_OPACITY>(), 4);
    assert_eq!(size_of::<TA_TRANSFORM_CLIP>(), 52);
    assert_eq!(align_of::<TA_TRANSFORM_CLIP>(), 4);
    assert_eq!(size_of::<TA_TIMINGFUNCTION>(), 4);
    assert_eq!(align_of::<TA_TIMINGFUNCTION>(), 4);
    assert_eq!(size_of::<TA_CUBIC_BEZIER>(), 20);
    assert_eq!(align_of::<TA_CUBIC_BEZIER>(), 4);
    assert_eq!(size_of::<DTBGOPTS>(), 24);
    assert_eq!(align_of::<DTBGOPTS>(), 4);
    assert_eq!(size_of::<MARGINS>(), 16);
    assert_eq!(align_of::<MARGINS>(), 4);
    assert_eq!(size_of::<INTLIST>(), 1612);
    assert_eq!(align_of::<INTLIST>(), 4);
    assert_eq!(size_of::<WTA_OPTIONS>(), 8);
    assert_eq!(align_of::<WTA_OPTIONS>(), 4);
    assert_eq!(size_of::<DTTOPTS>(), 72);
    assert_eq!(align_of::<DTTOPTS>(), 8);
    assert_eq!(size_of::<BP_ANIMATIONPARAMS>(), 16);
    assert_eq!(align_of::<BP_ANIMATIONPARAMS>(), 4);
    assert_eq!(size_of::<BP_PAINTPARAMS>(), 24);
    assert_eq!(align_of::<BP_PAINTPARAMS>(), 8);
}
#[cfg(feature = "vsbackup")]
#[test]
fn um_vsbackup() {
    use winapi::um::vsbackup::*;
    assert_eq!(size_of::<VSS_COMPONENTINFO>(), 72);
    assert_eq!(align_of::<VSS_COMPONENTINFO>(), 8);
}
#[cfg(feature = "vss")]
#[test]
fn um_vss() {
    use winapi::um::vss::*;
    assert_eq!(size_of::<VSS_SNAPSHOT_PROP>(), 128);
    assert_eq!(align_of::<VSS_SNAPSHOT_PROP>(), 8);
    assert_eq!(size_of::<VSS_PROVIDER_PROP>(), 72);
    assert_eq!(align_of::<VSS_PROVIDER_PROP>(), 8);
    assert_eq!(size_of::<VSS_OBJECT_UNION>(), 128);
    assert_eq!(align_of::<VSS_OBJECT_UNION>(), 8);
    assert_eq!(size_of::<VSS_OBJECT_PROP>(), 136);
    assert_eq!(align_of::<VSS_OBJECT_PROP>(), 8);
}
#[cfg(feature = "wbemcli")]
#[test]
fn um_wbemcli() {
    use winapi::um::wbemcli::*;
    assert_eq!(size_of::<WBEM_COMPILE_STATUS_INFO>(), 24);
    assert_eq!(align_of::<WBEM_COMPILE_STATUS_INFO>(), 4);
}
#[cfg(feature = "wct")]
#[test]
fn um_wct() {
    use winapi::um::wct::*;
    assert_eq!(size_of::<WAITCHAIN_NODE_INFO_LOCK_OBJECT>(), 272);
    assert_eq!(align_of::<WAITCHAIN_NODE_INFO_LOCK_OBJECT>(), 8);
    assert_eq!(size_of::<WAITCHAIN_NODE_INFO_THREAD_OBJECT>(), 16);
    assert_eq!(align_of::<WAITCHAIN_NODE_INFO_THREAD_OBJECT>(), 4);
    assert_eq!(size_of::<WAITCHAIN_NODE_INFO>(), 280);
    assert_eq!(align_of::<WAITCHAIN_NODE_INFO>(), 8);
}
#[cfg(feature = "winbase")]
#[test]
fn um_winbase() {
    use winapi::um::winbase::*;
    assert_eq!(size_of::<COMMPROP>(), 64);
    assert_eq!(align_of::<COMMPROP>(), 4);
    assert_eq!(size_of::<COMSTAT>(), 12);
    assert_eq!(align_of::<COMSTAT>(), 4);
    assert_eq!(size_of::<DCB>(), 28);
    assert_eq!(align_of::<DCB>(), 4);
    assert_eq!(size_of::<COMMTIMEOUTS>(), 20);
    assert_eq!(align_of::<COMMTIMEOUTS>(), 4);
    assert_eq!(size_of::<COMMCONFIG>(), 52);
    assert_eq!(align_of::<COMMCONFIG>(), 4);
    assert_eq!(size_of::<MEMORYSTATUS>(), 56);
    assert_eq!(align_of::<MEMORYSTATUS>(), 8);
    assert_eq!(size_of::<OFSTRUCT>(), 136);
    assert_eq!(align_of::<OFSTRUCT>(), 2);
    assert_eq!(size_of::<UMS_SCHEDULER_STARTUP_INFO>(), 32);
    assert_eq!(align_of::<UMS_SCHEDULER_STARTUP_INFO>(), 8);
    assert_eq!(size_of::<UMS_SYSTEM_THREAD_INFORMATION>(), 8);
    assert_eq!(align_of::<UMS_SYSTEM_THREAD_INFORMATION>(), 4);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_ChunkStarted>(), 56);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_ChunkStarted>(), 8);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_ChunkFinished>(), 72);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_ChunkFinished>(), 8);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_StreamStarted>(), 40);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_StreamStarted>(), 8);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_StreamFinished>(), 56);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_StreamFinished>(), 8);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_PollContinue>(), 4);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_PollContinue>(), 4);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_Error>(), 56);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_Error>(), 8);
    assert_eq!(size_of::<COPYFILE2_MESSAGE_Info>(), 72);
    assert_eq!(align_of::<COPYFILE2_MESSAGE_Info>(), 8);
    assert_eq!(size_of::<COPYFILE2_MESSAGE>(), 80);
    assert_eq!(align_of::<COPYFILE2_MESSAGE>(), 8);
    assert_eq!(size_of::<COPYFILE2_EXTENDED_PARAMETERS>(), 32);
    assert_eq!(align_of::<COPYFILE2_EXTENDED_PARAMETERS>(), 8);
    assert_eq!(size_of::<HW_PROFILE_INFOA>(), 124);
    assert_eq!(align_of::<HW_PROFILE_INFOA>(), 4);
    assert_eq!(size_of::<HW_PROFILE_INFOW>(), 244);
    assert_eq!(align_of::<HW_PROFILE_INFOW>(), 4);
    assert_eq!(size_of::<SYSTEM_POWER_STATUS>(), 12);
    assert_eq!(align_of::<SYSTEM_POWER_STATUS>(), 4);
    assert_eq!(size_of::<ACTCTXA>(), 56);
    assert_eq!(align_of::<ACTCTXA>(), 8);
    assert_eq!(size_of::<ACTCTXW>(), 56);
    assert_eq!(align_of::<ACTCTXW>(), 8);
    assert_eq!(size_of::<ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA>(), 40);
    assert_eq!(align_of::<ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA>(), 8);
    assert_eq!(size_of::<ACTCTX_SECTION_KEYED_DATA>(), 112);
    assert_eq!(align_of::<ACTCTX_SECTION_KEYED_DATA>(), 8);
    assert_eq!(size_of::<FILE_ID_DESCRIPTOR_u>(), 16);
    assert_eq!(align_of::<FILE_ID_DESCRIPTOR_u>(), 8);
    assert_eq!(size_of::<FILE_ID_DESCRIPTOR>(), 24);
    assert_eq!(align_of::<FILE_ID_DESCRIPTOR>(), 8);
}
#[cfg(feature = "wincodec")]
#[test]
fn um_wincodec() {
    use winapi::um::wincodec::*;
    assert_eq!(size_of::<WICRect>(), 16);
    assert_eq!(align_of::<WICRect>(), 4);
    assert_eq!(size_of::<WICBitmapPattern>(), 40);
    assert_eq!(align_of::<WICBitmapPattern>(), 8);
    assert_eq!(size_of::<WICImageParameters>(), 32);
    assert_eq!(align_of::<WICImageParameters>(), 4);
    assert_eq!(size_of::<WICBitmapPlaneDescription>(), 24);
    assert_eq!(align_of::<WICBitmapPlaneDescription>(), 4);
    assert_eq!(size_of::<WICBitmapPlane>(), 32);
    assert_eq!(align_of::<WICBitmapPlane>(), 8);
    assert_eq!(size_of::<WICJpegFrameHeader>(), 32);
    assert_eq!(align_of::<WICJpegFrameHeader>(), 4);
    assert_eq!(size_of::<WICJpegScanHeader>(), 20);
    assert_eq!(align_of::<WICJpegScanHeader>(), 4);
    assert_eq!(size_of::<WICRawCapabilitiesInfo>(), 72);
    assert_eq!(align_of::<WICRawCapabilitiesInfo>(), 4);
    assert_eq!(size_of::<WICRawToneCurvePoint>(), 16);
    assert_eq!(align_of::<WICRawToneCurvePoint>(), 8);
    assert_eq!(size_of::<WICRawToneCurve>(), 24);
    assert_eq!(align_of::<WICRawToneCurve>(), 8);
    assert_eq!(size_of::<WICDdsParameters>(), 32);
    assert_eq!(align_of::<WICDdsParameters>(), 4);
    assert_eq!(size_of::<WICDdsFormatInfo>(), 16);
    assert_eq!(align_of::<WICDdsFormatInfo>(), 4);
}
#[cfg(feature = "wincodecsdk")]
#[test]
fn um_wincodecsdk() {
    use winapi::um::wincodecsdk::*;
    assert_eq!(size_of::<WICMetadataPattern>(), 40);
    assert_eq!(align_of::<WICMetadataPattern>(), 8);
    assert_eq!(size_of::<WICMetadataHeader>(), 32);
    assert_eq!(align_of::<WICMetadataHeader>(), 8);
}
#[cfg(feature = "wincon")]
#[test]
fn um_wincon() {
    use winapi::um::wincon::*;
    assert_eq!(size_of::<COORD>(), 4);
    assert_eq!(align_of::<COORD>(), 2);
    assert_eq!(size_of::<SMALL_RECT>(), 8);
    assert_eq!(align_of::<SMALL_RECT>(), 2);
    assert_eq!(size_of::<KEY_EVENT_RECORD_uChar>(), 2);
    assert_eq!(align_of::<KEY_EVENT_RECORD_uChar>(), 2);
    assert_eq!(size_of::<KEY_EVENT_RECORD>(), 16);
    assert_eq!(align_of::<KEY_EVENT_RECORD>(), 4);
    assert_eq!(size_of::<MOUSE_EVENT_RECORD>(), 16);
    assert_eq!(align_of::<MOUSE_EVENT_RECORD>(), 4);
    assert_eq!(size_of::<WINDOW_BUFFER_SIZE_RECORD>(), 4);
    assert_eq!(align_of::<WINDOW_BUFFER_SIZE_RECORD>(), 2);
    assert_eq!(size_of::<MENU_EVENT_RECORD>(), 4);
    assert_eq!(align_of::<MENU_EVENT_RECORD>(), 4);
    assert_eq!(size_of::<FOCUS_EVENT_RECORD>(), 4);
    assert_eq!(align_of::<FOCUS_EVENT_RECORD>(), 4);
    assert_eq!(size_of::<INPUT_RECORD_Event>(), 16);
    assert_eq!(align_of::<INPUT_RECORD_Event>(), 4);
    assert_eq!(size_of::<INPUT_RECORD>(), 20);
    assert_eq!(align_of::<INPUT_RECORD>(), 4);
    assert_eq!(size_of::<CHAR_INFO_Char>(), 2);
    assert_eq!(align_of::<CHAR_INFO_Char>(), 2);
    assert_eq!(size_of::<CHAR_INFO>(), 4);
    assert_eq!(align_of::<CHAR_INFO>(), 2);
    assert_eq!(size_of::<CONSOLE_SCREEN_BUFFER_INFO>(), 22);
    assert_eq!(align_of::<CONSOLE_SCREEN_BUFFER_INFO>(), 2);
    assert_eq!(size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>(), 96);
    assert_eq!(align_of::<CONSOLE_SCREEN_BUFFER_INFOEX>(), 4);
    assert_eq!(size_of::<CONSOLE_CURSOR_INFO>(), 8);
    assert_eq!(align_of::<CONSOLE_CURSOR_INFO>(), 4);
    assert_eq!(size_of::<CONSOLE_FONT_INFO>(), 8);
    assert_eq!(align_of::<CONSOLE_FONT_INFO>(), 4);
    assert_eq!(size_of::<CONSOLE_FONT_INFOEX>(), 84);
    assert_eq!(align_of::<CONSOLE_FONT_INFOEX>(), 4);
    assert_eq!(size_of::<CONSOLE_HISTORY_INFO>(), 16);
    assert_eq!(align_of::<CONSOLE_HISTORY_INFO>(), 4);
    assert_eq!(size_of::<CONSOLE_SELECTION_INFO>(), 16);
    assert_eq!(align_of::<CONSOLE_SELECTION_INFO>(), 4);
    assert_eq!(size_of::<CONSOLE_READCONSOLE_CONTROL>(), 16);
    assert_eq!(align_of::<CONSOLE_READCONSOLE_CONTROL>(), 4);
}
#[cfg(feature = "wincred")]
#[test]
fn um_wincred() {
    use winapi::um::wincred::*;
    assert_eq!(size_of::<CREDENTIAL_ATTRIBUTEA>(), 24);
    assert_eq!(align_of::<CREDENTIAL_ATTRIBUTEA>(), 8);
    assert_eq!(size_of::<CREDENTIAL_ATTRIBUTEW>(), 24);
    assert_eq!(align_of::<CREDENTIAL_ATTRIBUTEW>(), 8);
    assert_eq!(size_of::<CREDENTIALA>(), 80);
    assert_eq!(align_of::<CREDENTIALA>(), 8);
    assert_eq!(size_of::<CREDENTIALW>(), 80);
    assert_eq!(align_of::<CREDENTIALW>(), 8);
    assert_eq!(size_of::<CREDENTIAL_TARGET_INFORMATIONA>(), 72);
    assert_eq!(align_of::<CREDENTIAL_TARGET_INFORMATIONA>(), 8);
    assert_eq!(size_of::<CREDENTIAL_TARGET_INFORMATIONW>(), 72);
    assert_eq!(align_of::<CREDENTIAL_TARGET_INFORMATIONW>(), 8);
    assert_eq!(size_of::<CERT_CREDENTIAL_INFO>(), 24);
    assert_eq!(align_of::<CERT_CREDENTIAL_INFO>(), 4);
    assert_eq!(size_of::<USERNAME_TARGET_CREDENTIAL_INFO>(), 8);
    assert_eq!(align_of::<USERNAME_TARGET_CREDENTIAL_INFO>(), 8);
    assert_eq!(size_of::<BINARY_BLOB_CREDENTIAL_INFO>(), 16);
    assert_eq!(align_of::<BINARY_BLOB_CREDENTIAL_INFO>(), 8);
    assert_eq!(size_of::<CREDUI_INFOA>(), 40);
    assert_eq!(align_of::<CREDUI_INFOA>(), 8);
    assert_eq!(size_of::<CREDUI_INFOW>(), 40);
    assert_eq!(align_of::<CREDUI_INFOW>(), 8);
}
#[cfg(feature = "wincrypt")]
#[test]
fn um_wincrypt() {
    use winapi::um::wincrypt::*;
    assert_eq!(size_of::<CMS_KEY_INFO>(), 24);
    assert_eq!(align_of::<CMS_KEY_INFO>(), 8);
    assert_eq!(size_of::<HMAC_INFO>(), 40);
    assert_eq!(align_of::<HMAC_INFO>(), 8);
    assert_eq!(size_of::<SCHANNEL_ALG>(), 20);
    assert_eq!(align_of::<SCHANNEL_ALG>(), 4);
    assert_eq!(size_of::<PROV_ENUMALGS>(), 32);
    assert_eq!(align_of::<PROV_ENUMALGS>(), 4);
    assert_eq!(size_of::<PROV_ENUMALGS_EX>(), 88);
    assert_eq!(align_of::<PROV_ENUMALGS_EX>(), 4);
    assert_eq!(size_of::<BLOBHEADER>(), 8);
    assert_eq!(align_of::<BLOBHEADER>(), 4);
    assert_eq!(size_of::<RSAPUBKEY>(), 12);
    assert_eq!(align_of::<RSAPUBKEY>(), 4);
    assert_eq!(size_of::<DHPUBKEY>(), 8);
    assert_eq!(align_of::<DHPUBKEY>(), 4);
    assert_eq!(size_of::<DSSSEED>(), 24);
    assert_eq!(align_of::<DSSSEED>(), 4);
    assert_eq!(size_of::<DHPUBKEY_VER3>(), 40);
    assert_eq!(align_of::<DHPUBKEY_VER3>(), 4);
    assert_eq!(size_of::<DHPRIVKEY_VER3>(), 44);
    assert_eq!(align_of::<DHPRIVKEY_VER3>(), 4);
    assert_eq!(size_of::<KEY_TYPE_SUBTYPE>(), 36);
    assert_eq!(align_of::<KEY_TYPE_SUBTYPE>(), 4);
    assert_eq!(size_of::<CERT_FORTEZZA_DATA_PROP>(), 48);
    assert_eq!(align_of::<CERT_FORTEZZA_DATA_PROP>(), 4);
    assert_eq!(size_of::<CRYPT_RC4_KEY_STATE>(), 274);
    assert_eq!(align_of::<CRYPT_RC4_KEY_STATE>(), 1);
    assert_eq!(size_of::<CRYPT_DES_KEY_STATE>(), 24);
    assert_eq!(align_of::<CRYPT_DES_KEY_STATE>(), 1);
    assert_eq!(size_of::<CRYPT_3DES_KEY_STATE>(), 40);
    assert_eq!(align_of::<CRYPT_3DES_KEY_STATE>(), 1);
    assert_eq!(size_of::<CRYPT_AES_128_KEY_STATE>(), 400);
    assert_eq!(align_of::<CRYPT_AES_128_KEY_STATE>(), 1);
    assert_eq!(size_of::<CRYPT_AES_256_KEY_STATE>(), 544);
    assert_eq!(align_of::<CRYPT_AES_256_KEY_STATE>(), 1);
    assert_eq!(size_of::<CRYPTOAPI_BLOB>(), 16);
    assert_eq!(align_of::<CRYPTOAPI_BLOB>(), 8);
    assert_eq!(size_of::<CMS_DH_KEY_INFO>(), 40);
    assert_eq!(align_of::<CMS_DH_KEY_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_BIT_BLOB>(), 24);
    assert_eq!(align_of::<CRYPT_BIT_BLOB>(), 8);
    assert_eq!(size_of::<CRYPT_ALGORITHM_IDENTIFIER>(), 24);
    assert_eq!(align_of::<CRYPT_ALGORITHM_IDENTIFIER>(), 8);
    assert_eq!(size_of::<CRYPT_OBJID_TABLE>(), 16);
    assert_eq!(align_of::<CRYPT_OBJID_TABLE>(), 8);
    assert_eq!(size_of::<CRYPT_HASH_INFO>(), 40);
    assert_eq!(align_of::<CRYPT_HASH_INFO>(), 8);
    assert_eq!(size_of::<CERT_EXTENSION>(), 32);
    assert_eq!(align_of::<CERT_EXTENSION>(), 8);
    assert_eq!(size_of::<CRYPT_ATTRIBUTE_TYPE_VALUE>(), 24);
    assert_eq!(align_of::<CRYPT_ATTRIBUTE_TYPE_VALUE>(), 8);
    assert_eq!(size_of::<CRYPT_ATTRIBUTE>(), 24);
    assert_eq!(align_of::<CRYPT_ATTRIBUTE>(), 8);
    assert_eq!(size_of::<CRYPT_ATTRIBUTES>(), 16);
    assert_eq!(align_of::<CRYPT_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<CERT_RDN_ATTR>(), 32);
    assert_eq!(align_of::<CERT_RDN_ATTR>(), 8);
    assert_eq!(size_of::<CERT_RDN>(), 16);
    assert_eq!(align_of::<CERT_RDN>(), 8);
    assert_eq!(size_of::<CERT_NAME_INFO>(), 16);
    assert_eq!(align_of::<CERT_NAME_INFO>(), 8);
    assert_eq!(size_of::<CERT_NAME_VALUE>(), 24);
    assert_eq!(align_of::<CERT_NAME_VALUE>(), 8);
    assert_eq!(size_of::<CERT_PUBLIC_KEY_INFO>(), 48);
    assert_eq!(align_of::<CERT_PUBLIC_KEY_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_ECC_PRIVATE_KEY_INFO>(), 56);
    assert_eq!(align_of::<CRYPT_ECC_PRIVATE_KEY_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_PRIVATE_KEY_INFO>(), 56);
    assert_eq!(align_of::<CRYPT_PRIVATE_KEY_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_ENCRYPTED_PRIVATE_KEY_INFO>(), 40);
    assert_eq!(align_of::<CRYPT_ENCRYPTED_PRIVATE_KEY_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_PKCS8_IMPORT_PARAMS>(), 48);
    assert_eq!(align_of::<CRYPT_PKCS8_IMPORT_PARAMS>(), 8);
    assert_eq!(size_of::<CRYPT_PKCS8_EXPORT_PARAMS>(), 40);
    assert_eq!(align_of::<CRYPT_PKCS8_EXPORT_PARAMS>(), 8);
    assert_eq!(size_of::<CERT_INFO>(), 208);
    assert_eq!(align_of::<CERT_INFO>(), 8);
    assert_eq!(size_of::<CRL_ENTRY>(), 40);
    assert_eq!(align_of::<CRL_ENTRY>(), 8);
    assert_eq!(size_of::<CRL_INFO>(), 96);
    assert_eq!(align_of::<CRL_INFO>(), 8);
    assert_eq!(size_of::<CERT_OR_CRL_BLOB>(), 16);
    assert_eq!(align_of::<CERT_OR_CRL_BLOB>(), 8);
    assert_eq!(size_of::<CERT_OR_CRL_BUNDLE>(), 16);
    assert_eq!(align_of::<CERT_OR_CRL_BUNDLE>(), 8);
    assert_eq!(size_of::<CERT_REQUEST_INFO>(), 88);
    assert_eq!(align_of::<CERT_REQUEST_INFO>(), 8);
    assert_eq!(size_of::<CERT_KEYGEN_REQUEST_INFO>(), 64);
    assert_eq!(align_of::<CERT_KEYGEN_REQUEST_INFO>(), 8);
    assert_eq!(size_of::<CERT_SIGNED_CONTENT_INFO>(), 64);
    assert_eq!(align_of::<CERT_SIGNED_CONTENT_INFO>(), 8);
    assert_eq!(size_of::<CTL_USAGE>(), 16);
    assert_eq!(align_of::<CTL_USAGE>(), 8);
    assert_eq!(size_of::<CTL_ENTRY>(), 32);
    assert_eq!(align_of::<CTL_ENTRY>(), 8);
    assert_eq!(size_of::<CTL_INFO>(), 128);
    assert_eq!(align_of::<CTL_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_TIME_STAMP_REQUEST_INFO>(), 48);
    assert_eq!(align_of::<CRYPT_TIME_STAMP_REQUEST_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_ENROLLMENT_NAME_VALUE_PAIR>(), 16);
    assert_eq!(align_of::<CRYPT_ENROLLMENT_NAME_VALUE_PAIR>(), 8);
    assert_eq!(size_of::<CRYPT_CSP_PROVIDER>(), 40);
    assert_eq!(align_of::<CRYPT_CSP_PROVIDER>(), 8);
    assert_eq!(size_of::<CRYPT_ENCODE_PARA>(), 24);
    assert_eq!(align_of::<CRYPT_ENCODE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_DECODE_PARA>(), 24);
    assert_eq!(align_of::<CRYPT_DECODE_PARA>(), 8);
    assert_eq!(size_of::<CERT_EXTENSIONS>(), 16);
    assert_eq!(align_of::<CERT_EXTENSIONS>(), 8);
    assert_eq!(size_of::<CERT_AUTHORITY_KEY_ID_INFO>(), 48);
    assert_eq!(align_of::<CERT_AUTHORITY_KEY_ID_INFO>(), 8);
    assert_eq!(size_of::<CERT_PRIVATE_KEY_VALIDITY>(), 16);
    assert_eq!(align_of::<CERT_PRIVATE_KEY_VALIDITY>(), 4);
    assert_eq!(size_of::<CERT_KEY_ATTRIBUTES_INFO>(), 48);
    assert_eq!(align_of::<CERT_KEY_ATTRIBUTES_INFO>(), 8);
    assert_eq!(size_of::<CERT_POLICY_ID>(), 16);
    assert_eq!(align_of::<CERT_POLICY_ID>(), 8);
    assert_eq!(size_of::<CERT_KEY_USAGE_RESTRICTION_INFO>(), 40);
    assert_eq!(align_of::<CERT_KEY_USAGE_RESTRICTION_INFO>(), 8);
    assert_eq!(size_of::<CERT_OTHER_NAME>(), 24);
    assert_eq!(align_of::<CERT_OTHER_NAME>(), 8);
    assert_eq!(size_of::<CERT_ALT_NAME_ENTRY_u>(), 16);
    assert_eq!(align_of::<CERT_ALT_NAME_ENTRY_u>(), 8);
    assert_eq!(size_of::<CERT_ALT_NAME_ENTRY>(), 24);
    assert_eq!(align_of::<CERT_ALT_NAME_ENTRY>(), 8);
    assert_eq!(size_of::<CERT_ALT_NAME_INFO>(), 16);
    assert_eq!(align_of::<CERT_ALT_NAME_INFO>(), 8);
    assert_eq!(size_of::<CERT_BASIC_CONSTRAINTS_INFO>(), 48);
    assert_eq!(align_of::<CERT_BASIC_CONSTRAINTS_INFO>(), 8);
    assert_eq!(size_of::<CERT_BASIC_CONSTRAINTS2_INFO>(), 12);
    assert_eq!(align_of::<CERT_BASIC_CONSTRAINTS2_INFO>(), 4);
    assert_eq!(size_of::<CERT_POLICY_QUALIFIER_INFO>(), 24);
    assert_eq!(align_of::<CERT_POLICY_QUALIFIER_INFO>(), 8);
    assert_eq!(size_of::<CERT_POLICY_INFO>(), 24);
    assert_eq!(align_of::<CERT_POLICY_INFO>(), 8);
    assert_eq!(size_of::<CERT_POLICIES_INFO>(), 16);
    assert_eq!(align_of::<CERT_POLICIES_INFO>(), 8);
    assert_eq!(size_of::<CERT_POLICY_QUALIFIER_NOTICE_REFERENCE>(), 24);
    assert_eq!(align_of::<CERT_POLICY_QUALIFIER_NOTICE_REFERENCE>(), 8);
    assert_eq!(size_of::<CERT_POLICY_QUALIFIER_USER_NOTICE>(), 16);
    assert_eq!(align_of::<CERT_POLICY_QUALIFIER_USER_NOTICE>(), 8);
    assert_eq!(size_of::<CPS_URLS>(), 24);
    assert_eq!(align_of::<CPS_URLS>(), 8);
    assert_eq!(size_of::<CERT_POLICY95_QUALIFIER1>(), 40);
    assert_eq!(align_of::<CERT_POLICY95_QUALIFIER1>(), 8);
    assert_eq!(size_of::<CERT_POLICY_MAPPING>(), 16);
    assert_eq!(align_of::<CERT_POLICY_MAPPING>(), 8);
    assert_eq!(size_of::<CERT_POLICY_MAPPINGS_INFO>(), 16);
    assert_eq!(align_of::<CERT_POLICY_MAPPINGS_INFO>(), 8);
    assert_eq!(size_of::<CERT_POLICY_CONSTRAINTS_INFO>(), 16);
    assert_eq!(align_of::<CERT_POLICY_CONSTRAINTS_INFO>(), 4);
    assert_eq!(size_of::<CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY>(), 24);
    assert_eq!(align_of::<CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY>(), 8);
    assert_eq!(size_of::<CRYPT_CONTENT_INFO>(), 24);
    assert_eq!(align_of::<CRYPT_CONTENT_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_SEQUENCE_OF_ANY>(), 16);
    assert_eq!(align_of::<CRYPT_SEQUENCE_OF_ANY>(), 8);
    assert_eq!(size_of::<CERT_AUTHORITY_KEY_ID2_INFO>(), 48);
    assert_eq!(align_of::<CERT_AUTHORITY_KEY_ID2_INFO>(), 8);
    assert_eq!(size_of::<CERT_ACCESS_DESCRIPTION>(), 32);
    assert_eq!(align_of::<CERT_ACCESS_DESCRIPTION>(), 8);
    assert_eq!(size_of::<CERT_AUTHORITY_INFO_ACCESS>(), 16);
    assert_eq!(align_of::<CERT_AUTHORITY_INFO_ACCESS>(), 8);
    assert_eq!(size_of::<CRL_DIST_POINT_NAME_u>(), 16);
    assert_eq!(align_of::<CRL_DIST_POINT_NAME_u>(), 8);
    assert_eq!(size_of::<CRL_DIST_POINT_NAME>(), 24);
    assert_eq!(align_of::<CRL_DIST_POINT_NAME>(), 8);
    assert_eq!(size_of::<CRL_DIST_POINT>(), 64);
    assert_eq!(align_of::<CRL_DIST_POINT>(), 8);
    assert_eq!(size_of::<CRL_DIST_POINTS_INFO>(), 16);
    assert_eq!(align_of::<CRL_DIST_POINTS_INFO>(), 8);
    assert_eq!(size_of::<CROSS_CERT_DIST_POINTS_INFO>(), 16);
    assert_eq!(align_of::<CROSS_CERT_DIST_POINTS_INFO>(), 8);
    assert_eq!(size_of::<CERT_PAIR>(), 32);
    assert_eq!(align_of::<CERT_PAIR>(), 8);
    assert_eq!(size_of::<CRL_ISSUING_DIST_POINT>(), 64);
    assert_eq!(align_of::<CRL_ISSUING_DIST_POINT>(), 8);
    assert_eq!(size_of::<CERT_GENERAL_SUBTREE>(), 40);
    assert_eq!(align_of::<CERT_GENERAL_SUBTREE>(), 8);
    assert_eq!(size_of::<CERT_NAME_CONSTRAINTS_INFO>(), 32);
    assert_eq!(align_of::<CERT_NAME_CONSTRAINTS_INFO>(), 8);
    assert_eq!(size_of::<CERT_DSS_PARAMETERS>(), 48);
    assert_eq!(align_of::<CERT_DSS_PARAMETERS>(), 8);
    assert_eq!(size_of::<CERT_DH_PARAMETERS>(), 32);
    assert_eq!(align_of::<CERT_DH_PARAMETERS>(), 8);
    assert_eq!(size_of::<CERT_ECC_SIGNATURE>(), 32);
    assert_eq!(align_of::<CERT_ECC_SIGNATURE>(), 8);
    assert_eq!(size_of::<CERT_X942_DH_VALIDATION_PARAMS>(), 32);
    assert_eq!(align_of::<CERT_X942_DH_VALIDATION_PARAMS>(), 8);
    assert_eq!(size_of::<CERT_X942_DH_PARAMETERS>(), 72);
    assert_eq!(align_of::<CERT_X942_DH_PARAMETERS>(), 8);
    assert_eq!(size_of::<CRYPT_X942_OTHER_INFO>(), 32);
    assert_eq!(align_of::<CRYPT_X942_OTHER_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_ECC_CMS_SHARED_INFO>(), 48);
    assert_eq!(align_of::<CRYPT_ECC_CMS_SHARED_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_RC2_CBC_PARAMETERS>(), 16);
    assert_eq!(align_of::<CRYPT_RC2_CBC_PARAMETERS>(), 4);
    assert_eq!(size_of::<CRYPT_SMIME_CAPABILITY>(), 24);
    assert_eq!(align_of::<CRYPT_SMIME_CAPABILITY>(), 8);
    assert_eq!(size_of::<CRYPT_SMIME_CAPABILITIES>(), 16);
    assert_eq!(align_of::<CRYPT_SMIME_CAPABILITIES>(), 8);
    assert_eq!(size_of::<CERT_QC_STATEMENT>(), 24);
    assert_eq!(align_of::<CERT_QC_STATEMENT>(), 8);
    assert_eq!(size_of::<CERT_QC_STATEMENTS_EXT_INFO>(), 16);
    assert_eq!(align_of::<CERT_QC_STATEMENTS_EXT_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_MASK_GEN_ALGORITHM>(), 32);
    assert_eq!(align_of::<CRYPT_MASK_GEN_ALGORITHM>(), 8);
    assert_eq!(size_of::<CRYPT_RSA_SSA_PSS_PARAMETERS>(), 64);
    assert_eq!(align_of::<CRYPT_RSA_SSA_PSS_PARAMETERS>(), 8);
    assert_eq!(size_of::<CRYPT_PSOURCE_ALGORITHM>(), 24);
    assert_eq!(align_of::<CRYPT_PSOURCE_ALGORITHM>(), 8);
    assert_eq!(size_of::<CRYPT_RSAES_OAEP_PARAMETERS>(), 80);
    assert_eq!(align_of::<CRYPT_RSAES_OAEP_PARAMETERS>(), 8);
    assert_eq!(size_of::<CMC_TAGGED_ATTRIBUTE>(), 32);
    assert_eq!(align_of::<CMC_TAGGED_ATTRIBUTE>(), 8);
    assert_eq!(size_of::<CMC_TAGGED_CERT_REQUEST>(), 24);
    assert_eq!(align_of::<CMC_TAGGED_CERT_REQUEST>(), 8);
    assert_eq!(size_of::<CMC_TAGGED_REQUEST_u>(), 8);
    assert_eq!(align_of::<CMC_TAGGED_REQUEST_u>(), 8);
    assert_eq!(size_of::<CMC_TAGGED_REQUEST>(), 16);
    assert_eq!(align_of::<CMC_TAGGED_REQUEST>(), 8);
    assert_eq!(size_of::<CMC_TAGGED_CONTENT_INFO>(), 24);
    assert_eq!(align_of::<CMC_TAGGED_CONTENT_INFO>(), 8);
    assert_eq!(size_of::<CMC_TAGGED_OTHER_MSG>(), 32);
    assert_eq!(align_of::<CMC_TAGGED_OTHER_MSG>(), 8);
    assert_eq!(size_of::<CMC_DATA_INFO>(), 64);
    assert_eq!(align_of::<CMC_DATA_INFO>(), 8);
    assert_eq!(size_of::<CMC_RESPONSE_INFO>(), 48);
    assert_eq!(align_of::<CMC_RESPONSE_INFO>(), 8);
    assert_eq!(size_of::<CMC_PEND_INFO>(), 24);
    assert_eq!(align_of::<CMC_PEND_INFO>(), 8);
    assert_eq!(size_of::<CMC_STATUS_INFO_u>(), 8);
    assert_eq!(align_of::<CMC_STATUS_INFO_u>(), 8);
    assert_eq!(size_of::<CMC_STATUS_INFO>(), 40);
    assert_eq!(align_of::<CMC_STATUS_INFO>(), 8);
    assert_eq!(size_of::<CMC_ADD_EXTENSIONS_INFO>(), 32);
    assert_eq!(align_of::<CMC_ADD_EXTENSIONS_INFO>(), 8);
    assert_eq!(size_of::<CMC_ADD_ATTRIBUTES_INFO>(), 32);
    assert_eq!(align_of::<CMC_ADD_ATTRIBUTES_INFO>(), 8);
    assert_eq!(size_of::<CERT_TEMPLATE_EXT>(), 24);
    assert_eq!(align_of::<CERT_TEMPLATE_EXT>(), 8);
    assert_eq!(size_of::<CERT_HASHED_URL>(), 48);
    assert_eq!(align_of::<CERT_HASHED_URL>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_DETAILS>(), 24);
    assert_eq!(align_of::<CERT_LOGOTYPE_DETAILS>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_REFERENCE>(), 16);
    assert_eq!(align_of::<CERT_LOGOTYPE_REFERENCE>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_IMAGE_INFO_u>(), 4);
    assert_eq!(align_of::<CERT_LOGOTYPE_IMAGE_INFO_u>(), 4);
    assert_eq!(size_of::<CERT_LOGOTYPE_IMAGE_INFO>(), 32);
    assert_eq!(align_of::<CERT_LOGOTYPE_IMAGE_INFO>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_IMAGE>(), 32);
    assert_eq!(align_of::<CERT_LOGOTYPE_IMAGE>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_AUDIO_INFO>(), 24);
    assert_eq!(align_of::<CERT_LOGOTYPE_AUDIO_INFO>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_AUDIO>(), 32);
    assert_eq!(align_of::<CERT_LOGOTYPE_AUDIO>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_DATA>(), 32);
    assert_eq!(align_of::<CERT_LOGOTYPE_DATA>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_INFO_u>(), 8);
    assert_eq!(align_of::<CERT_LOGOTYPE_INFO_u>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_INFO>(), 16);
    assert_eq!(align_of::<CERT_LOGOTYPE_INFO>(), 8);
    assert_eq!(size_of::<CERT_OTHER_LOGOTYPE_INFO>(), 24);
    assert_eq!(align_of::<CERT_OTHER_LOGOTYPE_INFO>(), 8);
    assert_eq!(size_of::<CERT_LOGOTYPE_EXT_INFO>(), 48);
    assert_eq!(align_of::<CERT_LOGOTYPE_EXT_INFO>(), 8);
    assert_eq!(size_of::<CERT_BIOMETRIC_DATA_u>(), 8);
    assert_eq!(align_of::<CERT_BIOMETRIC_DATA_u>(), 8);
    assert_eq!(size_of::<CERT_BIOMETRIC_DATA>(), 64);
    assert_eq!(align_of::<CERT_BIOMETRIC_DATA>(), 8);
    assert_eq!(size_of::<CERT_BIOMETRIC_EXT_INFO>(), 16);
    assert_eq!(align_of::<CERT_BIOMETRIC_EXT_INFO>(), 8);
    assert_eq!(size_of::<OCSP_SIGNATURE_INFO>(), 64);
    assert_eq!(align_of::<OCSP_SIGNATURE_INFO>(), 8);
    assert_eq!(size_of::<OCSP_SIGNED_REQUEST_INFO>(), 24);
    assert_eq!(align_of::<OCSP_SIGNED_REQUEST_INFO>(), 8);
    assert_eq!(size_of::<OCSP_CERT_ID>(), 72);
    assert_eq!(align_of::<OCSP_CERT_ID>(), 8);
    assert_eq!(size_of::<OCSP_REQUEST_ENTRY>(), 88);
    assert_eq!(align_of::<OCSP_REQUEST_ENTRY>(), 8);
    assert_eq!(size_of::<OCSP_REQUEST_INFO>(), 48);
    assert_eq!(align_of::<OCSP_REQUEST_INFO>(), 8);
    assert_eq!(size_of::<OCSP_RESPONSE_INFO>(), 32);
    assert_eq!(align_of::<OCSP_RESPONSE_INFO>(), 8);
    assert_eq!(size_of::<OCSP_BASIC_SIGNED_RESPONSE_INFO>(), 80);
    assert_eq!(align_of::<OCSP_BASIC_SIGNED_RESPONSE_INFO>(), 8);
    assert_eq!(size_of::<OCSP_BASIC_REVOKED_INFO>(), 12);
    assert_eq!(align_of::<OCSP_BASIC_REVOKED_INFO>(), 4);
    assert_eq!(size_of::<OCSP_BASIC_RESPONSE_ENTRY_u>(), 8);
    assert_eq!(align_of::<OCSP_BASIC_RESPONSE_ENTRY_u>(), 8);
    assert_eq!(size_of::<OCSP_BASIC_RESPONSE_ENTRY>(), 120);
    assert_eq!(align_of::<OCSP_BASIC_RESPONSE_ENTRY>(), 8);
    assert_eq!(size_of::<OCSP_BASIC_RESPONSE_INFO_u>(), 16);
    assert_eq!(align_of::<OCSP_BASIC_RESPONSE_INFO_u>(), 8);
    assert_eq!(size_of::<OCSP_BASIC_RESPONSE_INFO>(), 64);
    assert_eq!(align_of::<OCSP_BASIC_RESPONSE_INFO>(), 8);
    assert_eq!(size_of::<CERT_SUPPORTED_ALGORITHM_INFO>(), 64);
    assert_eq!(align_of::<CERT_SUPPORTED_ALGORITHM_INFO>(), 8);
    assert_eq!(size_of::<CERT_TPM_SPECIFICATION_INFO>(), 16);
    assert_eq!(align_of::<CERT_TPM_SPECIFICATION_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_OID_FUNC_ENTRY>(), 16);
    assert_eq!(align_of::<CRYPT_OID_FUNC_ENTRY>(), 8);
    assert_eq!(size_of::<CRYPT_OID_INFO_u>(), 4);
    assert_eq!(align_of::<CRYPT_OID_INFO_u>(), 4);
    assert_eq!(size_of::<CRYPT_OID_INFO>(), 64);
    assert_eq!(align_of::<CRYPT_OID_INFO>(), 8);
    assert_eq!(size_of::<CERT_STRONG_SIGN_SERIALIZED_INFO>(), 24);
    assert_eq!(align_of::<CERT_STRONG_SIGN_SERIALIZED_INFO>(), 8);
    assert_eq!(size_of::<CERT_STRONG_SIGN_PARA_u>(), 8);
    assert_eq!(align_of::<CERT_STRONG_SIGN_PARA_u>(), 8);
    assert_eq!(size_of::<CERT_STRONG_SIGN_PARA>(), 16);
    assert_eq!(align_of::<CERT_STRONG_SIGN_PARA>(), 8);
    assert_eq!(size_of::<CERT_ISSUER_SERIAL_NUMBER>(), 32);
    assert_eq!(align_of::<CERT_ISSUER_SERIAL_NUMBER>(), 8);
    assert_eq!(size_of::<CERT_ID_u>(), 32);
    assert_eq!(align_of::<CERT_ID_u>(), 8);
    assert_eq!(size_of::<CERT_ID>(), 40);
    assert_eq!(align_of::<CERT_ID>(), 8);
    assert_eq!(size_of::<CMSG_SIGNER_ENCODE_INFO_u>(), 8);
    assert_eq!(align_of::<CMSG_SIGNER_ENCODE_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_SIGNER_ENCODE_INFO>(), 168);
    assert_eq!(align_of::<CMSG_SIGNER_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_SIGNED_ENCODE_INFO>(), 64);
    assert_eq!(align_of::<CMSG_SIGNED_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_ENVELOPED_ENCODE_INFO>(), 136);
    assert_eq!(align_of::<CMSG_ENVELOPED_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO>(), 112);
    assert_eq!(align_of::<CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO>(), 88);
    assert_eq!(align_of::<CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_u>(), 8);
    assert_eq!(align_of::<CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO>(), 128);
    assert_eq!(align_of::<CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_u>(), 8);
    assert_eq!(align_of::<CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO>(), 96);
    assert_eq!(align_of::<CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_RECIPIENT_ENCODE_INFO_u>(), 8);
    assert_eq!(align_of::<CMSG_RECIPIENT_ENCODE_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_RECIPIENT_ENCODE_INFO>(), 16);
    assert_eq!(align_of::<CMSG_RECIPIENT_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_RC2_AUX_INFO>(), 8);
    assert_eq!(align_of::<CMSG_RC2_AUX_INFO>(), 4);
    assert_eq!(size_of::<CMSG_SP3_COMPATIBLE_AUX_INFO>(), 8);
    assert_eq!(align_of::<CMSG_SP3_COMPATIBLE_AUX_INFO>(), 4);
    assert_eq!(size_of::<CMSG_RC4_AUX_INFO>(), 8);
    assert_eq!(align_of::<CMSG_RC4_AUX_INFO>(), 4);
    assert_eq!(size_of::<CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO>(), 208);
    assert_eq!(align_of::<CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_HASHED_ENCODE_INFO>(), 48);
    assert_eq!(align_of::<CMSG_HASHED_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_ENCRYPTED_ENCODE_INFO>(), 40);
    assert_eq!(align_of::<CMSG_ENCRYPTED_ENCODE_INFO>(), 8);
    assert_eq!(size_of::<CMSG_STREAM_INFO>(), 24);
    assert_eq!(align_of::<CMSG_STREAM_INFO>(), 8);
    assert_eq!(size_of::<CMSG_SIGNER_INFO>(), 136);
    assert_eq!(align_of::<CMSG_SIGNER_INFO>(), 8);
    assert_eq!(size_of::<CMSG_CMS_SIGNER_INFO>(), 144);
    assert_eq!(align_of::<CMSG_CMS_SIGNER_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_TRANS_RECIPIENT_INFO>(), 88);
    assert_eq!(align_of::<CMSG_KEY_TRANS_RECIPIENT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_RECIPIENT_ENCRYPTED_KEY_INFO>(), 72);
    assert_eq!(align_of::<CMSG_RECIPIENT_ENCRYPTED_KEY_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_RECIPIENT_INFO_u>(), 48);
    assert_eq!(align_of::<CMSG_KEY_AGREE_RECIPIENT_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_RECIPIENT_INFO>(), 112);
    assert_eq!(align_of::<CMSG_KEY_AGREE_RECIPIENT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_MAIL_LIST_RECIPIENT_INFO>(), 80);
    assert_eq!(align_of::<CMSG_MAIL_LIST_RECIPIENT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_CMS_RECIPIENT_INFO_u>(), 8);
    assert_eq!(align_of::<CMSG_CMS_RECIPIENT_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_CMS_RECIPIENT_INFO>(), 16);
    assert_eq!(align_of::<CMSG_CMS_RECIPIENT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA>(), 32);
    assert_eq!(align_of::<CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_DECRYPT_PARA_u>(), 8);
    assert_eq!(align_of::<CMSG_CTRL_DECRYPT_PARA_u>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_DECRYPT_PARA>(), 24);
    assert_eq!(align_of::<CMSG_CTRL_DECRYPT_PARA>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_u>(), 8);
    assert_eq!(align_of::<CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_u>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_KEY_TRANS_DECRYPT_PARA>(), 40);
    assert_eq!(align_of::<CMSG_CTRL_KEY_TRANS_DECRYPT_PARA>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_u>(), 8);
    assert_eq!(align_of::<CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_u>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_KEY_AGREE_DECRYPT_PARA>(), 64);
    assert_eq!(align_of::<CMSG_CTRL_KEY_AGREE_DECRYPT_PARA>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_u>(), 8);
    assert_eq!(align_of::<CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_u>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_MAIL_LIST_DECRYPT_PARA>(), 40);
    assert_eq!(align_of::<CMSG_CTRL_MAIL_LIST_DECRYPT_PARA>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA>(), 24);
    assert_eq!(align_of::<CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA>(), 8);
    assert_eq!(size_of::<CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA>(), 12);
    assert_eq!(align_of::<CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA>(), 4);
    assert_eq!(size_of::<CMSG_CONTENT_ENCRYPT_INFO_u>(), 8);
    assert_eq!(align_of::<CMSG_CONTENT_ENCRYPT_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_CONTENT_ENCRYPT_INFO>(), 128);
    assert_eq!(align_of::<CMSG_CONTENT_ENCRYPT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_TRANS_ENCRYPT_INFO>(), 56);
    assert_eq!(align_of::<CMSG_KEY_TRANS_ENCRYPT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_KEY_ENCRYPT_INFO>(), 24);
    assert_eq!(align_of::<CMSG_KEY_AGREE_KEY_ENCRYPT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_ENCRYPT_INFO_u>(), 48);
    assert_eq!(align_of::<CMSG_KEY_AGREE_ENCRYPT_INFO_u>(), 8);
    assert_eq!(size_of::<CMSG_KEY_AGREE_ENCRYPT_INFO>(), 128);
    assert_eq!(align_of::<CMSG_KEY_AGREE_ENCRYPT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_MAIL_LIST_ENCRYPT_INFO>(), 56);
    assert_eq!(align_of::<CMSG_MAIL_LIST_ENCRYPT_INFO>(), 8);
    assert_eq!(size_of::<CMSG_CNG_CONTENT_DECRYPT_INFO>(), 88);
    assert_eq!(align_of::<CMSG_CNG_CONTENT_DECRYPT_INFO>(), 8);
    assert_eq!(size_of::<CERT_CONTEXT>(), 40);
    assert_eq!(align_of::<CERT_CONTEXT>(), 8);
    assert_eq!(size_of::<CRL_CONTEXT>(), 40);
    assert_eq!(align_of::<CRL_CONTEXT>(), 8);
    assert_eq!(size_of::<CTL_CONTEXT>(), 64);
    assert_eq!(align_of::<CTL_CONTEXT>(), 8);
    assert_eq!(size_of::<CRYPT_KEY_PROV_PARAM>(), 24);
    assert_eq!(align_of::<CRYPT_KEY_PROV_PARAM>(), 8);
    assert_eq!(size_of::<CRYPT_KEY_PROV_INFO>(), 48);
    assert_eq!(align_of::<CRYPT_KEY_PROV_INFO>(), 8);
    assert_eq!(size_of::<CERT_KEY_CONTEXT_u>(), 8);
    assert_eq!(align_of::<CERT_KEY_CONTEXT_u>(), 8);
    assert_eq!(size_of::<CERT_KEY_CONTEXT>(), 24);
    assert_eq!(align_of::<CERT_KEY_CONTEXT>(), 8);
    assert_eq!(size_of::<ROOT_INFO_LUID>(), 8);
    assert_eq!(align_of::<ROOT_INFO_LUID>(), 4);
    assert_eq!(size_of::<CRYPT_SMART_CARD_ROOT_INFO>(), 24);
    assert_eq!(align_of::<CRYPT_SMART_CARD_ROOT_INFO>(), 4);
    assert_eq!(size_of::<CERT_SYSTEM_STORE_RELOCATE_PARA_u1>(), 8);
    assert_eq!(align_of::<CERT_SYSTEM_STORE_RELOCATE_PARA_u1>(), 8);
    assert_eq!(size_of::<CERT_SYSTEM_STORE_RELOCATE_PARA_u2>(), 8);
    assert_eq!(align_of::<CERT_SYSTEM_STORE_RELOCATE_PARA_u2>(), 8);
    assert_eq!(size_of::<CERT_SYSTEM_STORE_RELOCATE_PARA>(), 16);
    assert_eq!(align_of::<CERT_SYSTEM_STORE_RELOCATE_PARA>(), 8);
    assert_eq!(size_of::<CERT_REGISTRY_STORE_CLIENT_GPT_PARA>(), 16);
    assert_eq!(align_of::<CERT_REGISTRY_STORE_CLIENT_GPT_PARA>(), 8);
    assert_eq!(size_of::<CERT_REGISTRY_STORE_ROAMING_PARA>(), 16);
    assert_eq!(align_of::<CERT_REGISTRY_STORE_ROAMING_PARA>(), 8);
    assert_eq!(size_of::<CERT_LDAP_STORE_OPENED_PARA>(), 16);
    assert_eq!(align_of::<CERT_LDAP_STORE_OPENED_PARA>(), 8);
    assert_eq!(size_of::<CERT_STORE_PROV_INFO>(), 40);
    assert_eq!(align_of::<CERT_STORE_PROV_INFO>(), 8);
    assert_eq!(size_of::<CERT_STORE_PROV_FIND_INFO>(), 24);
    assert_eq!(align_of::<CERT_STORE_PROV_FIND_INFO>(), 8);
    assert_eq!(size_of::<CRL_FIND_ISSUED_FOR_PARA>(), 16);
    assert_eq!(align_of::<CRL_FIND_ISSUED_FOR_PARA>(), 8);
    assert_eq!(size_of::<CTL_ANY_SUBJECT_INFO>(), 40);
    assert_eq!(align_of::<CTL_ANY_SUBJECT_INFO>(), 8);
    assert_eq!(size_of::<CTL_FIND_USAGE_PARA>(), 48);
    assert_eq!(align_of::<CTL_FIND_USAGE_PARA>(), 8);
    assert_eq!(size_of::<CTL_FIND_SUBJECT_PARA>(), 32);
    assert_eq!(align_of::<CTL_FIND_SUBJECT_PARA>(), 8);
    assert_eq!(size_of::<CERT_CREATE_CONTEXT_PARA>(), 40);
    assert_eq!(align_of::<CERT_CREATE_CONTEXT_PARA>(), 8);
    assert_eq!(size_of::<CERT_SYSTEM_STORE_INFO>(), 4);
    assert_eq!(align_of::<CERT_SYSTEM_STORE_INFO>(), 4);
    assert_eq!(size_of::<CERT_PHYSICAL_STORE_INFO>(), 48);
    assert_eq!(align_of::<CERT_PHYSICAL_STORE_INFO>(), 8);
    assert_eq!(size_of::<CTL_VERIFY_USAGE_PARA>(), 56);
    assert_eq!(align_of::<CTL_VERIFY_USAGE_PARA>(), 8);
    assert_eq!(size_of::<CTL_VERIFY_USAGE_STATUS>(), 48);
    assert_eq!(align_of::<CTL_VERIFY_USAGE_STATUS>(), 8);
    assert_eq!(size_of::<CERT_REVOCATION_CRL_INFO>(), 40);
    assert_eq!(align_of::<CERT_REVOCATION_CRL_INFO>(), 8);
    assert_eq!(size_of::<CERT_REVOCATION_PARA>(), 96);
    assert_eq!(align_of::<CERT_REVOCATION_PARA>(), 8);
    assert_eq!(size_of::<CERT_REVOCATION_STATUS>(), 24);
    assert_eq!(align_of::<CERT_REVOCATION_STATUS>(), 4);
    assert_eq!(
        size_of::<CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO>(),
        32
    );
    assert_eq!(
        align_of::<CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO>(),
        8
    );
    assert_eq!(size_of::<CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO>(), 24);
    assert_eq!(align_of::<CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA>(), 16);
    assert_eq!(align_of::<CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_SIGN_MESSAGE_PARA>(), 152);
    assert_eq!(align_of::<CRYPT_SIGN_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_VERIFY_MESSAGE_PARA>(), 40);
    assert_eq!(align_of::<CRYPT_VERIFY_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_ENCRYPT_MESSAGE_PARA>(), 56);
    assert_eq!(align_of::<CRYPT_ENCRYPT_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_DECRYPT_MESSAGE_PARA>(), 32);
    assert_eq!(align_of::<CRYPT_DECRYPT_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_HASH_MESSAGE_PARA>(), 48);
    assert_eq!(align_of::<CRYPT_HASH_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_KEY_SIGN_MESSAGE_PARA_u>(), 8);
    assert_eq!(align_of::<CRYPT_KEY_SIGN_MESSAGE_PARA_u>(), 8);
    assert_eq!(size_of::<CRYPT_KEY_SIGN_MESSAGE_PARA>(), 80);
    assert_eq!(align_of::<CRYPT_KEY_SIGN_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_KEY_VERIFY_MESSAGE_PARA>(), 16);
    assert_eq!(align_of::<CRYPT_KEY_VERIFY_MESSAGE_PARA>(), 8);
    assert_eq!(size_of::<CERT_CHAIN>(), 64);
    assert_eq!(align_of::<CERT_CHAIN>(), 8);
    assert_eq!(size_of::<CRYPT_BLOB_ARRAY>(), 16);
    assert_eq!(align_of::<CRYPT_BLOB_ARRAY>(), 8);
    assert_eq!(size_of::<CRYPT_CREDENTIALS>(), 24);
    assert_eq!(align_of::<CRYPT_CREDENTIALS>(), 8);
    assert_eq!(size_of::<CRYPT_PASSWORD_CREDENTIALSA>(), 24);
    assert_eq!(align_of::<CRYPT_PASSWORD_CREDENTIALSA>(), 8);
    assert_eq!(size_of::<CRYPT_PASSWORD_CREDENTIALSW>(), 24);
    assert_eq!(align_of::<CRYPT_PASSWORD_CREDENTIALSW>(), 8);
    assert_eq!(size_of::<CRYPTNET_URL_CACHE_PRE_FETCH_INFO>(), 40);
    assert_eq!(align_of::<CRYPTNET_URL_CACHE_PRE_FETCH_INFO>(), 4);
    assert_eq!(size_of::<CRYPTNET_URL_CACHE_FLUSH_INFO>(), 16);
    assert_eq!(align_of::<CRYPTNET_URL_CACHE_FLUSH_INFO>(), 4);
    assert_eq!(size_of::<CRYPTNET_URL_CACHE_RESPONSE_INFO>(), 40);
    assert_eq!(align_of::<CRYPTNET_URL_CACHE_RESPONSE_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_RETRIEVE_AUX_INFO>(), 88);
    assert_eq!(align_of::<CRYPT_RETRIEVE_AUX_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_ASYNC_RETRIEVAL_COMPLETION>(), 16);
    assert_eq!(align_of::<CRYPT_ASYNC_RETRIEVAL_COMPLETION>(), 8);
    assert_eq!(size_of::<CRYPT_URL_ARRAY>(), 16);
    assert_eq!(align_of::<CRYPT_URL_ARRAY>(), 8);
    assert_eq!(size_of::<CRYPT_URL_INFO>(), 24);
    assert_eq!(align_of::<CRYPT_URL_INFO>(), 8);
    assert_eq!(size_of::<CERT_CRL_CONTEXT_PAIR>(), 16);
    assert_eq!(align_of::<CERT_CRL_CONTEXT_PAIR>(), 8);
    assert_eq!(size_of::<CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO>(), 48);
    assert_eq!(align_of::<CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_ENGINE_CONFIG>(), 88);
    assert_eq!(align_of::<CERT_CHAIN_ENGINE_CONFIG>(), 8);
    assert_eq!(size_of::<CERT_TRUST_STATUS>(), 8);
    assert_eq!(align_of::<CERT_TRUST_STATUS>(), 4);
    assert_eq!(size_of::<CERT_REVOCATION_INFO>(), 40);
    assert_eq!(align_of::<CERT_REVOCATION_INFO>(), 8);
    assert_eq!(size_of::<CERT_TRUST_LIST_INFO>(), 24);
    assert_eq!(align_of::<CERT_TRUST_LIST_INFO>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_ELEMENT>(), 56);
    assert_eq!(align_of::<CERT_CHAIN_ELEMENT>(), 8);
    assert_eq!(size_of::<CERT_SIMPLE_CHAIN>(), 40);
    assert_eq!(align_of::<CERT_SIMPLE_CHAIN>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_CONTEXT>(), 72);
    assert_eq!(align_of::<CERT_CHAIN_CONTEXT>(), 8);
    assert_eq!(size_of::<CERT_USAGE_MATCH>(), 24);
    assert_eq!(align_of::<CERT_USAGE_MATCH>(), 8);
    assert_eq!(size_of::<CTL_USAGE_MATCH>(), 24);
    assert_eq!(align_of::<CTL_USAGE_MATCH>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_PARA>(), 96);
    assert_eq!(align_of::<CERT_CHAIN_PARA>(), 8);
    assert_eq!(size_of::<CERT_REVOCATION_CHAIN_PARA>(), 56);
    assert_eq!(align_of::<CERT_REVOCATION_CHAIN_PARA>(), 8);
    assert_eq!(size_of::<CRL_REVOCATION_INFO>(), 24);
    assert_eq!(align_of::<CRL_REVOCATION_INFO>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_FIND_ISSUER_PARA>(), 72);
    assert_eq!(align_of::<CERT_CHAIN_FIND_ISSUER_PARA>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_POLICY_PARA>(), 16);
    assert_eq!(align_of::<CERT_CHAIN_POLICY_PARA>(), 8);
    assert_eq!(size_of::<CERT_CHAIN_POLICY_STATUS>(), 24);
    assert_eq!(align_of::<CERT_CHAIN_POLICY_STATUS>(), 8);
    assert_eq!(size_of::<AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA>(), 16);
    assert_eq!(align_of::<AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA>(), 8);
    assert_eq!(size_of::<AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 8);
    assert_eq!(align_of::<AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 4);
    assert_eq!(
        size_of::<AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA>(),
        12
    );
    assert_eq!(
        align_of::<AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA>(),
        4
    );
    assert_eq!(size_of::<HTTPSPolicyCallbackData_u>(), 4);
    assert_eq!(align_of::<HTTPSPolicyCallbackData_u>(), 4);
    assert_eq!(size_of::<HTTPSPolicyCallbackData>(), 24);
    assert_eq!(align_of::<HTTPSPolicyCallbackData>(), 8);
    assert_eq!(size_of::<EV_EXTRA_CERT_CHAIN_POLICY_PARA>(), 8);
    assert_eq!(align_of::<EV_EXTRA_CERT_CHAIN_POLICY_PARA>(), 4);
    assert_eq!(size_of::<EV_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 12);
    assert_eq!(align_of::<EV_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 4);
    assert_eq!(size_of::<SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 528);
    assert_eq!(align_of::<SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 4);
    assert_eq!(
        size_of::<SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA>(),
        32
    );
    assert_eq!(
        align_of::<SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA>(),
        8
    );
    assert_eq!(size_of::<SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA>(), 16);
    assert_eq!(align_of::<SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA>(), 8);
    assert_eq!(
        size_of::<SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS>(),
        1032
    );
    assert_eq!(align_of::<SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS>(), 4);
    assert_eq!(size_of::<CRYPT_PKCS12_PBE_PARAMS>(), 8);
    assert_eq!(align_of::<CRYPT_PKCS12_PBE_PARAMS>(), 4);
    assert_eq!(size_of::<CERT_SERVER_OCSP_RESPONSE_CONTEXT>(), 24);
    assert_eq!(align_of::<CERT_SERVER_OCSP_RESPONSE_CONTEXT>(), 8);
    assert_eq!(size_of::<CERT_SERVER_OCSP_RESPONSE_OPEN_PARA>(), 40);
    assert_eq!(align_of::<CERT_SERVER_OCSP_RESPONSE_OPEN_PARA>(), 8);
    assert_eq!(size_of::<CERT_SELECT_CHAIN_PARA>(), 40);
    assert_eq!(align_of::<CERT_SELECT_CHAIN_PARA>(), 8);
    assert_eq!(size_of::<CERT_SELECT_CRITERIA>(), 16);
    assert_eq!(align_of::<CERT_SELECT_CRITERIA>(), 8);
    assert_eq!(size_of::<CRYPT_TIMESTAMP_REQUEST>(), 88);
    assert_eq!(align_of::<CRYPT_TIMESTAMP_REQUEST>(), 8);
    assert_eq!(size_of::<CRYPT_TIMESTAMP_RESPONSE>(), 56);
    assert_eq!(align_of::<CRYPT_TIMESTAMP_RESPONSE>(), 8);
    assert_eq!(size_of::<CRYPT_TIMESTAMP_ACCURACY>(), 12);
    assert_eq!(align_of::<CRYPT_TIMESTAMP_ACCURACY>(), 4);
    assert_eq!(size_of::<CRYPT_TIMESTAMP_INFO>(), 144);
    assert_eq!(align_of::<CRYPT_TIMESTAMP_INFO>(), 8);
    assert_eq!(size_of::<CRYPT_TIMESTAMP_CONTEXT>(), 24);
    assert_eq!(align_of::<CRYPT_TIMESTAMP_CONTEXT>(), 8);
    assert_eq!(size_of::<CRYPT_TIMESTAMP_PARA>(), 48);
    assert_eq!(align_of::<CRYPT_TIMESTAMP_PARA>(), 8);
    assert_eq!(size_of::<CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE>(), 48);
    assert_eq!(align_of::<CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE>(), 8);
}
#[cfg(feature = "winefs")]
#[test]
fn um_winefs() {
    use winapi::um::winefs::*;
    assert_eq!(size_of::<EFS_CERTIFICATE_BLOB>(), 16);
    assert_eq!(align_of::<EFS_CERTIFICATE_BLOB>(), 8);
    assert_eq!(size_of::<EFS_HASH_BLOB>(), 16);
    assert_eq!(align_of::<EFS_HASH_BLOB>(), 8);
    assert_eq!(size_of::<EFS_RPC_BLOB>(), 16);
    assert_eq!(align_of::<EFS_RPC_BLOB>(), 8);
    assert_eq!(size_of::<EFS_PIN_BLOB>(), 16);
    assert_eq!(align_of::<EFS_PIN_BLOB>(), 8);
    assert_eq!(size_of::<EFS_KEY_INFO>(), 16);
    assert_eq!(align_of::<EFS_KEY_INFO>(), 4);
    assert_eq!(size_of::<EFS_COMPATIBILITY_INFO>(), 4);
    assert_eq!(align_of::<EFS_COMPATIBILITY_INFO>(), 4);
    assert_eq!(size_of::<EFS_VERSION_INFO>(), 8);
    assert_eq!(align_of::<EFS_VERSION_INFO>(), 4);
    assert_eq!(size_of::<EFS_DECRYPTION_STATUS_INFO>(), 12);
    assert_eq!(align_of::<EFS_DECRYPTION_STATUS_INFO>(), 4);
    assert_eq!(size_of::<EFS_ENCRYPTION_STATUS_INFO>(), 8);
    assert_eq!(align_of::<EFS_ENCRYPTION_STATUS_INFO>(), 4);
    assert_eq!(size_of::<ENCRYPTION_CERTIFICATE>(), 24);
    assert_eq!(align_of::<ENCRYPTION_CERTIFICATE>(), 8);
    assert_eq!(size_of::<ENCRYPTION_CERTIFICATE_HASH>(), 32);
    assert_eq!(align_of::<ENCRYPTION_CERTIFICATE_HASH>(), 8);
    assert_eq!(size_of::<ENCRYPTION_CERTIFICATE_HASH_LIST>(), 16);
    assert_eq!(align_of::<ENCRYPTION_CERTIFICATE_HASH_LIST>(), 8);
    assert_eq!(size_of::<ENCRYPTION_CERTIFICATE_LIST>(), 16);
    assert_eq!(align_of::<ENCRYPTION_CERTIFICATE_LIST>(), 8);
    assert_eq!(size_of::<ENCRYPTED_FILE_METADATA_SIGNATURE>(), 32);
    assert_eq!(align_of::<ENCRYPTED_FILE_METADATA_SIGNATURE>(), 8);
    assert_eq!(size_of::<ENCRYPTION_PROTECTOR>(), 24);
    assert_eq!(align_of::<ENCRYPTION_PROTECTOR>(), 8);
    assert_eq!(size_of::<ENCRYPTION_PROTECTOR_LIST>(), 16);
    assert_eq!(align_of::<ENCRYPTION_PROTECTOR_LIST>(), 8);
}
#[cfg(feature = "winevt")]
#[test]
fn um_winevt() {
    use winapi::um::winevt::*;
    assert_eq!(size_of::<EVT_VARIANT>(), 16);
    assert_eq!(align_of::<EVT_VARIANT>(), 8);
    assert_eq!(size_of::<EVT_RPC_LOGIN>(), 40);
    assert_eq!(align_of::<EVT_RPC_LOGIN>(), 8);
}
#[cfg(feature = "wingdi")]
#[test]
fn um_wingdi() {
    use winapi::um::wingdi::*;
    assert_eq!(size_of::<DRAWPATRECT>(), 20);
    assert_eq!(align_of::<DRAWPATRECT>(), 4);
    assert_eq!(size_of::<PSINJECTDATA>(), 8);
    assert_eq!(align_of::<PSINJECTDATA>(), 4);
    assert_eq!(size_of::<PSFEATURE_OUTPUT>(), 8);
    assert_eq!(align_of::<PSFEATURE_OUTPUT>(), 4);
    assert_eq!(size_of::<PSFEATURE_CUSTPAPER>(), 20);
    assert_eq!(align_of::<PSFEATURE_CUSTPAPER>(), 4);
    assert_eq!(size_of::<XFORM>(), 24);
    assert_eq!(align_of::<XFORM>(), 4);
    assert_eq!(size_of::<BITMAP>(), 32);
    assert_eq!(align_of::<BITMAP>(), 8);
    assert_eq!(size_of::<RGBTRIPLE>(), 3);
    assert_eq!(align_of::<RGBTRIPLE>(), 1);
    assert_eq!(size_of::<RGBQUAD>(), 4);
    assert_eq!(align_of::<RGBQUAD>(), 1);
    assert_eq!(size_of::<CIEXYZ>(), 12);
    assert_eq!(align_of::<CIEXYZ>(), 4);
    assert_eq!(size_of::<CIEXYZTRIPLE>(), 36);
    assert_eq!(align_of::<CIEXYZTRIPLE>(), 4);
    assert_eq!(size_of::<LOGCOLORSPACEA>(), 328);
    assert_eq!(align_of::<LOGCOLORSPACEA>(), 4);
    assert_eq!(size_of::<LOGCOLORSPACEW>(), 588);
    assert_eq!(align_of::<LOGCOLORSPACEW>(), 4);
    assert_eq!(size_of::<BITMAPCOREHEADER>(), 12);
    assert_eq!(align_of::<BITMAPCOREHEADER>(), 4);
    assert_eq!(size_of::<BITMAPINFOHEADER>(), 40);
    assert_eq!(align_of::<BITMAPINFOHEADER>(), 4);
    assert_eq!(size_of::<BITMAPV4HEADER>(), 108);
    assert_eq!(align_of::<BITMAPV4HEADER>(), 4);
    assert_eq!(size_of::<BITMAPV5HEADER>(), 124);
    assert_eq!(align_of::<BITMAPV5HEADER>(), 4);
    assert_eq!(size_of::<BITMAPINFO>(), 44);
    assert_eq!(align_of::<BITMAPINFO>(), 4);
    assert_eq!(size_of::<BITMAPCOREINFO>(), 16);
    assert_eq!(align_of::<BITMAPCOREINFO>(), 4);
    // packed = 2
    // assert_eq!(size_of::<BITMAPFILEHEADER>(), 14);
    // assert_eq!(align_of::<BITMAPFILEHEADER>(), 2);
    assert_eq!(size_of::<FONTSIGNATURE>(), 24);
    assert_eq!(align_of::<FONTSIGNATURE>(), 4);
    assert_eq!(size_of::<CHARSETINFO>(), 32);
    assert_eq!(align_of::<CHARSETINFO>(), 4);
    assert_eq!(size_of::<LOCALESIGNATURE>(), 32);
    assert_eq!(align_of::<LOCALESIGNATURE>(), 4);
    assert_eq!(size_of::<HANDLETABLE>(), 8);
    assert_eq!(align_of::<HANDLETABLE>(), 8);
    assert_eq!(size_of::<METARECORD>(), 8);
    assert_eq!(align_of::<METARECORD>(), 4);
    assert_eq!(size_of::<METAFILEPICT>(), 24);
    assert_eq!(align_of::<METAFILEPICT>(), 8);
    // packed = 2
    // assert_eq!(size_of::<METAHEADER>(), 18);
    // assert_eq!(align_of::<METAHEADER>(), 2);
    assert_eq!(size_of::<ENHMETARECORD>(), 12);
    assert_eq!(align_of::<ENHMETARECORD>(), 4);
    assert_eq!(size_of::<ENHMETAHEADER>(), 108);
    assert_eq!(align_of::<ENHMETAHEADER>(), 4);
    assert_eq!(size_of::<TEXTMETRICA>(), 56);
    assert_eq!(align_of::<TEXTMETRICA>(), 4);
    assert_eq!(size_of::<TEXTMETRICW>(), 60);
    assert_eq!(align_of::<TEXTMETRICW>(), 4);
    assert_eq!(size_of::<NEWTEXTMETRICA>(), 72);
    assert_eq!(align_of::<NEWTEXTMETRICA>(), 4);
    assert_eq!(size_of::<NEWTEXTMETRICW>(), 76);
    assert_eq!(align_of::<NEWTEXTMETRICW>(), 4);
    assert_eq!(size_of::<NEWTEXTMETRICEXA>(), 96);
    assert_eq!(align_of::<NEWTEXTMETRICEXA>(), 4);
    assert_eq!(size_of::<NEWTEXTMETRICEXW>(), 100);
    assert_eq!(align_of::<NEWTEXTMETRICEXW>(), 4);
    assert_eq!(size_of::<PELARRAY>(), 20);
    assert_eq!(align_of::<PELARRAY>(), 4);
    assert_eq!(size_of::<LOGBRUSH>(), 16);
    assert_eq!(align_of::<LOGBRUSH>(), 8);
    assert_eq!(size_of::<LOGBRUSH32>(), 12);
    assert_eq!(align_of::<LOGBRUSH32>(), 4);
    assert_eq!(size_of::<LOGPEN>(), 16);
    assert_eq!(align_of::<LOGPEN>(), 4);
    assert_eq!(size_of::<EXTLOGPEN>(), 32);
    assert_eq!(align_of::<EXTLOGPEN>(), 8);
    assert_eq!(size_of::<EXTLOGPEN32>(), 28);
    assert_eq!(align_of::<EXTLOGPEN32>(), 4);
    assert_eq!(size_of::<PALETTEENTRY>(), 4);
    assert_eq!(align_of::<PALETTEENTRY>(), 1);
    assert_eq!(size_of::<LOGPALETTE>(), 8);
    assert_eq!(align_of::<LOGPALETTE>(), 2);
    assert_eq!(size_of::<LOGFONTA>(), 60);
    assert_eq!(align_of::<LOGFONTA>(), 4);
    assert_eq!(size_of::<LOGFONTW>(), 92);
    assert_eq!(align_of::<LOGFONTW>(), 4);
    assert_eq!(size_of::<ENUMLOGFONTA>(), 156);
    assert_eq!(align_of::<ENUMLOGFONTA>(), 4);
    assert_eq!(size_of::<ENUMLOGFONTW>(), 284);
    assert_eq!(align_of::<ENUMLOGFONTW>(), 4);
    assert_eq!(size_of::<ENUMLOGFONTEXA>(), 188);
    assert_eq!(align_of::<ENUMLOGFONTEXA>(), 4);
    assert_eq!(size_of::<ENUMLOGFONTEXW>(), 348);
    assert_eq!(align_of::<ENUMLOGFONTEXW>(), 4);
    assert_eq!(size_of::<PANOSE>(), 10);
    assert_eq!(align_of::<PANOSE>(), 1);
    assert_eq!(size_of::<EXTLOGFONTA>(), 192);
    assert_eq!(align_of::<EXTLOGFONTA>(), 4);
    assert_eq!(size_of::<EXTLOGFONTW>(), 320);
    assert_eq!(align_of::<EXTLOGFONTW>(), 4);
    assert_eq!(size_of::<DEVMODE_u1_s1>(), 16);
    assert_eq!(align_of::<DEVMODE_u1_s1>(), 2);
    assert_eq!(size_of::<DEVMODE_u1_s2>(), 16);
    assert_eq!(align_of::<DEVMODE_u1_s2>(), 4);
    assert_eq!(size_of::<DEVMODE_u1>(), 16);
    assert_eq!(align_of::<DEVMODE_u1>(), 4);
    assert_eq!(size_of::<DEVMODE_u2>(), 4);
    assert_eq!(align_of::<DEVMODE_u2>(), 4);
    assert_eq!(size_of::<DEVMODEA>(), 156);
    assert_eq!(align_of::<DEVMODEA>(), 4);
    assert_eq!(size_of::<DEVMODEW>(), 220);
    assert_eq!(align_of::<DEVMODEW>(), 4);
    assert_eq!(size_of::<DISPLAY_DEVICEA>(), 424);
    assert_eq!(align_of::<DISPLAY_DEVICEA>(), 4);
    assert_eq!(size_of::<DISPLAY_DEVICEW>(), 840);
    assert_eq!(align_of::<DISPLAY_DEVICEW>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_RATIONAL>(), 8);
    assert_eq!(align_of::<DISPLAYCONFIG_RATIONAL>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_2DREGION>(), 8);
    assert_eq!(align_of::<DISPLAYCONFIG_2DREGION>(), 4);
    assert_eq!(
        size_of::<DISPLAYCONFIG_VIDEO_SIGNAL_INFO_AdditionalSignalInfo>(),
        4
    );
    assert_eq!(
        align_of::<DISPLAYCONFIG_VIDEO_SIGNAL_INFO_AdditionalSignalInfo>(),
        4
    );
    assert_eq!(size_of::<DISPLAYCONFIG_VIDEO_SIGNAL_INFO_u>(), 4);
    assert_eq!(align_of::<DISPLAYCONFIG_VIDEO_SIGNAL_INFO_u>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_VIDEO_SIGNAL_INFO>(), 48);
    assert_eq!(align_of::<DISPLAYCONFIG_VIDEO_SIGNAL_INFO>(), 8);
    assert_eq!(size_of::<DISPLAYCONFIG_SOURCE_MODE>(), 20);
    assert_eq!(align_of::<DISPLAYCONFIG_SOURCE_MODE>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_TARGET_MODE>(), 48);
    assert_eq!(align_of::<DISPLAYCONFIG_TARGET_MODE>(), 8);
    assert_eq!(size_of::<DISPLAYCONFIG_DESKTOP_IMAGE_INFO>(), 40);
    assert_eq!(align_of::<DISPLAYCONFIG_DESKTOP_IMAGE_INFO>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_MODE_INFO_u>(), 48);
    assert_eq!(align_of::<DISPLAYCONFIG_MODE_INFO_u>(), 8);
    assert_eq!(size_of::<DISPLAYCONFIG_MODE_INFO>(), 64);
    assert_eq!(align_of::<DISPLAYCONFIG_MODE_INFO>(), 8);
    assert_eq!(size_of::<DISPLAYCONFIG_PATH_SOURCE_INFO>(), 20);
    assert_eq!(align_of::<DISPLAYCONFIG_PATH_SOURCE_INFO>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_PATH_TARGET_INFO>(), 48);
    assert_eq!(align_of::<DISPLAYCONFIG_PATH_TARGET_INFO>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_PATH_INFO>(), 72);
    assert_eq!(align_of::<DISPLAYCONFIG_PATH_INFO>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_DEVICE_INFO_HEADER>(), 20);
    assert_eq!(align_of::<DISPLAYCONFIG_DEVICE_INFO_HEADER>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>(), 84);
    assert_eq!(align_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS>(), 4);
    assert_eq!(align_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>(), 420);
    assert_eq!(align_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_TARGET_PREFERRED_MODE>(), 80);
    assert_eq!(align_of::<DISPLAYCONFIG_TARGET_PREFERRED_MODE>(), 8);
    assert_eq!(size_of::<DISPLAYCONFIG_ADAPTER_NAME>(), 276);
    assert_eq!(align_of::<DISPLAYCONFIG_ADAPTER_NAME>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_TARGET_BASE_TYPE>(), 24);
    assert_eq!(align_of::<DISPLAYCONFIG_TARGET_BASE_TYPE>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_SET_TARGET_PERSISTENCE>(), 24);
    assert_eq!(align_of::<DISPLAYCONFIG_SET_TARGET_PERSISTENCE>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION>(), 24);
    assert_eq!(align_of::<DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO>(), 32);
    assert_eq!(align_of::<DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO>(), 4);
    assert_eq!(size_of::<DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE>(), 24);
    assert_eq!(align_of::<DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE>(), 4);
    assert_eq!(size_of::<RGNDATAHEADER>(), 32);
    assert_eq!(align_of::<RGNDATAHEADER>(), 4);
    assert_eq!(size_of::<RGNDATA>(), 36);
    assert_eq!(align_of::<RGNDATA>(), 4);
    assert_eq!(size_of::<ABC>(), 12);
    assert_eq!(align_of::<ABC>(), 4);
    assert_eq!(size_of::<ABCFLOAT>(), 12);
    assert_eq!(align_of::<ABCFLOAT>(), 4);
    assert_eq!(size_of::<OUTLINETEXTMETRICA>(), 232);
    assert_eq!(align_of::<OUTLINETEXTMETRICA>(), 8);
    assert_eq!(size_of::<OUTLINETEXTMETRICW>(), 232);
    assert_eq!(align_of::<OUTLINETEXTMETRICW>(), 8);
    assert_eq!(size_of::<POLYTEXTA>(), 56);
    assert_eq!(align_of::<POLYTEXTA>(), 8);
    assert_eq!(size_of::<POLYTEXTW>(), 56);
    assert_eq!(align_of::<POLYTEXTW>(), 8);
    assert_eq!(size_of::<FIXED>(), 4);
    assert_eq!(align_of::<FIXED>(), 2);
    assert_eq!(size_of::<MAT2>(), 16);
    assert_eq!(align_of::<MAT2>(), 2);
    assert_eq!(size_of::<GLYPHMETRICS>(), 20);
    assert_eq!(align_of::<GLYPHMETRICS>(), 4);
    assert_eq!(size_of::<POINTFX>(), 8);
    assert_eq!(align_of::<POINTFX>(), 2);
    assert_eq!(size_of::<TTPOLYCURVE>(), 12);
    assert_eq!(align_of::<TTPOLYCURVE>(), 2);
    assert_eq!(size_of::<TTPOLYGONHEADER>(), 16);
    assert_eq!(align_of::<TTPOLYGONHEADER>(), 4);
    assert_eq!(size_of::<GCP_RESULTSA>(), 64);
    assert_eq!(align_of::<GCP_RESULTSA>(), 8);
    assert_eq!(size_of::<GCP_RESULTSW>(), 64);
    assert_eq!(align_of::<GCP_RESULTSW>(), 8);
    assert_eq!(size_of::<RASTERIZER_STATUS>(), 6);
    assert_eq!(align_of::<RASTERIZER_STATUS>(), 2);
    assert_eq!(size_of::<PIXELFORMATDESCRIPTOR>(), 40);
    assert_eq!(align_of::<PIXELFORMATDESCRIPTOR>(), 4);
    assert_eq!(size_of::<WCRANGE>(), 4);
    assert_eq!(align_of::<WCRANGE>(), 2);
    assert_eq!(size_of::<GLYPHSET>(), 20);
    assert_eq!(align_of::<GLYPHSET>(), 4);
    assert_eq!(size_of::<DESIGNVECTOR>(), 72);
    assert_eq!(align_of::<DESIGNVECTOR>(), 4);
    assert_eq!(size_of::<AXISINFOA>(), 24);
    assert_eq!(align_of::<AXISINFOA>(), 4);
    assert_eq!(size_of::<AXISINFOW>(), 40);
    assert_eq!(align_of::<AXISINFOW>(), 4);
    assert_eq!(size_of::<AXESLISTA>(), 392);
    assert_eq!(align_of::<AXESLISTA>(), 4);
    assert_eq!(size_of::<AXESLISTW>(), 648);
    assert_eq!(align_of::<AXESLISTW>(), 4);
    assert_eq!(size_of::<ENUMLOGFONTEXDVA>(), 260);
    assert_eq!(align_of::<ENUMLOGFONTEXDVA>(), 4);
    assert_eq!(size_of::<ENUMLOGFONTEXDVW>(), 420);
    assert_eq!(align_of::<ENUMLOGFONTEXDVW>(), 4);
    assert_eq!(size_of::<ENUMTEXTMETRICA>(), 488);
    assert_eq!(align_of::<ENUMTEXTMETRICA>(), 4);
    assert_eq!(size_of::<ENUMTEXTMETRICW>(), 748);
    assert_eq!(align_of::<ENUMTEXTMETRICW>(), 4);
    assert_eq!(size_of::<TRIVERTEX>(), 16);
    assert_eq!(align_of::<TRIVERTEX>(), 4);
    assert_eq!(size_of::<GRADIENT_RECT>(), 8);
    assert_eq!(align_of::<GRADIENT_RECT>(), 4);
    assert_eq!(size_of::<BLENDFUNCTION>(), 4);
    assert_eq!(align_of::<BLENDFUNCTION>(), 1);
    assert_eq!(size_of::<DIBSECTION>(), 104);
    assert_eq!(align_of::<DIBSECTION>(), 8);
    assert_eq!(size_of::<COLORADJUSTMENT>(), 24);
    assert_eq!(align_of::<COLORADJUSTMENT>(), 2);
    assert_eq!(size_of::<DOCINFOA>(), 40);
    assert_eq!(align_of::<DOCINFOA>(), 8);
    assert_eq!(size_of::<DOCINFOW>(), 40);
    assert_eq!(align_of::<DOCINFOW>(), 8);
    assert_eq!(size_of::<KERNINGPAIR>(), 8);
    assert_eq!(align_of::<KERNINGPAIR>(), 4);
    assert_eq!(size_of::<EMR>(), 8);
    assert_eq!(align_of::<EMR>(), 4);
    assert_eq!(size_of::<EMRTEXT>(), 40);
    assert_eq!(align_of::<EMRTEXT>(), 4);
    assert_eq!(size_of::<EMRABORTPATH>(), 8);
    assert_eq!(align_of::<EMRABORTPATH>(), 4);
    assert_eq!(size_of::<EMRSELECTCLIPPATH>(), 12);
    assert_eq!(align_of::<EMRSELECTCLIPPATH>(), 4);
    assert_eq!(size_of::<EMRSETMITERLIMIT>(), 12);
    assert_eq!(align_of::<EMRSETMITERLIMIT>(), 4);
    assert_eq!(size_of::<EMRRESTOREDC>(), 12);
    assert_eq!(align_of::<EMRRESTOREDC>(), 4);
    assert_eq!(size_of::<EMRSETARCDIRECTION>(), 12);
    assert_eq!(align_of::<EMRSETARCDIRECTION>(), 4);
    assert_eq!(size_of::<EMRSETMAPPERFLAGS>(), 12);
    assert_eq!(align_of::<EMRSETMAPPERFLAGS>(), 4);
    assert_eq!(size_of::<EMRSETBKCOLOR>(), 12);
    assert_eq!(align_of::<EMRSETBKCOLOR>(), 4);
    assert_eq!(size_of::<EMRSELECTOBJECT>(), 12);
    assert_eq!(align_of::<EMRSELECTOBJECT>(), 4);
    assert_eq!(size_of::<EMRSELECTPALETTE>(), 12);
    assert_eq!(align_of::<EMRSELECTPALETTE>(), 4);
    assert_eq!(size_of::<EMRRESIZEPALETTE>(), 16);
    assert_eq!(align_of::<EMRRESIZEPALETTE>(), 4);
    assert_eq!(size_of::<EMRSETPALETTEENTRIES>(), 24);
    assert_eq!(align_of::<EMRSETPALETTEENTRIES>(), 4);
    assert_eq!(size_of::<EMRSETCOLORADJUSTMENT>(), 32);
    assert_eq!(align_of::<EMRSETCOLORADJUSTMENT>(), 4);
    assert_eq!(size_of::<EMRGDICOMMENT>(), 16);
    assert_eq!(align_of::<EMRGDICOMMENT>(), 4);
    assert_eq!(size_of::<EMREOF>(), 20);
    assert_eq!(align_of::<EMREOF>(), 4);
    assert_eq!(size_of::<EMRLINETO>(), 16);
    assert_eq!(align_of::<EMRLINETO>(), 4);
    assert_eq!(size_of::<EMROFFSETCLIPRGN>(), 16);
    assert_eq!(align_of::<EMROFFSETCLIPRGN>(), 4);
    assert_eq!(size_of::<EMRFILLPATH>(), 24);
    assert_eq!(align_of::<EMRFILLPATH>(), 4);
    assert_eq!(size_of::<EMREXCLUDECLIPRECT>(), 24);
    assert_eq!(align_of::<EMREXCLUDECLIPRECT>(), 4);
    assert_eq!(size_of::<EMRSETVIEWPORTORGEX>(), 16);
    assert_eq!(align_of::<EMRSETVIEWPORTORGEX>(), 4);
    assert_eq!(size_of::<EMRSETVIEWPORTEXTEX>(), 16);
    assert_eq!(align_of::<EMRSETVIEWPORTEXTEX>(), 4);
    assert_eq!(size_of::<EMRSCALEVIEWPORTEXTEX>(), 24);
    assert_eq!(align_of::<EMRSCALEVIEWPORTEXTEX>(), 4);
    assert_eq!(size_of::<EMRSETWORLDTRANSFORM>(), 32);
    assert_eq!(align_of::<EMRSETWORLDTRANSFORM>(), 4);
    assert_eq!(size_of::<EMRMODIFYWORLDTRANSFORM>(), 36);
    assert_eq!(align_of::<EMRMODIFYWORLDTRANSFORM>(), 4);
    assert_eq!(size_of::<EMRSETPIXELV>(), 20);
    assert_eq!(align_of::<EMRSETPIXELV>(), 4);
    assert_eq!(size_of::<EMREXTFLOODFILL>(), 24);
    assert_eq!(align_of::<EMREXTFLOODFILL>(), 4);
    assert_eq!(size_of::<EMRELLIPSE>(), 24);
    assert_eq!(align_of::<EMRELLIPSE>(), 4);
    assert_eq!(size_of::<EMRROUNDRECT>(), 32);
    assert_eq!(align_of::<EMRROUNDRECT>(), 4);
    assert_eq!(size_of::<EMRARC>(), 40);
    assert_eq!(align_of::<EMRARC>(), 4);
    assert_eq!(size_of::<EMRANGLEARC>(), 28);
    assert_eq!(align_of::<EMRANGLEARC>(), 4);
    assert_eq!(size_of::<EMRPOLYLINE>(), 36);
    assert_eq!(align_of::<EMRPOLYLINE>(), 4);
    assert_eq!(size_of::<EMRPOLYLINE16>(), 32);
    assert_eq!(align_of::<EMRPOLYLINE16>(), 4);
    assert_eq!(size_of::<EMRPOLYDRAW>(), 40);
    assert_eq!(align_of::<EMRPOLYDRAW>(), 4);
    assert_eq!(size_of::<EMRPOLYDRAW16>(), 36);
    assert_eq!(align_of::<EMRPOLYDRAW16>(), 4);
    assert_eq!(size_of::<EMRPOLYPOLYLINE>(), 44);
    assert_eq!(align_of::<EMRPOLYPOLYLINE>(), 4);
    assert_eq!(size_of::<EMRPOLYPOLYLINE16>(), 40);
    assert_eq!(align_of::<EMRPOLYPOLYLINE16>(), 4);
    assert_eq!(size_of::<EMRINVERTRGN>(), 32);
    assert_eq!(align_of::<EMRINVERTRGN>(), 4);
    assert_eq!(size_of::<EMRFILLRGN>(), 36);
    assert_eq!(align_of::<EMRFILLRGN>(), 4);
    assert_eq!(size_of::<EMRFRAMERGN>(), 44);
    assert_eq!(align_of::<EMRFRAMERGN>(), 4);
    assert_eq!(size_of::<EMREXTSELECTCLIPRGN>(), 20);
    assert_eq!(align_of::<EMREXTSELECTCLIPRGN>(), 4);
    assert_eq!(size_of::<EMREXTTEXTOUTA>(), 76);
    assert_eq!(align_of::<EMREXTTEXTOUTA>(), 4);
    assert_eq!(size_of::<EMRPOLYTEXTOUTA>(), 80);
    assert_eq!(align_of::<EMRPOLYTEXTOUTA>(), 4);
    assert_eq!(size_of::<EMRBITBLT>(), 100);
    assert_eq!(align_of::<EMRBITBLT>(), 4);
    assert_eq!(size_of::<EMRSTRETCHBLT>(), 108);
    assert_eq!(align_of::<EMRSTRETCHBLT>(), 4);
    assert_eq!(size_of::<EMRMASKBLT>(), 128);
    assert_eq!(align_of::<EMRMASKBLT>(), 4);
    assert_eq!(size_of::<EMRPLGBLT>(), 140);
    assert_eq!(align_of::<EMRPLGBLT>(), 4);
    assert_eq!(size_of::<EMRSETDIBITSTODEVICE>(), 76);
    assert_eq!(align_of::<EMRSETDIBITSTODEVICE>(), 4);
    assert_eq!(size_of::<EMRSTRETCHDIBITS>(), 80);
    assert_eq!(align_of::<EMRSTRETCHDIBITS>(), 4);
    assert_eq!(size_of::<EMREXTCREATEFONTINDIRECTW>(), 332);
    assert_eq!(align_of::<EMREXTCREATEFONTINDIRECTW>(), 4);
    assert_eq!(size_of::<EMRCREATEPALETTE>(), 20);
    assert_eq!(align_of::<EMRCREATEPALETTE>(), 4);
    assert_eq!(size_of::<EMRCREATEPEN>(), 28);
    assert_eq!(align_of::<EMRCREATEPEN>(), 4);
    assert_eq!(size_of::<EMREXTCREATEPEN>(), 56);
    assert_eq!(align_of::<EMREXTCREATEPEN>(), 4);
    assert_eq!(size_of::<EMRCREATEBRUSHINDIRECT>(), 24);
    assert_eq!(align_of::<EMRCREATEBRUSHINDIRECT>(), 4);
    assert_eq!(size_of::<EMRCREATEMONOBRUSH>(), 32);
    assert_eq!(align_of::<EMRCREATEMONOBRUSH>(), 4);
    assert_eq!(size_of::<EMRCREATEDIBPATTERNBRUSHPT>(), 32);
    assert_eq!(align_of::<EMRCREATEDIBPATTERNBRUSHPT>(), 4);
    assert_eq!(size_of::<EMRFORMAT>(), 16);
    assert_eq!(align_of::<EMRFORMAT>(), 4);
    assert_eq!(size_of::<EMRGLSRECORD>(), 16);
    assert_eq!(align_of::<EMRGLSRECORD>(), 4);
    assert_eq!(size_of::<EMRGLSBOUNDEDRECORD>(), 32);
    assert_eq!(align_of::<EMRGLSBOUNDEDRECORD>(), 4);
    assert_eq!(size_of::<EMRPIXELFORMAT>(), 48);
    assert_eq!(align_of::<EMRPIXELFORMAT>(), 4);
    assert_eq!(size_of::<EMRCREATECOLORSPACE>(), 340);
    assert_eq!(align_of::<EMRCREATECOLORSPACE>(), 4);
    assert_eq!(size_of::<EMRSETCOLORSPACE>(), 12);
    assert_eq!(align_of::<EMRSETCOLORSPACE>(), 4);
    assert_eq!(size_of::<EMREXTESCAPE>(), 20);
    assert_eq!(align_of::<EMREXTESCAPE>(), 4);
    assert_eq!(size_of::<EMRNAMEDESCAPE>(), 24);
    assert_eq!(align_of::<EMRNAMEDESCAPE>(), 4);
    assert_eq!(size_of::<EMRSETICMPROFILE>(), 24);
    assert_eq!(align_of::<EMRSETICMPROFILE>(), 4);
    assert_eq!(size_of::<EMRCREATECOLORSPACEW>(), 612);
    assert_eq!(align_of::<EMRCREATECOLORSPACEW>(), 4);
    assert_eq!(size_of::<EMRCOLORMATCHTOTARGET>(), 28);
    assert_eq!(align_of::<EMRCOLORMATCHTOTARGET>(), 4);
    assert_eq!(size_of::<EMRCOLORCORRECTPALETTE>(), 24);
    assert_eq!(align_of::<EMRCOLORCORRECTPALETTE>(), 4);
    assert_eq!(size_of::<EMRALPHABLEND>(), 108);
    assert_eq!(align_of::<EMRALPHABLEND>(), 4);
    assert_eq!(size_of::<EMRGRADIENTFILL>(), 52);
    assert_eq!(align_of::<EMRGRADIENTFILL>(), 4);
    assert_eq!(size_of::<EMRTRANSPARENTBLT>(), 108);
    assert_eq!(align_of::<EMRTRANSPARENTBLT>(), 4);
    assert_eq!(size_of::<POINTFLOAT>(), 8);
    assert_eq!(align_of::<POINTFLOAT>(), 4);
    assert_eq!(size_of::<GLYPHMETRICSFLOAT>(), 24);
    assert_eq!(align_of::<GLYPHMETRICSFLOAT>(), 4);
    assert_eq!(size_of::<LAYERPLANEDESCRIPTOR>(), 32);
    assert_eq!(align_of::<LAYERPLANEDESCRIPTOR>(), 4);
    assert_eq!(size_of::<WGLSWAP>(), 16);
    assert_eq!(align_of::<WGLSWAP>(), 8);
}
#[cfg(feature = "winhttp")]
#[test]
fn um_winhttp() {
    use winapi::um::winhttp::*;
    assert_eq!(size_of::<WINHTTP_ASYNC_RESULT>(), 16);
    assert_eq!(align_of::<WINHTTP_ASYNC_RESULT>(), 8);
    assert_eq!(size_of::<URL_COMPONENTS>(), 104);
    assert_eq!(align_of::<URL_COMPONENTS>(), 8);
    assert_eq!(size_of::<WINHTTP_PROXY_INFO>(), 24);
    assert_eq!(align_of::<WINHTTP_PROXY_INFO>(), 8);
    assert_eq!(size_of::<WINHTTP_AUTOPROXY_OPTIONS>(), 32);
    assert_eq!(align_of::<WINHTTP_AUTOPROXY_OPTIONS>(), 8);
    assert_eq!(size_of::<WINHTTP_PROXY_RESULT_ENTRY>(), 32);
    assert_eq!(align_of::<WINHTTP_PROXY_RESULT_ENTRY>(), 8);
    assert_eq!(size_of::<WINHTTP_PROXY_RESULT>(), 16);
    assert_eq!(align_of::<WINHTTP_PROXY_RESULT>(), 8);
    assert_eq!(size_of::<WINHTTP_CURRENT_USER_IE_PROXY_CONFIG>(), 32);
    assert_eq!(align_of::<WINHTTP_CURRENT_USER_IE_PROXY_CONFIG>(), 8);
}
#[cfg(feature = "wininet")]
#[test]
fn um_wininet() {
    use winapi::um::wininet::*;
    assert_eq!(size_of::<INTERNET_ASYNC_RESULT>(), 16);
    assert_eq!(align_of::<INTERNET_ASYNC_RESULT>(), 8);
    assert_eq!(size_of::<INTERNET_DIAGNOSTIC_SOCKET_INFO>(), 24);
    assert_eq!(align_of::<INTERNET_DIAGNOSTIC_SOCKET_INFO>(), 8);
    assert_eq!(size_of::<INTERNET_PROXY_INFO>(), 24);
    assert_eq!(align_of::<INTERNET_PROXY_INFO>(), 8);
    assert_eq!(size_of::<INTERNET_PER_CONN_OPTIONA_Value>(), 8);
    assert_eq!(align_of::<INTERNET_PER_CONN_OPTIONA_Value>(), 8);
    assert_eq!(size_of::<INTERNET_PER_CONN_OPTIONA>(), 16);
    assert_eq!(align_of::<INTERNET_PER_CONN_OPTIONA>(), 8);
    assert_eq!(size_of::<INTERNET_PER_CONN_OPTIONW_Value>(), 8);
    assert_eq!(align_of::<INTERNET_PER_CONN_OPTIONW_Value>(), 8);
    assert_eq!(size_of::<INTERNET_PER_CONN_OPTIONW>(), 16);
    assert_eq!(align_of::<INTERNET_PER_CONN_OPTIONW>(), 8);
    assert_eq!(size_of::<INTERNET_PER_CONN_OPTION_LISTA>(), 32);
    assert_eq!(align_of::<INTERNET_PER_CONN_OPTION_LISTA>(), 8);
    assert_eq!(size_of::<INTERNET_PER_CONN_OPTION_LISTW>(), 32);
    assert_eq!(align_of::<INTERNET_PER_CONN_OPTION_LISTW>(), 8);
    assert_eq!(size_of::<INTERNET_VERSION_INFO>(), 8);
    assert_eq!(align_of::<INTERNET_VERSION_INFO>(), 4);
    assert_eq!(size_of::<HTTP_VERSION_INFO>(), 8);
    assert_eq!(align_of::<HTTP_VERSION_INFO>(), 4);
    assert_eq!(size_of::<INTERNET_CONNECTED_INFO>(), 8);
    assert_eq!(align_of::<INTERNET_CONNECTED_INFO>(), 4);
    assert_eq!(size_of::<URL_COMPONENTSA>(), 104);
    assert_eq!(align_of::<URL_COMPONENTSA>(), 8);
    assert_eq!(size_of::<URL_COMPONENTSW>(), 104);
    assert_eq!(align_of::<URL_COMPONENTSW>(), 8);
    assert_eq!(size_of::<INTERNET_CERTIFICATE_INFO>(), 64);
    assert_eq!(align_of::<INTERNET_CERTIFICATE_INFO>(), 8);
    assert_eq!(size_of::<INTERNET_BUFFERSA>(), 56);
    assert_eq!(align_of::<INTERNET_BUFFERSA>(), 8);
    assert_eq!(size_of::<INTERNET_BUFFERSW>(), 56);
    assert_eq!(align_of::<INTERNET_BUFFERSW>(), 8);
    assert_eq!(size_of::<IncomingCookieState>(), 32);
    assert_eq!(align_of::<IncomingCookieState>(), 8);
    assert_eq!(size_of::<OutgoingCookieState>(), 16);
    assert_eq!(align_of::<OutgoingCookieState>(), 8);
    assert_eq!(size_of::<InternetCookieHistory>(), 16);
    assert_eq!(align_of::<InternetCookieHistory>(), 4);
    assert_eq!(size_of::<CookieDecision>(), 8);
    assert_eq!(align_of::<CookieDecision>(), 4);
    assert_eq!(size_of::<GOPHER_FIND_DATAA>(), 808);
    assert_eq!(align_of::<GOPHER_FIND_DATAA>(), 4);
    assert_eq!(size_of::<GOPHER_FIND_DATAW>(), 1588);
    assert_eq!(align_of::<GOPHER_FIND_DATAW>(), 4);
    assert_eq!(size_of::<GOPHER_ADMIN_ATTRIBUTE_TYPE>(), 16);
    assert_eq!(align_of::<GOPHER_ADMIN_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_MOD_DATE_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_MOD_DATE_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_TTL_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(align_of::<GOPHER_TTL_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_SCORE_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(align_of::<GOPHER_SCORE_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_SITE_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_SITE_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_ORGANIZATION_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_ORGANIZATION_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_LOCATION_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_LOCATION_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE>(), 24);
    assert_eq!(align_of::<GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_TIMEZONE_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(align_of::<GOPHER_TIMEZONE_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_PROVIDER_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_PROVIDER_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_VERSION_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_VERSION_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_ABSTRACT_ATTRIBUTE_TYPE>(), 16);
    assert_eq!(align_of::<GOPHER_ABSTRACT_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_VIEW_ATTRIBUTE_TYPE>(), 24);
    assert_eq!(align_of::<GOPHER_VIEW_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_VERONICA_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(align_of::<GOPHER_VERONICA_ATTRIBUTE_TYPE>(), 4);
    assert_eq!(size_of::<GOPHER_ASK_ATTRIBUTE_TYPE>(), 16);
    assert_eq!(align_of::<GOPHER_ASK_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_UNKNOWN_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(align_of::<GOPHER_UNKNOWN_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<GOPHER_ATTRIBUTE_TYPE_AttributeType>(), 24);
    assert_eq!(align_of::<GOPHER_ATTRIBUTE_TYPE_AttributeType>(), 8);
    assert_eq!(size_of::<GOPHER_ATTRIBUTE_TYPE>(), 32);
    assert_eq!(align_of::<GOPHER_ATTRIBUTE_TYPE>(), 8);
    assert_eq!(size_of::<INTERNET_COOKIE2>(), 48);
    assert_eq!(align_of::<INTERNET_COOKIE2>(), 8);
    assert_eq!(size_of::<INTERNET_AUTH_NOTIFY_DATA>(), 24);
    assert_eq!(align_of::<INTERNET_AUTH_NOTIFY_DATA>(), 8);
    assert_eq!(size_of::<INTERNET_CACHE_ENTRY_INFOA>(), 112);
    assert_eq!(align_of::<INTERNET_CACHE_ENTRY_INFOA>(), 8);
    assert_eq!(size_of::<INTERNET_CACHE_ENTRY_INFOW>(), 112);
    assert_eq!(align_of::<INTERNET_CACHE_ENTRY_INFOW>(), 8);
    assert_eq!(size_of::<INTERNET_CACHE_TIMESTAMPS>(), 16);
    assert_eq!(align_of::<INTERNET_CACHE_TIMESTAMPS>(), 4);
    assert_eq!(size_of::<INTERNET_CACHE_GROUP_INFOA>(), 156);
    assert_eq!(align_of::<INTERNET_CACHE_GROUP_INFOA>(), 4);
    assert_eq!(size_of::<INTERNET_CACHE_GROUP_INFOW>(), 276);
    assert_eq!(align_of::<INTERNET_CACHE_GROUP_INFOW>(), 4);
    assert_eq!(size_of::<AutoProxyHelperVtbl>(), 72);
    assert_eq!(align_of::<AutoProxyHelperVtbl>(), 8);
    assert_eq!(size_of::<AUTO_PROXY_SCRIPT_BUFFER>(), 24);
    assert_eq!(align_of::<AUTO_PROXY_SCRIPT_BUFFER>(), 8);
    assert_eq!(size_of::<AutoProxyHelperFunctions>(), 8);
    assert_eq!(align_of::<AutoProxyHelperFunctions>(), 8);
}
#[cfg(feature = "winioctl")]
#[test]
fn um_winioctl() {
    use winapi::um::winioctl::*;
    assert_eq!(size_of::<STORAGE_PROPERTY_QUERY>(), 12);
    assert_eq!(align_of::<STORAGE_PROPERTY_QUERY>(), 4);
    assert_eq!(size_of::<DEVICE_TRIM_DESCRIPTOR>(), 12);
    assert_eq!(align_of::<DEVICE_TRIM_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<DISK_GEOMETRY>(), 24);
    assert_eq!(align_of::<DISK_GEOMETRY>(), 8);
    assert_eq!(size_of::<PARTITION_INFORMATION>(), 32);
    assert_eq!(align_of::<PARTITION_INFORMATION>(), 8);
    assert_eq!(size_of::<SET_PARTITION_INFORMATION>(), 1);
    assert_eq!(align_of::<SET_PARTITION_INFORMATION>(), 1);
    assert_eq!(size_of::<DRIVE_LAYOUT_INFORMATION>(), 40);
    assert_eq!(align_of::<DRIVE_LAYOUT_INFORMATION>(), 8);
    assert_eq!(size_of::<VERIFY_INFORMATION>(), 16);
    assert_eq!(align_of::<VERIFY_INFORMATION>(), 8);
    assert_eq!(size_of::<REASSIGN_BLOCKS>(), 8);
    assert_eq!(align_of::<REASSIGN_BLOCKS>(), 4);
    assert_eq!(size_of::<REASSIGN_BLOCKS_EX>(), 12);
    assert_eq!(align_of::<REASSIGN_BLOCKS_EX>(), 1);
    assert_eq!(size_of::<PARTITION_INFORMATION_GPT>(), 112);
    assert_eq!(align_of::<PARTITION_INFORMATION_GPT>(), 8);
    assert_eq!(size_of::<PARTITION_INFORMATION_MBR>(), 24);
    assert_eq!(align_of::<PARTITION_INFORMATION_MBR>(), 4);
    assert_eq!(size_of::<SET_PARTITION_INFORMATION_EX_u>(), 112);
    assert_eq!(align_of::<SET_PARTITION_INFORMATION_EX_u>(), 8);
    assert_eq!(size_of::<SET_PARTITION_INFORMATION_EX>(), 120);
    assert_eq!(align_of::<SET_PARTITION_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<CREATE_DISK_GPT>(), 20);
    assert_eq!(align_of::<CREATE_DISK_GPT>(), 4);
    assert_eq!(size_of::<CREATE_DISK_MBR>(), 4);
    assert_eq!(align_of::<CREATE_DISK_MBR>(), 4);
    assert_eq!(size_of::<CREATE_DISK_u>(), 20);
    assert_eq!(align_of::<CREATE_DISK_u>(), 4);
    assert_eq!(size_of::<CREATE_DISK>(), 24);
    assert_eq!(align_of::<CREATE_DISK>(), 4);
    assert_eq!(size_of::<GET_LENGTH_INFORMATION>(), 8);
    assert_eq!(align_of::<GET_LENGTH_INFORMATION>(), 8);
    assert_eq!(size_of::<PARTITION_INFORMATION_EX_u>(), 112);
    assert_eq!(align_of::<PARTITION_INFORMATION_EX_u>(), 8);
    assert_eq!(size_of::<PARTITION_INFORMATION_EX>(), 144);
    assert_eq!(align_of::<PARTITION_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<DRIVE_LAYOUT_INFORMATION_GPT>(), 40);
    assert_eq!(align_of::<DRIVE_LAYOUT_INFORMATION_GPT>(), 8);
    assert_eq!(size_of::<DRIVE_LAYOUT_INFORMATION_MBR>(), 8);
    assert_eq!(align_of::<DRIVE_LAYOUT_INFORMATION_MBR>(), 4);
    assert_eq!(size_of::<DRIVE_LAYOUT_INFORMATION_EX_u>(), 40);
    assert_eq!(align_of::<DRIVE_LAYOUT_INFORMATION_EX_u>(), 8);
    assert_eq!(size_of::<DRIVE_LAYOUT_INFORMATION_EX>(), 192);
    assert_eq!(align_of::<DRIVE_LAYOUT_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<PATHNAME_BUFFER>(), 8);
    assert_eq!(align_of::<PATHNAME_BUFFER>(), 4);
    assert_eq!(size_of::<FSCTL_QUERY_FAT_BPB_BUFFER>(), 36);
    assert_eq!(align_of::<FSCTL_QUERY_FAT_BPB_BUFFER>(), 1);
    assert_eq!(size_of::<NTFS_VOLUME_DATA_BUFFER>(), 96);
    assert_eq!(align_of::<NTFS_VOLUME_DATA_BUFFER>(), 8);
    assert_eq!(size_of::<NTFS_EXTENDED_VOLUME_DATA>(), 32);
    assert_eq!(align_of::<NTFS_EXTENDED_VOLUME_DATA>(), 4);
    assert_eq!(size_of::<REFS_VOLUME_DATA_BUFFER>(), 152);
    assert_eq!(align_of::<REFS_VOLUME_DATA_BUFFER>(), 8);
    assert_eq!(size_of::<STARTING_LCN_INPUT_BUFFER>(), 8);
    assert_eq!(align_of::<STARTING_LCN_INPUT_BUFFER>(), 8);
    assert_eq!(size_of::<VOLUME_BITMAP_BUFFER>(), 24);
    assert_eq!(align_of::<VOLUME_BITMAP_BUFFER>(), 8);
    assert_eq!(size_of::<STARTING_VCN_INPUT_BUFFER>(), 8);
    assert_eq!(align_of::<STARTING_VCN_INPUT_BUFFER>(), 8);
    assert_eq!(size_of::<RETRIEVAL_POINTERS_BUFFER_INTERNAL>(), 16);
    assert_eq!(align_of::<RETRIEVAL_POINTERS_BUFFER_INTERNAL>(), 8);
    assert_eq!(size_of::<RETRIEVAL_POINTERS_BUFFER>(), 32);
    assert_eq!(align_of::<RETRIEVAL_POINTERS_BUFFER>(), 8);
    assert_eq!(size_of::<NTFS_FILE_RECORD_INPUT_BUFFER>(), 8);
    assert_eq!(align_of::<NTFS_FILE_RECORD_INPUT_BUFFER>(), 8);
    assert_eq!(size_of::<NTFS_FILE_RECORD_OUTPUT_BUFFER>(), 16);
    assert_eq!(align_of::<NTFS_FILE_RECORD_OUTPUT_BUFFER>(), 8);
    assert_eq!(size_of::<MOVE_FILE_DATA>(), 32);
    assert_eq!(align_of::<MOVE_FILE_DATA>(), 8);
    assert_eq!(size_of::<MOVE_FILE_RECORD_DATA>(), 24);
    assert_eq!(align_of::<MOVE_FILE_RECORD_DATA>(), 8);
    assert_eq!(size_of::<DISK_EXTENT>(), 24);
    assert_eq!(align_of::<DISK_EXTENT>(), 8);
    assert_eq!(size_of::<VOLUME_DISK_EXTENTS>(), 32);
    assert_eq!(align_of::<VOLUME_DISK_EXTENTS>(), 8);
    assert_eq!(size_of::<DISK_PERFORMANCE>(), 88);
    assert_eq!(align_of::<DISK_PERFORMANCE>(), 8);
}
#[cfg(feature = "winnetwk")]
#[test]
fn um_winnetwk() {
    use winapi::um::winnetwk::*;
    assert_eq!(size_of::<NETRESOURCEA>(), 48);
    assert_eq!(align_of::<NETRESOURCEA>(), 8);
    assert_eq!(size_of::<NETRESOURCEW>(), 48);
    assert_eq!(align_of::<NETRESOURCEW>(), 8);
    assert_eq!(size_of::<CONNECTDLGSTRUCTA>(), 32);
    assert_eq!(align_of::<CONNECTDLGSTRUCTA>(), 8);
    assert_eq!(size_of::<CONNECTDLGSTRUCTW>(), 32);
    assert_eq!(align_of::<CONNECTDLGSTRUCTW>(), 8);
    assert_eq!(size_of::<DISCDLGSTRUCTA>(), 40);
    assert_eq!(align_of::<DISCDLGSTRUCTA>(), 8);
    assert_eq!(size_of::<DISCDLGSTRUCTW>(), 40);
    assert_eq!(align_of::<DISCDLGSTRUCTW>(), 8);
    assert_eq!(size_of::<UNIVERSAL_NAME_INFOA>(), 8);
    assert_eq!(align_of::<UNIVERSAL_NAME_INFOA>(), 8);
    assert_eq!(size_of::<UNIVERSAL_NAME_INFOW>(), 8);
    assert_eq!(align_of::<UNIVERSAL_NAME_INFOW>(), 8);
    assert_eq!(size_of::<REMOTE_NAME_INFOA>(), 24);
    assert_eq!(align_of::<REMOTE_NAME_INFOA>(), 8);
    assert_eq!(size_of::<REMOTE_NAME_INFOW>(), 24);
    assert_eq!(align_of::<REMOTE_NAME_INFOW>(), 8);
    assert_eq!(size_of::<NETINFOSTRUCT>(), 40);
    assert_eq!(align_of::<NETINFOSTRUCT>(), 8);
    assert_eq!(size_of::<NETCONNECTINFOSTRUCT>(), 20);
    assert_eq!(align_of::<NETCONNECTINFOSTRUCT>(), 4);
}
#[cfg(feature = "winnls")]
#[test]
fn um_winnls() {
    use winapi::um::winnls::*;
    assert_eq!(size_of::<CPINFO>(), 20);
    assert_eq!(align_of::<CPINFO>(), 4);
    assert_eq!(size_of::<CPINFOEXA>(), 284);
    assert_eq!(align_of::<CPINFOEXA>(), 4);
    assert_eq!(size_of::<CPINFOEXW>(), 544);
    assert_eq!(align_of::<CPINFOEXW>(), 4);
    assert_eq!(size_of::<NUMBERFMTA>(), 40);
    assert_eq!(align_of::<NUMBERFMTA>(), 8);
    assert_eq!(size_of::<NUMBERFMTW>(), 40);
    assert_eq!(align_of::<NUMBERFMTW>(), 8);
    assert_eq!(size_of::<CURRENCYFMTA>(), 48);
    assert_eq!(align_of::<CURRENCYFMTA>(), 8);
    assert_eq!(size_of::<CURRENCYFMTW>(), 48);
    assert_eq!(align_of::<CURRENCYFMTW>(), 8);
    assert_eq!(size_of::<NLSVERSIONINFO>(), 32);
    assert_eq!(align_of::<NLSVERSIONINFO>(), 4);
    assert_eq!(size_of::<NLSVERSIONINFOEX>(), 32);
    assert_eq!(align_of::<NLSVERSIONINFOEX>(), 4);
    assert_eq!(size_of::<FILEMUIINFO>(), 80);
    assert_eq!(align_of::<FILEMUIINFO>(), 4);
}
#[cfg(feature = "winnt")]
#[test]
fn um_winnt() {
    use winapi::um::winnt::*;
    assert_eq!(size_of::<PROCESSOR_NUMBER>(), 4);
    assert_eq!(align_of::<PROCESSOR_NUMBER>(), 2);
    assert_eq!(size_of::<GROUP_AFFINITY>(), 16);
    assert_eq!(align_of::<GROUP_AFFINITY>(), 8);
    assert_eq!(size_of::<FLOAT128>(), 16);
    assert_eq!(align_of::<FLOAT128>(), 8);
    assert_eq!(size_of::<LUID>(), 8);
    assert_eq!(align_of::<LUID>(), 4);
    assert_eq!(size_of::<LIST_ENTRY>(), 16);
    assert_eq!(align_of::<LIST_ENTRY>(), 8);
    assert_eq!(size_of::<SINGLE_LIST_ENTRY>(), 8);
    assert_eq!(align_of::<SINGLE_LIST_ENTRY>(), 8);
    assert_eq!(size_of::<LIST_ENTRY32>(), 8);
    assert_eq!(align_of::<LIST_ENTRY32>(), 4);
    assert_eq!(size_of::<LIST_ENTRY64>(), 16);
    assert_eq!(align_of::<LIST_ENTRY64>(), 8);
    assert_eq!(size_of::<OBJECTID>(), 20);
    assert_eq!(align_of::<OBJECTID>(), 4);
    // align = 16
    // assert_eq!(size_of::<M128A>(), 16);
    // assert_eq!(align_of::<M128A>(), 16);
    // align = 16
    // assert_eq!(size_of::<XSAVE_FORMAT>(), 512);
    // assert_eq!(align_of::<XSAVE_FORMAT>(), 16);
    assert_eq!(size_of::<XSAVE_AREA_HEADER>(), 64);
    assert_eq!(align_of::<XSAVE_AREA_HEADER>(), 8);
    // align = 16
    // assert_eq!(size_of::<XSAVE_AREA>(), 576);
    // assert_eq!(align_of::<XSAVE_AREA>(), 16);
    assert_eq!(size_of::<XSTATE_CONTEXT>(), 32);
    assert_eq!(align_of::<XSTATE_CONTEXT>(), 8);
    // align = 16
    // assert_eq!(size_of::<CONTEXT_u_s>(), 416);
    // assert_eq!(align_of::<CONTEXT_u_s>(), 16);
    // align = 16
    // assert_eq!(size_of::<CONTEXT_u>(), 512);
    // assert_eq!(align_of::<CONTEXT_u>(), 16);
    // align = 16
    // assert_eq!(size_of::<CONTEXT>(), 1232);
    // assert_eq!(align_of::<CONTEXT>(), 16);
    assert_eq!(size_of::<UNWIND_HISTORY_TABLE_ENTRY>(), 16);
    assert_eq!(align_of::<UNWIND_HISTORY_TABLE_ENTRY>(), 8);
    assert_eq!(size_of::<UNWIND_HISTORY_TABLE>(), 216);
    assert_eq!(align_of::<UNWIND_HISTORY_TABLE>(), 8);
    assert_eq!(size_of::<DISPATCHER_CONTEXT>(), 80);
    assert_eq!(align_of::<DISPATCHER_CONTEXT>(), 8);
    assert_eq!(size_of::<KNONVOLATILE_CONTEXT_POINTERS_u1_s>(), 128);
    assert_eq!(align_of::<KNONVOLATILE_CONTEXT_POINTERS_u1_s>(), 8);
    assert_eq!(size_of::<KNONVOLATILE_CONTEXT_POINTERS_u1>(), 128);
    assert_eq!(align_of::<KNONVOLATILE_CONTEXT_POINTERS_u1>(), 8);
    assert_eq!(size_of::<KNONVOLATILE_CONTEXT_POINTERS_u2_s>(), 128);
    assert_eq!(align_of::<KNONVOLATILE_CONTEXT_POINTERS_u2_s>(), 8);
    assert_eq!(size_of::<KNONVOLATILE_CONTEXT_POINTERS_u2>(), 128);
    assert_eq!(align_of::<KNONVOLATILE_CONTEXT_POINTERS_u2>(), 8);
    assert_eq!(size_of::<KNONVOLATILE_CONTEXT_POINTERS>(), 256);
    assert_eq!(align_of::<KNONVOLATILE_CONTEXT_POINTERS>(), 8);
    assert_eq!(size_of::<LDT_ENTRY_Bytes>(), 4);
    assert_eq!(align_of::<LDT_ENTRY_Bytes>(), 1);
    assert_eq!(size_of::<LDT_ENTRY_Bits>(), 4);
    assert_eq!(align_of::<LDT_ENTRY_Bits>(), 4);
    assert_eq!(size_of::<LDT_ENTRY_HighWord>(), 4);
    assert_eq!(align_of::<LDT_ENTRY_HighWord>(), 4);
    assert_eq!(size_of::<LDT_ENTRY>(), 8);
    assert_eq!(align_of::<LDT_ENTRY>(), 4);
    assert_eq!(size_of::<SCOPE_TABLE_AMD64>(), 20);
    assert_eq!(align_of::<SCOPE_TABLE_AMD64>(), 4);
    assert_eq!(size_of::<SCOPE_TABLE_AMD64>(), 20);
    assert_eq!(align_of::<SCOPE_TABLE_AMD64>(), 4);
    assert_eq!(size_of::<SCOPE_TABLE_AMD64_ScopeRecord>(), 16);
    assert_eq!(align_of::<SCOPE_TABLE_AMD64_ScopeRecord>(), 4);
    assert_eq!(size_of::<WOW64_FLOATING_SAVE_AREA>(), 112);
    assert_eq!(align_of::<WOW64_FLOATING_SAVE_AREA>(), 4);
    assert_eq!(size_of::<WOW64_CONTEXT>(), 716);
    assert_eq!(align_of::<WOW64_CONTEXT>(), 4);
    assert_eq!(size_of::<WOW64_LDT_ENTRY_Bytes>(), 4);
    assert_eq!(align_of::<WOW64_LDT_ENTRY_Bytes>(), 1);
    assert_eq!(size_of::<WOW64_LDT_ENTRY_Bits>(), 4);
    assert_eq!(align_of::<WOW64_LDT_ENTRY_Bits>(), 4);
    assert_eq!(size_of::<WOW64_LDT_ENTRY_HighWord>(), 4);
    assert_eq!(align_of::<WOW64_LDT_ENTRY_HighWord>(), 4);
    assert_eq!(size_of::<WOW64_LDT_ENTRY>(), 8);
    assert_eq!(align_of::<WOW64_LDT_ENTRY>(), 4);
    assert_eq!(size_of::<WOW64_DESCRIPTOR_TABLE_ENTRY>(), 12);
    assert_eq!(align_of::<WOW64_DESCRIPTOR_TABLE_ENTRY>(), 4);
    assert_eq!(size_of::<EXCEPTION_RECORD>(), 152);
    assert_eq!(align_of::<EXCEPTION_RECORD>(), 8);
    assert_eq!(size_of::<EXCEPTION_RECORD32>(), 80);
    assert_eq!(align_of::<EXCEPTION_RECORD32>(), 4);
    assert_eq!(size_of::<EXCEPTION_RECORD64>(), 152);
    assert_eq!(align_of::<EXCEPTION_RECORD64>(), 8);
    assert_eq!(size_of::<EXCEPTION_POINTERS>(), 16);
    assert_eq!(align_of::<EXCEPTION_POINTERS>(), 8);
    assert_eq!(size_of::<GENERIC_MAPPING>(), 16);
    assert_eq!(align_of::<GENERIC_MAPPING>(), 4);
    assert_eq!(size_of::<LUID_AND_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<LUID_AND_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<SID_IDENTIFIER_AUTHORITY>(), 6);
    assert_eq!(align_of::<SID_IDENTIFIER_AUTHORITY>(), 1);
    assert_eq!(size_of::<SID>(), 12);
    assert_eq!(align_of::<SID>(), 4);
    assert_eq!(size_of::<SE_SID>(), 68);
    assert_eq!(align_of::<SE_SID>(), 4);
    assert_eq!(size_of::<SID_AND_ATTRIBUTES>(), 16);
    assert_eq!(align_of::<SID_AND_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<SID_AND_ATTRIBUTES_HASH>(), 272);
    assert_eq!(align_of::<SID_AND_ATTRIBUTES_HASH>(), 8);
    assert_eq!(size_of::<ACL>(), 8);
    assert_eq!(align_of::<ACL>(), 2);
    assert_eq!(size_of::<ACE_HEADER>(), 4);
    assert_eq!(align_of::<ACE_HEADER>(), 2);
    assert_eq!(size_of::<ACCESS_ALLOWED_ACE>(), 12);
    assert_eq!(align_of::<ACCESS_ALLOWED_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_DENIED_ACE>(), 12);
    assert_eq!(align_of::<ACCESS_DENIED_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_AUDIT_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_AUDIT_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_ALARM_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_ALARM_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_RESOURCE_ATTRIBUTE_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_RESOURCE_ATTRIBUTE_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_SCOPED_POLICY_ID_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_SCOPED_POLICY_ID_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_MANDATORY_LABEL_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_MANDATORY_LABEL_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_PROCESS_TRUST_LABEL_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_PROCESS_TRUST_LABEL_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_ACCESS_FILTER_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_ACCESS_FILTER_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_ALLOWED_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<ACCESS_ALLOWED_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_DENIED_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<ACCESS_DENIED_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_AUDIT_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<SYSTEM_AUDIT_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_ALARM_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<SYSTEM_ALARM_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_ALLOWED_CALLBACK_ACE>(), 12);
    assert_eq!(align_of::<ACCESS_ALLOWED_CALLBACK_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_DENIED_CALLBACK_ACE>(), 12);
    assert_eq!(align_of::<ACCESS_DENIED_CALLBACK_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_AUDIT_CALLBACK_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_AUDIT_CALLBACK_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_ALARM_CALLBACK_ACE>(), 12);
    assert_eq!(align_of::<SYSTEM_ALARM_CALLBACK_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_ALLOWED_CALLBACK_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<ACCESS_ALLOWED_CALLBACK_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<ACCESS_DENIED_CALLBACK_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<ACCESS_DENIED_CALLBACK_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_AUDIT_CALLBACK_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<SYSTEM_AUDIT_CALLBACK_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<SYSTEM_ALARM_CALLBACK_OBJECT_ACE>(), 48);
    assert_eq!(align_of::<SYSTEM_ALARM_CALLBACK_OBJECT_ACE>(), 4);
    assert_eq!(size_of::<ACL_REVISION_INFORMATION>(), 4);
    assert_eq!(align_of::<ACL_REVISION_INFORMATION>(), 4);
    assert_eq!(size_of::<ACL_SIZE_INFORMATION>(), 12);
    assert_eq!(align_of::<ACL_SIZE_INFORMATION>(), 4);
    assert_eq!(size_of::<SECURITY_DESCRIPTOR_RELATIVE>(), 20);
    assert_eq!(align_of::<SECURITY_DESCRIPTOR_RELATIVE>(), 4);
    assert_eq!(size_of::<SECURITY_DESCRIPTOR>(), 40);
    assert_eq!(align_of::<SECURITY_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<SECURITY_OBJECT_AI_PARAMS>(), 8);
    assert_eq!(align_of::<SECURITY_OBJECT_AI_PARAMS>(), 4);
    assert_eq!(size_of::<OBJECT_TYPE_LIST>(), 16);
    assert_eq!(align_of::<OBJECT_TYPE_LIST>(), 8);
    assert_eq!(size_of::<PRIVILEGE_SET>(), 20);
    assert_eq!(align_of::<PRIVILEGE_SET>(), 4);
    assert_eq!(size_of::<ACCESS_REASONS>(), 128);
    assert_eq!(align_of::<ACCESS_REASONS>(), 4);
    assert_eq!(size_of::<SE_SECURITY_DESCRIPTOR>(), 16);
    assert_eq!(align_of::<SE_SECURITY_DESCRIPTOR>(), 8);
    assert_eq!(size_of::<SE_ACCESS_REQUEST>(), 56);
    assert_eq!(align_of::<SE_ACCESS_REQUEST>(), 8);
    assert_eq!(size_of::<SE_ACCESS_REPLY>(), 40);
    assert_eq!(align_of::<SE_ACCESS_REPLY>(), 8);
    assert_eq!(size_of::<TOKEN_USER>(), 16);
    assert_eq!(align_of::<TOKEN_USER>(), 8);
    assert_eq!(size_of::<SE_TOKEN_USER_u1>(), 16);
    assert_eq!(align_of::<SE_TOKEN_USER_u1>(), 8);
    assert_eq!(size_of::<SE_TOKEN_USER_u2>(), 68);
    assert_eq!(align_of::<SE_TOKEN_USER_u2>(), 4);
    assert_eq!(size_of::<SE_TOKEN_USER>(), 88);
    assert_eq!(align_of::<SE_TOKEN_USER>(), 8);
    assert_eq!(size_of::<TOKEN_GROUPS>(), 24);
    assert_eq!(align_of::<TOKEN_GROUPS>(), 8);
    assert_eq!(size_of::<TOKEN_PRIVILEGES>(), 16);
    assert_eq!(align_of::<TOKEN_PRIVILEGES>(), 4);
    assert_eq!(size_of::<TOKEN_OWNER>(), 8);
    assert_eq!(align_of::<TOKEN_OWNER>(), 8);
    assert_eq!(size_of::<TOKEN_PRIMARY_GROUP>(), 8);
    assert_eq!(align_of::<TOKEN_PRIMARY_GROUP>(), 8);
    assert_eq!(size_of::<TOKEN_DEFAULT_DACL>(), 8);
    assert_eq!(align_of::<TOKEN_DEFAULT_DACL>(), 8);
    assert_eq!(size_of::<TOKEN_USER_CLAIMS>(), 8);
    assert_eq!(align_of::<TOKEN_USER_CLAIMS>(), 8);
    assert_eq!(size_of::<TOKEN_DEVICE_CLAIMS>(), 8);
    assert_eq!(align_of::<TOKEN_DEVICE_CLAIMS>(), 8);
    assert_eq!(size_of::<TOKEN_GROUPS_AND_PRIVILEGES>(), 56);
    assert_eq!(align_of::<TOKEN_GROUPS_AND_PRIVILEGES>(), 8);
    assert_eq!(size_of::<TOKEN_LINKED_TOKEN>(), 8);
    assert_eq!(align_of::<TOKEN_LINKED_TOKEN>(), 8);
    assert_eq!(size_of::<TOKEN_ELEVATION>(), 4);
    assert_eq!(align_of::<TOKEN_ELEVATION>(), 4);
    assert_eq!(size_of::<TOKEN_MANDATORY_LABEL>(), 16);
    assert_eq!(align_of::<TOKEN_MANDATORY_LABEL>(), 8);
    assert_eq!(size_of::<TOKEN_MANDATORY_POLICY>(), 4);
    assert_eq!(align_of::<TOKEN_MANDATORY_POLICY>(), 4);
    assert_eq!(size_of::<TOKEN_ACCESS_INFORMATION>(), 88);
    assert_eq!(align_of::<TOKEN_ACCESS_INFORMATION>(), 8);
    assert_eq!(size_of::<TOKEN_AUDIT_POLICY>(), 30);
    assert_eq!(align_of::<TOKEN_AUDIT_POLICY>(), 1);
    assert_eq!(size_of::<TOKEN_SOURCE>(), 16);
    assert_eq!(align_of::<TOKEN_SOURCE>(), 4);
    assert_eq!(size_of::<TOKEN_STATISTICS>(), 56);
    assert_eq!(align_of::<TOKEN_STATISTICS>(), 8);
    assert_eq!(size_of::<TOKEN_CONTROL>(), 40);
    assert_eq!(align_of::<TOKEN_CONTROL>(), 4);
    assert_eq!(size_of::<TOKEN_ORIGIN>(), 8);
    assert_eq!(align_of::<TOKEN_ORIGIN>(), 4);
    assert_eq!(size_of::<TOKEN_APPCONTAINER_INFORMATION>(), 8);
    assert_eq!(align_of::<TOKEN_APPCONTAINER_INFORMATION>(), 8);
    assert_eq!(size_of::<TOKEN_SID_INFORMATION>(), 8);
    assert_eq!(align_of::<TOKEN_SID_INFORMATION>(), 8);
    assert_eq!(size_of::<TOKEN_BNO_ISOLATION_INFORMATION>(), 16);
    assert_eq!(align_of::<TOKEN_BNO_ISOLATION_INFORMATION>(), 8);
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE>(), 16);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE>(), 8);
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE>(), 16);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE>(), 8);
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTE_V1_Values>(), 8);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTE_V1_Values>(), 8);
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTE_V1>(), 32);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTE_V1>(), 8);
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_Values>(), 4);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_Values>(), 4);
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1>(), 20);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1>(), 4);
    assert_eq!(
        size_of::<CLAIM_SECURITY_ATTRIBUTES_INFORMATION_Attribute>(),
        8
    );
    assert_eq!(
        align_of::<CLAIM_SECURITY_ATTRIBUTES_INFORMATION_Attribute>(),
        8
    );
    assert_eq!(size_of::<CLAIM_SECURITY_ATTRIBUTES_INFORMATION>(), 16);
    assert_eq!(align_of::<CLAIM_SECURITY_ATTRIBUTES_INFORMATION>(), 8);
    assert_eq!(size_of::<SECURITY_QUALITY_OF_SERVICE>(), 12);
    assert_eq!(align_of::<SECURITY_QUALITY_OF_SERVICE>(), 4);
    assert_eq!(size_of::<SE_IMPERSONATION_STATE>(), 16);
    assert_eq!(align_of::<SE_IMPERSONATION_STATE>(), 8);
    assert_eq!(size_of::<SECURITY_CAPABILITIES>(), 24);
    assert_eq!(align_of::<SECURITY_CAPABILITIES>(), 8);
    assert_eq!(size_of::<JOB_SET_ARRAY>(), 16);
    assert_eq!(align_of::<JOB_SET_ARRAY>(), 8);
    assert_eq!(size_of::<EXCEPTION_REGISTRATION_RECORD>(), 16);
    assert_eq!(align_of::<EXCEPTION_REGISTRATION_RECORD>(), 8);
    assert_eq!(size_of::<NT_TIB>(), 56);
    assert_eq!(align_of::<NT_TIB>(), 8);
    assert_eq!(size_of::<NT_TIB32>(), 28);
    assert_eq!(align_of::<NT_TIB32>(), 4);
    assert_eq!(size_of::<NT_TIB64>(), 56);
    assert_eq!(align_of::<NT_TIB64>(), 8);
    assert_eq!(size_of::<UMS_CREATE_THREAD_ATTRIBUTES>(), 24);
    assert_eq!(align_of::<UMS_CREATE_THREAD_ATTRIBUTES>(), 8);
    assert_eq!(size_of::<WOW64_ARCHITECTURE_INFORMATION>(), 4);
    assert_eq!(align_of::<WOW64_ARCHITECTURE_INFORMATION>(), 4);
    assert_eq!(size_of::<QUOTA_LIMITS>(), 48);
    assert_eq!(align_of::<QUOTA_LIMITS>(), 8);
    assert_eq!(size_of::<RATE_QUOTA_LIMIT>(), 4);
    assert_eq!(align_of::<RATE_QUOTA_LIMIT>(), 4);
    assert_eq!(size_of::<QUOTA_LIMITS_EX>(), 88);
    assert_eq!(align_of::<QUOTA_LIMITS_EX>(), 8);
    assert_eq!(size_of::<IO_COUNTERS>(), 48);
    assert_eq!(align_of::<IO_COUNTERS>(), 8);
    assert_eq!(size_of::<PROCESS_MITIGATION_ASLR_POLICY>(), 4);
    assert_eq!(align_of::<PROCESS_MITIGATION_ASLR_POLICY>(), 4);
    assert_eq!(size_of::<PROCESS_MITIGATION_DEP_POLICY>(), 8);
    assert_eq!(align_of::<PROCESS_MITIGATION_DEP_POLICY>(), 4);
    assert_eq!(
        size_of::<PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY>(),
        4
    );
    assert_eq!(
        align_of::<PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY>(),
        4
    );
    assert_eq!(
        size_of::<PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY>(),
        4
    );
    assert_eq!(
        align_of::<PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY>(),
        4
    );
    assert_eq!(
        size_of::<PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY>(),
        4
    );
    assert_eq!(
        align_of::<PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY>(),
        4
    );
    assert_eq!(size_of::<PROCESS_MITIGATION_DYNAMIC_CODE_POLICY>(), 4);
    assert_eq!(align_of::<PROCESS_MITIGATION_DYNAMIC_CODE_POLICY>(), 4);
    assert_eq!(size_of::<PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY>(), 4);
    assert_eq!(
        align_of::<PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY>(),
        4
    );
    assert_eq!(size_of::<PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY>(), 4);
    assert_eq!(align_of::<PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY>(), 4);
    assert_eq!(size_of::<PROCESS_MITIGATION_FONT_DISABLE_POLICY>(), 4);
    assert_eq!(align_of::<PROCESS_MITIGATION_FONT_DISABLE_POLICY>(), 4);
    assert_eq!(size_of::<PROCESS_MITIGATION_IMAGE_LOAD_POLICY>(), 4);
    assert_eq!(align_of::<PROCESS_MITIGATION_IMAGE_LOAD_POLICY>(), 4);
    assert_eq!(size_of::<PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY>(), 4);
    assert_eq!(
        align_of::<PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY>(),
        4
    );
    assert_eq!(
        size_of::<PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY>(),
        4
    );
    assert_eq!(
        align_of::<PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY>(),
        4
    );
    assert_eq!(size_of::<PROCESS_MITIGATION_CHILD_PROCESS_POLICY>(), 4);
    assert_eq!(align_of::<PROCESS_MITIGATION_CHILD_PROCESS_POLICY>(), 4);
    assert_eq!(size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>(), 48);
    assert_eq!(align_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_BASIC_LIMIT_INFORMATION>(), 64);
    assert_eq!(align_of::<JOBOBJECT_BASIC_LIMIT_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>(), 144);
    assert_eq!(align_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_BASIC_PROCESS_ID_LIST>(), 16);
    assert_eq!(align_of::<JOBOBJECT_BASIC_PROCESS_ID_LIST>(), 8);
    assert_eq!(size_of::<JOBOBJECT_BASIC_UI_RESTRICTIONS>(), 4);
    assert_eq!(align_of::<JOBOBJECT_BASIC_UI_RESTRICTIONS>(), 4);
    assert_eq!(size_of::<JOBOBJECT_SECURITY_LIMIT_INFORMATION>(), 40);
    assert_eq!(align_of::<JOBOBJECT_SECURITY_LIMIT_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>(), 4);
    assert_eq!(align_of::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>(), 4);
    assert_eq!(size_of::<JOBOBJECT_ASSOCIATE_COMPLETION_PORT>(), 16);
    assert_eq!(align_of::<JOBOBJECT_ASSOCIATE_COMPLETION_PORT>(), 8);
    assert_eq!(
        size_of::<JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION>(),
        96
    );
    assert_eq!(
        align_of::<JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<JOBOBJECT_JOBSET_INFORMATION>(), 4);
    assert_eq!(align_of::<JOBOBJECT_JOBSET_INFORMATION>(), 4);
    assert_eq!(size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION>(), 48);
    assert_eq!(align_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION>(), 8);
    assert_eq!(
        size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_u1>(),
        8
    );
    assert_eq!(
        align_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_u1>(),
        8
    );
    assert_eq!(
        size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_u2>(),
        4
    );
    assert_eq!(
        align_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_u2>(),
        4
    );
    assert_eq!(
        size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_u3>(),
        4
    );
    assert_eq!(
        align_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_u3>(),
        4
    );
    assert_eq!(size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2>(), 72);
    assert_eq!(align_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2>(), 8);
    assert_eq!(size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION>(), 80);
    assert_eq!(align_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION>(), 8);
    assert_eq!(size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_u1>(), 8);
    assert_eq!(align_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_u1>(), 8);
    assert_eq!(size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_u2>(), 4);
    assert_eq!(align_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_u2>(), 4);
    assert_eq!(size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_u3>(), 4);
    assert_eq!(align_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_u3>(), 4);
    assert_eq!(size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2>(), 104);
    assert_eq!(align_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2>(), 8);
    assert_eq!(size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_u_s>(), 4);
    assert_eq!(align_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_u_s>(), 2);
    assert_eq!(size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_u>(), 4);
    assert_eq!(align_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_u>(), 4);
    assert_eq!(size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>(), 8);
    assert_eq!(align_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>(), 4);
    assert_eq!(size_of::<JOBOBJECT_NET_RATE_CONTROL_INFORMATION>(), 16);
    assert_eq!(align_of::<JOBOBJECT_NET_RATE_CONTROL_INFORMATION>(), 8);
    assert_eq!(
        size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE>(),
        48
    );
    assert_eq!(
        align_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE>(),
        8
    );
    assert_eq!(
        size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2>(),
        96
    );
    assert_eq!(
        align_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2>(),
        8
    );
    assert_eq!(
        size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3>(),
        144
    );
    assert_eq!(
        align_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3>(),
        8
    );
    assert_eq!(size_of::<JOBOBJECT_IO_ATTRIBUTION_STATS>(), 32);
    assert_eq!(align_of::<JOBOBJECT_IO_ATTRIBUTION_STATS>(), 8);
    assert_eq!(size_of::<JOBOBJECT_IO_ATTRIBUTION_INFORMATION>(), 72);
    assert_eq!(align_of::<JOBOBJECT_IO_ATTRIBUTION_INFORMATION>(), 8);
    assert_eq!(size_of::<SILOOBJECT_BASIC_INFORMATION>(), 16);
    assert_eq!(align_of::<SILOOBJECT_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<SERVERSILO_BASIC_INFORMATION>(), 12);
    assert_eq!(align_of::<SERVERSILO_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<CACHE_DESCRIPTOR>(), 12);
    assert_eq!(align_of::<CACHE_DESCRIPTOR>(), 4);
    assert_eq!(
        size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_ProcessorCore>(),
        1
    );
    assert_eq!(
        align_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_ProcessorCore>(),
        1
    );
    assert_eq!(
        size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_NumaNode>(),
        4
    );
    assert_eq!(
        align_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_NumaNode>(),
        4
    );
    assert_eq!(size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_u>(), 16);
    assert_eq!(align_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_u>(), 8);
    assert_eq!(size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>(), 8);
    assert_eq!(size_of::<PROCESSOR_RELATIONSHIP>(), 40);
    assert_eq!(align_of::<PROCESSOR_RELATIONSHIP>(), 8);
    assert_eq!(size_of::<NUMA_NODE_RELATIONSHIP>(), 40);
    assert_eq!(align_of::<NUMA_NODE_RELATIONSHIP>(), 8);
    assert_eq!(size_of::<CACHE_RELATIONSHIP>(), 48);
    assert_eq!(align_of::<CACHE_RELATIONSHIP>(), 8);
    assert_eq!(size_of::<PROCESSOR_GROUP_INFO>(), 48);
    assert_eq!(align_of::<PROCESSOR_GROUP_INFO>(), 8);
    assert_eq!(size_of::<GROUP_RELATIONSHIP>(), 72);
    assert_eq!(align_of::<GROUP_RELATIONSHIP>(), 8);
    assert_eq!(size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_u>(), 72);
    assert_eq!(align_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_u>(), 8);
    assert_eq!(size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>(), 80);
    assert_eq!(align_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>(), 8);
    assert_eq!(size_of::<SYSTEM_CPU_SET_INFORMATION_CpuSet>(), 24);
    assert_eq!(align_of::<SYSTEM_CPU_SET_INFORMATION_CpuSet>(), 8);
    assert_eq!(size_of::<SYSTEM_CPU_SET_INFORMATION>(), 32);
    assert_eq!(align_of::<SYSTEM_CPU_SET_INFORMATION>(), 8);
    assert_eq!(size_of::<SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION>(), 8);
    assert_eq!(align_of::<SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION>(), 8);
    assert_eq!(size_of::<XSTATE_FEATURE>(), 8);
    assert_eq!(align_of::<XSTATE_FEATURE>(), 4);
    assert_eq!(size_of::<XSTATE_CONFIGURATION>(), 816);
    assert_eq!(align_of::<XSTATE_CONFIGURATION>(), 8);
    assert_eq!(size_of::<MEMORY_BASIC_INFORMATION>(), 48);
    assert_eq!(align_of::<MEMORY_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<MEMORY_BASIC_INFORMATION32>(), 28);
    assert_eq!(align_of::<MEMORY_BASIC_INFORMATION32>(), 4);
    // align = 16
    // assert_eq!(size_of::<MEMORY_BASIC_INFORMATION64>(), 48);
    // assert_eq!(align_of::<MEMORY_BASIC_INFORMATION64>(), 16);
    assert_eq!(size_of::<CFG_CALL_TARGET_INFO>(), 16);
    assert_eq!(align_of::<CFG_CALL_TARGET_INFO>(), 8);
    assert_eq!(size_of::<ENCLAVE_CREATE_INFO_SGX>(), 4096);
    assert_eq!(align_of::<ENCLAVE_CREATE_INFO_SGX>(), 1);
    assert_eq!(size_of::<ENCLAVE_INIT_INFO_SGX>(), 4096);
    assert_eq!(align_of::<ENCLAVE_INIT_INFO_SGX>(), 1);
    assert_eq!(size_of::<FILE_ID_128>(), 16);
    assert_eq!(align_of::<FILE_ID_128>(), 1);
    assert_eq!(size_of::<FILE_NOTIFY_INFORMATION>(), 16);
    assert_eq!(align_of::<FILE_NOTIFY_INFORMATION>(), 4);
    assert_eq!(size_of::<FILE_SEGMENT_ELEMENT>(), 8);
    assert_eq!(align_of::<FILE_SEGMENT_ELEMENT>(), 8);
    assert_eq!(
        size_of::<REPARSE_GUID_DATA_BUFFER_GenericReparseBuffer>(),
        1
    );
    assert_eq!(
        align_of::<REPARSE_GUID_DATA_BUFFER_GenericReparseBuffer>(),
        1
    );
    assert_eq!(size_of::<REPARSE_GUID_DATA_BUFFER>(), 28);
    assert_eq!(align_of::<REPARSE_GUID_DATA_BUFFER>(), 4);
    assert_eq!(size_of::<SCRUB_DATA_INPUT>(), 896);
    assert_eq!(align_of::<SCRUB_DATA_INPUT>(), 4);
    assert_eq!(size_of::<SCRUB_PARITY_EXTENT>(), 16);
    assert_eq!(align_of::<SCRUB_PARITY_EXTENT>(), 8);
    assert_eq!(size_of::<SCRUB_PARITY_EXTENT_DATA>(), 24);
    assert_eq!(align_of::<SCRUB_PARITY_EXTENT_DATA>(), 8);
    assert_eq!(size_of::<SCRUB_DATA_OUTPUT>(), 896);
    assert_eq!(align_of::<SCRUB_DATA_OUTPUT>(), 8);
    assert_eq!(size_of::<SHARED_VIRTUAL_DISK_SUPPORT>(), 8);
    assert_eq!(align_of::<SHARED_VIRTUAL_DISK_SUPPORT>(), 4);
    assert_eq!(size_of::<NETWORK_APP_INSTANCE_EA>(), 20);
    assert_eq!(align_of::<NETWORK_APP_INSTANCE_EA>(), 4);
    assert_eq!(size_of::<CM_POWER_DATA>(), 56);
    assert_eq!(align_of::<CM_POWER_DATA>(), 4);
    assert_eq!(size_of::<POWER_USER_PRESENCE>(), 4);
    assert_eq!(align_of::<POWER_USER_PRESENCE>(), 4);
    assert_eq!(size_of::<POWER_SESSION_CONNECT>(), 2);
    assert_eq!(align_of::<POWER_SESSION_CONNECT>(), 1);
    assert_eq!(size_of::<POWER_SESSION_TIMEOUTS>(), 8);
    assert_eq!(align_of::<POWER_SESSION_TIMEOUTS>(), 4);
    assert_eq!(size_of::<POWER_SESSION_RIT_STATE>(), 8);
    assert_eq!(align_of::<POWER_SESSION_RIT_STATE>(), 4);
    assert_eq!(size_of::<POWER_SESSION_WINLOGON>(), 8);
    assert_eq!(align_of::<POWER_SESSION_WINLOGON>(), 4);
    assert_eq!(size_of::<POWER_IDLE_RESILIENCY>(), 8);
    assert_eq!(align_of::<POWER_IDLE_RESILIENCY>(), 4);
    assert_eq!(size_of::<POWER_MONITOR_INVOCATION>(), 8);
    assert_eq!(align_of::<POWER_MONITOR_INVOCATION>(), 4);
    assert_eq!(size_of::<RESUME_PERFORMANCE>(), 24);
    assert_eq!(align_of::<RESUME_PERFORMANCE>(), 8);
    assert_eq!(size_of::<SET_POWER_SETTING_VALUE>(), 32);
    assert_eq!(align_of::<SET_POWER_SETTING_VALUE>(), 4);
    assert_eq!(size_of::<NOTIFY_USER_POWER_SETTING>(), 16);
    assert_eq!(align_of::<NOTIFY_USER_POWER_SETTING>(), 4);
    assert_eq!(size_of::<APPLICATIONLAUNCH_SETTING_VALUE>(), 16);
    assert_eq!(align_of::<APPLICATIONLAUNCH_SETTING_VALUE>(), 8);
    assert_eq!(size_of::<POWER_PLATFORM_INFORMATION>(), 1);
    assert_eq!(align_of::<POWER_PLATFORM_INFORMATION>(), 1);
    assert_eq!(size_of::<BATTERY_REPORTING_SCALE>(), 8);
    assert_eq!(align_of::<BATTERY_REPORTING_SCALE>(), 4);
    assert_eq!(size_of::<PPM_WMI_LEGACY_PERFSTATE>(), 12);
    assert_eq!(align_of::<PPM_WMI_LEGACY_PERFSTATE>(), 4);
    assert_eq!(size_of::<PPM_WMI_IDLE_STATE>(), 32);
    assert_eq!(align_of::<PPM_WMI_IDLE_STATE>(), 4);
    assert_eq!(size_of::<PPM_WMI_IDLE_STATES>(), 56);
    assert_eq!(align_of::<PPM_WMI_IDLE_STATES>(), 8);
    assert_eq!(size_of::<PPM_WMI_IDLE_STATES_EX>(), 56);
    assert_eq!(align_of::<PPM_WMI_IDLE_STATES_EX>(), 8);
    assert_eq!(size_of::<PPM_WMI_PERF_STATE>(), 64);
    assert_eq!(align_of::<PPM_WMI_PERF_STATE>(), 8);
    assert_eq!(size_of::<PPM_WMI_PERF_STATES>(), 144);
    assert_eq!(align_of::<PPM_WMI_PERF_STATES>(), 8);
    assert_eq!(size_of::<PPM_WMI_PERF_STATES_EX>(), 144);
    assert_eq!(align_of::<PPM_WMI_PERF_STATES_EX>(), 8);
    assert_eq!(size_of::<PPM_IDLE_STATE_ACCOUNTING>(), 48);
    assert_eq!(align_of::<PPM_IDLE_STATE_ACCOUNTING>(), 8);
    assert_eq!(size_of::<PPM_IDLE_ACCOUNTING>(), 72);
    assert_eq!(align_of::<PPM_IDLE_ACCOUNTING>(), 8);
    assert_eq!(size_of::<PPM_IDLE_STATE_BUCKET_EX>(), 24);
    assert_eq!(align_of::<PPM_IDLE_STATE_BUCKET_EX>(), 8);
    assert_eq!(size_of::<PPM_IDLE_STATE_ACCOUNTING_EX>(), 416);
    assert_eq!(align_of::<PPM_IDLE_STATE_ACCOUNTING_EX>(), 8);
    assert_eq!(size_of::<PPM_IDLE_ACCOUNTING_EX>(), 440);
    assert_eq!(align_of::<PPM_IDLE_ACCOUNTING_EX>(), 8);
    assert_eq!(size_of::<PPM_PERFSTATE_EVENT>(), 20);
    assert_eq!(align_of::<PPM_PERFSTATE_EVENT>(), 4);
    assert_eq!(size_of::<PPM_PERFSTATE_DOMAIN_EVENT>(), 24);
    assert_eq!(align_of::<PPM_PERFSTATE_DOMAIN_EVENT>(), 8);
    assert_eq!(size_of::<PPM_IDLESTATE_EVENT>(), 16);
    assert_eq!(align_of::<PPM_IDLESTATE_EVENT>(), 8);
    assert_eq!(size_of::<PPM_THERMALCHANGE_EVENT>(), 16);
    assert_eq!(align_of::<PPM_THERMALCHANGE_EVENT>(), 8);
    assert_eq!(size_of::<PPM_THERMAL_POLICY_EVENT>(), 16);
    assert_eq!(align_of::<PPM_THERMAL_POLICY_EVENT>(), 8);
    assert_eq!(size_of::<POWER_ACTION_POLICY>(), 12);
    assert_eq!(align_of::<POWER_ACTION_POLICY>(), 4);
    assert_eq!(size_of::<SYSTEM_POWER_LEVEL>(), 24);
    assert_eq!(align_of::<SYSTEM_POWER_LEVEL>(), 4);
    assert_eq!(size_of::<SYSTEM_POWER_POLICY>(), 232);
    assert_eq!(align_of::<SYSTEM_POWER_POLICY>(), 4);
    assert_eq!(size_of::<PROCESSOR_IDLESTATE_INFO>(), 8);
    assert_eq!(align_of::<PROCESSOR_IDLESTATE_INFO>(), 4);
    assert_eq!(size_of::<PROCESSOR_IDLESTATE_POLICY_Flags>(), 2);
    assert_eq!(align_of::<PROCESSOR_IDLESTATE_POLICY_Flags>(), 2);
    assert_eq!(size_of::<PROCESSOR_IDLESTATE_POLICY>(), 32);
    assert_eq!(align_of::<PROCESSOR_IDLESTATE_POLICY>(), 4);
    assert_eq!(size_of::<PROCESSOR_POWER_POLICY_INFO>(), 20);
    assert_eq!(align_of::<PROCESSOR_POWER_POLICY_INFO>(), 4);
    assert_eq!(size_of::<PROCESSOR_POWER_POLICY>(), 76);
    assert_eq!(align_of::<PROCESSOR_POWER_POLICY>(), 4);
    assert_eq!(size_of::<PROCESSOR_PERFSTATE_POLICY_u_Flags>(), 1);
    assert_eq!(align_of::<PROCESSOR_PERFSTATE_POLICY_u_Flags>(), 1);
    assert_eq!(size_of::<PROCESSOR_PERFSTATE_POLICY_u>(), 1);
    assert_eq!(align_of::<PROCESSOR_PERFSTATE_POLICY_u>(), 1);
    assert_eq!(size_of::<PROCESSOR_PERFSTATE_POLICY>(), 28);
    assert_eq!(align_of::<PROCESSOR_PERFSTATE_POLICY>(), 4);
    assert_eq!(size_of::<ADMINISTRATOR_POWER_POLICY>(), 24);
    assert_eq!(align_of::<ADMINISTRATOR_POWER_POLICY>(), 4);
    assert_eq!(size_of::<HIBERFILE_BUCKET>(), 24);
    assert_eq!(align_of::<HIBERFILE_BUCKET>(), 8);
    assert_eq!(size_of::<SYSTEM_POWER_CAPABILITIES>(), 76);
    assert_eq!(align_of::<SYSTEM_POWER_CAPABILITIES>(), 4);
    assert_eq!(size_of::<SYSTEM_BATTERY_STATE>(), 32);
    assert_eq!(align_of::<SYSTEM_BATTERY_STATE>(), 4);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_DOS_HEADER>(), 64);
    // assert_eq!(align_of::<IMAGE_DOS_HEADER>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_OS2_HEADER>(), 64);
    // assert_eq!(align_of::<IMAGE_OS2_HEADER>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_VXD_HEADER>(), 196);
    // assert_eq!(align_of::<IMAGE_VXD_HEADER>(), 2);
    assert_eq!(size_of::<IMAGE_FILE_HEADER>(), 20);
    assert_eq!(align_of::<IMAGE_FILE_HEADER>(), 4);
    assert_eq!(size_of::<IMAGE_DATA_DIRECTORY>(), 8);
    assert_eq!(align_of::<IMAGE_DATA_DIRECTORY>(), 4);
    assert_eq!(size_of::<IMAGE_OPTIONAL_HEADER32>(), 224);
    assert_eq!(align_of::<IMAGE_OPTIONAL_HEADER32>(), 4);
    assert_eq!(size_of::<IMAGE_ROM_OPTIONAL_HEADER>(), 56);
    assert_eq!(align_of::<IMAGE_ROM_OPTIONAL_HEADER>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_OPTIONAL_HEADER64>(), 240);
    // assert_eq!(align_of::<IMAGE_OPTIONAL_HEADER64>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_NT_HEADERS64>(), 264);
    // assert_eq!(align_of::<IMAGE_NT_HEADERS64>(), 4);
    assert_eq!(size_of::<IMAGE_NT_HEADERS32>(), 248);
    assert_eq!(align_of::<IMAGE_NT_HEADERS32>(), 4);
    assert_eq!(size_of::<IMAGE_ROM_HEADERS>(), 76);
    assert_eq!(align_of::<IMAGE_ROM_HEADERS>(), 4);
    assert_eq!(size_of::<ANON_OBJECT_HEADER>(), 32);
    assert_eq!(align_of::<ANON_OBJECT_HEADER>(), 4);
    assert_eq!(size_of::<ANON_OBJECT_HEADER_V2>(), 44);
    assert_eq!(align_of::<ANON_OBJECT_HEADER_V2>(), 4);
    assert_eq!(size_of::<ANON_OBJECT_HEADER_BIGOBJ>(), 56);
    assert_eq!(align_of::<ANON_OBJECT_HEADER_BIGOBJ>(), 4);
    assert_eq!(size_of::<IMAGE_SECTION_HEADER_Misc>(), 4);
    assert_eq!(align_of::<IMAGE_SECTION_HEADER_Misc>(), 4);
    assert_eq!(size_of::<IMAGE_SECTION_HEADER>(), 40);
    assert_eq!(align_of::<IMAGE_SECTION_HEADER>(), 4);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_SYMBOL_N_Name>(), 8);
    // assert_eq!(align_of::<IMAGE_SYMBOL_N_Name>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_SYMBOL_N>(), 8);
    // assert_eq!(align_of::<IMAGE_SYMBOL_N>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_SYMBOL>(), 18);
    // assert_eq!(align_of::<IMAGE_SYMBOL>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_SYMBOL_EX_N_Name>(), 8);
    // assert_eq!(align_of::<IMAGE_SYMBOL_EX_N_Name>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_SYMBOL_EX_N>(), 8);
    // assert_eq!(align_of::<IMAGE_SYMBOL_EX_N>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_SYMBOL_EX>(), 20);
    // assert_eq!(align_of::<IMAGE_SYMBOL_EX>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_TOKEN_DEF>(), 18);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_TOKEN_DEF>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Sym_Misc_LnSz>(), 4);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Sym_Misc_LnSz>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Sym_Misc>(), 4);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Sym_Misc>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Sym_FcnAry_Function>(), 8);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Sym_FcnAry_Function>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Sym_FcnAry_Array>(), 8);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Sym_FcnAry_Array>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Sym_FcnAry>(), 8);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Sym_FcnAry>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Sym>(), 18);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Sym>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_File>(), 18);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_File>(), 1);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_Section>(), 18);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_Section>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_CRC>(), 18);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_CRC>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL>(), 18);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_EX_Sym>(), 20);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_EX_Sym>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_EX_File>(), 20);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_EX_File>(), 1);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_EX_Section>(), 20);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_EX_Section>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_EX_s>(), 20);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_EX_s>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_EX_CRC>(), 20);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_EX_CRC>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_AUX_SYMBOL_EX>(), 20);
    // assert_eq!(align_of::<IMAGE_AUX_SYMBOL_EX>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_RELOCATION_u>(), 4);
    // assert_eq!(align_of::<IMAGE_RELOCATION_u>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_RELOCATION>(), 10);
    // assert_eq!(align_of::<IMAGE_RELOCATION>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_LINENUMBER_Type>(), 4);
    // assert_eq!(align_of::<IMAGE_LINENUMBER_Type>(), 2);
    // packed = 2
    // assert_eq!(size_of::<IMAGE_LINENUMBER>(), 6);
    // assert_eq!(align_of::<IMAGE_LINENUMBER>(), 2);
    assert_eq!(size_of::<IMAGE_BASE_RELOCATION>(), 8);
    assert_eq!(align_of::<IMAGE_BASE_RELOCATION>(), 4);
    assert_eq!(size_of::<IMAGE_ARCHIVE_MEMBER_HEADER>(), 60);
    assert_eq!(align_of::<IMAGE_ARCHIVE_MEMBER_HEADER>(), 1);
    assert_eq!(size_of::<IMAGE_EXPORT_DIRECTORY>(), 40);
    assert_eq!(align_of::<IMAGE_EXPORT_DIRECTORY>(), 4);
    assert_eq!(size_of::<IMAGE_IMPORT_BY_NAME>(), 4);
    assert_eq!(align_of::<IMAGE_IMPORT_BY_NAME>(), 2);
    assert_eq!(size_of::<IMAGE_THUNK_DATA64_u1>(), 8);
    assert_eq!(align_of::<IMAGE_THUNK_DATA64_u1>(), 8);
    assert_eq!(size_of::<IMAGE_THUNK_DATA64>(), 8);
    assert_eq!(align_of::<IMAGE_THUNK_DATA64>(), 8);
    assert_eq!(size_of::<IMAGE_THUNK_DATA32_u1>(), 4);
    assert_eq!(align_of::<IMAGE_THUNK_DATA32_u1>(), 4);
    assert_eq!(size_of::<IMAGE_THUNK_DATA32>(), 4);
    assert_eq!(align_of::<IMAGE_THUNK_DATA32>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_TLS_DIRECTORY64>(), 40);
    // assert_eq!(align_of::<IMAGE_TLS_DIRECTORY64>(), 4);
    assert_eq!(size_of::<IMAGE_TLS_DIRECTORY32>(), 24);
    assert_eq!(align_of::<IMAGE_TLS_DIRECTORY32>(), 4);
    assert_eq!(size_of::<IMAGE_IMPORT_DESCRIPTOR_u>(), 4);
    assert_eq!(align_of::<IMAGE_IMPORT_DESCRIPTOR_u>(), 4);
    assert_eq!(size_of::<IMAGE_IMPORT_DESCRIPTOR>(), 20);
    assert_eq!(align_of::<IMAGE_IMPORT_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<IMAGE_BOUND_IMPORT_DESCRIPTOR>(), 8);
    assert_eq!(align_of::<IMAGE_BOUND_IMPORT_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<IMAGE_BOUND_FORWARDER_REF>(), 8);
    assert_eq!(align_of::<IMAGE_BOUND_FORWARDER_REF>(), 4);
    assert_eq!(size_of::<IMAGE_DELAYLOAD_DESCRIPTOR_Attributes>(), 4);
    assert_eq!(align_of::<IMAGE_DELAYLOAD_DESCRIPTOR_Attributes>(), 4);
    assert_eq!(size_of::<IMAGE_DELAYLOAD_DESCRIPTOR>(), 32);
    assert_eq!(align_of::<IMAGE_DELAYLOAD_DESCRIPTOR>(), 4);
    assert_eq!(size_of::<IMAGE_RESOURCE_DIRECTORY>(), 16);
    assert_eq!(align_of::<IMAGE_RESOURCE_DIRECTORY>(), 4);
    assert_eq!(size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY_u_s>(), 4);
    assert_eq!(align_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY_u_s>(), 4);
    assert_eq!(size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY_u>(), 4);
    assert_eq!(align_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY_u>(), 4);
    assert_eq!(size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>(), 8);
    assert_eq!(align_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_RESOURCE_DIRECTORY_STRING>(), 4);
    assert_eq!(align_of::<IMAGE_RESOURCE_DIRECTORY_STRING>(), 2);
    assert_eq!(size_of::<IMAGE_RESOURCE_DIR_STRING_U>(), 4);
    assert_eq!(align_of::<IMAGE_RESOURCE_DIR_STRING_U>(), 2);
    assert_eq!(size_of::<IMAGE_RESOURCE_DATA_ENTRY>(), 16);
    assert_eq!(align_of::<IMAGE_RESOURCE_DATA_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_LOAD_CONFIG_CODE_INTEGRITY>(), 12);
    assert_eq!(align_of::<IMAGE_LOAD_CONFIG_CODE_INTEGRITY>(), 4);
    assert_eq!(size_of::<IMAGE_DYNAMIC_RELOCATION_TABLE>(), 8);
    assert_eq!(align_of::<IMAGE_DYNAMIC_RELOCATION_TABLE>(), 4);
    assert_eq!(size_of::<IMAGE_DYNAMIC_RELOCATION32>(), 8);
    assert_eq!(align_of::<IMAGE_DYNAMIC_RELOCATION32>(), 1);
    assert_eq!(size_of::<IMAGE_DYNAMIC_RELOCATION64>(), 12);
    assert_eq!(align_of::<IMAGE_DYNAMIC_RELOCATION64>(), 1);
    assert_eq!(size_of::<IMAGE_DYNAMIC_RELOCATION32_V2>(), 20);
    assert_eq!(align_of::<IMAGE_DYNAMIC_RELOCATION32_V2>(), 1);
    assert_eq!(size_of::<IMAGE_DYNAMIC_RELOCATION64_V2>(), 24);
    assert_eq!(align_of::<IMAGE_DYNAMIC_RELOCATION64_V2>(), 1);
    assert_eq!(size_of::<IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER>(), 1);
    assert_eq!(align_of::<IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER>(), 1);
    assert_eq!(size_of::<IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER>(), 8);
    assert_eq!(align_of::<IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER>(), 1);
    assert_eq!(size_of::<IMAGE_LOAD_CONFIG_DIRECTORY32>(), 160);
    assert_eq!(align_of::<IMAGE_LOAD_CONFIG_DIRECTORY32>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_LOAD_CONFIG_DIRECTORY64>(), 256);
    // assert_eq!(align_of::<IMAGE_LOAD_CONFIG_DIRECTORY64>(), 4);
    assert_eq!(size_of::<IMAGE_HOT_PATCH_INFO>(), 24);
    assert_eq!(align_of::<IMAGE_HOT_PATCH_INFO>(), 4);
    assert_eq!(size_of::<IMAGE_HOT_PATCH_BASE>(), 32);
    assert_eq!(align_of::<IMAGE_HOT_PATCH_BASE>(), 4);
    assert_eq!(size_of::<IMAGE_HOT_PATCH_HASHES>(), 52);
    assert_eq!(align_of::<IMAGE_HOT_PATCH_HASHES>(), 1);
    assert_eq!(size_of::<IMAGE_CE_RUNTIME_FUNCTION_ENTRY>(), 8);
    assert_eq!(align_of::<IMAGE_CE_RUNTIME_FUNCTION_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_ARM_RUNTIME_FUNCTION_ENTRY>(), 8);
    assert_eq!(align_of::<IMAGE_ARM_RUNTIME_FUNCTION_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>(), 8);
    assert_eq!(align_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY>(), 40);
    // assert_eq!(align_of::<IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY>(), 20);
    assert_eq!(align_of::<IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_RUNTIME_FUNCTION_ENTRY_u>(), 4);
    assert_eq!(align_of::<IMAGE_RUNTIME_FUNCTION_ENTRY_u>(), 4);
    assert_eq!(size_of::<IMAGE_RUNTIME_FUNCTION_ENTRY>(), 12);
    assert_eq!(align_of::<IMAGE_RUNTIME_FUNCTION_ENTRY>(), 4);
    assert_eq!(size_of::<IMAGE_DEBUG_DIRECTORY>(), 28);
    assert_eq!(align_of::<IMAGE_DEBUG_DIRECTORY>(), 4);
    assert_eq!(size_of::<IMAGE_COFF_SYMBOLS_HEADER>(), 32);
    assert_eq!(align_of::<IMAGE_COFF_SYMBOLS_HEADER>(), 4);
    assert_eq!(size_of::<FPO_DATA>(), 16);
    assert_eq!(align_of::<FPO_DATA>(), 4);
    assert_eq!(size_of::<IMAGE_DEBUG_MISC>(), 16);
    assert_eq!(align_of::<IMAGE_DEBUG_MISC>(), 4);
    assert_eq!(size_of::<IMAGE_FUNCTION_ENTRY>(), 12);
    assert_eq!(align_of::<IMAGE_FUNCTION_ENTRY>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_FUNCTION_ENTRY64_u>(), 8);
    // assert_eq!(align_of::<IMAGE_FUNCTION_ENTRY64_u>(), 4);
    // packed = 4
    // assert_eq!(size_of::<IMAGE_FUNCTION_ENTRY64>(), 24);
    // assert_eq!(align_of::<IMAGE_FUNCTION_ENTRY64>(), 4);
    assert_eq!(size_of::<IMAGE_SEPARATE_DEBUG_HEADER>(), 48);
    assert_eq!(align_of::<IMAGE_SEPARATE_DEBUG_HEADER>(), 4);
    // packed = 4
    // assert_eq!(size_of::<NON_PAGED_DEBUG_INFO>(), 32);
    // assert_eq!(align_of::<NON_PAGED_DEBUG_INFO>(), 4);
    assert_eq!(size_of::<IMAGE_ARCHITECTURE_HEADER>(), 8);
    assert_eq!(align_of::<IMAGE_ARCHITECTURE_HEADER>(), 4);
    assert_eq!(size_of::<IMAGE_ARCHITECTURE_ENTRY>(), 8);
    assert_eq!(align_of::<IMAGE_ARCHITECTURE_ENTRY>(), 4);
    assert_eq!(size_of::<IMPORT_OBJECT_HEADER_u>(), 2);
    assert_eq!(align_of::<IMPORT_OBJECT_HEADER_u>(), 2);
    assert_eq!(size_of::<IMPORT_OBJECT_HEADER>(), 20);
    assert_eq!(align_of::<IMPORT_OBJECT_HEADER>(), 4);
    assert_eq!(size_of::<IMAGE_COR20_HEADER_u>(), 4);
    assert_eq!(align_of::<IMAGE_COR20_HEADER_u>(), 4);
    assert_eq!(size_of::<IMAGE_COR20_HEADER>(), 72);
    assert_eq!(align_of::<IMAGE_COR20_HEADER>(), 4);
    // align = 16
    // assert_eq!(size_of::<SLIST_ENTRY>(), 16);
    // assert_eq!(align_of::<SLIST_ENTRY>(), 16);
    // align = 16
    // assert_eq!(size_of::<SLIST_HEADER_s>(), 16);
    // assert_eq!(align_of::<SLIST_HEADER_s>(), 8);
    // align = 16
    // assert_eq!(size_of::<SLIST_HEADER_HeaderX64>(), 16);
    // assert_eq!(align_of::<SLIST_HEADER_HeaderX64>(), 8);
    // align = 16
    // assert_eq!(size_of::<SLIST_HEADER>(), 16);
    // assert_eq!(align_of::<SLIST_HEADER>(), 16);
    assert_eq!(size_of::<RTL_RUN_ONCE>(), 8);
    assert_eq!(align_of::<RTL_RUN_ONCE>(), 8);
    assert_eq!(size_of::<RTL_BARRIER>(), 32);
    assert_eq!(align_of::<RTL_BARRIER>(), 8);
    assert_eq!(size_of::<MESSAGE_RESOURCE_ENTRY>(), 6);
    assert_eq!(align_of::<MESSAGE_RESOURCE_ENTRY>(), 2);
    assert_eq!(size_of::<MESSAGE_RESOURCE_BLOCK>(), 12);
    assert_eq!(align_of::<MESSAGE_RESOURCE_BLOCK>(), 4);
    assert_eq!(size_of::<MESSAGE_RESOURCE_DATA>(), 16);
    assert_eq!(align_of::<MESSAGE_RESOURCE_DATA>(), 4);
    assert_eq!(size_of::<OSVERSIONINFOA>(), 148);
    assert_eq!(align_of::<OSVERSIONINFOA>(), 4);
    assert_eq!(size_of::<OSVERSIONINFOW>(), 276);
    assert_eq!(align_of::<OSVERSIONINFOW>(), 4);
    assert_eq!(size_of::<OSVERSIONINFOEXA>(), 156);
    assert_eq!(align_of::<OSVERSIONINFOEXA>(), 4);
    assert_eq!(size_of::<OSVERSIONINFOEXW>(), 284);
    assert_eq!(align_of::<OSVERSIONINFOEXW>(), 4);
    assert_eq!(size_of::<NV_MEMORY_RANGE>(), 16);
    assert_eq!(align_of::<NV_MEMORY_RANGE>(), 8);
    assert_eq!(size_of::<RTL_CRITICAL_SECTION_DEBUG>(), 48);
    assert_eq!(align_of::<RTL_CRITICAL_SECTION_DEBUG>(), 8);
    assert_eq!(size_of::<RTL_CRITICAL_SECTION>(), 40);
    assert_eq!(align_of::<RTL_CRITICAL_SECTION>(), 8);
    assert_eq!(size_of::<RTL_SRWLOCK>(), 8);
    assert_eq!(align_of::<RTL_SRWLOCK>(), 8);
    assert_eq!(size_of::<RTL_CONDITION_VARIABLE>(), 8);
    assert_eq!(align_of::<RTL_CONDITION_VARIABLE>(), 8);
    assert_eq!(size_of::<HEAP_OPTIMIZE_RESOURCES_INFORMATION>(), 8);
    assert_eq!(align_of::<HEAP_OPTIMIZE_RESOURCES_INFORMATION>(), 4);
    assert_eq!(size_of::<ACTIVATION_CONTEXT_QUERY_INDEX>(), 8);
    assert_eq!(align_of::<ACTIVATION_CONTEXT_QUERY_INDEX>(), 4);
    assert_eq!(size_of::<ASSEMBLY_FILE_DETAILED_INFORMATION>(), 32);
    assert_eq!(align_of::<ASSEMBLY_FILE_DETAILED_INFORMATION>(), 8);
    assert_eq!(
        size_of::<ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION>(),
        104
    );
    assert_eq!(
        align_of::<ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION>(),
        8
    );
    assert_eq!(size_of::<ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION>(), 12);
    assert_eq!(align_of::<ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION>(), 4);
    assert_eq!(size_of::<COMPATIBILITY_CONTEXT_ELEMENT>(), 20);
    assert_eq!(align_of::<COMPATIBILITY_CONTEXT_ELEMENT>(), 4);
    assert_eq!(size_of::<ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION>(), 4);
    assert_eq!(
        align_of::<ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION>(),
        4
    );
    assert_eq!(size_of::<SUPPORTED_OS_INFO>(), 4);
    assert_eq!(align_of::<SUPPORTED_OS_INFO>(), 2);
    assert_eq!(size_of::<ACTIVATION_CONTEXT_DETAILED_INFORMATION>(), 64);
    assert_eq!(align_of::<ACTIVATION_CONTEXT_DETAILED_INFORMATION>(), 8);
    assert_eq!(size_of::<HARDWARE_COUNTER_DATA>(), 16);
    assert_eq!(align_of::<HARDWARE_COUNTER_DATA>(), 8);
    assert_eq!(size_of::<PERFORMANCE_DATA>(), 288);
    assert_eq!(align_of::<PERFORMANCE_DATA>(), 8);
    assert_eq!(size_of::<EVENTLOGRECORD>(), 56);
    assert_eq!(align_of::<EVENTLOGRECORD>(), 4);
    assert_eq!(size_of::<EVENTSFORLOGFILE>(), 520);
    assert_eq!(align_of::<EVENTSFORLOGFILE>(), 4);
    assert_eq!(size_of::<PACKEDEVENTINFO>(), 8);
    assert_eq!(align_of::<PACKEDEVENTINFO>(), 4);
    assert_eq!(size_of::<TAPE_ERASE>(), 8);
    assert_eq!(align_of::<TAPE_ERASE>(), 4);
    assert_eq!(size_of::<TAPE_PREPARE>(), 8);
    assert_eq!(align_of::<TAPE_PREPARE>(), 4);
    assert_eq!(size_of::<TAPE_WRITE_MARKS>(), 12);
    assert_eq!(align_of::<TAPE_WRITE_MARKS>(), 4);
    assert_eq!(size_of::<TAPE_GET_POSITION>(), 16);
    assert_eq!(align_of::<TAPE_GET_POSITION>(), 8);
    assert_eq!(size_of::<TAPE_SET_POSITION>(), 24);
    assert_eq!(align_of::<TAPE_SET_POSITION>(), 8);
    assert_eq!(size_of::<TAPE_GET_DRIVE_PARAMETERS>(), 32);
    assert_eq!(align_of::<TAPE_GET_DRIVE_PARAMETERS>(), 4);
    assert_eq!(size_of::<TAPE_SET_DRIVE_PARAMETERS>(), 8);
    assert_eq!(align_of::<TAPE_SET_DRIVE_PARAMETERS>(), 4);
    assert_eq!(size_of::<TAPE_GET_MEDIA_PARAMETERS>(), 32);
    assert_eq!(align_of::<TAPE_GET_MEDIA_PARAMETERS>(), 8);
    assert_eq!(size_of::<TAPE_SET_MEDIA_PARAMETERS>(), 4);
    assert_eq!(align_of::<TAPE_SET_MEDIA_PARAMETERS>(), 4);
    assert_eq!(size_of::<TAPE_CREATE_PARTITION>(), 12);
    assert_eq!(align_of::<TAPE_CREATE_PARTITION>(), 4);
    assert_eq!(size_of::<TAPE_WMI_OPERATIONS>(), 16);
    assert_eq!(align_of::<TAPE_WMI_OPERATIONS>(), 8);
    assert_eq!(size_of::<TRANSACTION_BASIC_INFORMATION>(), 24);
    assert_eq!(align_of::<TRANSACTION_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<TRANSACTIONMANAGER_BASIC_INFORMATION>(), 24);
    assert_eq!(align_of::<TRANSACTIONMANAGER_BASIC_INFORMATION>(), 8);
    assert_eq!(size_of::<TRANSACTIONMANAGER_LOG_INFORMATION>(), 16);
    assert_eq!(align_of::<TRANSACTIONMANAGER_LOG_INFORMATION>(), 4);
    assert_eq!(size_of::<TRANSACTIONMANAGER_LOGPATH_INFORMATION>(), 8);
    assert_eq!(align_of::<TRANSACTIONMANAGER_LOGPATH_INFORMATION>(), 4);
    assert_eq!(size_of::<TRANSACTIONMANAGER_RECOVERY_INFORMATION>(), 8);
    assert_eq!(align_of::<TRANSACTIONMANAGER_RECOVERY_INFORMATION>(), 8);
    assert_eq!(size_of::<TRANSACTIONMANAGER_OLDEST_INFORMATION>(), 16);
    assert_eq!(align_of::<TRANSACTIONMANAGER_OLDEST_INFORMATION>(), 4);
    assert_eq!(size_of::<TRANSACTION_PROPERTIES_INFORMATION>(), 32);
    assert_eq!(align_of::<TRANSACTION_PROPERTIES_INFORMATION>(), 8);
    assert_eq!(size_of::<TRANSACTION_BIND_INFORMATION>(), 8);
    assert_eq!(align_of::<TRANSACTION_BIND_INFORMATION>(), 8);
    assert_eq!(size_of::<TRANSACTION_ENLISTMENT_PAIR>(), 32);
    assert_eq!(align_of::<TRANSACTION_ENLISTMENT_PAIR>(), 4);
    assert_eq!(size_of::<TRANSACTION_ENLISTMENTS_INFORMATION>(), 36);
    assert_eq!(align_of::<TRANSACTION_ENLISTMENTS_INFORMATION>(), 4);
    assert_eq!(size_of::<TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION>(), 32);
    assert_eq!(align_of::<TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION>(), 4);
    assert_eq!(size_of::<RESOURCEMANAGER_BASIC_INFORMATION>(), 24);
    assert_eq!(align_of::<RESOURCEMANAGER_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<RESOURCEMANAGER_COMPLETION_INFORMATION>(), 16);
    assert_eq!(align_of::<RESOURCEMANAGER_COMPLETION_INFORMATION>(), 8);
    assert_eq!(size_of::<ENLISTMENT_BASIC_INFORMATION>(), 48);
    assert_eq!(align_of::<ENLISTMENT_BASIC_INFORMATION>(), 4);
    assert_eq!(size_of::<ENLISTMENT_CRM_INFORMATION>(), 48);
    assert_eq!(align_of::<ENLISTMENT_CRM_INFORMATION>(), 4);
    assert_eq!(size_of::<TRANSACTION_LIST_ENTRY>(), 16);
    assert_eq!(align_of::<TRANSACTION_LIST_ENTRY>(), 4);
    assert_eq!(size_of::<TRANSACTION_LIST_INFORMATION>(), 20);
    assert_eq!(align_of::<TRANSACTION_LIST_INFORMATION>(), 4);
    assert_eq!(size_of::<KTMOBJECT_CURSOR>(), 36);
    assert_eq!(align_of::<KTMOBJECT_CURSOR>(), 4);
    assert_eq!(size_of::<TP_POOL_STACK_INFORMATION>(), 16);
    assert_eq!(align_of::<TP_POOL_STACK_INFORMATION>(), 8);
    assert_eq!(size_of::<TP_CALLBACK_ENVIRON_V3_u_s>(), 4);
    assert_eq!(align_of::<TP_CALLBACK_ENVIRON_V3_u_s>(), 4);
    assert_eq!(size_of::<TP_CALLBACK_ENVIRON_V3_u>(), 4);
    assert_eq!(align_of::<TP_CALLBACK_ENVIRON_V3_u>(), 4);
    assert_eq!(size_of::<TP_CALLBACK_ENVIRON_V3>(), 72);
    assert_eq!(align_of::<TP_CALLBACK_ENVIRON_V3>(), 8);
}
#[cfg(feature = "winreg")]
#[test]
fn um_winreg() {
    use winapi::um::winreg::*;
    assert_eq!(size_of::<VALENTA>(), 32);
    assert_eq!(align_of::<VALENTA>(), 8);
    assert_eq!(size_of::<VALENTW>(), 32);
    assert_eq!(align_of::<VALENTW>(), 8);
}
#[cfg(feature = "winsafer")]
#[test]
fn um_winsafer() {
    use winapi::um::winsafer::*;
    assert_eq!(size_of::<SAFER_CODE_PROPERTIES_V1>(), 136);
    assert_eq!(align_of::<SAFER_CODE_PROPERTIES_V1>(), 8);
    assert_eq!(size_of::<SAFER_CODE_PROPERTIES_V2>(), 176);
    assert_eq!(align_of::<SAFER_CODE_PROPERTIES_V2>(), 8);
    assert_eq!(size_of::<SAFER_IDENTIFICATION_HEADER>(), 32);
    assert_eq!(align_of::<SAFER_IDENTIFICATION_HEADER>(), 4);
    assert_eq!(size_of::<SAFER_PATHNAME_IDENTIFICATION>(), 560);
    assert_eq!(align_of::<SAFER_PATHNAME_IDENTIFICATION>(), 8);
    assert_eq!(size_of::<SAFER_HASH_IDENTIFICATION>(), 1144);
    assert_eq!(align_of::<SAFER_HASH_IDENTIFICATION>(), 8);
    assert_eq!(size_of::<SAFER_HASH_IDENTIFICATION2>(), 1216);
    assert_eq!(align_of::<SAFER_HASH_IDENTIFICATION2>(), 8);
    assert_eq!(size_of::<SAFER_URLZONE_IDENTIFICATION>(), 40);
    assert_eq!(align_of::<SAFER_URLZONE_IDENTIFICATION>(), 4);
}
#[cfg(feature = "winscard")]
#[test]
fn um_winscard() {
    use winapi::um::winscard::*;
    assert_eq!(size_of::<SCARD_READERSTATEA>(), 64);
    assert_eq!(align_of::<SCARD_READERSTATEA>(), 8);
    assert_eq!(size_of::<SCARD_READERSTATEW>(), 64);
    assert_eq!(align_of::<SCARD_READERSTATEW>(), 8);
    assert_eq!(size_of::<SCARD_ATRMASK>(), 76);
    assert_eq!(align_of::<SCARD_ATRMASK>(), 4);
    assert_eq!(size_of::<OPENCARD_SEARCH_CRITERIAA>(), 96);
    assert_eq!(align_of::<OPENCARD_SEARCH_CRITERIAA>(), 8);
    assert_eq!(size_of::<OPENCARD_SEARCH_CRITERIAW>(), 96);
    assert_eq!(align_of::<OPENCARD_SEARCH_CRITERIAW>(), 8);
    assert_eq!(size_of::<OPENCARDNAME_EXA>(), 128);
    assert_eq!(align_of::<OPENCARDNAME_EXA>(), 8);
    assert_eq!(size_of::<OPENCARDNAME_EXW>(), 128);
    assert_eq!(align_of::<OPENCARDNAME_EXW>(), 8);
    assert_eq!(
        size_of::<READER_SEL_REQUEST_ReaderAndContainerParameter>(),
        24
    );
    assert_eq!(
        align_of::<READER_SEL_REQUEST_ReaderAndContainerParameter>(),
        4
    );
    assert_eq!(size_of::<READER_SEL_REQUEST_SerialNumberParameter>(), 12);
    assert_eq!(align_of::<READER_SEL_REQUEST_SerialNumberParameter>(), 4);
    assert_eq!(size_of::<READER_SEL_REQUEST_u>(), 24);
    assert_eq!(align_of::<READER_SEL_REQUEST_u>(), 4);
    assert_eq!(size_of::<READER_SEL_REQUEST>(), 36);
    assert_eq!(align_of::<READER_SEL_REQUEST>(), 4);
    assert_eq!(size_of::<READER_SEL_RESPONSE>(), 16);
    assert_eq!(align_of::<READER_SEL_RESPONSE>(), 4);
    assert_eq!(size_of::<OPENCARDNAMEA>(), 176);
    assert_eq!(align_of::<OPENCARDNAMEA>(), 8);
    assert_eq!(size_of::<OPENCARDNAMEW>(), 176);
    assert_eq!(align_of::<OPENCARDNAMEW>(), 8);
}
#[cfg(feature = "winsmcrd")]
#[test]
fn um_winsmcrd() {
    use winapi::um::winsmcrd::*;
    assert_eq!(size_of::<SCARD_IO_REQUEST>(), 8);
    assert_eq!(align_of::<SCARD_IO_REQUEST>(), 4);
    assert_eq!(size_of::<SCARD_T0_COMMAND>(), 5);
    assert_eq!(align_of::<SCARD_T0_COMMAND>(), 1);
    assert_eq!(size_of::<SCARD_T0_REQUEST_u>(), 5);
    assert_eq!(align_of::<SCARD_T0_REQUEST_u>(), 1);
    assert_eq!(size_of::<SCARD_T0_REQUEST>(), 16);
    assert_eq!(align_of::<SCARD_T0_REQUEST>(), 4);
    assert_eq!(size_of::<SCARD_T1_REQUEST>(), 8);
    assert_eq!(align_of::<SCARD_T1_REQUEST>(), 4);
}
#[cfg(feature = "winsock2")]
#[test]
fn um_winsock2() {
    use winapi::um::winsock2::*;
    assert_eq!(size_of::<fd_set>(), 520);
    assert_eq!(align_of::<fd_set>(), 8);
    assert_eq!(size_of::<timeval>(), 8);
    assert_eq!(align_of::<timeval>(), 4);
    assert_eq!(size_of::<hostent>(), 32);
    assert_eq!(align_of::<hostent>(), 8);
    assert_eq!(size_of::<netent>(), 24);
    assert_eq!(align_of::<netent>(), 8);
    assert_eq!(size_of::<servent>(), 32);
    assert_eq!(align_of::<servent>(), 8);
    assert_eq!(size_of::<servent>(), 32);
    assert_eq!(align_of::<servent>(), 8);
    assert_eq!(size_of::<protoent>(), 24);
    assert_eq!(align_of::<protoent>(), 8);
    assert_eq!(size_of::<WSADATA>(), 408);
    assert_eq!(align_of::<WSADATA>(), 8);
    assert_eq!(size_of::<WSADATA>(), 408);
    assert_eq!(align_of::<WSADATA>(), 8);
    assert_eq!(size_of::<sockproto>(), 4);
    assert_eq!(align_of::<sockproto>(), 2);
    assert_eq!(size_of::<linger>(), 4);
    assert_eq!(align_of::<linger>(), 2);
    assert_eq!(size_of::<QOS>(), 80);
    assert_eq!(align_of::<QOS>(), 8);
    assert_eq!(size_of::<WSANETWORKEVENTS>(), 44);
    assert_eq!(align_of::<WSANETWORKEVENTS>(), 4);
    assert_eq!(size_of::<WSAPROTOCOLCHAIN>(), 32);
    assert_eq!(align_of::<WSAPROTOCOLCHAIN>(), 4);
    assert_eq!(size_of::<WSAPROTOCOL_INFOA>(), 372);
    assert_eq!(align_of::<WSAPROTOCOL_INFOA>(), 4);
    assert_eq!(size_of::<WSAPROTOCOL_INFOW>(), 628);
    assert_eq!(align_of::<WSAPROTOCOL_INFOW>(), 4);
    assert_eq!(size_of::<WSACOMPLETION_WindowMessage>(), 24);
    assert_eq!(align_of::<WSACOMPLETION_WindowMessage>(), 8);
    assert_eq!(size_of::<WSACOMPLETION_Event>(), 8);
    assert_eq!(align_of::<WSACOMPLETION_Event>(), 8);
    assert_eq!(size_of::<WSACOMPLETION_Apc>(), 16);
    assert_eq!(align_of::<WSACOMPLETION_Apc>(), 8);
    assert_eq!(size_of::<WSACOMPLETION_Port>(), 24);
    assert_eq!(align_of::<WSACOMPLETION_Port>(), 8);
    assert_eq!(size_of::<WSACOMPLETION_Parameter>(), 24);
    assert_eq!(align_of::<WSACOMPLETION_Parameter>(), 8);
    assert_eq!(size_of::<WSACOMPLETION>(), 32);
    assert_eq!(align_of::<WSACOMPLETION>(), 8);
    assert_eq!(size_of::<AFPROTOCOLS>(), 8);
    assert_eq!(align_of::<AFPROTOCOLS>(), 4);
    assert_eq!(size_of::<WSAVERSION>(), 8);
    assert_eq!(align_of::<WSAVERSION>(), 4);
    assert_eq!(size_of::<WSAQUERYSETA>(), 120);
    assert_eq!(align_of::<WSAQUERYSETA>(), 8);
    assert_eq!(size_of::<WSAQUERYSETW>(), 120);
    assert_eq!(align_of::<WSAQUERYSETW>(), 8);
    assert_eq!(size_of::<WSAQUERYSET2A>(), 112);
    assert_eq!(align_of::<WSAQUERYSET2A>(), 8);
    assert_eq!(size_of::<WSAQUERYSET2W>(), 112);
    assert_eq!(align_of::<WSAQUERYSET2W>(), 8);
    assert_eq!(size_of::<WSANSCLASSINFOA>(), 32);
    assert_eq!(align_of::<WSANSCLASSINFOA>(), 8);
    assert_eq!(size_of::<WSANSCLASSINFOW>(), 32);
    assert_eq!(align_of::<WSANSCLASSINFOW>(), 8);
    assert_eq!(size_of::<WSASERVICECLASSINFOA>(), 32);
    assert_eq!(align_of::<WSASERVICECLASSINFOA>(), 8);
    assert_eq!(size_of::<WSASERVICECLASSINFOW>(), 32);
    assert_eq!(align_of::<WSASERVICECLASSINFOW>(), 8);
    assert_eq!(size_of::<WSANAMESPACE_INFOA>(), 40);
    assert_eq!(align_of::<WSANAMESPACE_INFOA>(), 8);
    assert_eq!(size_of::<WSANAMESPACE_INFOW>(), 40);
    assert_eq!(align_of::<WSANAMESPACE_INFOW>(), 8);
    assert_eq!(size_of::<WSANAMESPACE_INFOEXA>(), 56);
    assert_eq!(align_of::<WSANAMESPACE_INFOEXA>(), 8);
    assert_eq!(size_of::<WSANAMESPACE_INFOEXW>(), 56);
    assert_eq!(align_of::<WSANAMESPACE_INFOEXW>(), 8);
    assert_eq!(size_of::<WSAPOLLFD>(), 16);
    assert_eq!(align_of::<WSAPOLLFD>(), 8);
}
#[cfg(feature = "winspool")]
#[test]
fn um_winspool() {
    use winapi::um::winspool::*;
    assert_eq!(size_of::<PRINTER_INFO_1A>(), 32);
    assert_eq!(align_of::<PRINTER_INFO_1A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_1W>(), 32);
    assert_eq!(align_of::<PRINTER_INFO_1W>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_2A>(), 136);
    assert_eq!(align_of::<PRINTER_INFO_2A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_2W>(), 136);
    assert_eq!(align_of::<PRINTER_INFO_2W>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_3>(), 8);
    assert_eq!(align_of::<PRINTER_INFO_3>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_4A>(), 24);
    assert_eq!(align_of::<PRINTER_INFO_4A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_4W>(), 24);
    assert_eq!(align_of::<PRINTER_INFO_4W>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_5A>(), 32);
    assert_eq!(align_of::<PRINTER_INFO_5A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_5W>(), 32);
    assert_eq!(align_of::<PRINTER_INFO_5W>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_6>(), 4);
    assert_eq!(align_of::<PRINTER_INFO_6>(), 4);
    assert_eq!(size_of::<PRINTER_INFO_7A>(), 16);
    assert_eq!(align_of::<PRINTER_INFO_7A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_7W>(), 16);
    assert_eq!(align_of::<PRINTER_INFO_7W>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_8A>(), 8);
    assert_eq!(align_of::<PRINTER_INFO_8A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_8W>(), 8);
    assert_eq!(align_of::<PRINTER_INFO_8W>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_9A>(), 8);
    assert_eq!(align_of::<PRINTER_INFO_9A>(), 8);
    assert_eq!(size_of::<PRINTER_INFO_9W>(), 8);
    assert_eq!(align_of::<PRINTER_INFO_9W>(), 8);
    assert_eq!(size_of::<JOB_INFO_1A>(), 96);
    assert_eq!(align_of::<JOB_INFO_1A>(), 8);
    assert_eq!(size_of::<JOB_INFO_1W>(), 96);
    assert_eq!(align_of::<JOB_INFO_1W>(), 8);
    assert_eq!(size_of::<JOB_INFO_2A>(), 160);
    assert_eq!(align_of::<JOB_INFO_2A>(), 8);
    assert_eq!(size_of::<JOB_INFO_2W>(), 160);
    assert_eq!(align_of::<JOB_INFO_2W>(), 8);
    assert_eq!(size_of::<JOB_INFO_3>(), 12);
    assert_eq!(align_of::<JOB_INFO_3>(), 4);
    assert_eq!(size_of::<JOB_INFO_4A>(), 160);
    assert_eq!(align_of::<JOB_INFO_4A>(), 8);
    assert_eq!(size_of::<JOB_INFO_4W>(), 160);
    assert_eq!(align_of::<JOB_INFO_4W>(), 8);
    assert_eq!(size_of::<ADDJOB_INFO_1A>(), 16);
    assert_eq!(align_of::<ADDJOB_INFO_1A>(), 8);
    assert_eq!(size_of::<ADDJOB_INFO_1W>(), 16);
    assert_eq!(align_of::<ADDJOB_INFO_1W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_1A>(), 8);
    assert_eq!(align_of::<DRIVER_INFO_1A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_1W>(), 8);
    assert_eq!(align_of::<DRIVER_INFO_1W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_2A>(), 48);
    assert_eq!(align_of::<DRIVER_INFO_2A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_2W>(), 48);
    assert_eq!(align_of::<DRIVER_INFO_2W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_3A>(), 80);
    assert_eq!(align_of::<DRIVER_INFO_3A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_3W>(), 80);
    assert_eq!(align_of::<DRIVER_INFO_3W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_4A>(), 88);
    assert_eq!(align_of::<DRIVER_INFO_4A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_4W>(), 88);
    assert_eq!(align_of::<DRIVER_INFO_4W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_5A>(), 64);
    assert_eq!(align_of::<DRIVER_INFO_5A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_5W>(), 64);
    assert_eq!(align_of::<DRIVER_INFO_5W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_6A>(), 136);
    assert_eq!(align_of::<DRIVER_INFO_6A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_6W>(), 136);
    assert_eq!(align_of::<DRIVER_INFO_6W>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_8A>(), 200);
    assert_eq!(align_of::<DRIVER_INFO_8A>(), 8);
    assert_eq!(size_of::<DRIVER_INFO_8W>(), 200);
    assert_eq!(align_of::<DRIVER_INFO_8W>(), 8);
    assert_eq!(size_of::<DOC_INFO_1A>(), 24);
    assert_eq!(align_of::<DOC_INFO_1A>(), 8);
    assert_eq!(size_of::<DOC_INFO_1W>(), 24);
    assert_eq!(align_of::<DOC_INFO_1W>(), 8);
    assert_eq!(size_of::<FORM_INFO_1A>(), 40);
    assert_eq!(align_of::<FORM_INFO_1A>(), 8);
    assert_eq!(size_of::<FORM_INFO_1W>(), 40);
    assert_eq!(align_of::<FORM_INFO_1W>(), 8);
    assert_eq!(size_of::<FORM_INFO_2A>(), 88);
    assert_eq!(align_of::<FORM_INFO_2A>(), 8);
    assert_eq!(size_of::<FORM_INFO_2W>(), 88);
    assert_eq!(align_of::<FORM_INFO_2W>(), 8);
    assert_eq!(size_of::<DOC_INFO_2A>(), 32);
    assert_eq!(align_of::<DOC_INFO_2A>(), 8);
    assert_eq!(size_of::<DOC_INFO_2W>(), 32);
    assert_eq!(align_of::<DOC_INFO_2W>(), 8);
    assert_eq!(size_of::<DOC_INFO_3A>(), 32);
    assert_eq!(align_of::<DOC_INFO_3A>(), 8);
    assert_eq!(size_of::<DOC_INFO_3W>(), 32);
    assert_eq!(align_of::<DOC_INFO_3W>(), 8);
    assert_eq!(size_of::<PRINTPROCESSOR_INFO_1A>(), 8);
    assert_eq!(align_of::<PRINTPROCESSOR_INFO_1A>(), 8);
    assert_eq!(size_of::<PRINTPROCESSOR_INFO_1W>(), 8);
    assert_eq!(align_of::<PRINTPROCESSOR_INFO_1W>(), 8);
    assert_eq!(size_of::<PRINTPROCESSOR_CAPS_1>(), 16);
    assert_eq!(align_of::<PRINTPROCESSOR_CAPS_1>(), 4);
    assert_eq!(size_of::<PRINTPROCESSOR_CAPS_2>(), 36);
    assert_eq!(align_of::<PRINTPROCESSOR_CAPS_2>(), 4);
    assert_eq!(size_of::<PORT_INFO_1A>(), 8);
    assert_eq!(align_of::<PORT_INFO_1A>(), 8);
    assert_eq!(size_of::<PORT_INFO_1W>(), 8);
    assert_eq!(align_of::<PORT_INFO_1W>(), 8);
    assert_eq!(size_of::<PORT_INFO_2A>(), 32);
    assert_eq!(align_of::<PORT_INFO_2A>(), 8);
    assert_eq!(size_of::<PORT_INFO_2W>(), 32);
    assert_eq!(align_of::<PORT_INFO_2W>(), 8);
    assert_eq!(size_of::<PORT_INFO_3A>(), 24);
    assert_eq!(align_of::<PORT_INFO_3A>(), 8);
    assert_eq!(size_of::<PORT_INFO_3W>(), 24);
    assert_eq!(align_of::<PORT_INFO_3W>(), 8);
    assert_eq!(size_of::<MONITOR_INFO_1A>(), 8);
    assert_eq!(align_of::<MONITOR_INFO_1A>(), 8);
    assert_eq!(size_of::<MONITOR_INFO_1W>(), 8);
    assert_eq!(align_of::<MONITOR_INFO_1W>(), 8);
    assert_eq!(size_of::<MONITOR_INFO_2A>(), 24);
    assert_eq!(align_of::<MONITOR_INFO_2A>(), 8);
    assert_eq!(size_of::<MONITOR_INFO_2W>(), 24);
    assert_eq!(align_of::<MONITOR_INFO_2W>(), 8);
    assert_eq!(size_of::<DATATYPES_INFO_1A>(), 8);
    assert_eq!(align_of::<DATATYPES_INFO_1A>(), 8);
    assert_eq!(size_of::<DATATYPES_INFO_1W>(), 8);
    assert_eq!(align_of::<DATATYPES_INFO_1W>(), 8);
    assert_eq!(size_of::<PRINTER_DEFAULTSA>(), 24);
    assert_eq!(align_of::<PRINTER_DEFAULTSA>(), 8);
    assert_eq!(size_of::<PRINTER_DEFAULTSW>(), 24);
    assert_eq!(align_of::<PRINTER_DEFAULTSW>(), 8);
    assert_eq!(size_of::<PRINTER_ENUM_VALUESA>(), 32);
    assert_eq!(align_of::<PRINTER_ENUM_VALUESA>(), 8);
    assert_eq!(size_of::<PRINTER_ENUM_VALUESW>(), 32);
    assert_eq!(align_of::<PRINTER_ENUM_VALUESW>(), 8);
    assert_eq!(size_of::<PRINTER_NOTIFY_OPTIONS_TYPE>(), 24);
    assert_eq!(align_of::<PRINTER_NOTIFY_OPTIONS_TYPE>(), 8);
    assert_eq!(size_of::<PRINTER_NOTIFY_OPTIONS>(), 24);
    assert_eq!(align_of::<PRINTER_NOTIFY_OPTIONS>(), 8);
    assert_eq!(size_of::<PRINTER_NOTIFY_INFO_DATA_NotifyData_Data>(), 16);
    assert_eq!(align_of::<PRINTER_NOTIFY_INFO_DATA_NotifyData_Data>(), 8);
    assert_eq!(size_of::<PRINTER_NOTIFY_INFO_DATA_NotifyData>(), 16);
    assert_eq!(align_of::<PRINTER_NOTIFY_INFO_DATA_NotifyData>(), 8);
    assert_eq!(size_of::<PRINTER_NOTIFY_INFO_DATA>(), 32);
    assert_eq!(align_of::<PRINTER_NOTIFY_INFO_DATA>(), 8);
    assert_eq!(size_of::<PRINTER_NOTIFY_INFO>(), 48);
    assert_eq!(align_of::<PRINTER_NOTIFY_INFO>(), 8);
    assert_eq!(size_of::<BINARY_CONTAINER>(), 16);
    assert_eq!(align_of::<BINARY_CONTAINER>(), 8);
    assert_eq!(size_of::<BIDI_DATA_u>(), 16);
    assert_eq!(align_of::<BIDI_DATA_u>(), 8);
    assert_eq!(size_of::<BIDI_DATA>(), 24);
    assert_eq!(align_of::<BIDI_DATA>(), 8);
    assert_eq!(size_of::<BIDI_REQUEST_DATA>(), 40);
    assert_eq!(align_of::<BIDI_REQUEST_DATA>(), 8);
    assert_eq!(size_of::<BIDI_REQUEST_CONTAINER>(), 56);
    assert_eq!(align_of::<BIDI_REQUEST_CONTAINER>(), 8);
    assert_eq!(size_of::<BIDI_RESPONSE_DATA>(), 40);
    assert_eq!(align_of::<BIDI_RESPONSE_DATA>(), 8);
    assert_eq!(size_of::<BIDI_RESPONSE_CONTAINER>(), 56);
    assert_eq!(align_of::<BIDI_RESPONSE_CONTAINER>(), 8);
    assert_eq!(size_of::<PROVIDOR_INFO_1A>(), 24);
    assert_eq!(align_of::<PROVIDOR_INFO_1A>(), 8);
    assert_eq!(size_of::<PROVIDOR_INFO_1W>(), 24);
    assert_eq!(align_of::<PROVIDOR_INFO_1W>(), 8);
    assert_eq!(size_of::<PROVIDOR_INFO_2A>(), 8);
    assert_eq!(align_of::<PROVIDOR_INFO_2A>(), 8);
    assert_eq!(size_of::<PROVIDOR_INFO_2W>(), 8);
    assert_eq!(align_of::<PROVIDOR_INFO_2W>(), 8);
    assert_eq!(size_of::<PRINTER_OPTIONSA>(), 8);
    assert_eq!(align_of::<PRINTER_OPTIONSA>(), 4);
    assert_eq!(size_of::<PRINTER_OPTIONSW>(), 8);
    assert_eq!(align_of::<PRINTER_OPTIONSW>(), 4);
    assert_eq!(size_of::<PRINTER_CONNECTION_INFO_1A>(), 16);
    assert_eq!(align_of::<PRINTER_CONNECTION_INFO_1A>(), 8);
    assert_eq!(size_of::<PRINTER_CONNECTION_INFO_1W>(), 16);
    assert_eq!(align_of::<PRINTER_CONNECTION_INFO_1W>(), 8);
    assert_eq!(size_of::<CORE_PRINTER_DRIVERA>(), 296);
    assert_eq!(align_of::<CORE_PRINTER_DRIVERA>(), 8);
    assert_eq!(size_of::<CORE_PRINTER_DRIVERW>(), 552);
    assert_eq!(align_of::<CORE_PRINTER_DRIVERW>(), 8);
    assert_eq!(size_of::<PrintPropertyValue_value_propertyBlob>(), 16);
    assert_eq!(align_of::<PrintPropertyValue_value_propertyBlob>(), 8);
    assert_eq!(size_of::<PrintPropertyValue_value>(), 16);
    assert_eq!(align_of::<PrintPropertyValue_value>(), 8);
    assert_eq!(size_of::<PrintPropertyValue>(), 24);
    assert_eq!(align_of::<PrintPropertyValue>(), 8);
    assert_eq!(size_of::<PrintNamedProperty>(), 32);
    assert_eq!(align_of::<PrintNamedProperty>(), 8);
    assert_eq!(size_of::<PrintPropertiesCollection>(), 16);
    assert_eq!(align_of::<PrintPropertiesCollection>(), 8);
    assert_eq!(size_of::<PRINT_EXECUTION_DATA>(), 8);
    assert_eq!(align_of::<PRINT_EXECUTION_DATA>(), 4);
}
#[cfg(feature = "winsvc")]
#[test]
fn um_winsvc() {
    use winapi::um::winsvc::*;
    assert_eq!(size_of::<SERVICE_STATUS>(), 28);
    assert_eq!(align_of::<SERVICE_STATUS>(), 4);
    assert_eq!(size_of::<SERVICE_STATUS_PROCESS>(), 36);
    assert_eq!(align_of::<SERVICE_STATUS_PROCESS>(), 4);
    assert_eq!(size_of::<ENUM_SERVICE_STATUSA>(), 48);
    assert_eq!(align_of::<ENUM_SERVICE_STATUSA>(), 8);
    assert_eq!(size_of::<ENUM_SERVICE_STATUSW>(), 48);
    assert_eq!(align_of::<ENUM_SERVICE_STATUSW>(), 8);
    assert_eq!(size_of::<ENUM_SERVICE_STATUS_PROCESSA>(), 56);
    assert_eq!(align_of::<ENUM_SERVICE_STATUS_PROCESSA>(), 8);
    assert_eq!(size_of::<ENUM_SERVICE_STATUS_PROCESSW>(), 56);
    assert_eq!(align_of::<ENUM_SERVICE_STATUS_PROCESSW>(), 8);
    assert_eq!(size_of::<QUERY_SERVICE_LOCK_STATUSA>(), 24);
    assert_eq!(align_of::<QUERY_SERVICE_LOCK_STATUSA>(), 8);
    assert_eq!(size_of::<QUERY_SERVICE_LOCK_STATUSW>(), 24);
    assert_eq!(align_of::<QUERY_SERVICE_LOCK_STATUSW>(), 8);
    assert_eq!(size_of::<QUERY_SERVICE_CONFIGA>(), 64);
    assert_eq!(align_of::<QUERY_SERVICE_CONFIGA>(), 8);
    assert_eq!(size_of::<QUERY_SERVICE_CONFIGW>(), 64);
    assert_eq!(align_of::<QUERY_SERVICE_CONFIGW>(), 8);
    assert_eq!(size_of::<SERVICE_DESCRIPTIONA>(), 8);
    assert_eq!(align_of::<SERVICE_DESCRIPTIONA>(), 8);
    assert_eq!(size_of::<SERVICE_DESCRIPTIONW>(), 8);
    assert_eq!(align_of::<SERVICE_DESCRIPTIONW>(), 8);
    assert_eq!(size_of::<SERVICE_TABLE_ENTRYA>(), 16);
    assert_eq!(align_of::<SERVICE_TABLE_ENTRYA>(), 8);
    assert_eq!(size_of::<SERVICE_TABLE_ENTRYW>(), 16);
    assert_eq!(align_of::<SERVICE_TABLE_ENTRYW>(), 8);
    assert_eq!(size_of::<SERVICE_NOTIFY_1>(), 64);
    assert_eq!(align_of::<SERVICE_NOTIFY_1>(), 8);
    assert_eq!(size_of::<SERVICE_NOTIFY_2A>(), 80);
    assert_eq!(align_of::<SERVICE_NOTIFY_2A>(), 8);
    assert_eq!(size_of::<SERVICE_NOTIFY_2W>(), 80);
    assert_eq!(align_of::<SERVICE_NOTIFY_2W>(), 8);
}
#[cfg(feature = "winusb")]
#[test]
fn um_winusb() {
    use winapi::um::winusb::*;
    assert_eq!(size_of::<WINUSB_SETUP_PACKET>(), 8);
    assert_eq!(align_of::<WINUSB_SETUP_PACKET>(), 1);
    assert_eq!(size_of::<USB_INTERFACE_DESCRIPTOR>(), 9);
    assert_eq!(align_of::<USB_INTERFACE_DESCRIPTOR>(), 1);
}
#[cfg(feature = "winuser")]
#[test]
fn um_winuser() {
    use winapi::um::winuser::*;
    assert_eq!(size_of::<CBT_CREATEWNDA>(), 16);
    assert_eq!(align_of::<CBT_CREATEWNDA>(), 8);
    assert_eq!(size_of::<CBT_CREATEWNDW>(), 16);
    assert_eq!(align_of::<CBT_CREATEWNDW>(), 8);
    assert_eq!(size_of::<CBTACTIVATESTRUCT>(), 16);
    assert_eq!(align_of::<CBTACTIVATESTRUCT>(), 8);
    assert_eq!(size_of::<WTSSESSION_NOTIFICATION>(), 8);
    assert_eq!(align_of::<WTSSESSION_NOTIFICATION>(), 4);
    assert_eq!(size_of::<SHELLHOOKINFO>(), 24);
    assert_eq!(align_of::<SHELLHOOKINFO>(), 8);
    assert_eq!(size_of::<EVENTMSG>(), 24);
    assert_eq!(align_of::<EVENTMSG>(), 8);
    assert_eq!(size_of::<CWPSTRUCT>(), 32);
    assert_eq!(align_of::<CWPSTRUCT>(), 8);
    assert_eq!(size_of::<CWPRETSTRUCT>(), 40);
    assert_eq!(align_of::<CWPRETSTRUCT>(), 8);
    assert_eq!(size_of::<KBDLLHOOKSTRUCT>(), 24);
    assert_eq!(align_of::<KBDLLHOOKSTRUCT>(), 8);
    assert_eq!(size_of::<MSLLHOOKSTRUCT>(), 32);
    assert_eq!(align_of::<MSLLHOOKSTRUCT>(), 8);
    assert_eq!(size_of::<DEBUGHOOKINFO>(), 32);
    assert_eq!(align_of::<DEBUGHOOKINFO>(), 8);
    assert_eq!(size_of::<MOUSEHOOKSTRUCT>(), 32);
    assert_eq!(align_of::<MOUSEHOOKSTRUCT>(), 8);
    assert_eq!(size_of::<MOUSEHOOKSTRUCTEX>(), 40);
    assert_eq!(align_of::<MOUSEHOOKSTRUCTEX>(), 8);
    assert_eq!(size_of::<HARDWAREHOOKSTRUCT>(), 32);
    assert_eq!(align_of::<HARDWAREHOOKSTRUCT>(), 8);
    assert_eq!(size_of::<MOUSEMOVEPOINT>(), 24);
    assert_eq!(align_of::<MOUSEMOVEPOINT>(), 8);
    assert_eq!(size_of::<USEROBJECTFLAGS>(), 12);
    assert_eq!(align_of::<USEROBJECTFLAGS>(), 4);
    assert_eq!(size_of::<WNDCLASSEXA>(), 80);
    assert_eq!(align_of::<WNDCLASSEXA>(), 8);
    assert_eq!(size_of::<WNDCLASSEXW>(), 80);
    assert_eq!(align_of::<WNDCLASSEXW>(), 8);
    assert_eq!(size_of::<WNDCLASSA>(), 72);
    assert_eq!(align_of::<WNDCLASSA>(), 8);
    assert_eq!(size_of::<WNDCLASSW>(), 72);
    assert_eq!(align_of::<WNDCLASSW>(), 8);
    assert_eq!(size_of::<MSG>(), 48);
    assert_eq!(align_of::<MSG>(), 8);
    assert_eq!(size_of::<MINMAXINFO>(), 40);
    assert_eq!(align_of::<MINMAXINFO>(), 4);
    assert_eq!(size_of::<COPYDATASTRUCT>(), 24);
    assert_eq!(align_of::<COPYDATASTRUCT>(), 8);
    assert_eq!(size_of::<MDINEXTMENU>(), 24);
    assert_eq!(align_of::<MDINEXTMENU>(), 8);
    assert_eq!(size_of::<POWERBROADCAST_SETTING>(), 24);
    assert_eq!(align_of::<POWERBROADCAST_SETTING>(), 4);
    assert_eq!(size_of::<WINDOWPOS>(), 40);
    assert_eq!(align_of::<WINDOWPOS>(), 8);
    assert_eq!(size_of::<NCCALCSIZE_PARAMS>(), 56);
    assert_eq!(align_of::<NCCALCSIZE_PARAMS>(), 8);
    assert_eq!(size_of::<TRACKMOUSEEVENT>(), 24);
    assert_eq!(align_of::<TRACKMOUSEEVENT>(), 8);
    assert_eq!(size_of::<ACCEL>(), 6);
    assert_eq!(align_of::<ACCEL>(), 2);
    assert_eq!(size_of::<PAINTSTRUCT>(), 72);
    assert_eq!(align_of::<PAINTSTRUCT>(), 8);
    assert_eq!(size_of::<CREATESTRUCTA>(), 80);
    assert_eq!(align_of::<CREATESTRUCTA>(), 8);
    assert_eq!(size_of::<CREATESTRUCTW>(), 80);
    assert_eq!(align_of::<CREATESTRUCTW>(), 8);
    assert_eq!(size_of::<WINDOWPLACEMENT>(), 44);
    assert_eq!(align_of::<WINDOWPLACEMENT>(), 4);
    assert_eq!(size_of::<NMHDR>(), 24);
    assert_eq!(align_of::<NMHDR>(), 8);
    assert_eq!(size_of::<STYLESTRUCT>(), 8);
    assert_eq!(align_of::<STYLESTRUCT>(), 4);
    assert_eq!(size_of::<MEASUREITEMSTRUCT>(), 32);
    assert_eq!(align_of::<MEASUREITEMSTRUCT>(), 8);
    assert_eq!(size_of::<DRAWITEMSTRUCT>(), 64);
    assert_eq!(align_of::<DRAWITEMSTRUCT>(), 8);
    assert_eq!(size_of::<DELETEITEMSTRUCT>(), 32);
    assert_eq!(align_of::<DELETEITEMSTRUCT>(), 8);
    assert_eq!(size_of::<COMPAREITEMSTRUCT>(), 56);
    assert_eq!(align_of::<COMPAREITEMSTRUCT>(), 8);
    assert_eq!(size_of::<BSMINFO>(), 32);
    assert_eq!(align_of::<BSMINFO>(), 8);
    assert_eq!(size_of::<UPDATELAYEREDWINDOWINFO>(), 80);
    assert_eq!(align_of::<UPDATELAYEREDWINDOWINFO>(), 8);
    assert_eq!(size_of::<FLASHWINFO>(), 32);
    assert_eq!(align_of::<FLASHWINFO>(), 8);
    // packed = 2
    // assert_eq!(size_of::<DLGTEMPLATE>(), 18);
    // assert_eq!(align_of::<DLGTEMPLATE>(), 2);
    // packed = 2
    // assert_eq!(size_of::<DLGITEMTEMPLATE>(), 18);
    // assert_eq!(align_of::<DLGITEMTEMPLATE>(), 2);
    assert_eq!(size_of::<MOUSEINPUT>(), 32);
    assert_eq!(align_of::<MOUSEINPUT>(), 8);
    assert_eq!(size_of::<KEYBDINPUT>(), 24);
    assert_eq!(align_of::<KEYBDINPUT>(), 8);
    assert_eq!(size_of::<HARDWAREINPUT>(), 8);
    assert_eq!(align_of::<HARDWAREINPUT>(), 4);
    assert_eq!(size_of::<INPUT_u>(), 32);
    assert_eq!(align_of::<INPUT_u>(), 8);
    assert_eq!(size_of::<INPUT>(), 40);
    assert_eq!(align_of::<INPUT>(), 8);
    assert_eq!(size_of::<TOUCHINPUT>(), 48);
    assert_eq!(align_of::<TOUCHINPUT>(), 8);
    assert_eq!(size_of::<POINTER_INFO>(), 96);
    assert_eq!(align_of::<POINTER_INFO>(), 8);
    assert_eq!(size_of::<POINTER_TOUCH_INFO>(), 144);
    assert_eq!(align_of::<POINTER_TOUCH_INFO>(), 8);
    assert_eq!(size_of::<POINTER_PEN_INFO>(), 120);
    assert_eq!(align_of::<POINTER_PEN_INFO>(), 8);
    assert_eq!(size_of::<USAGE_PROPERTIES>(), 32);
    assert_eq!(align_of::<USAGE_PROPERTIES>(), 4);
    assert_eq!(size_of::<POINTER_TYPE_INFO_u>(), 144);
    assert_eq!(align_of::<POINTER_TYPE_INFO_u>(), 8);
    assert_eq!(size_of::<POINTER_TYPE_INFO>(), 152);
    assert_eq!(align_of::<POINTER_TYPE_INFO>(), 8);
    assert_eq!(size_of::<INPUT_INJECTION_VALUE>(), 12);
    assert_eq!(align_of::<INPUT_INJECTION_VALUE>(), 4);
    assert_eq!(size_of::<TOUCH_HIT_TESTING_PROXIMITY_EVALUATION>(), 12);
    assert_eq!(align_of::<TOUCH_HIT_TESTING_PROXIMITY_EVALUATION>(), 4);
    assert_eq!(size_of::<TOUCH_HIT_TESTING_INPUT>(), 48);
    assert_eq!(align_of::<TOUCH_HIT_TESTING_INPUT>(), 4);
    assert_eq!(size_of::<INPUT_TRANSFORM>(), 64);
    assert_eq!(align_of::<INPUT_TRANSFORM>(), 4);
    assert_eq!(size_of::<LASTINPUTINFO>(), 8);
    assert_eq!(align_of::<LASTINPUTINFO>(), 4);
    assert_eq!(size_of::<TPMPARAMS>(), 20);
    assert_eq!(align_of::<TPMPARAMS>(), 4);
    assert_eq!(size_of::<MENUINFO>(), 40);
    assert_eq!(align_of::<MENUINFO>(), 8);
    assert_eq!(size_of::<MENUGETOBJECTINFO>(), 32);
    assert_eq!(align_of::<MENUGETOBJECTINFO>(), 8);
    assert_eq!(size_of::<MENUITEMINFOA>(), 80);
    assert_eq!(align_of::<MENUITEMINFOA>(), 8);
    assert_eq!(size_of::<MENUITEMINFOW>(), 80);
    assert_eq!(align_of::<MENUITEMINFOW>(), 8);
    assert_eq!(size_of::<DROPSTRUCT>(), 48);
    assert_eq!(align_of::<DROPSTRUCT>(), 8);
    assert_eq!(size_of::<DRAWTEXTPARAMS>(), 20);
    assert_eq!(align_of::<DRAWTEXTPARAMS>(), 4);
    assert_eq!(size_of::<HELPINFO>(), 40);
    assert_eq!(align_of::<HELPINFO>(), 8);
    assert_eq!(size_of::<MSGBOXPARAMSA>(), 80);
    assert_eq!(align_of::<MSGBOXPARAMSA>(), 8);
    assert_eq!(size_of::<MSGBOXPARAMSW>(), 80);
    assert_eq!(align_of::<MSGBOXPARAMSW>(), 8);
    assert_eq!(size_of::<SCROLLINFO>(), 28);
    assert_eq!(align_of::<SCROLLINFO>(), 4);
    assert_eq!(size_of::<ICONINFO>(), 32);
    assert_eq!(align_of::<ICONINFO>(), 8);
    assert_eq!(size_of::<NONCLIENTMETRICSA>(), 344);
    assert_eq!(align_of::<NONCLIENTMETRICSA>(), 4);
    assert_eq!(size_of::<NONCLIENTMETRICSW>(), 504);
    assert_eq!(align_of::<NONCLIENTMETRICSW>(), 4);
    assert_eq!(size_of::<MONITORINFO>(), 40);
    assert_eq!(align_of::<MONITORINFO>(), 4);
    assert_eq!(size_of::<MONITORINFOEXA>(), 72);
    assert_eq!(align_of::<MONITORINFOEXA>(), 4);
    assert_eq!(size_of::<MONITORINFOEXW>(), 104);
    assert_eq!(align_of::<MONITORINFOEXW>(), 4);
    assert_eq!(size_of::<GUITHREADINFO>(), 72);
    assert_eq!(align_of::<GUITHREADINFO>(), 8);
    assert_eq!(size_of::<CURSORINFO>(), 24);
    assert_eq!(align_of::<CURSORINFO>(), 8);
    assert_eq!(size_of::<WINDOWINFO>(), 60);
    assert_eq!(align_of::<WINDOWINFO>(), 4);
    assert_eq!(size_of::<TITLEBARINFO>(), 44);
    assert_eq!(align_of::<TITLEBARINFO>(), 4);
    assert_eq!(size_of::<TITLEBARINFOEX>(), 140);
    assert_eq!(align_of::<TITLEBARINFOEX>(), 4);
    assert_eq!(size_of::<MENUBARINFO>(), 48);
    assert_eq!(align_of::<MENUBARINFO>(), 8);
    assert_eq!(size_of::<SCROLLBARINFO>(), 60);
    assert_eq!(align_of::<SCROLLBARINFO>(), 4);
    assert_eq!(size_of::<COMBOBOXINFO>(), 64);
    assert_eq!(align_of::<COMBOBOXINFO>(), 8);
    assert_eq!(size_of::<ALTTABINFO>(), 40);
    assert_eq!(align_of::<ALTTABINFO>(), 4);
    assert_eq!(size_of::<RAWINPUTHEADER>(), 24);
    assert_eq!(align_of::<RAWINPUTHEADER>(), 8);
    assert_eq!(size_of::<RAWMOUSE>(), 24);
    assert_eq!(align_of::<RAWMOUSE>(), 4);
    assert_eq!(size_of::<RAWKEYBOARD>(), 16);
    assert_eq!(align_of::<RAWKEYBOARD>(), 4);
    assert_eq!(size_of::<RAWHID>(), 12);
    assert_eq!(align_of::<RAWHID>(), 4);
    assert_eq!(size_of::<RAWINPUT_data>(), 24);
    assert_eq!(align_of::<RAWINPUT_data>(), 4);
    assert_eq!(size_of::<RAWINPUT>(), 48);
    assert_eq!(align_of::<RAWINPUT>(), 8);
    assert_eq!(size_of::<RID_DEVICE_INFO_MOUSE>(), 16);
    assert_eq!(align_of::<RID_DEVICE_INFO_MOUSE>(), 4);
    assert_eq!(size_of::<RID_DEVICE_INFO_KEYBOARD>(), 24);
    assert_eq!(align_of::<RID_DEVICE_INFO_KEYBOARD>(), 4);
    assert_eq!(size_of::<RID_DEVICE_INFO_HID>(), 16);
    assert_eq!(align_of::<RID_DEVICE_INFO_HID>(), 4);
    assert_eq!(size_of::<RID_DEVICE_INFO_u>(), 24);
    assert_eq!(align_of::<RID_DEVICE_INFO_u>(), 4);
    assert_eq!(size_of::<RID_DEVICE_INFO>(), 32);
    assert_eq!(align_of::<RID_DEVICE_INFO>(), 4);
    assert_eq!(size_of::<RAWINPUTDEVICE>(), 16);
    assert_eq!(align_of::<RAWINPUTDEVICE>(), 8);
    assert_eq!(size_of::<RAWINPUTDEVICELIST>(), 16);
    assert_eq!(align_of::<RAWINPUTDEVICELIST>(), 8);
    assert_eq!(size_of::<CHANGEFILTERSTRUCT>(), 8);
    assert_eq!(align_of::<CHANGEFILTERSTRUCT>(), 4);
    assert_eq!(size_of::<ANIMATIONINFO>(), 8);
    assert_eq!(align_of::<ANIMATIONINFO>(), 4);
    assert_eq!(size_of::<WINDOWINFO>(), 60);
}
#[cfg(feature = "ws2bth")]
#[test]
fn um_ws2bth() {
    use winapi::um::ws2bth::*;
    assert_eq!(size_of::<SOCKADDR_BTH>(), 30);
    assert_eq!(align_of::<SOCKADDR_BTH>(), 1);
    assert_eq!(size_of::<BTH_SET_SERVICE>(), 45);
    assert_eq!(align_of::<BTH_SET_SERVICE>(), 1);
    assert_eq!(size_of::<BTH_QUERY_DEVICE>(), 5);
    assert_eq!(align_of::<BTH_QUERY_DEVICE>(), 1);
    assert_eq!(size_of::<BTH_QUERY_SERVICE>(), 256);
    assert_eq!(align_of::<BTH_QUERY_SERVICE>(), 1);
    assert_eq!(size_of::<RFCOMM_MSC_DATA>(), 2);
    assert_eq!(align_of::<RFCOMM_MSC_DATA>(), 1);
    assert_eq!(size_of::<RFCOMM_RLS_DATA>(), 1);
    assert_eq!(align_of::<RFCOMM_RLS_DATA>(), 1);
    assert_eq!(size_of::<RFCOMM_RPN_DATA>(), 7);
    assert_eq!(align_of::<RFCOMM_RPN_DATA>(), 1);
    assert_eq!(size_of::<RFCOMM_COMMAND>(), 11);
    assert_eq!(align_of::<RFCOMM_COMMAND>(), 1);
    assert_eq!(size_of::<BTH_PING_REQ>(), 53);
    assert_eq!(align_of::<BTH_PING_REQ>(), 1);
    assert_eq!(size_of::<BTH_PING_RSP>(), 45);
    assert_eq!(align_of::<BTH_PING_RSP>(), 1);
    assert_eq!(size_of::<BTH_INFO_REQ>(), 10);
    assert_eq!(align_of::<BTH_INFO_REQ>(), 1);
    assert_eq!(size_of::<BTH_INFO_RSP>(), 47);
    assert_eq!(align_of::<BTH_INFO_RSP>(), 1);
}
#[cfg(feature = "wlanapi")]
#[test]
fn um_wlanapi() {
    use winapi::um::wlanapi::*;
    assert_eq!(size_of::<WLAN_PROFILE_INFO>(), 516);
    assert_eq!(align_of::<WLAN_PROFILE_INFO>(), 4);
    assert_eq!(size_of::<DOT11_NETWORK>(), 40);
    assert_eq!(align_of::<DOT11_NETWORK>(), 4);
    assert_eq!(size_of::<WLAN_RAW_DATA>(), 8);
    assert_eq!(align_of::<WLAN_RAW_DATA>(), 4);
    assert_eq!(size_of::<WLAN_RAW_DATA_LIST_DataList>(), 8);
    assert_eq!(align_of::<WLAN_RAW_DATA_LIST_DataList>(), 4);
    assert_eq!(size_of::<WLAN_RAW_DATA_LIST>(), 16);
    assert_eq!(align_of::<WLAN_RAW_DATA_LIST>(), 4);
    assert_eq!(size_of::<WLAN_RATE_SET>(), 256);
    assert_eq!(align_of::<WLAN_RATE_SET>(), 4);
    assert_eq!(size_of::<WLAN_AVAILABLE_NETWORK>(), 628);
    assert_eq!(align_of::<WLAN_AVAILABLE_NETWORK>(), 4);
    assert_eq!(size_of::<WLAN_AVAILABLE_NETWORK_V2>(), 644);
    assert_eq!(align_of::<WLAN_AVAILABLE_NETWORK_V2>(), 4);
    assert_eq!(size_of::<WLAN_BSS_ENTRY>(), 360);
    assert_eq!(align_of::<WLAN_BSS_ENTRY>(), 8);
    assert_eq!(size_of::<WLAN_BSS_LIST>(), 368);
    assert_eq!(align_of::<WLAN_BSS_LIST>(), 8);
    assert_eq!(size_of::<WLAN_INTERFACE_INFO>(), 532);
    assert_eq!(align_of::<WLAN_INTERFACE_INFO>(), 4);
    assert_eq!(size_of::<WLAN_ASSOCIATION_ATTRIBUTES>(), 68);
    assert_eq!(align_of::<WLAN_ASSOCIATION_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<WLAN_SECURITY_ATTRIBUTES>(), 16);
    assert_eq!(align_of::<WLAN_SECURITY_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<WLAN_CONNECTION_ATTRIBUTES>(), 604);
    assert_eq!(align_of::<WLAN_CONNECTION_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<WLAN_PHY_RADIO_STATE>(), 12);
    assert_eq!(align_of::<WLAN_PHY_RADIO_STATE>(), 4);
    assert_eq!(size_of::<WLAN_RADIO_STATE>(), 772);
    assert_eq!(align_of::<WLAN_RADIO_STATE>(), 4);
    assert_eq!(size_of::<WLAN_INTERFACE_CAPABILITY>(), 276);
    assert_eq!(align_of::<WLAN_INTERFACE_CAPABILITY>(), 4);
    assert_eq!(size_of::<WLAN_AUTH_CIPHER_PAIR_LIST>(), 12);
    assert_eq!(align_of::<WLAN_AUTH_CIPHER_PAIR_LIST>(), 4);
    assert_eq!(size_of::<WLAN_COUNTRY_OR_REGION_STRING_LIST>(), 8);
    assert_eq!(align_of::<WLAN_COUNTRY_OR_REGION_STRING_LIST>(), 4);
    assert_eq!(size_of::<WLAN_PROFILE_INFO_LIST>(), 524);
    assert_eq!(align_of::<WLAN_PROFILE_INFO_LIST>(), 4);
    assert_eq!(size_of::<WLAN_AVAILABLE_NETWORK_LIST>(), 636);
    assert_eq!(align_of::<WLAN_AVAILABLE_NETWORK_LIST>(), 4);
    assert_eq!(size_of::<WLAN_AVAILABLE_NETWORK_LIST_V2>(), 652);
    assert_eq!(align_of::<WLAN_AVAILABLE_NETWORK_LIST_V2>(), 4);
    assert_eq!(size_of::<WLAN_INTERFACE_INFO_LIST>(), 540);
    assert_eq!(align_of::<WLAN_INTERFACE_INFO_LIST>(), 4);
    assert_eq!(size_of::<DOT11_NETWORK_LIST>(), 48);
    assert_eq!(align_of::<DOT11_NETWORK_LIST>(), 4);
    assert_eq!(size_of::<WLAN_CONNECTION_PARAMETERS>(), 40);
    assert_eq!(align_of::<WLAN_CONNECTION_PARAMETERS>(), 8);
    assert_eq!(size_of::<WLAN_CONNECTION_PARAMETERS_V2>(), 56);
    assert_eq!(align_of::<WLAN_CONNECTION_PARAMETERS_V2>(), 8);
    assert_eq!(size_of::<WLAN_MSM_NOTIFICATION_DATA>(), 580);
    assert_eq!(align_of::<WLAN_MSM_NOTIFICATION_DATA>(), 4);
    assert_eq!(size_of::<WLAN_CONNECTION_NOTIFICATION_DATA>(), 572);
    assert_eq!(align_of::<WLAN_CONNECTION_NOTIFICATION_DATA>(), 4);
    assert_eq!(size_of::<WLAN_PHY_FRAME_STATISTICS>(), 144);
    assert_eq!(align_of::<WLAN_PHY_FRAME_STATISTICS>(), 8);
    assert_eq!(size_of::<WLAN_MAC_FRAME_STATISTICS>(), 96);
    assert_eq!(align_of::<WLAN_MAC_FRAME_STATISTICS>(), 8);
    assert_eq!(size_of::<WLAN_STATISTICS>(), 368);
    assert_eq!(align_of::<WLAN_STATISTICS>(), 8);
    assert_eq!(size_of::<WLAN_DEVICE_SERVICE_GUID_LIST>(), 24);
    assert_eq!(align_of::<WLAN_DEVICE_SERVICE_GUID_LIST>(), 4);
    assert_eq!(size_of::<WFD_GROUP_ID>(), 44);
    assert_eq!(align_of::<WFD_GROUP_ID>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_PEER_STATE>(), 12);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_PEER_STATE>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_RADIO_STATE>(), 8);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_RADIO_STATE>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_STATE_CHANGE>(), 12);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_STATE_CHANGE>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE>(), 28);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS>(), 40);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_SECURITY_SETTINGS>(), 8);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_SECURITY_SETTINGS>(), 4);
    assert_eq!(size_of::<WLAN_HOSTED_NETWORK_STATUS>(), 52);
    assert_eq!(align_of::<WLAN_HOSTED_NETWORK_STATUS>(), 4);
}
#[cfg(feature = "wlanihv")]
#[test]
fn um_wlanihv() {
    use winapi::um::wlanihv::*;
    assert_eq!(size_of::<DOT11EXT_APIS>(), 176);
    assert_eq!(align_of::<DOT11EXT_APIS>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_HANDLERS>(), 152);
    assert_eq!(align_of::<DOT11EXT_IHV_HANDLERS>(), 8);
    assert_eq!(size_of::<DOT11EXT_VIRTUAL_STATION_APIS>(), 32);
    assert_eq!(align_of::<DOT11EXT_VIRTUAL_STATION_APIS>(), 8);
    assert_eq!(size_of::<DOT11_IHV_VERSION_INFO>(), 8);
    assert_eq!(align_of::<DOT11_IHV_VERSION_INFO>(), 4);
    assert_eq!(size_of::<DOT11EXT_IHV_UI_REQUEST>(), 48);
    assert_eq!(align_of::<DOT11EXT_IHV_UI_REQUEST>(), 8);
    assert_eq!(size_of::<DOT11_EAP_RESULT>(), 16);
    assert_eq!(align_of::<DOT11_EAP_RESULT>(), 8);
    assert_eq!(size_of::<DOT11_MSONEX_RESULT_PARAMS>(), 48);
    assert_eq!(align_of::<DOT11_MSONEX_RESULT_PARAMS>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_CONNECTIVITY_PROFILE>(), 8);
    assert_eq!(align_of::<DOT11EXT_IHV_CONNECTIVITY_PROFILE>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_SECURITY_PROFILE>(), 16);
    assert_eq!(align_of::<DOT11EXT_IHV_SECURITY_PROFILE>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_DISCOVERY_PROFILE>(), 24);
    assert_eq!(align_of::<DOT11EXT_IHV_DISCOVERY_PROFILE>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_DISCOVERY_PROFILE_LIST>(), 16);
    assert_eq!(align_of::<DOT11EXT_IHV_DISCOVERY_PROFILE_LIST>(), 8);
    assert_eq!(size_of::<DOT11EXT_VIRTUAL_STATION_AP_PROPERTY>(), 116);
    assert_eq!(align_of::<DOT11EXT_VIRTUAL_STATION_AP_PROPERTY>(), 4);
    assert_eq!(size_of::<WDIAG_IHV_WLAN_ID>(), 560);
    assert_eq!(align_of::<WDIAG_IHV_WLAN_ID>(), 4);
}
#[cfg(feature = "wlanihvtypes")]
#[test]
fn um_wlanihvtypes() {
    use winapi::um::wlanihvtypes::*;
    assert_eq!(size_of::<DOT11_MSSECURITY_SETTINGS>(), 40);
    assert_eq!(align_of::<DOT11_MSSECURITY_SETTINGS>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_SSID_LIST>(), 40);
    assert_eq!(align_of::<DOT11EXT_IHV_SSID_LIST>(), 4);
    assert_eq!(size_of::<DOT11EXT_IHV_PROFILE_PARAMS>(), 24);
    assert_eq!(align_of::<DOT11EXT_IHV_PROFILE_PARAMS>(), 8);
    assert_eq!(size_of::<DOT11EXT_IHV_PARAMS>(), 560);
    assert_eq!(align_of::<DOT11EXT_IHV_PARAMS>(), 8);
}
#[cfg(feature = "wlclient")]
#[test]
fn um_wlclient() {
    use winapi::um::wlclient::*;
    assert_eq!(size_of::<DOT11_ADAPTER>(), 32);
    assert_eq!(align_of::<DOT11_ADAPTER>(), 8);
    assert_eq!(size_of::<DOT11_BSS_LIST>(), 16);
    assert_eq!(align_of::<DOT11_BSS_LIST>(), 8);
    assert_eq!(size_of::<DOT11_PORT_STATE>(), 20);
    assert_eq!(align_of::<DOT11_PORT_STATE>(), 4);
    assert_eq!(size_of::<DOT11_SECURITY_PACKET_HEADER>(), 9);
    assert_eq!(align_of::<DOT11_SECURITY_PACKET_HEADER>(), 1);
}
#[cfg(feature = "ws2spi")]
#[test]
fn um_ws2spi() {
    use winapi::um::ws2spi::*;
    assert_eq!(size_of::<WSPDATA>(), 516);
    assert_eq!(align_of::<WSPDATA>(), 2);
    assert_eq!(size_of::<WSATHREADID>(), 16);
    assert_eq!(align_of::<WSATHREADID>(), 8);
    assert_eq!(size_of::<WSPPROC_TABLE>(), 240);
    assert_eq!(align_of::<WSPPROC_TABLE>(), 8);
    assert_eq!(size_of::<WSPUPCALLTABLE>(), 120);
    assert_eq!(align_of::<WSPUPCALLTABLE>(), 8);
    assert_eq!(size_of::<WSC_PROVIDER_AUDIT_INFO>(), 16);
    assert_eq!(align_of::<WSC_PROVIDER_AUDIT_INFO>(), 8);
    assert_eq!(size_of::<NSP_ROUTINE>(), 88);
    assert_eq!(align_of::<NSP_ROUTINE>(), 8);
    assert_eq!(size_of::<NSPV2_ROUTINE>(), 72);
    assert_eq!(align_of::<NSPV2_ROUTINE>(), 8);
}
#[cfg(feature = "xinput")]
#[test]
fn um_xinput() {
    use winapi::um::xinput::*;
    assert_eq!(size_of::<XINPUT_GAMEPAD>(), 12);
    assert_eq!(align_of::<XINPUT_GAMEPAD>(), 2);
    assert_eq!(size_of::<XINPUT_STATE>(), 16);
    assert_eq!(align_of::<XINPUT_STATE>(), 4);
    assert_eq!(size_of::<XINPUT_VIBRATION>(), 4);
    assert_eq!(align_of::<XINPUT_VIBRATION>(), 2);
    assert_eq!(size_of::<XINPUT_CAPABILITIES>(), 20);
    assert_eq!(align_of::<XINPUT_CAPABILITIES>(), 2);
    assert_eq!(size_of::<XINPUT_BATTERY_INFORMATION>(), 2);
    assert_eq!(align_of::<XINPUT_BATTERY_INFORMATION>(), 1);
    assert_eq!(size_of::<XINPUT_KEYSTROKE>(), 8);
    assert_eq!(align_of::<XINPUT_KEYSTROKE>(), 2);
}
#[cfg(feature = "hstring")]
#[test]
fn winrt_hstring() {
    use winapi::winrt::hstring::*;
    assert_eq!(size_of::<HSTRING_HEADER_Reserved>(), 24);
    assert_eq!(align_of::<HSTRING_HEADER_Reserved>(), 8);
    assert_eq!(size_of::<HSTRING_HEADER_Reserved>(), 24);
    assert_eq!(align_of::<HSTRING_HEADER_Reserved>(), 8);
    assert_eq!(size_of::<HSTRING_HEADER>(), 24);
    assert_eq!(align_of::<HSTRING_HEADER>(), 8);
}
