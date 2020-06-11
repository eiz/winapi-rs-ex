// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! VSS Error header file
use um::winnt::HRESULT;
pub const VSS_E_BAD_STATE: HRESULT = 0x80042301;
pub const VSS_E_UNEXPECTED: HRESULT = 0x80042302;
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: HRESULT = 0x80042303;
pub const VSS_E_PROVIDER_NOT_REGISTERED: HRESULT = 0x80042304;
pub const VSS_E_PROVIDER_VETO: HRESULT = 0x80042306;
pub const VSS_E_PROVIDER_IN_USE: HRESULT = 0x80042307;
pub const VSS_E_OBJECT_NOT_FOUND: HRESULT = 0x80042308;
pub const VSS_S_ASYNC_PENDING: HRESULT = 0x00042309;
pub const VSS_S_ASYNC_FINISHED: HRESULT = 0x0004230A;
pub const VSS_S_ASYNC_CANCELLED: HRESULT = 0x0004230B;
pub const VSS_E_VOLUME_NOT_SUPPORTED: HRESULT = 0x8004230C;
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: HRESULT = 0x8004230E;
pub const VSS_E_OBJECT_ALREADY_EXISTS: HRESULT = 0x8004230D;
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: HRESULT = 0x8004230F;
pub const VSS_E_CORRUPT_XML_DOCUMENT: HRESULT = 0x80042310;
pub const VSS_E_INVALID_XML_DOCUMENT: HRESULT = 0x80042311;
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: HRESULT = 0x80042312;
pub const VSS_E_FLUSH_WRITES_TIMEOUT: HRESULT = 0x80042313;
pub const VSS_E_HOLD_WRITES_TIMEOUT: HRESULT = 0x80042314;
pub const VSS_E_UNEXPECTED_WRITER_ERROR: HRESULT = 0x80042315;
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: HRESULT = 0x80042316;
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: HRESULT = 0x80042317;
pub const VSS_E_WRITER_INFRASTRUCTURE: HRESULT = 0x80042318;
pub const VSS_E_WRITER_NOT_RESPONDING: HRESULT = 0x80042319;
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: HRESULT = 0x8004231A;
pub const VSS_E_UNSUPPORTED_CONTEXT: HRESULT = 0x8004231B;
pub const VSS_E_VOLUME_IN_USE: HRESULT = 0x8004231D;
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: HRESULT = 0x8004231E;
pub const VSS_E_INSUFFICIENT_STORAGE: HRESULT = 0x8004231F;
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: HRESULT = 0x80042320;
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: HRESULT = 0x00042321;
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: HRESULT = 0x80042321;
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: HRESULT = 0x80042322;
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: HRESULT = 0x80042323;
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: HRESULT = 0x80042324;
pub const VSS_E_REVERT_IN_PROGRESS: HRESULT = 0x80042325;
pub const VSS_E_REVERT_VOLUME_LOST: HRESULT = 0x80042326;
pub const VSS_E_REBOOT_REQUIRED: HRESULT = 0x80042327;
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: HRESULT = 0x80042328;
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: HRESULT = 0x80042329;
pub const VSS_E_VOLUME_NOT_LOCAL: HRESULT = 0x8004232D;
pub const VSS_E_CLUSTER_TIMEOUT: HRESULT = 0x8004232E;
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: HRESULT = 0x800423F0;
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: HRESULT = 0x800423F1;
pub const VSS_E_WRITERERROR_TIMEOUT: HRESULT = 0x800423F2;
pub const VSS_E_WRITERERROR_RETRYABLE: HRESULT = 0x800423F3;
pub const VSS_E_WRITERERROR_NONRETRYABLE: HRESULT = 0x800423F4;
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: HRESULT = 0x800423F5;
pub const VSS_E_BREAK_REVERT_ID_FAILED: HRESULT = 0x800423F6;
pub const VSS_E_LEGACY_PROVIDER: HRESULT = 0x800423F7;
pub const VSS_E_MISSING_DISK: HRESULT = 0x800423F8;
pub const VSS_E_MISSING_HIDDEN_VOLUME: HRESULT = 0x800423F9;
pub const VSS_E_MISSING_VOLUME: HRESULT = 0x800423FA;
pub const VSS_E_AUTORECOVERY_FAILED: HRESULT = 0x800423FB;
pub const VSS_E_DYNAMIC_DISK_ERROR: HRESULT = 0x800423FC;
pub const VSS_E_NONTRANSPORTABLE_BCD: HRESULT = 0x800423FD;
pub const VSS_E_CANNOT_REVERT_DISKID: HRESULT = 0x800423FE;
pub const VSS_E_RESYNC_IN_PROGRESS: HRESULT = 0x800423FF;
pub const VSS_E_CLUSTER_ERROR: HRESULT = 0x80042400;
pub const VSS_E_UNSELECTED_VOLUME: HRESULT = 0x8004232A;
pub const VSS_E_SNAPSHOT_NOT_IN_SET: HRESULT = 0x8004232B;
pub const VSS_E_NESTED_VOLUME_LIMIT: HRESULT = 0x8004232C;
pub const VSS_E_NOT_SUPPORTED: HRESULT = 0x8004232F;
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: HRESULT = 0x80042336;
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: HRESULT = 0x80042401;
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: HRESULT = 0x80042402;
pub const VSS_E_ASRERROR_NO_ARCPATH: HRESULT = 0x80042403;
pub const VSS_E_ASRERROR_MISSING_DYNDISK: HRESULT = 0x80042404;
pub const VSS_E_ASRERROR_SHARED_CRIDISK: HRESULT = 0x80042405;
pub const VSS_E_ASRERROR_DATADISK_RDISK0: HRESULT = 0x80042406;
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: HRESULT = 0x80042407;
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: HRESULT = 0x80042408;
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: HRESULT = 0x80042409;
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: HRESULT = 0x8004240A;
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: HRESULT = 0x80042411;
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: HRESULT = 0x80042412;
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: HRESULT = 0x80042413;
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: HRESULT = 0x80042414;
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: HRESULT = 0x80042415;
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: HRESULT = 0x80042416;
pub const VSS_E_FSS_TIMEOUT: HRESULT = 0x80042417;
