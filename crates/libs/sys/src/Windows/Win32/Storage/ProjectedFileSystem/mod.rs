windows_targets::link!("projectedfslib.dll" "system" fn PrjAllocateAlignedBuffer(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, size : usize) -> *mut core::ffi::c_void);
windows_targets::link!("projectedfslib.dll" "system" fn PrjClearNegativePathCache(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, totalentrynumber : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjCompleteCommand(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, commandid : i32, completionresult : windows_sys::core::HRESULT, extendedparameters : *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjDeleteFile(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_sys::core::PCWSTR, updateflags : PRJ_UPDATE_TYPES, failurereason : *mut PRJ_UPDATE_FAILURE_CAUSES) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjDoesNameContainWildCards(filename : windows_sys::core::PCWSTR) -> bool);
windows_targets::link!("projectedfslib.dll" "system" fn PrjFileNameCompare(filename1 : windows_sys::core::PCWSTR, filename2 : windows_sys::core::PCWSTR) -> i32);
windows_targets::link!("projectedfslib.dll" "system" fn PrjFileNameMatch(filenametocheck : windows_sys::core::PCWSTR, pattern : windows_sys::core::PCWSTR) -> bool);
windows_targets::link!("projectedfslib.dll" "system" fn PrjFillDirEntryBuffer(filename : windows_sys::core::PCWSTR, filebasicinfo : *const PRJ_FILE_BASIC_INFO, direntrybufferhandle : PRJ_DIR_ENTRY_BUFFER_HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjFillDirEntryBuffer2(direntrybufferhandle : PRJ_DIR_ENTRY_BUFFER_HANDLE, filename : windows_sys::core::PCWSTR, filebasicinfo : *const PRJ_FILE_BASIC_INFO, extendedinfo : *const PRJ_EXTENDED_INFO) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjFreeAlignedBuffer(buffer : *const core::ffi::c_void));
windows_targets::link!("projectedfslib.dll" "system" fn PrjGetOnDiskFileState(destinationfilename : windows_sys::core::PCWSTR, filestate : *mut PRJ_FILE_STATE) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjGetVirtualizationInstanceInfo(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, virtualizationinstanceinfo : *mut PRJ_VIRTUALIZATION_INSTANCE_INFO) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjMarkDirectoryAsPlaceholder(rootpathname : windows_sys::core::PCWSTR, targetpathname : windows_sys::core::PCWSTR, versioninfo : *const PRJ_PLACEHOLDER_VERSION_INFO, virtualizationinstanceid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjStartVirtualizing(virtualizationrootpath : windows_sys::core::PCWSTR, callbacks : *const PRJ_CALLBACKS, instancecontext : *const core::ffi::c_void, options : *const PRJ_STARTVIRTUALIZING_OPTIONS, namespacevirtualizationcontext : *mut PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjStopVirtualizing(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT));
windows_targets::link!("projectedfslib.dll" "system" fn PrjUpdateFileIfNeeded(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_sys::core::PCWSTR, placeholderinfo : *const PRJ_PLACEHOLDER_INFO, placeholderinfosize : u32, updateflags : PRJ_UPDATE_TYPES, failurereason : *mut PRJ_UPDATE_FAILURE_CAUSES) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjWriteFileData(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, datastreamid : *const windows_sys::core::GUID, buffer : *const core::ffi::c_void, byteoffset : u64, length : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjWritePlaceholderInfo(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_sys::core::PCWSTR, placeholderinfo : *const PRJ_PLACEHOLDER_INFO, placeholderinfosize : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("projectedfslib.dll" "system" fn PrjWritePlaceholderInfo2(namespacevirtualizationcontext : PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename : windows_sys::core::PCWSTR, placeholderinfo : *const PRJ_PLACEHOLDER_INFO, placeholderinfosize : u32, extendedinfo : *const PRJ_EXTENDED_INFO) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_CALLBACKS {
    pub StartDirectoryEnumerationCallback: PRJ_START_DIRECTORY_ENUMERATION_CB,
    pub EndDirectoryEnumerationCallback: PRJ_END_DIRECTORY_ENUMERATION_CB,
    pub GetDirectoryEnumerationCallback: PRJ_GET_DIRECTORY_ENUMERATION_CB,
    pub GetPlaceholderInfoCallback: PRJ_GET_PLACEHOLDER_INFO_CB,
    pub GetFileDataCallback: PRJ_GET_FILE_DATA_CB,
    pub QueryFileNameCallback: PRJ_QUERY_FILE_NAME_CB,
    pub NotificationCallback: PRJ_NOTIFICATION_CB,
    pub CancelCommandCallback: PRJ_CANCEL_COMMAND_CB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_CALLBACK_DATA {
    pub Size: u32,
    pub Flags: PRJ_CALLBACK_DATA_FLAGS,
    pub NamespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    pub CommandId: i32,
    pub FileId: windows_sys::core::GUID,
    pub DataStreamId: windows_sys::core::GUID,
    pub FilePathName: windows_sys::core::PCWSTR,
    pub VersionInfo: *mut PRJ_PLACEHOLDER_VERSION_INFO,
    pub TriggeringProcessId: u32,
    pub TriggeringProcessImageFileName: windows_sys::core::PCWSTR,
    pub InstanceContext: *mut core::ffi::c_void,
}
impl Default for PRJ_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRJ_CALLBACK_DATA_FLAGS = i32;
pub type PRJ_CANCEL_COMMAND_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA)>;
pub const PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN: PRJ_CALLBACK_DATA_FLAGS = 1i32;
pub const PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY: PRJ_CALLBACK_DATA_FLAGS = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    pub CommandType: PRJ_COMPLETE_COMMAND_TYPE,
    pub Anonymous: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0,
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    pub Notification: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0,
    pub Enumeration: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1,
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    pub DirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
}
impl Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
pub type PRJ_COMPLETE_COMMAND_TYPE = i32;
pub const PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION: PRJ_COMPLETE_COMMAND_TYPE = 2i32;
pub const PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION: PRJ_COMPLETE_COMMAND_TYPE = 1i32;
pub type PRJ_DIR_ENTRY_BUFFER_HANDLE = *mut core::ffi::c_void;
pub type PRJ_END_DIRECTORY_ENUMERATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const windows_sys::core::GUID) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_EXTENDED_INFO {
    pub InfoType: PRJ_EXT_INFO_TYPE,
    pub NextInfoOffset: u32,
    pub Anonymous: PRJ_EXTENDED_INFO_0,
}
impl Default for PRJ_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRJ_EXTENDED_INFO_0 {
    pub Symlink: PRJ_EXTENDED_INFO_0_0,
}
impl Default for PRJ_EXTENDED_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_EXTENDED_INFO_0_0 {
    pub TargetName: windows_sys::core::PCWSTR,
}
impl Default for PRJ_EXTENDED_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRJ_EXT_INFO_TYPE = i32;
pub const PRJ_EXT_INFO_TYPE_SYMLINK: PRJ_EXT_INFO_TYPE = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_FILE_BASIC_INFO {
    pub IsDirectory: bool,
    pub FileSize: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
pub type PRJ_FILE_STATE = i32;
pub const PRJ_FILE_STATE_DIRTY_PLACEHOLDER: PRJ_FILE_STATE = 4i32;
pub const PRJ_FILE_STATE_FULL: PRJ_FILE_STATE = 8i32;
pub const PRJ_FILE_STATE_HYDRATED_PLACEHOLDER: PRJ_FILE_STATE = 2i32;
pub const PRJ_FILE_STATE_PLACEHOLDER: PRJ_FILE_STATE = 1i32;
pub const PRJ_FILE_STATE_TOMBSTONE: PRJ_FILE_STATE = 16i32;
pub const PRJ_FLAG_NONE: PRJ_STARTVIRTUALIZING_FLAGS = 0i32;
pub const PRJ_FLAG_USE_NEGATIVE_PATH_CACHE: PRJ_STARTVIRTUALIZING_FLAGS = 1i32;
pub type PRJ_GET_DIRECTORY_ENUMERATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const windows_sys::core::GUID, searchexpression: windows_sys::core::PCWSTR, direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE) -> windows_sys::core::HRESULT>;
pub type PRJ_GET_FILE_DATA_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, byteoffset: u64, length: u32) -> windows_sys::core::HRESULT>;
pub type PRJ_GET_PLACEHOLDER_INFO_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> windows_sys::core::HRESULT>;
pub type PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT = *mut core::ffi::c_void;
pub type PRJ_NOTIFICATION = i32;
pub type PRJ_NOTIFICATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, isdirectory: bool, notification: PRJ_NOTIFICATION, destinationfilename: windows_sys::core::PCWSTR, operationparameters: *mut PRJ_NOTIFICATION_PARAMETERS) -> windows_sys::core::HRESULT>;
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFICATION = 2048i32;
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFICATION = 1024i32;
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFICATION = 512i32;
pub const PRJ_NOTIFICATION_FILE_OPENED: PRJ_NOTIFICATION = 2i32;
pub const PRJ_NOTIFICATION_FILE_OVERWRITTEN: PRJ_NOTIFICATION = 8i32;
pub const PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFICATION = 4096i32;
pub const PRJ_NOTIFICATION_FILE_RENAMED: PRJ_NOTIFICATION = 128i32;
pub const PRJ_NOTIFICATION_HARDLINK_CREATED: PRJ_NOTIFICATION = 256i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_NOTIFICATION_MAPPING {
    pub NotificationBitMask: PRJ_NOTIFY_TYPES,
    pub NotificationRoot: windows_sys::core::PCWSTR,
}
impl Default for PRJ_NOTIFICATION_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRJ_NOTIFICATION_NEW_FILE_CREATED: PRJ_NOTIFICATION = 4i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PRJ_NOTIFICATION_PARAMETERS {
    pub PostCreate: PRJ_NOTIFICATION_PARAMETERS_0,
    pub FileRenamed: PRJ_NOTIFICATION_PARAMETERS_1,
    pub FileDeletedOnHandleClose: PRJ_NOTIFICATION_PARAMETERS_2,
}
impl Default for PRJ_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_NOTIFICATION_PARAMETERS_2 {
    pub IsFileModified: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_NOTIFICATION_PARAMETERS_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_NOTIFICATION_PARAMETERS_0 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
pub const PRJ_NOTIFICATION_PRE_DELETE: PRJ_NOTIFICATION = 16i32;
pub const PRJ_NOTIFICATION_PRE_RENAME: PRJ_NOTIFICATION = 32i32;
pub const PRJ_NOTIFICATION_PRE_SET_HARDLINK: PRJ_NOTIFICATION = 64i32;
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFY_TYPES = 2048u32;
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFY_TYPES = 1024u32;
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFY_TYPES = 512u32;
pub const PRJ_NOTIFY_FILE_OPENED: PRJ_NOTIFY_TYPES = 2u32;
pub const PRJ_NOTIFY_FILE_OVERWRITTEN: PRJ_NOTIFY_TYPES = 8u32;
pub const PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFY_TYPES = 4096u32;
pub const PRJ_NOTIFY_FILE_RENAMED: PRJ_NOTIFY_TYPES = 128u32;
pub const PRJ_NOTIFY_HARDLINK_CREATED: PRJ_NOTIFY_TYPES = 256u32;
pub const PRJ_NOTIFY_NEW_FILE_CREATED: PRJ_NOTIFY_TYPES = 4u32;
pub const PRJ_NOTIFY_NONE: PRJ_NOTIFY_TYPES = 0u32;
pub const PRJ_NOTIFY_PRE_DELETE: PRJ_NOTIFY_TYPES = 16u32;
pub const PRJ_NOTIFY_PRE_RENAME: PRJ_NOTIFY_TYPES = 32u32;
pub const PRJ_NOTIFY_PRE_SET_HARDLINK: PRJ_NOTIFY_TYPES = 64u32;
pub const PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS: PRJ_NOTIFY_TYPES = 1u32;
pub type PRJ_NOTIFY_TYPES = u32;
pub const PRJ_NOTIFY_USE_EXISTING_MASK: PRJ_NOTIFY_TYPES = 4294967295u32;
pub type PRJ_PLACEHOLDER_ID = i32;
pub const PRJ_PLACEHOLDER_ID_LENGTH: PRJ_PLACEHOLDER_ID = 128i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_PLACEHOLDER_INFO {
    pub FileBasicInfo: PRJ_FILE_BASIC_INFO,
    pub EaInformation: PRJ_PLACEHOLDER_INFO_0,
    pub SecurityInformation: PRJ_PLACEHOLDER_INFO_1,
    pub StreamsInformation: PRJ_PLACEHOLDER_INFO_2,
    pub VersionInfo: PRJ_PLACEHOLDER_VERSION_INFO,
    pub VariableData: [u8; 1],
}
impl Default for PRJ_PLACEHOLDER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_PLACEHOLDER_INFO_0 {
    pub EaBufferSize: u32,
    pub OffsetToFirstEa: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_PLACEHOLDER_INFO_1 {
    pub SecurityBufferSize: u32,
    pub OffsetToSecurityDescriptor: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_PLACEHOLDER_INFO_2 {
    pub StreamsInfoBufferSize: u32,
    pub OffsetToFirstStreamInfo: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_PLACEHOLDER_VERSION_INFO {
    pub ProviderID: [u8; 128],
    pub ContentID: [u8; 128],
}
impl Default for PRJ_PLACEHOLDER_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRJ_QUERY_FILE_NAME_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> windows_sys::core::HRESULT>;
pub type PRJ_STARTVIRTUALIZING_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRJ_STARTVIRTUALIZING_OPTIONS {
    pub Flags: PRJ_STARTVIRTUALIZING_FLAGS,
    pub PoolThreadCount: u32,
    pub ConcurrentThreadCount: u32,
    pub NotificationMappings: *mut PRJ_NOTIFICATION_MAPPING,
    pub NotificationMappingsCount: u32,
}
impl Default for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRJ_START_DIRECTORY_ENUMERATION_CB = Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const windows_sys::core::GUID) -> windows_sys::core::HRESULT>;
pub const PRJ_UPDATE_ALLOW_DIRTY_DATA: PRJ_UPDATE_TYPES = 2i32;
pub const PRJ_UPDATE_ALLOW_DIRTY_METADATA: PRJ_UPDATE_TYPES = 1i32;
pub const PRJ_UPDATE_ALLOW_READ_ONLY: PRJ_UPDATE_TYPES = 32i32;
pub const PRJ_UPDATE_ALLOW_TOMBSTONE: PRJ_UPDATE_TYPES = 4i32;
pub type PRJ_UPDATE_FAILURE_CAUSES = i32;
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA: PRJ_UPDATE_FAILURE_CAUSES = 2i32;
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA: PRJ_UPDATE_FAILURE_CAUSES = 1i32;
pub const PRJ_UPDATE_FAILURE_CAUSE_NONE: PRJ_UPDATE_FAILURE_CAUSES = 0i32;
pub const PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY: PRJ_UPDATE_FAILURE_CAUSES = 8i32;
pub const PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE: PRJ_UPDATE_FAILURE_CAUSES = 4i32;
pub const PRJ_UPDATE_MAX_VAL: PRJ_UPDATE_TYPES = 64i32;
pub const PRJ_UPDATE_NONE: PRJ_UPDATE_TYPES = 0i32;
pub const PRJ_UPDATE_RESERVED1: PRJ_UPDATE_TYPES = 8i32;
pub const PRJ_UPDATE_RESERVED2: PRJ_UPDATE_TYPES = 16i32;
pub type PRJ_UPDATE_TYPES = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRJ_VIRTUALIZATION_INSTANCE_INFO {
    pub InstanceID: windows_sys::core::GUID,
    pub WriteAlignment: u32,
}
