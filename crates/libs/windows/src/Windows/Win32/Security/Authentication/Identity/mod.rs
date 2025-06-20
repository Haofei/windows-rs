#[cfg(feature = "Win32_Security_Authentication_Identity_Provider")]
pub mod Provider;
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AcceptSecurityContext(phcredential: Option<*const super::super::Credentials::SecHandle>, phcontext: Option<*const super::super::Credentials::SecHandle>, pinput: Option<*const SecBufferDesc>, fcontextreq: ASC_REQ_FLAGS, targetdatarep: u32, phnewcontext: Option<*mut super::super::Credentials::SecHandle>, poutput: Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: Option<*mut i64>) -> windows_core::HRESULT {
    windows_link::link!("secur32.dll" "system" fn AcceptSecurityContext(phcredential : *const super::super::Credentials:: SecHandle, phcontext : *const super::super::Credentials:: SecHandle, pinput : *const SecBufferDesc, fcontextreq : ASC_REQ_FLAGS, targetdatarep : u32, phnewcontext : *mut super::super::Credentials:: SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { AcceptSecurityContext(phcredential.unwrap_or(core::mem::zeroed()) as _, phcontext.unwrap_or(core::mem::zeroed()) as _, pinput.unwrap_or(core::mem::zeroed()) as _, fcontextreq, targetdatarep, phnewcontext.unwrap_or(core::mem::zeroed()) as _, poutput.unwrap_or(core::mem::zeroed()) as _, pfcontextattr as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AcquireCredentialsHandleA<P0, P1>(pszprincipal: P0, pszpackage: P1, fcredentialuse: SECPKG_CRED, pvlogonid: Option<*const core::ffi::c_void>, pauthdata: Option<*const core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: Option<*const core::ffi::c_void>, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: Option<*mut i64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn AcquireCredentialsHandleA(pszprincipal : windows_core::PCSTR, pszpackage : windows_core::PCSTR, fcredentialuse : SECPKG_CRED, pvlogonid : *const core::ffi::c_void, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, phcredential : *mut super::super::Credentials:: SecHandle, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { AcquireCredentialsHandleA(pszprincipal.param().abi(), pszpackage.param().abi(), fcredentialuse, pvlogonid.unwrap_or(core::mem::zeroed()) as _, pauthdata.unwrap_or(core::mem::zeroed()) as _, pgetkeyfn, pvgetkeyargument.unwrap_or(core::mem::zeroed()) as _, phcredential as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AcquireCredentialsHandleW<P0, P1>(pszprincipal: P0, pszpackage: P1, fcredentialuse: SECPKG_CRED, pvlogonid: Option<*const core::ffi::c_void>, pauthdata: Option<*const core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: Option<*const core::ffi::c_void>, phcredential: *mut super::super::Credentials::SecHandle, ptsexpiry: Option<*mut i64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn AcquireCredentialsHandleW(pszprincipal : windows_core::PCWSTR, pszpackage : windows_core::PCWSTR, fcredentialuse : SECPKG_CRED, pvlogonid : *const core::ffi::c_void, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, phcredential : *mut super::super::Credentials:: SecHandle, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { AcquireCredentialsHandleW(pszprincipal.param().abi(), pszpackage.param().abi(), fcredentialuse, pvlogonid.unwrap_or(core::mem::zeroed()) as _, pauthdata.unwrap_or(core::mem::zeroed()) as _, pgetkeyfn, pvgetkeyargument.unwrap_or(core::mem::zeroed()) as _, phcredential as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AddCredentialsA<P1, P2>(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: P1, pszpackage: P2, fcredentialuse: u32, pauthdata: Option<*const core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: Option<*const core::ffi::c_void>, ptsexpiry: Option<*mut i64>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn AddCredentialsA(hcredentials : *const super::super::Credentials:: SecHandle, pszprincipal : windows_core::PCSTR, pszpackage : windows_core::PCSTR, fcredentialuse : u32, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { AddCredentialsA(hcredentials, pszprincipal.param().abi(), pszpackage.param().abi(), fcredentialuse, pauthdata.unwrap_or(core::mem::zeroed()) as _, pgetkeyfn, pvgetkeyargument.unwrap_or(core::mem::zeroed()) as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn AddCredentialsW<P1, P2>(hcredentials: *const super::super::Credentials::SecHandle, pszprincipal: P1, pszpackage: P2, fcredentialuse: u32, pauthdata: Option<*const core::ffi::c_void>, pgetkeyfn: SEC_GET_KEY_FN, pvgetkeyargument: Option<*const core::ffi::c_void>, ptsexpiry: Option<*mut i64>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn AddCredentialsW(hcredentials : *const super::super::Credentials:: SecHandle, pszprincipal : windows_core::PCWSTR, pszpackage : windows_core::PCWSTR, fcredentialuse : u32, pauthdata : *const core::ffi::c_void, pgetkeyfn : SEC_GET_KEY_FN, pvgetkeyargument : *const core::ffi::c_void, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { AddCredentialsW(hcredentials, pszprincipal.param().abi(), pszpackage.param().abi(), fcredentialuse, pauthdata.unwrap_or(core::mem::zeroed()) as _, pgetkeyfn, pvgetkeyargument.unwrap_or(core::mem::zeroed()) as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AddSecurityPackageA<P0>(pszpackagename: P0, poptions: Option<*const SECURITY_PACKAGE_OPTIONS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn AddSecurityPackageA(pszpackagename : windows_core::PCSTR, poptions : *const SECURITY_PACKAGE_OPTIONS) -> windows_core::HRESULT);
    unsafe { AddSecurityPackageA(pszpackagename.param().abi(), poptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AddSecurityPackageW<P0>(pszpackagename: P0, poptions: Option<*const SECURITY_PACKAGE_OPTIONS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn AddSecurityPackageW(pszpackagename : windows_core::PCWSTR, poptions : *const SECURITY_PACKAGE_OPTIONS) -> windows_core::HRESULT);
    unsafe { AddSecurityPackageW(pszpackagename.param().abi(), poptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ApplyControlToken(phcontext: *const super::super::Credentials::SecHandle, pinput: *const SecBufferDesc) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn ApplyControlToken(phcontext : *const super::super::Credentials:: SecHandle, pinput : *const SecBufferDesc) -> windows_core::HRESULT);
    unsafe { ApplyControlToken(phcontext, pinput).ok() }
}
#[inline]
pub unsafe fn AuditComputeEffectivePolicyBySid(psid: super::super::PSID, psubcategoryguids: &[windows_core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditComputeEffectivePolicyBySid(psid : super::super:: PSID, psubcategoryguids : *const windows_core::GUID, dwpolicycount : u32, ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION) -> bool);
    unsafe { AuditComputeEffectivePolicyBySid(psid, core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len().try_into().unwrap(), ppauditpolicy as _) }
}
#[inline]
pub unsafe fn AuditComputeEffectivePolicyByToken(htokenhandle: super::super::super::Foundation::HANDLE, psubcategoryguids: &[windows_core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditComputeEffectivePolicyByToken(htokenhandle : super::super::super::Foundation:: HANDLE, psubcategoryguids : *const windows_core::GUID, dwpolicycount : u32, ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION) -> bool);
    unsafe { AuditComputeEffectivePolicyByToken(htokenhandle, core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len().try_into().unwrap(), ppauditpolicy as _) }
}
#[inline]
pub unsafe fn AuditEnumerateCategories(ppauditcategoriesarray: *mut *mut windows_core::GUID, pdwcountreturned: *mut u32) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditEnumerateCategories(ppauditcategoriesarray : *mut *mut windows_core::GUID, pdwcountreturned : *mut u32) -> bool);
    unsafe { AuditEnumerateCategories(ppauditcategoriesarray as _, pdwcountreturned as _) }
}
#[inline]
pub unsafe fn AuditEnumeratePerUserPolicy(ppauditsidarray: *mut *mut POLICY_AUDIT_SID_ARRAY) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditEnumeratePerUserPolicy(ppauditsidarray : *mut *mut POLICY_AUDIT_SID_ARRAY) -> bool);
    unsafe { AuditEnumeratePerUserPolicy(ppauditsidarray as _) }
}
#[inline]
pub unsafe fn AuditEnumerateSubCategories(pauditcategoryguid: Option<*const windows_core::GUID>, bretrieveallsubcategories: bool, ppauditsubcategoriesarray: *mut *mut windows_core::GUID, pdwcountreturned: *mut u32) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditEnumerateSubCategories(pauditcategoryguid : *const windows_core::GUID, bretrieveallsubcategories : bool, ppauditsubcategoriesarray : *mut *mut windows_core::GUID, pdwcountreturned : *mut u32) -> bool);
    unsafe { AuditEnumerateSubCategories(pauditcategoryguid.unwrap_or(core::mem::zeroed()) as _, bretrieveallsubcategories, ppauditsubcategoriesarray as _, pdwcountreturned as _) }
}
#[inline]
pub unsafe fn AuditFree(buffer: *const core::ffi::c_void) {
    windows_link::link!("advapi32.dll" "system" fn AuditFree(buffer : *const core::ffi::c_void));
    unsafe { AuditFree(buffer) }
}
#[inline]
pub unsafe fn AuditLookupCategoryGuidFromCategoryId(auditcategoryid: POLICY_AUDIT_EVENT_TYPE, pauditcategoryguid: *mut windows_core::GUID) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryGuidFromCategoryId(auditcategoryid : POLICY_AUDIT_EVENT_TYPE, pauditcategoryguid : *mut windows_core::GUID) -> bool);
    unsafe { AuditLookupCategoryGuidFromCategoryId(auditcategoryid, pauditcategoryguid as _) }
}
#[inline]
pub unsafe fn AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid: *const windows_core::GUID, pauditcategoryid: *mut POLICY_AUDIT_EVENT_TYPE) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid : *const windows_core::GUID, pauditcategoryid : *mut POLICY_AUDIT_EVENT_TYPE) -> bool);
    unsafe { AuditLookupCategoryIdFromCategoryGuid(pauditcategoryguid, pauditcategoryid as _) }
}
#[inline]
pub unsafe fn AuditLookupCategoryNameA(pauditcategoryguid: *const windows_core::GUID, ppszcategoryname: *mut windows_core::PSTR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryNameA(pauditcategoryguid : *const windows_core::GUID, ppszcategoryname : *mut windows_core::PSTR) -> bool);
    unsafe { AuditLookupCategoryNameA(pauditcategoryguid, ppszcategoryname as _) }
}
#[inline]
pub unsafe fn AuditLookupCategoryNameW(pauditcategoryguid: *const windows_core::GUID, ppszcategoryname: *mut windows_core::PWSTR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditLookupCategoryNameW(pauditcategoryguid : *const windows_core::GUID, ppszcategoryname : *mut windows_core::PWSTR) -> bool);
    unsafe { AuditLookupCategoryNameW(pauditcategoryguid, ppszcategoryname as _) }
}
#[inline]
pub unsafe fn AuditLookupSubCategoryNameA(pauditsubcategoryguid: *const windows_core::GUID, ppszsubcategoryname: *mut windows_core::PSTR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditLookupSubCategoryNameA(pauditsubcategoryguid : *const windows_core::GUID, ppszsubcategoryname : *mut windows_core::PSTR) -> bool);
    unsafe { AuditLookupSubCategoryNameA(pauditsubcategoryguid, ppszsubcategoryname as _) }
}
#[inline]
pub unsafe fn AuditLookupSubCategoryNameW(pauditsubcategoryguid: *const windows_core::GUID, ppszsubcategoryname: *mut windows_core::PWSTR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditLookupSubCategoryNameW(pauditsubcategoryguid : *const windows_core::GUID, ppszsubcategoryname : *mut windows_core::PWSTR) -> bool);
    unsafe { AuditLookupSubCategoryNameW(pauditsubcategoryguid, ppszsubcategoryname as _) }
}
#[inline]
pub unsafe fn AuditQueryGlobalSaclA<P0>(objecttypename: P0, acl: *mut *mut super::super::ACL) -> bool
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn AuditQueryGlobalSaclA(objecttypename : windows_core::PCSTR, acl : *mut *mut super::super:: ACL) -> bool);
    unsafe { AuditQueryGlobalSaclA(objecttypename.param().abi(), acl as _) }
}
#[inline]
pub unsafe fn AuditQueryGlobalSaclW<P0>(objecttypename: P0, acl: *mut *mut super::super::ACL) -> bool
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn AuditQueryGlobalSaclW(objecttypename : windows_core::PCWSTR, acl : *mut *mut super::super:: ACL) -> bool);
    unsafe { AuditQueryGlobalSaclW(objecttypename.param().abi(), acl as _) }
}
#[inline]
pub unsafe fn AuditQueryPerUserPolicy(psid: super::super::PSID, psubcategoryguids: &[windows_core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditQueryPerUserPolicy(psid : super::super:: PSID, psubcategoryguids : *const windows_core::GUID, dwpolicycount : u32, ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION) -> bool);
    unsafe { AuditQueryPerUserPolicy(psid, core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len().try_into().unwrap(), ppauditpolicy as _) }
}
#[inline]
pub unsafe fn AuditQuerySecurity(securityinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditQuerySecurity(securityinformation : super::super:: OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor : *mut super::super:: PSECURITY_DESCRIPTOR) -> bool);
    unsafe { AuditQuerySecurity(securityinformation, ppsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn AuditQuerySystemPolicy(psubcategoryguids: &[windows_core::GUID], ppauditpolicy: *mut *mut AUDIT_POLICY_INFORMATION) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditQuerySystemPolicy(psubcategoryguids : *const windows_core::GUID, dwpolicycount : u32, ppauditpolicy : *mut *mut AUDIT_POLICY_INFORMATION) -> bool);
    unsafe { AuditQuerySystemPolicy(core::mem::transmute(psubcategoryguids.as_ptr()), psubcategoryguids.len().try_into().unwrap(), ppauditpolicy as _) }
}
#[inline]
pub unsafe fn AuditSetGlobalSaclA<P0>(objecttypename: P0, acl: Option<*const super::super::ACL>) -> bool
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn AuditSetGlobalSaclA(objecttypename : windows_core::PCSTR, acl : *const super::super:: ACL) -> bool);
    unsafe { AuditSetGlobalSaclA(objecttypename.param().abi(), acl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AuditSetGlobalSaclW<P0>(objecttypename: P0, acl: Option<*const super::super::ACL>) -> bool
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("advapi32.dll" "system" fn AuditSetGlobalSaclW(objecttypename : windows_core::PCWSTR, acl : *const super::super:: ACL) -> bool);
    unsafe { AuditSetGlobalSaclW(objecttypename.param().abi(), acl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AuditSetPerUserPolicy(psid: super::super::PSID, pauditpolicy: &[AUDIT_POLICY_INFORMATION]) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditSetPerUserPolicy(psid : super::super:: PSID, pauditpolicy : *const AUDIT_POLICY_INFORMATION, dwpolicycount : u32) -> bool);
    unsafe { AuditSetPerUserPolicy(psid, core::mem::transmute(pauditpolicy.as_ptr()), pauditpolicy.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn AuditSetSecurity(securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::PSECURITY_DESCRIPTOR) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditSetSecurity(securityinformation : super::super:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super:: PSECURITY_DESCRIPTOR) -> bool);
    unsafe { AuditSetSecurity(securityinformation, psecuritydescriptor) }
}
#[inline]
pub unsafe fn AuditSetSystemPolicy(pauditpolicy: &[AUDIT_POLICY_INFORMATION]) -> bool {
    windows_link::link!("advapi32.dll" "system" fn AuditSetSystemPolicy(pauditpolicy : *const AUDIT_POLICY_INFORMATION, dwpolicycount : u32) -> bool);
    unsafe { AuditSetSystemPolicy(core::mem::transmute(pauditpolicy.as_ptr()), pauditpolicy.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn ChangeAccountPasswordA(pszpackagename: *const i8, pszdomainname: *const i8, pszaccountname: *const i8, pszoldpassword: *const i8, psznewpassword: *const i8, bimpersonating: bool, dwreserved: u32, poutput: *mut SecBufferDesc) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn ChangeAccountPasswordA(pszpackagename : *const i8, pszdomainname : *const i8, pszaccountname : *const i8, pszoldpassword : *const i8, psznewpassword : *const i8, bimpersonating : bool, dwreserved : u32, poutput : *mut SecBufferDesc) -> windows_core::HRESULT);
    unsafe { ChangeAccountPasswordA(pszpackagename, pszdomainname, pszaccountname, pszoldpassword, psznewpassword, bimpersonating, dwreserved, poutput as _).ok() }
}
#[inline]
pub unsafe fn ChangeAccountPasswordW(pszpackagename: *const u16, pszdomainname: *const u16, pszaccountname: *const u16, pszoldpassword: *const u16, psznewpassword: *const u16, bimpersonating: bool, dwreserved: u32, poutput: *mut SecBufferDesc) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn ChangeAccountPasswordW(pszpackagename : *const u16, pszdomainname : *const u16, pszaccountname : *const u16, pszoldpassword : *const u16, psznewpassword : *const u16, bimpersonating : bool, dwreserved : u32, poutput : *mut SecBufferDesc) -> windows_core::HRESULT);
    unsafe { ChangeAccountPasswordW(pszpackagename, pszdomainname, pszaccountname, pszoldpassword, psznewpassword, bimpersonating, dwreserved, poutput as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn CompleteAuthToken(phcontext: *const super::super::Credentials::SecHandle, ptoken: *const SecBufferDesc) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn CompleteAuthToken(phcontext : *const super::super::Credentials:: SecHandle, ptoken : *const SecBufferDesc) -> windows_core::HRESULT);
    unsafe { CompleteAuthToken(phcontext, ptoken).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn CredMarshalTargetInfo(intargetinfo: *const super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW, buffer: *mut *mut u16, buffersize: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn CredMarshalTargetInfo(intargetinfo : *const super::super::Credentials:: CREDENTIAL_TARGET_INFORMATIONW, buffer : *mut *mut u16, buffersize : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { CredMarshalTargetInfo(intargetinfo, buffer as _, buffersize as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn CredUnmarshalTargetInfo(buffer: *const u16, buffersize: u32, rettargetinfo: Option<*mut *mut super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW>, retactualsize: Option<*mut u32>) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn CredUnmarshalTargetInfo(buffer : *const u16, buffersize : u32, rettargetinfo : *mut *mut super::super::Credentials:: CREDENTIAL_TARGET_INFORMATIONW, retactualsize : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { CredUnmarshalTargetInfo(buffer, buffersize, rettargetinfo.unwrap_or(core::mem::zeroed()) as _, retactualsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn DecryptMessage(phcontext: *const super::super::Credentials::SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32, pfqop: Option<*mut u32>) -> windows_core::HRESULT {
    windows_link::link!("secur32.dll" "system" fn DecryptMessage(phcontext : *const super::super::Credentials:: SecHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> windows_core::HRESULT);
    unsafe { DecryptMessage(phcontext, pmessage, messageseqno, pfqop.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn DeleteSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn DeleteSecurityContext(phcontext : *const super::super::Credentials:: SecHandle) -> windows_core::HRESULT);
    unsafe { DeleteSecurityContext(phcontext).ok() }
}
#[inline]
pub unsafe fn DeleteSecurityPackageA<P0>(pszpackagename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn DeleteSecurityPackageA(pszpackagename : windows_core::PCSTR) -> windows_core::HRESULT);
    unsafe { DeleteSecurityPackageA(pszpackagename.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeleteSecurityPackageW<P0>(pszpackagename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn DeleteSecurityPackageW(pszpackagename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteSecurityPackageW(pszpackagename.param().abi()).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn EncryptMessage(phcontext: *const super::super::Credentials::SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> windows_core::HRESULT {
    windows_link::link!("secur32.dll" "system" fn EncryptMessage(phcontext : *const super::super::Credentials:: SecHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> windows_core::HRESULT);
    unsafe { EncryptMessage(phcontext, fqop, pmessage, messageseqno) }
}
#[inline]
pub unsafe fn EnumerateSecurityPackagesA(pcpackages: *mut u32, pppackageinfo: *mut *mut SecPkgInfoA) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn EnumerateSecurityPackagesA(pcpackages : *mut u32, pppackageinfo : *mut *mut SecPkgInfoA) -> windows_core::HRESULT);
    unsafe { EnumerateSecurityPackagesA(pcpackages as _, pppackageinfo as _).ok() }
}
#[inline]
pub unsafe fn EnumerateSecurityPackagesW(pcpackages: *mut u32, pppackageinfo: *mut *mut SecPkgInfoW) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn EnumerateSecurityPackagesW(pcpackages : *mut u32, pppackageinfo : *mut *mut SecPkgInfoW) -> windows_core::HRESULT);
    unsafe { EnumerateSecurityPackagesW(pcpackages as _, pppackageinfo as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ExportSecurityContext(phcontext: *const super::super::Credentials::SecHandle, fflags: EXPORT_SECURITY_CONTEXT_FLAGS, ppackedcontext: *mut SecBuffer, ptoken: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn ExportSecurityContext(phcontext : *const super::super::Credentials:: SecHandle, fflags : EXPORT_SECURITY_CONTEXT_FLAGS, ppackedcontext : *mut SecBuffer, ptoken : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ExportSecurityContext(phcontext, fflags, ppackedcontext as _, ptoken as _).ok() }
}
#[inline]
pub unsafe fn FreeContextBuffer(pvcontextbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn FreeContextBuffer(pvcontextbuffer : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { FreeContextBuffer(pvcontextbuffer as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn FreeCredentialsHandle(phcredential: *const super::super::Credentials::SecHandle) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn FreeCredentialsHandle(phcredential : *const super::super::Credentials:: SecHandle) -> windows_core::HRESULT);
    unsafe { FreeCredentialsHandle(phcredential).ok() }
}
#[inline]
pub unsafe fn GetComputerObjectNameA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> bool {
    windows_link::link!("secur32.dll" "system" fn GetComputerObjectNameA(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PSTR, nsize : *mut u32) -> bool);
    unsafe { GetComputerObjectNameA(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetComputerObjectNameW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> bool {
    windows_link::link!("secur32.dll" "system" fn GetComputerObjectNameW(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PWSTR, nsize : *mut u32) -> bool);
    unsafe { GetComputerObjectNameW(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetUserNameExA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> bool {
    windows_link::link!("secur32.dll" "system" fn GetUserNameExA(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PSTR, nsize : *mut u32) -> bool);
    unsafe { GetUserNameExA(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetUserNameExW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> bool {
    windows_link::link!("secur32.dll" "system" fn GetUserNameExW(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PWSTR, nsize : *mut u32) -> bool);
    unsafe { GetUserNameExW(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ImpersonateSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn ImpersonateSecurityContext(phcontext : *const super::super::Credentials:: SecHandle) -> windows_core::HRESULT);
    unsafe { ImpersonateSecurityContext(phcontext).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ImportSecurityContextA<P0>(pszpackage: P0, ppackedcontext: *const SecBuffer, token: *const core::ffi::c_void) -> windows_core::Result<super::super::Credentials::SecHandle>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn ImportSecurityContextA(pszpackage : windows_core::PCSTR, ppackedcontext : *const SecBuffer, token : *const core::ffi::c_void, phcontext : *mut super::super::Credentials:: SecHandle) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ImportSecurityContextA(pszpackage.param().abi(), ppackedcontext, token, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn ImportSecurityContextW<P0>(pszpackage: P0, ppackedcontext: *const SecBuffer, token: *const core::ffi::c_void) -> windows_core::Result<super::super::Credentials::SecHandle>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn ImportSecurityContextW(pszpackage : windows_core::PCWSTR, ppackedcontext : *const SecBuffer, token : *const core::ffi::c_void, phcontext : *mut super::super::Credentials:: SecHandle) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ImportSecurityContextW(pszpackage.param().abi(), ppackedcontext, token, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn InitSecurityInterfaceA() -> *mut SecurityFunctionTableA {
    windows_link::link!("secur32.dll" "system" fn InitSecurityInterfaceA() -> *mut SecurityFunctionTableA);
    unsafe { InitSecurityInterfaceA() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn InitSecurityInterfaceW() -> *mut SecurityFunctionTableW {
    windows_link::link!("secur32.dll" "system" fn InitSecurityInterfaceW() -> *mut SecurityFunctionTableW);
    unsafe { InitSecurityInterfaceW() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn InitializeSecurityContextA(phcredential: Option<*const super::super::Credentials::SecHandle>, phcontext: Option<*const super::super::Credentials::SecHandle>, psztargetname: Option<*const i8>, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: Option<*mut super::super::Credentials::SecHandle>, poutput: Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: Option<*mut i64>) -> windows_core::HRESULT {
    windows_link::link!("secur32.dll" "system" fn InitializeSecurityContextA(phcredential : *const super::super::Credentials:: SecHandle, phcontext : *const super::super::Credentials:: SecHandle, psztargetname : *const i8, fcontextreq : ISC_REQ_FLAGS, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *mut super::super::Credentials:: SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { InitializeSecurityContextA(phcredential.unwrap_or(core::mem::zeroed()) as _, phcontext.unwrap_or(core::mem::zeroed()) as _, psztargetname.unwrap_or(core::mem::zeroed()) as _, fcontextreq, reserved1, targetdatarep, pinput.unwrap_or(core::mem::zeroed()) as _, reserved2, phnewcontext.unwrap_or(core::mem::zeroed()) as _, poutput.unwrap_or(core::mem::zeroed()) as _, pfcontextattr as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn InitializeSecurityContextW(phcredential: Option<*const super::super::Credentials::SecHandle>, phcontext: Option<*const super::super::Credentials::SecHandle>, psztargetname: Option<*const u16>, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: Option<*mut super::super::Credentials::SecHandle>, poutput: Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: Option<*mut i64>) -> windows_core::HRESULT {
    windows_link::link!("secur32.dll" "system" fn InitializeSecurityContextW(phcredential : *const super::super::Credentials:: SecHandle, phcontext : *const super::super::Credentials:: SecHandle, psztargetname : *const u16, fcontextreq : ISC_REQ_FLAGS, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *mut super::super::Credentials:: SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { InitializeSecurityContextW(phcredential.unwrap_or(core::mem::zeroed()) as _, phcontext.unwrap_or(core::mem::zeroed()) as _, psztargetname.unwrap_or(core::mem::zeroed()) as _, fcontextreq, reserved1, targetdatarep, pinput.unwrap_or(core::mem::zeroed()) as _, reserved2, phnewcontext.unwrap_or(core::mem::zeroed()) as _, poutput.unwrap_or(core::mem::zeroed()) as _, pfcontextattr as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LsaAddAccountRights(policyhandle: LSA_HANDLE, accountsid: super::super::PSID, userrights: &[LSA_UNICODE_STRING]) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaAddAccountRights(policyhandle : LSA_HANDLE, accountsid : super::super:: PSID, userrights : *const LSA_UNICODE_STRING, countofrights : u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaAddAccountRights(policyhandle, accountsid, core::mem::transmute(userrights.as_ptr()), userrights.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn LsaCallAuthenticationPackage(lsahandle: super::super::super::Foundation::HANDLE, authenticationpackage: u32, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: Option<*mut *mut core::ffi::c_void>, returnbufferlength: Option<*mut u32>, protocolstatus: Option<*mut i32>) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaCallAuthenticationPackage(lsahandle : super::super::super::Foundation:: HANDLE, authenticationpackage : u32, protocolsubmitbuffer : *const core::ffi::c_void, submitbufferlength : u32, protocolreturnbuffer : *mut *mut core::ffi::c_void, returnbufferlength : *mut u32, protocolstatus : *mut i32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaCallAuthenticationPackage(lsahandle, authenticationpackage, protocolsubmitbuffer, submitbufferlength, protocolreturnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbufferlength.unwrap_or(core::mem::zeroed()) as _, protocolstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LsaClose(objecthandle: LSA_HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaClose(objecthandle : LSA_HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaClose(objecthandle) }
}
#[inline]
pub unsafe fn LsaConnectUntrusted(lsahandle: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaConnectUntrusted(lsahandle : *mut super::super::super::Foundation:: HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaConnectUntrusted(lsahandle as _) }
}
#[inline]
pub unsafe fn LsaCreateTrustedDomainEx(policyhandle: LSA_HANDLE, trusteddomaininformation: *const TRUSTED_DOMAIN_INFORMATION_EX, authenticationinformation: *const TRUSTED_DOMAIN_AUTH_INFORMATION, desiredaccess: u32, trusteddomainhandle: *mut LSA_HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaCreateTrustedDomainEx(policyhandle : LSA_HANDLE, trusteddomaininformation : *const TRUSTED_DOMAIN_INFORMATION_EX, authenticationinformation : *const TRUSTED_DOMAIN_AUTH_INFORMATION, desiredaccess : u32, trusteddomainhandle : *mut LSA_HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaCreateTrustedDomainEx(policyhandle, trusteddomaininformation, authenticationinformation, desiredaccess, trusteddomainhandle as _) }
}
#[inline]
pub unsafe fn LsaDeleteTrustedDomain(policyhandle: LSA_HANDLE, trusteddomainsid: super::super::PSID) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaDeleteTrustedDomain(policyhandle : LSA_HANDLE, trusteddomainsid : super::super:: PSID) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaDeleteTrustedDomain(policyhandle, trusteddomainsid) }
}
#[inline]
pub unsafe fn LsaDeregisterLogonProcess(lsahandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaDeregisterLogonProcess(lsahandle : super::super::super::Foundation:: HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaDeregisterLogonProcess(lsahandle) }
}
#[inline]
pub unsafe fn LsaEnumerateAccountRights(policyhandle: LSA_HANDLE, accountsid: super::super::PSID, userrights: *mut *mut LSA_UNICODE_STRING, countofrights: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaEnumerateAccountRights(policyhandle : LSA_HANDLE, accountsid : super::super:: PSID, userrights : *mut *mut LSA_UNICODE_STRING, countofrights : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaEnumerateAccountRights(policyhandle, accountsid, userrights as _, countofrights as _) }
}
#[inline]
pub unsafe fn LsaEnumerateAccountsWithUserRight(policyhandle: LSA_HANDLE, userright: Option<*const LSA_UNICODE_STRING>, buffer: *mut *mut core::ffi::c_void, countreturned: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaEnumerateAccountsWithUserRight(policyhandle : LSA_HANDLE, userright : *const LSA_UNICODE_STRING, buffer : *mut *mut core::ffi::c_void, countreturned : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaEnumerateAccountsWithUserRight(policyhandle, userright.unwrap_or(core::mem::zeroed()) as _, buffer as _, countreturned as _) }
}
#[inline]
pub unsafe fn LsaEnumerateLogonSessions(logonsessioncount: *mut u32, logonsessionlist: *mut *mut super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaEnumerateLogonSessions(logonsessioncount : *mut u32, logonsessionlist : *mut *mut super::super::super::Foundation:: LUID) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaEnumerateLogonSessions(logonsessioncount as _, logonsessionlist as _) }
}
#[inline]
pub unsafe fn LsaEnumerateTrustedDomains(policyhandle: LSA_HANDLE, enumerationcontext: *mut u32, buffer: *mut *mut core::ffi::c_void, preferedmaximumlength: u32, countreturned: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaEnumerateTrustedDomains(policyhandle : LSA_HANDLE, enumerationcontext : *mut u32, buffer : *mut *mut core::ffi::c_void, preferedmaximumlength : u32, countreturned : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaEnumerateTrustedDomains(policyhandle, enumerationcontext as _, buffer as _, preferedmaximumlength, countreturned as _) }
}
#[inline]
pub unsafe fn LsaEnumerateTrustedDomainsEx(policyhandle: LSA_HANDLE, enumerationcontext: *mut u32, buffer: *mut *mut core::ffi::c_void, preferedmaximumlength: u32, countreturned: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaEnumerateTrustedDomainsEx(policyhandle : LSA_HANDLE, enumerationcontext : *mut u32, buffer : *mut *mut core::ffi::c_void, preferedmaximumlength : u32, countreturned : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaEnumerateTrustedDomainsEx(policyhandle, enumerationcontext as _, buffer as _, preferedmaximumlength, countreturned as _) }
}
#[inline]
pub unsafe fn LsaFreeMemory(buffer: Option<*const core::ffi::c_void>) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaFreeMemory(buffer : *const core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaFreeMemory(buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LsaFreeReturnBuffer(buffer: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaFreeReturnBuffer(buffer : *const core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaFreeReturnBuffer(buffer) }
}
#[inline]
pub unsafe fn LsaGetAppliedCAPIDs(systemname: Option<*const LSA_UNICODE_STRING>, capids: *mut *mut super::super::PSID, capidcount: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaGetAppliedCAPIDs(systemname : *const LSA_UNICODE_STRING, capids : *mut *mut super::super:: PSID, capidcount : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaGetAppliedCAPIDs(systemname.unwrap_or(core::mem::zeroed()) as _, capids as _, capidcount as _) }
}
#[inline]
pub unsafe fn LsaGetLogonSessionData(logonid: *const super::super::super::Foundation::LUID, pplogonsessiondata: *mut *mut SECURITY_LOGON_SESSION_DATA) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaGetLogonSessionData(logonid : *const super::super::super::Foundation:: LUID, pplogonsessiondata : *mut *mut SECURITY_LOGON_SESSION_DATA) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaGetLogonSessionData(logonid, pplogonsessiondata as _) }
}
#[inline]
pub unsafe fn LsaLogonUser(lsahandle: super::super::super::Foundation::HANDLE, originname: *const LSA_STRING, logontype: SECURITY_LOGON_TYPE, authenticationpackage: u32, authenticationinformation: *const core::ffi::c_void, authenticationinformationlength: u32, localgroups: Option<*const super::super::TOKEN_GROUPS>, sourcecontext: *const super::super::TOKEN_SOURCE, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: *mut u32, logonid: *mut super::super::super::Foundation::LUID, token: *mut super::super::super::Foundation::HANDLE, quotas: *mut super::super::QUOTA_LIMITS, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaLogonUser(lsahandle : super::super::super::Foundation:: HANDLE, originname : *const LSA_STRING, logontype : SECURITY_LOGON_TYPE, authenticationpackage : u32, authenticationinformation : *const core::ffi::c_void, authenticationinformationlength : u32, localgroups : *const super::super:: TOKEN_GROUPS, sourcecontext : *const super::super:: TOKEN_SOURCE, profilebuffer : *mut *mut core::ffi::c_void, profilebufferlength : *mut u32, logonid : *mut super::super::super::Foundation:: LUID, token : *mut super::super::super::Foundation:: HANDLE, quotas : *mut super::super:: QUOTA_LIMITS, substatus : *mut i32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaLogonUser(lsahandle, originname, logontype, authenticationpackage, authenticationinformation, authenticationinformationlength, localgroups.unwrap_or(core::mem::zeroed()) as _, sourcecontext, profilebuffer as _, profilebufferlength as _, logonid as _, token as _, quotas as _, substatus as _) }
}
#[inline]
pub unsafe fn LsaLookupAuthenticationPackage(lsahandle: super::super::super::Foundation::HANDLE, packagename: *const LSA_STRING, authenticationpackage: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaLookupAuthenticationPackage(lsahandle : super::super::super::Foundation:: HANDLE, packagename : *const LSA_STRING, authenticationpackage : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaLookupAuthenticationPackage(lsahandle, packagename, authenticationpackage as _) }
}
#[inline]
pub unsafe fn LsaLookupNames(policyhandle: LSA_HANDLE, count: u32, names: *const LSA_UNICODE_STRING, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids: *mut *mut LSA_TRANSLATED_SID) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaLookupNames(policyhandle : LSA_HANDLE, count : u32, names : *const LSA_UNICODE_STRING, referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids : *mut *mut LSA_TRANSLATED_SID) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaLookupNames(policyhandle, count, names, referenceddomains as _, sids as _) }
}
#[inline]
pub unsafe fn LsaLookupNames2(policyhandle: LSA_HANDLE, flags: u32, count: u32, names: *const LSA_UNICODE_STRING, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids: *mut *mut LSA_TRANSLATED_SID2) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaLookupNames2(policyhandle : LSA_HANDLE, flags : u32, count : u32, names : *const LSA_UNICODE_STRING, referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST, sids : *mut *mut LSA_TRANSLATED_SID2) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaLookupNames2(policyhandle, flags, count, names, referenceddomains as _, sids as _) }
}
#[inline]
pub unsafe fn LsaLookupSids(policyhandle: LSA_HANDLE, count: u32, sids: *const super::super::PSID, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, names: *mut *mut LSA_TRANSLATED_NAME) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaLookupSids(policyhandle : LSA_HANDLE, count : u32, sids : *const super::super:: PSID, referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST, names : *mut *mut LSA_TRANSLATED_NAME) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaLookupSids(policyhandle, count, sids, referenceddomains as _, names as _) }
}
#[inline]
pub unsafe fn LsaLookupSids2(policyhandle: LSA_HANDLE, lookupoptions: u32, count: u32, sids: *const super::super::PSID, referenceddomains: *mut *mut LSA_REFERENCED_DOMAIN_LIST, names: *mut *mut LSA_TRANSLATED_NAME) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaLookupSids2(policyhandle : LSA_HANDLE, lookupoptions : u32, count : u32, sids : *const super::super:: PSID, referenceddomains : *mut *mut LSA_REFERENCED_DOMAIN_LIST, names : *mut *mut LSA_TRANSLATED_NAME) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaLookupSids2(policyhandle, lookupoptions, count, sids, referenceddomains as _, names as _) }
}
#[inline]
pub unsafe fn LsaNtStatusToWinError(status: super::super::super::Foundation::NTSTATUS) -> u32 {
    windows_link::link!("advapi32.dll" "system" fn LsaNtStatusToWinError(status : super::super::super::Foundation:: NTSTATUS) -> u32);
    unsafe { LsaNtStatusToWinError(status) }
}
#[inline]
pub unsafe fn LsaOpenPolicy(systemname: Option<*const LSA_UNICODE_STRING>, objectattributes: *const LSA_OBJECT_ATTRIBUTES, desiredaccess: u32, policyhandle: *mut LSA_HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaOpenPolicy(systemname : *const LSA_UNICODE_STRING, objectattributes : *const LSA_OBJECT_ATTRIBUTES, desiredaccess : u32, policyhandle : *mut LSA_HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaOpenPolicy(systemname.unwrap_or(core::mem::zeroed()) as _, objectattributes, desiredaccess, policyhandle as _) }
}
#[inline]
pub unsafe fn LsaOpenTrustedDomainByName(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, desiredaccess: u32, trusteddomainhandle: *mut LSA_HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaOpenTrustedDomainByName(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, desiredaccess : u32, trusteddomainhandle : *mut LSA_HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaOpenTrustedDomainByName(policyhandle, trusteddomainname, desiredaccess, trusteddomainhandle as _) }
}
#[inline]
pub unsafe fn LsaQueryCAPs(capids: Option<&[super::super::PSID]>, caps: *mut *mut CENTRAL_ACCESS_POLICY, capcount: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryCAPs(capids : *const super::super:: PSID, capidcount : u32, caps : *mut *mut CENTRAL_ACCESS_POLICY, capcount : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryCAPs(core::mem::transmute(capids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), capids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), caps as _, capcount as _) }
}
#[inline]
pub unsafe fn LsaQueryDomainInformationPolicy(policyhandle: LSA_HANDLE, informationclass: POLICY_DOMAIN_INFORMATION_CLASS, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryDomainInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_DOMAIN_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryDomainInformationPolicy(policyhandle, informationclass, buffer as _) }
}
#[inline]
pub unsafe fn LsaQueryForestTrustInformation(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, foresttrustinfo: *mut *mut LSA_FOREST_TRUST_INFORMATION) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryForestTrustInformation(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, foresttrustinfo : *mut *mut LSA_FOREST_TRUST_INFORMATION) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryForestTrustInformation(policyhandle, trusteddomainname, foresttrustinfo as _) }
}
#[inline]
pub unsafe fn LsaQueryForestTrustInformation2(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, highestrecordtype: LSA_FOREST_TRUST_RECORD_TYPE, foresttrustinfo: *mut *mut LSA_FOREST_TRUST_INFORMATION2) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryForestTrustInformation2(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, highestrecordtype : LSA_FOREST_TRUST_RECORD_TYPE, foresttrustinfo : *mut *mut LSA_FOREST_TRUST_INFORMATION2) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryForestTrustInformation2(policyhandle, trusteddomainname, highestrecordtype, foresttrustinfo as _) }
}
#[inline]
pub unsafe fn LsaQueryInformationPolicy(policyhandle: LSA_HANDLE, informationclass: POLICY_INFORMATION_CLASS, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryInformationPolicy(policyhandle, informationclass, buffer as _) }
}
#[inline]
pub unsafe fn LsaQueryTrustedDomainInfo(policyhandle: LSA_HANDLE, trusteddomainsid: super::super::PSID, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryTrustedDomainInfo(policyhandle : LSA_HANDLE, trusteddomainsid : super::super:: PSID, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryTrustedDomainInfo(policyhandle, trusteddomainsid, informationclass, buffer as _) }
}
#[inline]
pub unsafe fn LsaQueryTrustedDomainInfoByName(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaQueryTrustedDomainInfoByName(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *mut *mut core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaQueryTrustedDomainInfoByName(policyhandle, trusteddomainname, informationclass, buffer as _) }
}
#[inline]
pub unsafe fn LsaRegisterLogonProcess(logonprocessname: *const LSA_STRING, lsahandle: *mut super::super::super::Foundation::HANDLE, securitymode: *mut u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaRegisterLogonProcess(logonprocessname : *const LSA_STRING, lsahandle : *mut super::super::super::Foundation:: HANDLE, securitymode : *mut u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaRegisterLogonProcess(logonprocessname, lsahandle as _, securitymode as _) }
}
#[inline]
pub unsafe fn LsaRegisterPolicyChangeNotification(informationclass: POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaRegisterPolicyChangeNotification(informationclass : POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle : super::super::super::Foundation:: HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaRegisterPolicyChangeNotification(informationclass, notificationeventhandle) }
}
#[inline]
pub unsafe fn LsaRemoveAccountRights(policyhandle: LSA_HANDLE, accountsid: super::super::PSID, allrights: bool, userrights: Option<&[LSA_UNICODE_STRING]>) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaRemoveAccountRights(policyhandle : LSA_HANDLE, accountsid : super::super:: PSID, allrights : bool, userrights : *const LSA_UNICODE_STRING, countofrights : u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaRemoveAccountRights(policyhandle, accountsid, allrights, core::mem::transmute(userrights.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), userrights.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn LsaRetrievePrivateData(policyhandle: LSA_HANDLE, keyname: *const LSA_UNICODE_STRING, privatedata: *mut *mut LSA_UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaRetrievePrivateData(policyhandle : LSA_HANDLE, keyname : *const LSA_UNICODE_STRING, privatedata : *mut *mut LSA_UNICODE_STRING) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaRetrievePrivateData(policyhandle, keyname, privatedata as _) }
}
#[inline]
pub unsafe fn LsaSetCAPs(capdns: Option<&[LSA_UNICODE_STRING]>, flags: u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetCAPs(capdns : *const LSA_UNICODE_STRING, capdncount : u32, flags : u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetCAPs(core::mem::transmute(capdns.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), capdns.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn LsaSetDomainInformationPolicy(policyhandle: LSA_HANDLE, informationclass: POLICY_DOMAIN_INFORMATION_CLASS, buffer: Option<*const core::ffi::c_void>) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetDomainInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_DOMAIN_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetDomainInformationPolicy(policyhandle, informationclass, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LsaSetForestTrustInformation(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, foresttrustinfo: *const LSA_FOREST_TRUST_INFORMATION, checkonly: bool, collisioninfo: *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetForestTrustInformation(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, foresttrustinfo : *const LSA_FOREST_TRUST_INFORMATION, checkonly : bool, collisioninfo : *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetForestTrustInformation(policyhandle, trusteddomainname, foresttrustinfo, checkonly, collisioninfo as _) }
}
#[inline]
pub unsafe fn LsaSetForestTrustInformation2(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, highestrecordtype: LSA_FOREST_TRUST_RECORD_TYPE, foresttrustinfo: *const LSA_FOREST_TRUST_INFORMATION2, checkonly: bool, collisioninfo: *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetForestTrustInformation2(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, highestrecordtype : LSA_FOREST_TRUST_RECORD_TYPE, foresttrustinfo : *const LSA_FOREST_TRUST_INFORMATION2, checkonly : bool, collisioninfo : *mut *mut LSA_FOREST_TRUST_COLLISION_INFORMATION) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetForestTrustInformation2(policyhandle, trusteddomainname, highestrecordtype, foresttrustinfo, checkonly, collisioninfo as _) }
}
#[inline]
pub unsafe fn LsaSetInformationPolicy(policyhandle: LSA_HANDLE, informationclass: POLICY_INFORMATION_CLASS, buffer: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetInformationPolicy(policyhandle : LSA_HANDLE, informationclass : POLICY_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetInformationPolicy(policyhandle, informationclass, buffer) }
}
#[inline]
pub unsafe fn LsaSetTrustedDomainInfoByName(policyhandle: LSA_HANDLE, trusteddomainname: *const LSA_UNICODE_STRING, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetTrustedDomainInfoByName(policyhandle : LSA_HANDLE, trusteddomainname : *const LSA_UNICODE_STRING, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetTrustedDomainInfoByName(policyhandle, trusteddomainname, informationclass, buffer) }
}
#[inline]
pub unsafe fn LsaSetTrustedDomainInformation(policyhandle: LSA_HANDLE, trusteddomainsid: super::super::PSID, informationclass: TRUSTED_INFORMATION_CLASS, buffer: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaSetTrustedDomainInformation(policyhandle : LSA_HANDLE, trusteddomainsid : super::super:: PSID, informationclass : TRUSTED_INFORMATION_CLASS, buffer : *const core::ffi::c_void) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaSetTrustedDomainInformation(policyhandle, trusteddomainsid, informationclass, buffer) }
}
#[inline]
pub unsafe fn LsaStorePrivateData(policyhandle: LSA_HANDLE, keyname: *const LSA_UNICODE_STRING, privatedata: Option<*const LSA_UNICODE_STRING>) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" fn LsaStorePrivateData(policyhandle : LSA_HANDLE, keyname : *const LSA_UNICODE_STRING, privatedata : *const LSA_UNICODE_STRING) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaStorePrivateData(policyhandle, keyname, privatedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LsaUnregisterPolicyChangeNotification(informationclass: POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("secur32.dll" "system" fn LsaUnregisterPolicyChangeNotification(informationclass : POLICY_NOTIFICATION_INFORMATION_CLASS, notificationeventhandle : super::super::super::Foundation:: HANDLE) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { LsaUnregisterPolicyChangeNotification(informationclass, notificationeventhandle) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn MakeSignature(phcontext: *const super::super::Credentials::SecHandle, fqop: u32, pmessage: *const SecBufferDesc, messageseqno: u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn MakeSignature(phcontext : *const super::super::Credentials:: SecHandle, fqop : u32, pmessage : *const SecBufferDesc, messageseqno : u32) -> windows_core::HRESULT);
    unsafe { MakeSignature(phcontext, fqop, pmessage, messageseqno).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn QueryContextAttributesA(phcontext : *const super::super::Credentials:: SecHandle, ulattribute : SECPKG_ATTR, pbuffer : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { QueryContextAttributesA(phcontext, ulattribute, pbuffer as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesExA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn QueryContextAttributesExA(phcontext : *const super::super::Credentials:: SecHandle, ulattribute : SECPKG_ATTR, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { QueryContextAttributesExA(phcontext, ulattribute, pbuffer as _, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesExW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn QueryContextAttributesExW(phcontext : *const super::super::Credentials:: SecHandle, ulattribute : SECPKG_ATTR, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { QueryContextAttributesExW(phcontext, ulattribute, pbuffer as _, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryContextAttributesW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn QueryContextAttributesW(phcontext : *const super::super::Credentials:: SecHandle, ulattribute : SECPKG_ATTR, pbuffer : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { QueryContextAttributesW(phcontext, ulattribute, pbuffer as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn QueryCredentialsAttributesA(phcredential : *const super::super::Credentials:: SecHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { QueryCredentialsAttributesA(phcredential, ulattribute, pbuffer as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesExA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn QueryCredentialsAttributesExA(phcredential : *const super::super::Credentials:: SecHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { QueryCredentialsAttributesExA(phcredential, ulattribute, pbuffer as _, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesExW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn QueryCredentialsAttributesExW(phcredential : *const super::super::Credentials:: SecHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { QueryCredentialsAttributesExW(phcredential, ulattribute, pbuffer as _, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QueryCredentialsAttributesW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn QueryCredentialsAttributesW(phcredential : *const super::super::Credentials:: SecHandle, ulattribute : u32, pbuffer : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { QueryCredentialsAttributesW(phcredential, ulattribute, pbuffer as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn QuerySecurityContextToken(phcontext: *const super::super::Credentials::SecHandle, token: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn QuerySecurityContextToken(phcontext : *const super::super::Credentials:: SecHandle, token : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { QuerySecurityContextToken(phcontext, token as _).ok() }
}
#[inline]
pub unsafe fn QuerySecurityPackageInfoA<P0>(pszpackagename: P0) -> windows_core::Result<*mut SecPkgInfoA>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn QuerySecurityPackageInfoA(pszpackagename : windows_core::PCSTR, pppackageinfo : *mut *mut SecPkgInfoA) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        QuerySecurityPackageInfoA(pszpackagename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn QuerySecurityPackageInfoW<P0>(pszpackagename: P0) -> windows_core::Result<*mut SecPkgInfoW>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn QuerySecurityPackageInfoW(pszpackagename : windows_core::PCWSTR, pppackageinfo : *mut *mut SecPkgInfoW) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        QuerySecurityPackageInfoW(pszpackagename.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn RevertSecurityContext(phcontext: *const super::super::Credentials::SecHandle) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn RevertSecurityContext(phcontext : *const super::super::Credentials:: SecHandle) -> windows_core::HRESULT);
    unsafe { RevertSecurityContext(phcontext).ok() }
}
#[inline]
pub unsafe fn RtlDecryptMemory(memory: *mut core::ffi::c_void, memorysize: u32, optionflags: u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" "SystemFunction041" fn RtlDecryptMemory(memory : *mut core::ffi::c_void, memorysize : u32, optionflags : u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { RtlDecryptMemory(memory as _, memorysize, optionflags) }
}
#[inline]
pub unsafe fn RtlEncryptMemory(memory: *mut core::ffi::c_void, memorysize: u32, optionflags: u32) -> super::super::super::Foundation::NTSTATUS {
    windows_link::link!("advapi32.dll" "system" "SystemFunction040" fn RtlEncryptMemory(memory : *mut core::ffi::c_void, memorysize : u32, optionflags : u32) -> super::super::super::Foundation:: NTSTATUS);
    unsafe { RtlEncryptMemory(memory as _, memorysize, optionflags) }
}
#[inline]
pub unsafe fn RtlGenRandom(randombuffer: *mut core::ffi::c_void, randombufferlength: u32) -> bool {
    windows_link::link!("advapi32.dll" "system" "SystemFunction036" fn RtlGenRandom(randombuffer : *mut core::ffi::c_void, randombufferlength : u32) -> bool);
    unsafe { RtlGenRandom(randombuffer as _, randombufferlength) }
}
#[inline]
pub unsafe fn SLAcquireGenuineTicket<P2, P3, P4>(ppticketblob: *mut *mut core::ffi::c_void, pcbticketblob: *mut u32, pwsztemplateid: P2, pwszserverurl: P3, pwszclienttoken: P4) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slcext.dll" "system" fn SLAcquireGenuineTicket(ppticketblob : *mut *mut core::ffi::c_void, pcbticketblob : *mut u32, pwsztemplateid : windows_core::PCWSTR, pwszserverurl : windows_core::PCWSTR, pwszclienttoken : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SLAcquireGenuineTicket(ppticketblob as _, pcbticketblob as _, pwsztemplateid.param().abi(), pwszserverurl.param().abi(), pwszclienttoken.param().abi()).ok() }
}
#[inline]
pub unsafe fn SLActivateProduct<P5>(hslc: *const core::ffi::c_void, pproductskuid: *const windows_core::GUID, cbappspecificdata: Option<u32>, pvappspecificdata: Option<*const core::ffi::c_void>, pactivationinfo: Option<*const SL_ACTIVATION_INFO_HEADER>, pwszproxyserver: P5, wproxyport: Option<u16>) -> windows_core::Result<()>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slcext.dll" "system" fn SLActivateProduct(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, cbappspecificdata : u32, pvappspecificdata : *const core::ffi::c_void, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, pwszproxyserver : windows_core::PCWSTR, wproxyport : u16) -> windows_core::HRESULT);
    unsafe { SLActivateProduct(hslc, pproductskuid, cbappspecificdata.unwrap_or(core::mem::zeroed()) as _, pvappspecificdata.unwrap_or(core::mem::zeroed()) as _, pactivationinfo.unwrap_or(core::mem::zeroed()) as _, pwszproxyserver.param().abi(), wproxyport.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SLClose(hslc: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLClose(hslc : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SLClose(hslc).ok() }
}
#[inline]
pub unsafe fn SLConsumeRight<P3>(hslc: *const core::ffi::c_void, pappid: *const windows_core::GUID, pproductskuid: Option<*const windows_core::GUID>, pwszrightname: P3, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLConsumeRight(hslc : *const core::ffi::c_void, pappid : *const windows_core::GUID, pproductskuid : *const windows_core::GUID, pwszrightname : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SLConsumeRight(hslc, pappid, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pwszrightname.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SLDepositOfflineConfirmationId<P2, P3>(hslc: *const core::ffi::c_void, pproductskuid: *const windows_core::GUID, pwszinstallationid: P2, pwszconfirmationid: P3) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLDepositOfflineConfirmationId(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, pwszinstallationid : windows_core::PCWSTR, pwszconfirmationid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SLDepositOfflineConfirmationId(hslc, pproductskuid, pwszinstallationid.param().abi(), pwszconfirmationid.param().abi()).ok() }
}
#[inline]
pub unsafe fn SLDepositOfflineConfirmationIdEx<P3, P4>(hslc: *const core::ffi::c_void, pproductskuid: Option<*const windows_core::GUID>, pactivationinfo: Option<*const SL_ACTIVATION_INFO_HEADER>, pwszinstallationid: P3, pwszconfirmationid: P4) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLDepositOfflineConfirmationIdEx(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, pwszinstallationid : windows_core::PCWSTR, pwszconfirmationid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SLDepositOfflineConfirmationIdEx(hslc, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pactivationinfo.unwrap_or(core::mem::zeroed()) as _, pwszinstallationid.param().abi(), pwszconfirmationid.param().abi()).ok() }
}
#[inline]
pub unsafe fn SLFireEvent<P1>(hslc: *const core::ffi::c_void, pwszeventid: P1, papplicationid: *const windows_core::GUID) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLFireEvent(hslc : *const core::ffi::c_void, pwszeventid : windows_core::PCWSTR, papplicationid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SLFireEvent(hslc, pwszeventid.param().abi(), papplicationid).ok() }
}
#[inline]
pub unsafe fn SLGenerateOfflineInstallationId(hslc: *const core::ffi::c_void, pproductskuid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_link::link!("slc.dll" "system" fn SLGenerateOfflineInstallationId(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, ppwszinstallationid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGenerateOfflineInstallationId(hslc, pproductskuid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGenerateOfflineInstallationIdEx(hslc: *const core::ffi::c_void, pproductskuid: Option<*const windows_core::GUID>, pactivationinfo: Option<*const SL_ACTIVATION_INFO_HEADER>) -> windows_core::Result<windows_core::PWSTR> {
    windows_link::link!("slc.dll" "system" fn SLGenerateOfflineInstallationIdEx(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, ppwszinstallationid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGenerateOfflineInstallationIdEx(hslc, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pactivationinfo.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetApplicationInformation<P2>(hslc: *const core::ffi::c_void, papplicationid: *const windows_core::GUID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetApplicationInformation(hslc : *const core::ffi::c_void, papplicationid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetApplicationInformation(hslc, papplicationid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetGenuineInformation<P1>(pqueryid: *const windows_core::GUID, pwszvaluename: P1, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetGenuineInformation(pqueryid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetGenuineInformation(pqueryid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetInstalledProductKeyIds(hslc: *const core::ffi::c_void, pproductskuid: *const windows_core::GUID, pnproductkeyids: *mut u32, ppproductkeyids: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLGetInstalledProductKeyIds(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, pnproductkeyids : *mut u32, ppproductkeyids : *mut *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SLGetInstalledProductKeyIds(hslc, pproductskuid, pnproductkeyids as _, ppproductkeyids as _).ok() }
}
#[inline]
pub unsafe fn SLGetLicense(hslc: *const core::ffi::c_void, plicensefileid: *const windows_core::GUID, pcblicensefile: *mut u32, ppblicensefile: *mut *mut u8) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLGetLicense(hslc : *const core::ffi::c_void, plicensefileid : *const windows_core::GUID, pcblicensefile : *mut u32, ppblicensefile : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetLicense(hslc, plicensefileid, pcblicensefile as _, ppblicensefile as _).ok() }
}
#[inline]
pub unsafe fn SLGetLicenseFileId(hslc: *const core::ffi::c_void, pblicenseblob: &[u8]) -> windows_core::Result<windows_core::GUID> {
    windows_link::link!("slc.dll" "system" fn SLGetLicenseFileId(hslc : *const core::ffi::c_void, cblicenseblob : u32, pblicenseblob : *const u8, plicensefileid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetLicenseFileId(hslc, pblicenseblob.len().try_into().unwrap(), core::mem::transmute(pblicenseblob.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetLicenseInformation<P2>(hslc: *const core::ffi::c_void, psllicenseid: *const windows_core::GUID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetLicenseInformation(hslc : *const core::ffi::c_void, psllicenseid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetLicenseInformation(hslc, psllicenseid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetLicensingStatusInformation<P3>(hslc: *const core::ffi::c_void, pappid: Option<*const windows_core::GUID>, pproductskuid: Option<*const windows_core::GUID>, pwszrightname: P3, pnstatuscount: *mut u32, pplicensingstatus: *mut *mut SL_LICENSING_STATUS) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetLicensingStatusInformation(hslc : *const core::ffi::c_void, pappid : *const windows_core::GUID, pproductskuid : *const windows_core::GUID, pwszrightname : windows_core::PCWSTR, pnstatuscount : *mut u32, pplicensingstatus : *mut *mut SL_LICENSING_STATUS) -> windows_core::HRESULT);
    unsafe { SLGetLicensingStatusInformation(hslc, pappid.unwrap_or(core::mem::zeroed()) as _, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pwszrightname.param().abi(), pnstatuscount as _, pplicensingstatus as _).ok() }
}
#[inline]
pub unsafe fn SLGetPKeyId<P1, P2>(hslc: *const core::ffi::c_void, pwszpkeyalgorithm: P1, pwszpkeystring: P2, pbpkeyspecificdata: Option<&[u8]>) -> windows_core::Result<windows_core::GUID>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetPKeyId(hslc : *const core::ffi::c_void, pwszpkeyalgorithm : windows_core::PCWSTR, pwszpkeystring : windows_core::PCWSTR, cbpkeyspecificdata : u32, pbpkeyspecificdata : *const u8, ppkeyid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetPKeyId(hslc, pwszpkeyalgorithm.param().abi(), pwszpkeystring.param().abi(), pbpkeyspecificdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbpkeyspecificdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetPKeyInformation<P2>(hslc: *const core::ffi::c_void, ppkeyid: *const windows_core::GUID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetPKeyInformation(hslc : *const core::ffi::c_void, ppkeyid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetPKeyInformation(hslc, ppkeyid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetPolicyInformation<P1>(hslc: *const core::ffi::c_void, pwszvaluename: P1, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetPolicyInformation(hslc : *const core::ffi::c_void, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetPolicyInformation(hslc, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetPolicyInformationDWORD<P1>(hslc: *const core::ffi::c_void, pwszvaluename: P1) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetPolicyInformationDWORD(hslc : *const core::ffi::c_void, pwszvaluename : windows_core::PCWSTR, pdwvalue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetPolicyInformationDWORD(hslc, pwszvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetProductSkuInformation<P2>(hslc: *const core::ffi::c_void, pproductskuid: *const windows_core::GUID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetProductSkuInformation(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetProductSkuInformation(hslc, pproductskuid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetReferralInformation<P3>(hslc: *const core::ffi::c_void, ereferraltype: SLREFERRALTYPE, pskuorappid: *const windows_core::GUID, pwszvaluename: P3) -> windows_core::Result<windows_core::PWSTR>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slcext.dll" "system" fn SLGetReferralInformation(hslc : *const core::ffi::c_void, ereferraltype : SLREFERRALTYPE, pskuorappid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetReferralInformation(hslc, ereferraltype, pskuorappid, pwszvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetSLIDList(hslc: *const core::ffi::c_void, equeryidtype: SLIDTYPE, pqueryid: Option<*const windows_core::GUID>, ereturnidtype: SLIDTYPE, pnreturnids: *mut u32, ppreturnids: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLGetSLIDList(hslc : *const core::ffi::c_void, equeryidtype : SLIDTYPE, pqueryid : *const windows_core::GUID, ereturnidtype : SLIDTYPE, pnreturnids : *mut u32, ppreturnids : *mut *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SLGetSLIDList(hslc, equeryidtype, pqueryid.unwrap_or(core::mem::zeroed()) as _, ereturnidtype, pnreturnids as _, ppreturnids as _).ok() }
}
#[inline]
pub unsafe fn SLGetServerStatus<P0, P1, P2>(pwszserverurl: P0, pwszacquisitiontype: P1, pwszproxyserver: P2, wproxyport: Option<u16>) -> windows_core::Result<windows_core::HRESULT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slcext.dll" "system" fn SLGetServerStatus(pwszserverurl : windows_core::PCWSTR, pwszacquisitiontype : windows_core::PCWSTR, pwszproxyserver : windows_core::PCWSTR, wproxyport : u16, phrstatus : *mut windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetServerStatus(pwszserverurl.param().abi(), pwszacquisitiontype.param().abi(), pwszproxyserver.param().abi(), wproxyport.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetServiceInformation<P1>(hslc: *const core::ffi::c_void, pwszvaluename: P1, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetServiceInformation(hslc : *const core::ffi::c_void, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetServiceInformation(hslc, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetWindowsInformation<P0>(pwszvaluename: P0, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetWindowsInformation(pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetWindowsInformation(pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _).ok() }
}
#[inline]
pub unsafe fn SLGetWindowsInformationDWORD<P0>(pwszvaluename: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLGetWindowsInformationDWORD(pwszvaluename : windows_core::PCWSTR, pdwvalue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetWindowsInformationDWORD(pwszvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLInstallLicense(hslc: *const core::ffi::c_void, pblicenseblob: &[u8]) -> windows_core::Result<windows_core::GUID> {
    windows_link::link!("slc.dll" "system" fn SLInstallLicense(hslc : *const core::ffi::c_void, cblicenseblob : u32, pblicenseblob : *const u8, plicensefileid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLInstallLicense(hslc, pblicenseblob.len().try_into().unwrap(), core::mem::transmute(pblicenseblob.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLInstallProofOfPurchase<P1, P2>(hslc: *const core::ffi::c_void, pwszpkeyalgorithm: P1, pwszpkeystring: P2, pbpkeyspecificdata: Option<&[u8]>) -> windows_core::Result<windows_core::GUID>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLInstallProofOfPurchase(hslc : *const core::ffi::c_void, pwszpkeyalgorithm : windows_core::PCWSTR, pwszpkeystring : windows_core::PCWSTR, cbpkeyspecificdata : u32, pbpkeyspecificdata : *const u8, ppkeyid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLInstallProofOfPurchase(hslc, pwszpkeyalgorithm.param().abi(), pwszpkeystring.param().abi(), pbpkeyspecificdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbpkeyspecificdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLIsGenuineLocal(pappid: *const windows_core::GUID, pgenuinestate: *mut SL_GENUINE_STATE, puioptions: Option<*mut SL_NONGENUINE_UI_OPTIONS>) -> windows_core::Result<()> {
    windows_link::link!("slwga.dll" "system" fn SLIsGenuineLocal(pappid : *const windows_core::GUID, pgenuinestate : *mut SL_GENUINE_STATE, puioptions : *mut SL_NONGENUINE_UI_OPTIONS) -> windows_core::HRESULT);
    unsafe { SLIsGenuineLocal(pappid, pgenuinestate as _, puioptions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SLOpen(phslc: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLOpen(phslc : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SLOpen(phslc as _).ok() }
}
#[inline]
pub unsafe fn SLQueryLicenseValueFromApp<P0>(valuename: P0, valuetype: Option<*mut u32>, databuffer: Option<*mut core::ffi::c_void>, datasize: u32, resultdatasize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("api-ms-win-core-slapi-l1-1-0.dll" "system" fn SLQueryLicenseValueFromApp(valuename : windows_core::PCWSTR, valuetype : *mut u32, databuffer : *mut core::ffi::c_void, datasize : u32, resultdatasize : *mut u32) -> windows_core::HRESULT);
    unsafe { SLQueryLicenseValueFromApp(valuename.param().abi(), valuetype.unwrap_or(core::mem::zeroed()) as _, databuffer.unwrap_or(core::mem::zeroed()) as _, datasize, resultdatasize as _).ok() }
}
#[inline]
pub unsafe fn SLRegisterEvent<P1>(hslc: Option<*const core::ffi::c_void>, pwszeventid: P1, papplicationid: *const windows_core::GUID, hevent: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLRegisterEvent(hslc : *const core::ffi::c_void, pwszeventid : windows_core::PCWSTR, papplicationid : *const windows_core::GUID, hevent : super::super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { SLRegisterEvent(hslc.unwrap_or(core::mem::zeroed()) as _, pwszeventid.param().abi(), papplicationid, hevent).ok() }
}
#[inline]
pub unsafe fn SLSetCurrentProductKey(hslc: *const core::ffi::c_void, pproductskuid: *const windows_core::GUID, pproductkeyid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLSetCurrentProductKey(hslc : *const core::ffi::c_void, pproductskuid : *const windows_core::GUID, pproductkeyid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SLSetCurrentProductKey(hslc, pproductskuid, pproductkeyid).ok() }
}
#[inline]
pub unsafe fn SLSetGenuineInformation<P1>(pqueryid: *const windows_core::GUID, pwszvaluename: P1, edatatype: SLDATATYPE, pbvalue: Option<&[u8]>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLSetGenuineInformation(pqueryid : *const windows_core::GUID, pwszvaluename : windows_core::PCWSTR, edatatype : SLDATATYPE, cbvalue : u32, pbvalue : *const u8) -> windows_core::HRESULT);
    unsafe { SLSetGenuineInformation(pqueryid, pwszvaluename.param().abi(), edatatype, pbvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
}
#[inline]
pub unsafe fn SLUninstallLicense(hslc: *const core::ffi::c_void, plicensefileid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLUninstallLicense(hslc : *const core::ffi::c_void, plicensefileid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SLUninstallLicense(hslc, plicensefileid).ok() }
}
#[inline]
pub unsafe fn SLUninstallProofOfPurchase(hslc: *const core::ffi::c_void, ppkeyid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_link::link!("slc.dll" "system" fn SLUninstallProofOfPurchase(hslc : *const core::ffi::c_void, ppkeyid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SLUninstallProofOfPurchase(hslc, ppkeyid).ok() }
}
#[inline]
pub unsafe fn SLUnregisterEvent<P1>(hslc: Option<*const core::ffi::c_void>, pwszeventid: P1, papplicationid: *const windows_core::GUID, hevent: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("slc.dll" "system" fn SLUnregisterEvent(hslc : *const core::ffi::c_void, pwszeventid : windows_core::PCWSTR, papplicationid : *const windows_core::GUID, hevent : super::super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { SLUnregisterEvent(hslc.unwrap_or(core::mem::zeroed()) as _, pwszeventid.param().abi(), papplicationid, hevent).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslAcceptSecurityContext(phcredential: Option<*const super::super::Credentials::SecHandle>, phcontext: Option<*const super::super::Credentials::SecHandle>, pinput: Option<*const SecBufferDesc>, fcontextreq: ASC_REQ_FLAGS, targetdatarep: u32, phnewcontext: Option<*mut super::super::Credentials::SecHandle>, poutput: Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: Option<*mut i64>) -> windows_core::HRESULT {
    windows_link::link!("secur32.dll" "system" fn SaslAcceptSecurityContext(phcredential : *const super::super::Credentials:: SecHandle, phcontext : *const super::super::Credentials:: SecHandle, pinput : *const SecBufferDesc, fcontextreq : ASC_REQ_FLAGS, targetdatarep : u32, phnewcontext : *mut super::super::Credentials:: SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { SaslAcceptSecurityContext(phcredential.unwrap_or(core::mem::zeroed()) as _, phcontext.unwrap_or(core::mem::zeroed()) as _, pinput.unwrap_or(core::mem::zeroed()) as _, fcontextreq, targetdatarep, phnewcontext.unwrap_or(core::mem::zeroed()) as _, poutput.unwrap_or(core::mem::zeroed()) as _, pfcontextattr as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SaslEnumerateProfilesA(profilelist: *mut windows_core::PSTR, profilecount: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SaslEnumerateProfilesA(profilelist : *mut windows_core::PSTR, profilecount : *mut u32) -> windows_core::HRESULT);
    unsafe { SaslEnumerateProfilesA(profilelist as _, profilecount as _).ok() }
}
#[inline]
pub unsafe fn SaslEnumerateProfilesW(profilelist: *mut windows_core::PWSTR, profilecount: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SaslEnumerateProfilesW(profilelist : *mut windows_core::PWSTR, profilecount : *mut u32) -> windows_core::HRESULT);
    unsafe { SaslEnumerateProfilesW(profilelist as _, profilecount as _).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslGetContextOption(contexthandle: *const super::super::Credentials::SecHandle, option: u32, value: *mut core::ffi::c_void, size: u32, needed: Option<*mut u32>) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SaslGetContextOption(contexthandle : *const super::super::Credentials:: SecHandle, option : u32, value : *mut core::ffi::c_void, size : u32, needed : *mut u32) -> windows_core::HRESULT);
    unsafe { SaslGetContextOption(contexthandle, option, value as _, size, needed.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SaslGetProfilePackageA<P0>(profilename: P0) -> windows_core::Result<*mut SecPkgInfoA>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SaslGetProfilePackageA(profilename : windows_core::PCSTR, packageinfo : *mut *mut SecPkgInfoA) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SaslGetProfilePackageA(profilename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SaslGetProfilePackageW<P0>(profilename: P0) -> windows_core::Result<*mut SecPkgInfoW>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SaslGetProfilePackageW(profilename : windows_core::PCWSTR, packageinfo : *mut *mut SecPkgInfoW) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SaslGetProfilePackageW(profilename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SaslIdentifyPackageA(pinput: *const SecBufferDesc) -> windows_core::Result<*mut SecPkgInfoA> {
    windows_link::link!("secur32.dll" "system" fn SaslIdentifyPackageA(pinput : *const SecBufferDesc, packageinfo : *mut *mut SecPkgInfoA) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SaslIdentifyPackageA(pinput, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SaslIdentifyPackageW(pinput: *const SecBufferDesc) -> windows_core::Result<*mut SecPkgInfoW> {
    windows_link::link!("secur32.dll" "system" fn SaslIdentifyPackageW(pinput : *const SecBufferDesc, packageinfo : *mut *mut SecPkgInfoW) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SaslIdentifyPackageW(pinput, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslInitializeSecurityContextA<P2>(phcredential: Option<*const super::super::Credentials::SecHandle>, phcontext: Option<*const super::super::Credentials::SecHandle>, psztargetname: P2, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: Option<*mut super::super::Credentials::SecHandle>, poutput: Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: Option<*mut i64>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SaslInitializeSecurityContextA(phcredential : *const super::super::Credentials:: SecHandle, phcontext : *const super::super::Credentials:: SecHandle, psztargetname : windows_core::PCSTR, fcontextreq : ISC_REQ_FLAGS, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *mut super::super::Credentials:: SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { SaslInitializeSecurityContextA(phcredential.unwrap_or(core::mem::zeroed()) as _, phcontext.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), fcontextreq, reserved1, targetdatarep, pinput.unwrap_or(core::mem::zeroed()) as _, reserved2, phnewcontext.unwrap_or(core::mem::zeroed()) as _, poutput.unwrap_or(core::mem::zeroed()) as _, pfcontextattr as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslInitializeSecurityContextW<P2>(phcredential: Option<*const super::super::Credentials::SecHandle>, phcontext: Option<*const super::super::Credentials::SecHandle>, psztargetname: P2, fcontextreq: ISC_REQ_FLAGS, reserved1: u32, targetdatarep: u32, pinput: Option<*const SecBufferDesc>, reserved2: u32, phnewcontext: Option<*mut super::super::Credentials::SecHandle>, poutput: Option<*mut SecBufferDesc>, pfcontextattr: *mut u32, ptsexpiry: Option<*mut i64>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SaslInitializeSecurityContextW(phcredential : *const super::super::Credentials:: SecHandle, phcontext : *const super::super::Credentials:: SecHandle, psztargetname : windows_core::PCWSTR, fcontextreq : ISC_REQ_FLAGS, reserved1 : u32, targetdatarep : u32, pinput : *const SecBufferDesc, reserved2 : u32, phnewcontext : *mut super::super::Credentials:: SecHandle, poutput : *mut SecBufferDesc, pfcontextattr : *mut u32, ptsexpiry : *mut i64) -> windows_core::HRESULT);
    unsafe { SaslInitializeSecurityContextW(phcredential.unwrap_or(core::mem::zeroed()) as _, phcontext.unwrap_or(core::mem::zeroed()) as _, psztargetname.param().abi(), fcontextreq, reserved1, targetdatarep, pinput.unwrap_or(core::mem::zeroed()) as _, reserved2, phnewcontext.unwrap_or(core::mem::zeroed()) as _, poutput.unwrap_or(core::mem::zeroed()) as _, pfcontextattr as _, ptsexpiry.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SaslSetContextOption(contexthandle: *const super::super::Credentials::SecHandle, option: u32, value: *const core::ffi::c_void, size: u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SaslSetContextOption(contexthandle : *const super::super::Credentials:: SecHandle, option : u32, value : *const core::ffi::c_void, size : u32) -> windows_core::HRESULT);
    unsafe { SaslSetContextOption(contexthandle, option, value, size).ok() }
}
#[inline]
pub unsafe fn SendSAS(asuser: bool) {
    windows_link::link!("sas.dll" "system" fn SendSAS(asuser : windows_core::BOOL));
    unsafe { SendSAS(asuser.into()) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetContextAttributesA(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SetContextAttributesA(phcontext : *const super::super::Credentials:: SecHandle, ulattribute : SECPKG_ATTR, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { SetContextAttributesA(phcontext, ulattribute, pbuffer, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetContextAttributesW(phcontext: *const super::super::Credentials::SecHandle, ulattribute: SECPKG_ATTR, pbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SetContextAttributesW(phcontext : *const super::super::Credentials:: SecHandle, ulattribute : SECPKG_ATTR, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { SetContextAttributesW(phcontext, ulattribute, pbuffer, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetCredentialsAttributesA(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SetCredentialsAttributesA(phcredential : *const super::super::Credentials:: SecHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { SetCredentialsAttributesA(phcredential, ulattribute, pbuffer, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn SetCredentialsAttributesW(phcredential: *const super::super::Credentials::SecHandle, ulattribute: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SetCredentialsAttributesW(phcredential : *const super::super::Credentials:: SecHandle, ulattribute : u32, pbuffer : *const core::ffi::c_void, cbbuffer : u32) -> windows_core::HRESULT);
    unsafe { SetCredentialsAttributesW(phcredential, ulattribute, pbuffer, cbbuffer).ok() }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn SslCrackCertificate(pbcertificate: *mut u8, cbcertificate: u32, dwflags: u32, ppcertificate: *mut *mut X509Certificate) -> windows_core::BOOL {
    windows_link::link!("schannel.dll" "system" fn SslCrackCertificate(pbcertificate : *mut u8, cbcertificate : u32, dwflags : u32, ppcertificate : *mut *mut X509Certificate) -> windows_core::BOOL);
    unsafe { SslCrackCertificate(pbcertificate as _, cbcertificate, dwflags, ppcertificate as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn SslDeserializeCertificateStore(serializedcertificatestore: super::super::Cryptography::CRYPT_INTEGER_BLOB) -> windows_core::Result<*mut super::super::Cryptography::CERT_CONTEXT> {
    windows_link::link!("schannel.dll" "system" fn SslDeserializeCertificateStore(serializedcertificatestore : super::super::Cryptography:: CRYPT_INTEGER_BLOB, ppcertcontext : *mut *mut super::super::Cryptography:: CERT_CONTEXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SslDeserializeCertificateStore(core::mem::transmute(serializedcertificatestore), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SslEmptyCacheA<P0>(psztargetname: P0, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("schannel.dll" "system" fn SslEmptyCacheA(psztargetname : windows_core::PCSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { SslEmptyCacheA(psztargetname.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn SslEmptyCacheW<P0>(psztargetname: P0, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("schannel.dll" "system" fn SslEmptyCacheW(psztargetname : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { SslEmptyCacheW(psztargetname.param().abi(), dwflags) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn SslFreeCertificate(pcertificate: *mut X509Certificate) {
    windows_link::link!("schannel.dll" "system" fn SslFreeCertificate(pcertificate : *mut X509Certificate));
    unsafe { SslFreeCertificate(pcertificate as _) }
}
#[inline]
pub unsafe fn SslGenerateRandomBits(prandomdata: *mut u8, crandomdata: i32) {
    windows_link::link!("schannel.dll" "system" fn SslGenerateRandomBits(prandomdata : *mut u8, crandomdata : i32));
    unsafe { SslGenerateRandomBits(prandomdata as _, crandomdata) }
}
#[inline]
pub unsafe fn SslGetExtensions(clienthello: &[u8], genericextensions: &mut [SCH_EXTENSION_DATA], bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> windows_core::Result<()> {
    windows_link::link!("schannel.dll" "system" fn SslGetExtensions(clienthello : *const u8, clienthellobytesize : u32, genericextensions : *mut SCH_EXTENSION_DATA, genericextensionscount : u8, bytestoread : *mut u32, flags : SchGetExtensionsOptions) -> windows_core::HRESULT);
    unsafe { SslGetExtensions(core::mem::transmute(clienthello.as_ptr()), clienthello.len().try_into().unwrap(), core::mem::transmute(genericextensions.as_ptr()), genericextensions.len().try_into().unwrap(), bytestoread as _, flags).ok() }
}
#[inline]
pub unsafe fn SslGetMaximumKeySize(reserved: u32) -> u32 {
    windows_link::link!("schannel.dll" "system" fn SslGetMaximumKeySize(reserved : u32) -> u32);
    unsafe { SslGetMaximumKeySize(reserved) }
}
#[inline]
pub unsafe fn SslGetServerIdentity(clienthello: &[u8], serveridentity: *mut *mut u8, serveridentitysize: *mut u32, flags: u32) -> windows_core::Result<()> {
    windows_link::link!("schannel.dll" "system" fn SslGetServerIdentity(clienthello : *const u8, clienthellosize : u32, serveridentity : *mut *mut u8, serveridentitysize : *mut u32, flags : u32) -> windows_core::HRESULT);
    unsafe { SslGetServerIdentity(core::mem::transmute(clienthello.as_ptr()), clienthello.len().try_into().unwrap(), serveridentity as _, serveridentitysize as _, flags).ok() }
}
#[inline]
pub unsafe fn SspiCompareAuthIdentities(authidentity1: Option<*const core::ffi::c_void>, authidentity2: Option<*const core::ffi::c_void>, samesupplieduser: Option<*mut bool>, samesuppliedidentity: Option<*mut bool>) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiCompareAuthIdentities(authidentity1 : *const core::ffi::c_void, authidentity2 : *const core::ffi::c_void, samesupplieduser : *mut bool, samesuppliedidentity : *mut bool) -> windows_core::HRESULT);
    unsafe { SspiCompareAuthIdentities(authidentity1.unwrap_or(core::mem::zeroed()) as _, authidentity2.unwrap_or(core::mem::zeroed()) as _, samesupplieduser.unwrap_or(core::mem::zeroed()) as _, samesuppliedidentity.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SspiCopyAuthIdentity(authdata: *const core::ffi::c_void, authdatacopy: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiCopyAuthIdentity(authdata : *const core::ffi::c_void, authdatacopy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiCopyAuthIdentity(authdata, authdatacopy as _).ok() }
}
#[inline]
pub unsafe fn SspiDecryptAuthIdentity(encryptedauthdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiDecryptAuthIdentity(encryptedauthdata : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiDecryptAuthIdentity(encryptedauthdata as _).ok() }
}
#[inline]
pub unsafe fn SspiDecryptAuthIdentityEx(options: u32, encryptedauthdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn SspiDecryptAuthIdentityEx(options : u32, encryptedauthdata : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiDecryptAuthIdentityEx(options, encryptedauthdata as _).ok() }
}
#[inline]
pub unsafe fn SspiEncodeAuthIdentityAsStrings(pauthidentity: *const core::ffi::c_void, ppszusername: *mut windows_core::PCWSTR, ppszdomainname: *mut windows_core::PCWSTR, ppszpackedcredentialsstring: Option<*mut windows_core::PCWSTR>) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiEncodeAuthIdentityAsStrings(pauthidentity : *const core::ffi::c_void, ppszusername : *mut windows_core::PCWSTR, ppszdomainname : *mut windows_core::PCWSTR, ppszpackedcredentialsstring : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SspiEncodeAuthIdentityAsStrings(pauthidentity, ppszusername as _, ppszdomainname as _, ppszpackedcredentialsstring.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SspiEncodeStringsAsAuthIdentity<P0, P1, P2>(pszusername: P0, pszdomainname: P1, pszpackedcredentialsstring: P2, ppauthidentity: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SspiEncodeStringsAsAuthIdentity(pszusername : windows_core::PCWSTR, pszdomainname : windows_core::PCWSTR, pszpackedcredentialsstring : windows_core::PCWSTR, ppauthidentity : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiEncodeStringsAsAuthIdentity(pszusername.param().abi(), pszdomainname.param().abi(), pszpackedcredentialsstring.param().abi(), ppauthidentity as _).ok() }
}
#[inline]
pub unsafe fn SspiEncryptAuthIdentity(authdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiEncryptAuthIdentity(authdata : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiEncryptAuthIdentity(authdata as _).ok() }
}
#[inline]
pub unsafe fn SspiEncryptAuthIdentityEx(options: u32, authdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn SspiEncryptAuthIdentityEx(options : u32, authdata : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiEncryptAuthIdentityEx(options, authdata as _).ok() }
}
#[inline]
pub unsafe fn SspiExcludePackage<P1>(authidentity: Option<*const core::ffi::c_void>, pszpackagename: P1, ppnewauthidentity: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SspiExcludePackage(authidentity : *const core::ffi::c_void, pszpackagename : windows_core::PCWSTR, ppnewauthidentity : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiExcludePackage(authidentity.unwrap_or(core::mem::zeroed()) as _, pszpackagename.param().abi(), ppnewauthidentity as _).ok() }
}
#[inline]
pub unsafe fn SspiFreeAuthIdentity(authdata: Option<*const core::ffi::c_void>) {
    windows_link::link!("secur32.dll" "system" fn SspiFreeAuthIdentity(authdata : *const core::ffi::c_void));
    unsafe { SspiFreeAuthIdentity(authdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SspiGetTargetHostName<P0>(psztargetname: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SspiGetTargetHostName(psztargetname : windows_core::PCWSTR, pszhostname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SspiGetTargetHostName(psztargetname.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SspiIsAuthIdentityEncrypted(encryptedauthdata: *const core::ffi::c_void) -> bool {
    windows_link::link!("secur32.dll" "system" fn SspiIsAuthIdentityEncrypted(encryptedauthdata : *const core::ffi::c_void) -> bool);
    unsafe { SspiIsAuthIdentityEncrypted(encryptedauthdata) }
}
#[inline]
pub unsafe fn SspiIsPromptingNeeded(errororntstatus: u32) -> bool {
    windows_link::link!("credui.dll" "system" fn SspiIsPromptingNeeded(errororntstatus : u32) -> bool);
    unsafe { SspiIsPromptingNeeded(errororntstatus) }
}
#[inline]
pub unsafe fn SspiLocalFree(databuffer: Option<*const core::ffi::c_void>) {
    windows_link::link!("secur32.dll" "system" fn SspiLocalFree(databuffer : *const core::ffi::c_void));
    unsafe { SspiLocalFree(databuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SspiMarshalAuthIdentity(authidentity: *const core::ffi::c_void, authidentitylength: *mut u32, authidentitybytearray: *mut *mut i8) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiMarshalAuthIdentity(authidentity : *const core::ffi::c_void, authidentitylength : *mut u32, authidentitybytearray : *mut *mut i8) -> windows_core::HRESULT);
    unsafe { SspiMarshalAuthIdentity(authidentity, authidentitylength as _, authidentitybytearray as _).ok() }
}
#[inline]
pub unsafe fn SspiPrepareForCredRead<P1>(authidentity: *const core::ffi::c_void, psztargetname: P1, pcredmancredentialtype: *mut u32, ppszcredmantargetname: *mut windows_core::PCWSTR) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SspiPrepareForCredRead(authidentity : *const core::ffi::c_void, psztargetname : windows_core::PCWSTR, pcredmancredentialtype : *mut u32, ppszcredmantargetname : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SspiPrepareForCredRead(authidentity, psztargetname.param().abi(), pcredmancredentialtype as _, ppszcredmantargetname as _).ok() }
}
#[inline]
pub unsafe fn SspiPrepareForCredWrite<P1>(authidentity: *const core::ffi::c_void, psztargetname: P1, pcredmancredentialtype: *mut u32, ppszcredmantargetname: *mut windows_core::PCWSTR, ppszcredmanusername: *mut windows_core::PCWSTR, ppcredentialblob: *mut *mut u8, pcredentialblobsize: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn SspiPrepareForCredWrite(authidentity : *const core::ffi::c_void, psztargetname : windows_core::PCWSTR, pcredmancredentialtype : *mut u32, ppszcredmantargetname : *mut windows_core::PCWSTR, ppszcredmanusername : *mut windows_core::PCWSTR, ppcredentialblob : *mut *mut u8, pcredentialblobsize : *mut u32) -> windows_core::HRESULT);
    unsafe { SspiPrepareForCredWrite(authidentity, psztargetname.param().abi(), pcredmancredentialtype as _, ppszcredmantargetname as _, ppszcredmanusername as _, ppcredentialblob as _, pcredentialblobsize as _).ok() }
}
#[inline]
pub unsafe fn SspiPromptForCredentialsA<P0, P3>(psztargetname: P0, puiinfo: Option<*const core::ffi::c_void>, dwautherror: u32, pszpackage: P3, pinputauthidentity: Option<*const core::ffi::c_void>, ppauthidentity: *mut *mut core::ffi::c_void, pfsave: Option<*mut i32>, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("credui.dll" "system" fn SspiPromptForCredentialsA(psztargetname : windows_core::PCSTR, puiinfo : *const core::ffi::c_void, dwautherror : u32, pszpackage : windows_core::PCSTR, pinputauthidentity : *const core::ffi::c_void, ppauthidentity : *mut *mut core::ffi::c_void, pfsave : *mut i32, dwflags : u32) -> u32);
    unsafe { SspiPromptForCredentialsA(psztargetname.param().abi(), puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pszpackage.param().abi(), pinputauthidentity.unwrap_or(core::mem::zeroed()) as _, ppauthidentity as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn SspiPromptForCredentialsW<P0, P3>(psztargetname: P0, puiinfo: Option<*const core::ffi::c_void>, dwautherror: u32, pszpackage: P3, pinputauthidentity: Option<*const core::ffi::c_void>, ppauthidentity: *mut *mut core::ffi::c_void, pfsave: Option<*mut i32>, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("credui.dll" "system" fn SspiPromptForCredentialsW(psztargetname : windows_core::PCWSTR, puiinfo : *const core::ffi::c_void, dwautherror : u32, pszpackage : windows_core::PCWSTR, pinputauthidentity : *const core::ffi::c_void, ppauthidentity : *mut *mut core::ffi::c_void, pfsave : *mut i32, dwflags : u32) -> u32);
    unsafe { SspiPromptForCredentialsW(psztargetname.param().abi(), puiinfo.unwrap_or(core::mem::zeroed()) as _, dwautherror, pszpackage.param().abi(), pinputauthidentity.unwrap_or(core::mem::zeroed()) as _, ppauthidentity as _, pfsave.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn SspiSetChannelBindingFlags(pbindings: *mut SecPkgContext_Bindings, flags: u32) -> windows_core::Result<()> {
    windows_link::link!("sspicli.dll" "system" fn SspiSetChannelBindingFlags(pbindings : *mut SecPkgContext_Bindings, flags : u32) -> windows_core::HRESULT);
    unsafe { SspiSetChannelBindingFlags(pbindings as _, flags).ok() }
}
#[inline]
pub unsafe fn SspiUnmarshalAuthIdentity(authidentitybytearray: &[u8], ppauthidentity: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiUnmarshalAuthIdentity(authidentitylength : u32, authidentitybytearray : windows_core::PCSTR, ppauthidentity : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiUnmarshalAuthIdentity(authidentitybytearray.len().try_into().unwrap(), core::mem::transmute(authidentitybytearray.as_ptr()), ppauthidentity as _).ok() }
}
#[inline]
pub unsafe fn SspiValidateAuthIdentity(authdata: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("secur32.dll" "system" fn SspiValidateAuthIdentity(authdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SspiValidateAuthIdentity(authdata).ok() }
}
#[inline]
pub unsafe fn SspiZeroAuthIdentity(authdata: Option<*const core::ffi::c_void>) {
    windows_link::link!("secur32.dll" "system" fn SspiZeroAuthIdentity(authdata : *const core::ffi::c_void));
    unsafe { SspiZeroAuthIdentity(authdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TokenBindingDeleteAllBindings() -> windows_core::Result<()> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingDeleteAllBindings() -> windows_core::HRESULT);
    unsafe { TokenBindingDeleteAllBindings().ok() }
}
#[inline]
pub unsafe fn TokenBindingDeleteBinding<P0>(targeturl: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingDeleteBinding(targeturl : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { TokenBindingDeleteBinding(targeturl.param().abi()).ok() }
}
#[inline]
pub unsafe fn TokenBindingGenerateBinding<P1>(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, targeturl: P1, bindingtype: TOKENBINDING_TYPE, tlsekm: *const core::ffi::c_void, tlsekmsize: u32, extensionformat: TOKENBINDING_EXTENSION_FORMAT, extensiondata: *const core::ffi::c_void, tokenbinding: *mut *mut core::ffi::c_void, tokenbindingsize: *mut u32, resultdata: Option<*mut *mut TOKENBINDING_RESULT_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGenerateBinding(keytype : TOKENBINDING_KEY_PARAMETERS_TYPE, targeturl : windows_core::PCWSTR, bindingtype : TOKENBINDING_TYPE, tlsekm : *const core::ffi::c_void, tlsekmsize : u32, extensionformat : TOKENBINDING_EXTENSION_FORMAT, extensiondata : *const core::ffi::c_void, tokenbinding : *mut *mut core::ffi::c_void, tokenbindingsize : *mut u32, resultdata : *mut *mut TOKENBINDING_RESULT_DATA) -> windows_core::HRESULT);
    unsafe { TokenBindingGenerateBinding(keytype, targeturl.param().abi(), bindingtype, tlsekm, tlsekmsize, extensionformat, extensiondata, tokenbinding as _, tokenbindingsize as _, resultdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn TokenBindingGenerateID(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, publickey: *const core::ffi::c_void, publickeysize: u32) -> windows_core::Result<*mut TOKENBINDING_RESULT_DATA> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGenerateID(keytype : TOKENBINDING_KEY_PARAMETERS_TYPE, publickey : *const core::ffi::c_void, publickeysize : u32, resultdata : *mut *mut TOKENBINDING_RESULT_DATA) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TokenBindingGenerateID(keytype, publickey, publickeysize, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn TokenBindingGenerateIDForUri<P1>(keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, targeturi: P1) -> windows_core::Result<*mut TOKENBINDING_RESULT_DATA>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGenerateIDForUri(keytype : TOKENBINDING_KEY_PARAMETERS_TYPE, targeturi : windows_core::PCWSTR, resultdata : *mut *mut TOKENBINDING_RESULT_DATA) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TokenBindingGenerateIDForUri(keytype, targeturi.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn TokenBindingGenerateMessage(tokenbindings: *const *const core::ffi::c_void, tokenbindingssize: *const u32, tokenbindingscount: u32, tokenbindingmessage: *mut *mut core::ffi::c_void, tokenbindingmessagesize: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGenerateMessage(tokenbindings : *const *const core::ffi::c_void, tokenbindingssize : *const u32, tokenbindingscount : u32, tokenbindingmessage : *mut *mut core::ffi::c_void, tokenbindingmessagesize : *mut u32) -> windows_core::HRESULT);
    unsafe { TokenBindingGenerateMessage(tokenbindings, tokenbindingssize, tokenbindingscount, tokenbindingmessage as _, tokenbindingmessagesize as _).ok() }
}
#[inline]
pub unsafe fn TokenBindingGetHighestSupportedVersion(majorversion: *mut u8, minorversion: *mut u8) -> windows_core::Result<()> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGetHighestSupportedVersion(majorversion : *mut u8, minorversion : *mut u8) -> windows_core::HRESULT);
    unsafe { TokenBindingGetHighestSupportedVersion(majorversion as _, minorversion as _).ok() }
}
#[inline]
pub unsafe fn TokenBindingGetKeyTypesClient() -> windows_core::Result<*mut TOKENBINDING_KEY_TYPES> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGetKeyTypesClient(keytypes : *mut *mut TOKENBINDING_KEY_TYPES) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TokenBindingGetKeyTypesClient(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn TokenBindingGetKeyTypesServer() -> windows_core::Result<*mut TOKENBINDING_KEY_TYPES> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingGetKeyTypesServer(keytypes : *mut *mut TOKENBINDING_KEY_TYPES) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TokenBindingGetKeyTypesServer(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn TokenBindingVerifyMessage(tokenbindingmessage: *const core::ffi::c_void, tokenbindingmessagesize: u32, keytype: TOKENBINDING_KEY_PARAMETERS_TYPE, tlsekm: *const core::ffi::c_void, tlsekmsize: u32) -> windows_core::Result<*mut TOKENBINDING_RESULT_LIST> {
    windows_link::link!("tokenbinding.dll" "system" fn TokenBindingVerifyMessage(tokenbindingmessage : *const core::ffi::c_void, tokenbindingmessagesize : u32, keytype : TOKENBINDING_KEY_PARAMETERS_TYPE, tlsekm : *const core::ffi::c_void, tlsekmsize : u32, resultlist : *mut *mut TOKENBINDING_RESULT_LIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TokenBindingVerifyMessage(tokenbindingmessage, tokenbindingmessagesize, keytype, tlsekm, tlsekmsize, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn TranslateNameA<P0>(lpaccountname: P0, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: Option<windows_core::PSTR>, nsize: *mut u32) -> bool
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("secur32.dll" "system" fn TranslateNameA(lpaccountname : windows_core::PCSTR, accountnameformat : EXTENDED_NAME_FORMAT, desirednameformat : EXTENDED_NAME_FORMAT, lptranslatedname : windows_core::PSTR, nsize : *mut u32) -> bool);
    unsafe { TranslateNameA(lpaccountname.param().abi(), accountnameformat, desirednameformat, lptranslatedname.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn TranslateNameW<P0>(lpaccountname: P0, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: Option<windows_core::PWSTR>, nsize: *mut u32) -> bool
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("secur32.dll" "system" fn TranslateNameW(lpaccountname : windows_core::PCWSTR, accountnameformat : EXTENDED_NAME_FORMAT, desirednameformat : EXTENDED_NAME_FORMAT, lptranslatedname : windows_core::PWSTR, nsize : *mut u32) -> bool);
    unsafe { TranslateNameW(lpaccountname.param().abi(), accountnameformat, desirednameformat, lptranslatedname.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(feature = "Win32_Security_Credentials")]
#[inline]
pub unsafe fn VerifySignature(phcontext: *const super::super::Credentials::SecHandle, pmessage: *const SecBufferDesc, messageseqno: u32) -> windows_core::Result<u32> {
    windows_link::link!("secur32.dll" "system" fn VerifySignature(phcontext : *const super::super::Credentials:: SecHandle, pmessage : *const SecBufferDesc, messageseqno : u32, pfqop : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VerifySignature(phcontext, pmessage, messageseqno, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security_Credentials")]
pub type ACCEPT_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut super::super::Credentials::SecHandle, param2: *mut SecBufferDesc, param3: u32, param4: u32, param5: *mut super::super::Credentials::SecHandle, param6: *mut SecBufferDesc, param7: *mut u32, param8: *mut i64) -> windows_core::HRESULT>;
pub const ACCOUNT_ADJUST_PRIVILEGES: i32 = 2i32;
pub const ACCOUNT_ADJUST_QUOTAS: i32 = 4i32;
pub const ACCOUNT_ADJUST_SYSTEM_ACCESS: i32 = 8i32;
pub const ACCOUNT_VIEW: i32 = 1i32;
#[cfg(feature = "Win32_Security_Credentials")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_A = Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut i8, param2: u32, param3: *mut core::ffi::c_void, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: *mut super::super::Credentials::SecHandle, param8: *mut i64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_W = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: u32, param3: *mut core::ffi::c_void, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: *mut super::super::Credentials::SecHandle, param8: *mut i64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type ADD_CREDENTIALS_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut i8, param2: *mut i8, param3: u32, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: *mut i64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type ADD_CREDENTIALS_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut u16, param2: *mut u16, param3: u32, param4: *mut core::ffi::c_void, param5: SEC_GET_KEY_FN, param6: *mut core::ffi::c_void, param7: *mut i64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type APPLY_CONTROL_TOKEN_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc) -> windows_core::HRESULT>;
pub const ASC_REQ_ALLOCATE_MEMORY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(256u32);
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(4194304u32);
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: ASC_REQ_FLAGS = ASC_REQ_FLAGS(268435456u32);
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: ASC_REQ_FLAGS = ASC_REQ_FLAGS(2097152u32);
pub const ASC_REQ_ALLOW_NULL_SESSION: ASC_REQ_FLAGS = ASC_REQ_FLAGS(1048576u32);
pub const ASC_REQ_CALL_LEVEL: ASC_REQ_FLAGS = ASC_REQ_FLAGS(4096u32);
pub const ASC_REQ_CONFIDENTIALITY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(16u32);
pub const ASC_REQ_CONNECTION: ASC_REQ_FLAGS = ASC_REQ_FLAGS(2048u32);
pub const ASC_REQ_DATAGRAM: ASC_REQ_FLAGS = ASC_REQ_FLAGS(1024u32);
pub const ASC_REQ_DELEGATE: ASC_REQ_FLAGS = ASC_REQ_FLAGS(1u32);
pub const ASC_REQ_EXTENDED_ERROR: ASC_REQ_FLAGS = ASC_REQ_FLAGS(32768u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASC_REQ_FLAGS(pub u32);
impl ASC_REQ_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ASC_REQ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ASC_REQ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ASC_REQ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ASC_REQ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ASC_REQ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ASC_REQ_FRAGMENT_SUPPLIED: ASC_REQ_FLAGS = ASC_REQ_FLAGS(8192u32);
pub const ASC_REQ_FRAGMENT_TO_FIT: ASC_REQ_FLAGS = ASC_REQ_FLAGS(8388608u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASC_REQ_HIGH_FLAGS(pub u64);
impl ASC_REQ_HIGH_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ASC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ASC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ASC_REQ_HIGH_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ASC_REQ_HIGH_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ASC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ASC_REQ_IDENTIFY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(524288u32);
pub const ASC_REQ_INTEGRITY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(131072u32);
pub const ASC_REQ_LICENSING: ASC_REQ_FLAGS = ASC_REQ_FLAGS(262144u32);
pub const ASC_REQ_MESSAGES: ASC_REQ_HIGH_FLAGS = ASC_REQ_HIGH_FLAGS(4294967296u64);
pub const ASC_REQ_MUTUAL_AUTH: ASC_REQ_FLAGS = ASC_REQ_FLAGS(2u32);
pub const ASC_REQ_NO_TOKEN: ASC_REQ_FLAGS = ASC_REQ_FLAGS(16777216u32);
pub const ASC_REQ_PROXY_BINDINGS: ASC_REQ_FLAGS = ASC_REQ_FLAGS(67108864u32);
pub const ASC_REQ_REPLAY_DETECT: ASC_REQ_FLAGS = ASC_REQ_FLAGS(4u32);
pub const ASC_REQ_SEQUENCE_DETECT: ASC_REQ_FLAGS = ASC_REQ_FLAGS(8u32);
pub const ASC_REQ_SESSION_TICKET: ASC_REQ_FLAGS = ASC_REQ_FLAGS(64u32);
pub const ASC_REQ_STREAM: ASC_REQ_FLAGS = ASC_REQ_FLAGS(65536u32);
pub const ASC_REQ_USE_DCE_STYLE: ASC_REQ_FLAGS = ASC_REQ_FLAGS(512u32);
pub const ASC_REQ_USE_SESSION_KEY: ASC_REQ_FLAGS = ASC_REQ_FLAGS(32u32);
pub const ASC_RET_ALLOCATED_MEMORY: u32 = 256u32;
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: u32 = 4194304u32;
pub const ASC_RET_ALLOW_NON_USER_LOGONS: u32 = 2097152u32;
pub const ASC_RET_CALL_LEVEL: u32 = 8192u32;
pub const ASC_RET_CONFIDENTIALITY: u32 = 16u32;
pub const ASC_RET_CONNECTION: u32 = 2048u32;
pub const ASC_RET_DATAGRAM: u32 = 1024u32;
pub const ASC_RET_DELEGATE: u32 = 1u32;
pub const ASC_RET_EXTENDED_ERROR: u32 = 32768u32;
pub const ASC_RET_FRAGMENT_ONLY: u32 = 8388608u32;
pub const ASC_RET_IDENTIFY: u32 = 524288u32;
pub const ASC_RET_INTEGRITY: u32 = 131072u32;
pub const ASC_RET_LICENSING: u32 = 262144u32;
pub const ASC_RET_MESSAGES: u64 = 4294967296u64;
pub const ASC_RET_MUTUAL_AUTH: u32 = 2u32;
pub const ASC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
pub const ASC_RET_NO_TOKEN: u32 = 16777216u32;
pub const ASC_RET_NULL_SESSION: u32 = 1048576u32;
pub const ASC_RET_REPLAY_DETECT: u32 = 4u32;
pub const ASC_RET_SEQUENCE_DETECT: u32 = 8u32;
pub const ASC_RET_SESSION_TICKET: u32 = 64u32;
pub const ASC_RET_STREAM: u32 = 65536u32;
pub const ASC_RET_THIRD_LEG_FAILED: u32 = 16384u32;
pub const ASC_RET_USED_DCE_STYLE: u32 = 512u32;
pub const ASC_RET_USE_SESSION_KEY: u32 = 32u32;
pub const AUDIT_ENUMERATE_USERS: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUDIT_POLICY_INFORMATION {
    pub AuditSubCategoryGuid: windows_core::GUID,
    pub AuditingInformation: u32,
    pub AuditCategoryGuid: windows_core::GUID,
}
pub const AUDIT_QUERY_MISC_POLICY: u32 = 64u32;
pub const AUDIT_QUERY_SYSTEM_POLICY: u32 = 2u32;
pub const AUDIT_QUERY_USER_POLICY: u32 = 8u32;
pub const AUDIT_SET_MISC_POLICY: u32 = 32u32;
pub const AUDIT_SET_SYSTEM_POLICY: u32 = 1u32;
pub const AUDIT_SET_USER_POLICY: u32 = 4u32;
pub const AUTH_REQ_ALLOW_ENC_TKT_IN_SKEY: u32 = 32u32;
pub const AUTH_REQ_ALLOW_FORWARDABLE: u32 = 1u32;
pub const AUTH_REQ_ALLOW_NOADDRESS: u32 = 16u32;
pub const AUTH_REQ_ALLOW_POSTDATE: u32 = 4u32;
pub const AUTH_REQ_ALLOW_PROXIABLE: u32 = 2u32;
pub const AUTH_REQ_ALLOW_RENEWABLE: u32 = 8u32;
pub const AUTH_REQ_ALLOW_S4U_DELEGATE: u32 = 2048u32;
pub const AUTH_REQ_ALLOW_VALIDATE: u32 = 64u32;
pub const AUTH_REQ_OK_AS_DELEGATE: u32 = 256u32;
pub const AUTH_REQ_PREAUTH_REQUIRED: u32 = 512u32;
pub const AUTH_REQ_TRANSITIVE_TRUST: u32 = 1024u32;
pub const AUTH_REQ_VALIDATE_CLIENT: u32 = 128u32;
pub const AccountDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = LSA_LOOKUP_DOMAIN_INFO_CLASS(5i32);
pub const AuditCategoryAccountLogon: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(8i32);
pub const AuditCategoryAccountManagement: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(6i32);
pub const AuditCategoryDetailedTracking: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(4i32);
pub const AuditCategoryDirectoryServiceAccess: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(7i32);
pub const AuditCategoryLogon: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(1i32);
pub const AuditCategoryObjectAccess: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(2i32);
pub const AuditCategoryPolicyChange: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(5i32);
pub const AuditCategoryPrivilegeUse: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(3i32);
pub const AuditCategorySystem: POLICY_AUDIT_EVENT_TYPE = POLICY_AUDIT_EVENT_TYPE(0i32);
pub const Audit_AccountLogon: windows_core::GUID = windows_core::GUID::from_u128(0x69979850_797a_11d9_bed3_505054503030);
pub const Audit_AccountLogon_CredentialValidation: windows_core::GUID = windows_core::GUID::from_u128(0x0cce923f_69ae_11d9_bed3_505054503030);
pub const Audit_AccountLogon_KerbCredentialValidation: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9242_69ae_11d9_bed3_505054503030);
pub const Audit_AccountLogon_Kerberos: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9240_69ae_11d9_bed3_505054503030);
pub const Audit_AccountLogon_Others: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9241_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement: windows_core::GUID = windows_core::GUID::from_u128(0x6997984e_797a_11d9_bed3_505054503030);
pub const Audit_AccountManagement_ApplicationGroup: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9239_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_ComputerAccount: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9236_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_DistributionGroup: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9238_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_Others: windows_core::GUID = windows_core::GUID::from_u128(0x0cce923a_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_SecurityGroup: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9237_69ae_11d9_bed3_505054503030);
pub const Audit_AccountManagement_UserAccount: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9235_69ae_11d9_bed3_505054503030);
pub const Audit_DSAccess_DSAccess: windows_core::GUID = windows_core::GUID::from_u128(0x0cce923b_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking: windows_core::GUID = windows_core::GUID::from_u128(0x6997984c_797a_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_DpapiActivity: windows_core::GUID = windows_core::GUID::from_u128(0x0cce922d_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_PnpActivity: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9248_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_ProcessCreation: windows_core::GUID = windows_core::GUID::from_u128(0x0cce922b_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_ProcessTermination: windows_core::GUID = windows_core::GUID::from_u128(0x0cce922c_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_RpcCall: windows_core::GUID = windows_core::GUID::from_u128(0x0cce922e_69ae_11d9_bed3_505054503030);
pub const Audit_DetailedTracking_TokenRightAdjusted: windows_core::GUID = windows_core::GUID::from_u128(0x0cce924a_69ae_11d9_bed3_505054503030);
pub const Audit_DirectoryServiceAccess: windows_core::GUID = windows_core::GUID::from_u128(0x6997984f_797a_11d9_bed3_505054503030);
pub const Audit_DsAccess_AdAuditChanges: windows_core::GUID = windows_core::GUID::from_u128(0x0cce923c_69ae_11d9_bed3_505054503030);
pub const Audit_Ds_DetailedReplication: windows_core::GUID = windows_core::GUID::from_u128(0x0cce923e_69ae_11d9_bed3_505054503030);
pub const Audit_Ds_Replication: windows_core::GUID = windows_core::GUID::from_u128(0x0cce923d_69ae_11d9_bed3_505054503030);
pub const Audit_Logon: windows_core::GUID = windows_core::GUID::from_u128(0x69979849_797a_11d9_bed3_505054503030);
pub const Audit_Logon_AccountLockout: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9217_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Claims: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9247_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Groups: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9249_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_IPSecMainMode: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9218_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_IPSecQuickMode: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9219_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_IPSecUserMode: windows_core::GUID = windows_core::GUID::from_u128(0x0cce921a_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Logoff: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9216_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Logon: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9215_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_NPS: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9243_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_Others: windows_core::GUID = windows_core::GUID::from_u128(0x0cce921c_69ae_11d9_bed3_505054503030);
pub const Audit_Logon_SpecialLogon: windows_core::GUID = windows_core::GUID::from_u128(0x0cce921b_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess: windows_core::GUID = windows_core::GUID::from_u128(0x6997984a_797a_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_ApplicationGenerated: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9222_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_CbacStaging: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9246_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_CertificationServices: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9221_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_DetailedFileShare: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9244_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_FileSystem: windows_core::GUID = windows_core::GUID::from_u128(0x0cce921d_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_FirewallConnection: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9226_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_FirewallPacketDrops: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9225_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Handle: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9223_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Kernel: windows_core::GUID = windows_core::GUID::from_u128(0x0cce921f_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Other: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9227_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Registry: windows_core::GUID = windows_core::GUID::from_u128(0x0cce921e_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_RemovableStorage: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9245_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Sam: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9220_69ae_11d9_bed3_505054503030);
pub const Audit_ObjectAccess_Share: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9224_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange: windows_core::GUID = windows_core::GUID::from_u128(0x6997984d_797a_11d9_bed3_505054503030);
pub const Audit_PolicyChange_AuditPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x0cce922f_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_AuthenticationPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9230_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_AuthorizationPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9231_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_MpsscvRulePolicy: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9232_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_Others: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9234_69ae_11d9_bed3_505054503030);
pub const Audit_PolicyChange_WfpIPSecPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9233_69ae_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse: windows_core::GUID = windows_core::GUID::from_u128(0x6997984b_797a_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse_NonSensitive: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9229_69ae_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse_Others: windows_core::GUID = windows_core::GUID::from_u128(0x0cce922a_69ae_11d9_bed3_505054503030);
pub const Audit_PrivilegeUse_Sensitive: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9228_69ae_11d9_bed3_505054503030);
pub const Audit_System: windows_core::GUID = windows_core::GUID::from_u128(0x69979848_797a_11d9_bed3_505054503030);
pub const Audit_System_IPSecDriverEvents: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9213_69ae_11d9_bed3_505054503030);
pub const Audit_System_Integrity: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9212_69ae_11d9_bed3_505054503030);
pub const Audit_System_Others: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9214_69ae_11d9_bed3_505054503030);
pub const Audit_System_SecurityStateChange: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9210_69ae_11d9_bed3_505054503030);
pub const Audit_System_SecuritySubsystemExtension: windows_core::GUID = windows_core::GUID::from_u128(0x0cce9211_69ae_11d9_bed3_505054503030);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CENTRAL_ACCESS_POLICY {
    pub CAPID: super::super::PSID,
    pub Name: LSA_UNICODE_STRING,
    pub Description: LSA_UNICODE_STRING,
    pub ChangeId: LSA_UNICODE_STRING,
    pub Flags: u32,
    pub CAPECount: u32,
    pub CAPEs: *mut *mut CENTRAL_ACCESS_POLICY_ENTRY,
}
impl Default for CENTRAL_ACCESS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CENTRAL_ACCESS_POLICY_ENTRY {
    pub Name: LSA_UNICODE_STRING,
    pub Description: LSA_UNICODE_STRING,
    pub ChangeId: LSA_UNICODE_STRING,
    pub LengthAppliesTo: u32,
    pub AppliesTo: *mut u8,
    pub LengthSD: u32,
    pub SD: super::super::PSECURITY_DESCRIPTOR,
    pub LengthStagedSD: u32,
    pub StagedSD: super::super::PSECURITY_DESCRIPTOR,
    pub Flags: u32,
}
impl Default for CENTRAL_ACCESS_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: u32 = 1u32;
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: u32 = 65536u32;
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: u32 = 256u32;
pub type CHANGE_PASSWORD_FN_A = Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut i8, param2: *mut i8, param3: *mut i8, param4: *mut i8, param5: bool, param6: u32, param7: *mut SecBufferDesc) -> windows_core::HRESULT>;
pub type CHANGE_PASSWORD_FN_W = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: *mut u16, param3: *mut u16, param4: *mut u16, param5: bool, param6: u32, param7: *mut SecBufferDesc) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLEAR_BLOCK {
    pub data: [i8; 8],
}
impl Default for CLEAR_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLEAR_BLOCK_LENGTH: u32 = 8u32;
pub const CLOUDAP_NAME: windows_core::PCWSTR = windows_core::w!("CloudAP");
pub const CLOUDAP_NAME_W: windows_core::PCWSTR = windows_core::w!("CloudAP");
#[cfg(feature = "Win32_Security_Credentials")]
pub type COMPLETE_AUTH_TOKEN_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc) -> windows_core::HRESULT>;
pub const CREDP_FLAGS_CLEAR_PASSWORD: u32 = 8u32;
pub const CREDP_FLAGS_DONT_CACHE_TI: u32 = 4u32;
pub const CREDP_FLAGS_IN_PROCESS: u32 = 1u32;
pub const CREDP_FLAGS_TRUSTED_CALLER: u32 = 32u32;
pub const CREDP_FLAGS_USER_ENCRYPTED_PASSWORD: u32 = 16u32;
pub const CREDP_FLAGS_USE_MIDL_HEAP: u32 = 2u32;
pub const CREDP_FLAGS_VALIDATE_PROXY_TARGET: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRED_FETCH(pub i32);
pub const CRED_MARSHALED_TI_SIZE_SIZE: u32 = 12u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTO_SETTINGS {
    pub eAlgorithmUsage: eTlsAlgorithmUsage,
    pub strCngAlgId: LSA_UNICODE_STRING,
    pub cChainingModes: u32,
    pub rgstrChainingModes: *mut LSA_UNICODE_STRING,
    pub dwMinBitLength: u32,
    pub dwMaxBitLength: u32,
}
impl Default for CRYPTO_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CYPHER_BLOCK_LENGTH: u32 = 8u32;
pub const CertHashInfo: KERB_CERTIFICATE_INFO_TYPE = KERB_CERTIFICATE_INFO_TYPE(1i32);
pub const ClOUDAP_NAME_A: windows_core::PCSTR = windows_core::s!("CloudAP");
pub const CollisionOther: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(2i32);
pub const CollisionTdo: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(0i32);
pub const CollisionXref: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE = LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(1i32);
pub const CredFetchDPAPI: CRED_FETCH = CRED_FETCH(1i32);
pub const CredFetchDefault: CRED_FETCH = CRED_FETCH(0i32);
pub const CredFetchForced: CRED_FETCH = CRED_FETCH(2i32);
#[cfg(feature = "Win32_Security_Credentials")]
pub type CredFreeCredentialsFn = Option<unsafe extern "system" fn(count: u32, credentials: *mut *mut ENCRYPTED_CREDENTIALW)>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type CredReadDomainCredentialsFn = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, credflags: u32, targetinfo: *const super::super::Credentials::CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut ENCRYPTED_CREDENTIALW) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type CredReadFn = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, credflags: u32, targetname: windows_core::PCWSTR, r#type: u32, flags: u32, credential: *mut *mut ENCRYPTED_CREDENTIALW) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type CredWriteFn = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, credflags: u32, credential: *const ENCRYPTED_CREDENTIALW, flags: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type CrediUnmarshalandDecodeStringFn = Option<unsafe extern "system" fn(marshaledstring: windows_core::PCWSTR, blob: *mut *mut u8, blobsize: *mut u32, isfailurefatal: *mut u8) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type DECRYPT_MESSAGE_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> windows_core::HRESULT>;
pub const DEFAULT_TLS_SSP_NAME: windows_core::PCWSTR = windows_core::w!("Default TLS SSP");
pub const DEFAULT_TLS_SSP_NAME_A: windows_core::PCSTR = windows_core::s!("Default TLS SSP");
pub const DEFAULT_TLS_SSP_NAME_W: windows_core::PCWSTR = windows_core::w!("Default TLS SSP");
#[cfg(feature = "Win32_Security_Credentials")]
pub type DELETE_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> windows_core::HRESULT>;
pub const DOMAIN_LOCKOUT_ADMINS: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(8u32);
pub const DOMAIN_NO_LM_OWF_CHANGE: i32 = 64i32;
pub const DOMAIN_PASSWORD_COMPLEX: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOMAIN_PASSWORD_INFORMATION {
    pub MinPasswordLength: u16,
    pub PasswordHistoryLength: u16,
    pub PasswordProperties: DOMAIN_PASSWORD_PROPERTIES,
    pub MaxPasswordAge: i64,
    pub MinPasswordAge: i64,
}
pub const DOMAIN_PASSWORD_NO_ANON_CHANGE: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(2u32);
pub const DOMAIN_PASSWORD_NO_CLEAR_CHANGE: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOMAIN_PASSWORD_PROPERTIES(pub u32);
impl DOMAIN_PASSWORD_PROPERTIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DOMAIN_PASSWORD_PROPERTIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DOMAIN_PASSWORD_PROPERTIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DOMAIN_PASSWORD_PROPERTIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DOMAIN_PASSWORD_PROPERTIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DOMAIN_PASSWORD_PROPERTIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DOMAIN_PASSWORD_STORE_CLEARTEXT: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(16u32);
pub const DOMAIN_REFUSE_PASSWORD_CHANGE: DOMAIN_PASSWORD_PROPERTIES = DOMAIN_PASSWORD_PROPERTIES(32u32);
pub const DS_INET_ADDRESS: KERB_ADDRESS_TYPE = KERB_ADDRESS_TYPE(1u32);
pub const DS_NETBIOS_ADDRESS: KERB_ADDRESS_TYPE = KERB_ADDRESS_TYPE(2u32);
pub const DS_UNKNOWN_ADDRESS_TYPE: u32 = 0u32;
pub const DeprecatedIUMCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(1i32);
pub const DnsDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = LSA_LOOKUP_DOMAIN_INFO_CLASS(12i32);
pub const DomainUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(2i32);
pub const ENABLE_TLS_CLIENT_EARLY_START: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Credentials")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENCRYPTED_CREDENTIALW {
    pub Cred: super::super::Credentials::CREDENTIALW,
    pub ClearCredentialBlobSize: u32,
}
#[cfg(feature = "Win32_Security_Credentials")]
pub type ENCRYPT_MESSAGE_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> windows_core::HRESULT>;
pub type ENUMERATE_SECURITY_PACKAGES_FN_A = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut *mut SecPkgInfoA) -> windows_core::HRESULT>;
pub type ENUMERATE_SECURITY_PACKAGES_FN_W = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut *mut SecPkgInfoW) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPORT_SECURITY_CONTEXT_FLAGS(pub u32);
impl EXPORT_SECURITY_CONTEXT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Security_Credentials")]
pub type EXPORT_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut SecBuffer, param3: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXTENDED_NAME_FORMAT(pub i32);
pub const E_RM_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004FC03_u32 as _);
pub const ExternallySuppliedCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(4i32);
pub const FACILITY_SL_ITF: u32 = 4u32;
pub type FREE_CONTEXT_BUFFER_FN = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type FREE_CREDENTIALS_HANDLE_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> windows_core::HRESULT>;
pub const ForestTrustBinaryInfo: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(3i32);
pub const ForestTrustDomainInfo: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(2i32);
pub const ForestTrustRecordTypeLast: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(4i32);
pub const ForestTrustScannerInfo: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(4i32);
pub const ForestTrustTopLevelName: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(0i32);
pub const ForestTrustTopLevelNameEx: LSA_FOREST_TRUST_RECORD_TYPE = LSA_FOREST_TRUST_RECORD_TYPE(1i32);
windows_core::imp::define_interface!(ICcgDomainAuthCredentials, ICcgDomainAuthCredentials_Vtbl, 0x6ecda518_2010_4437_8bc3_46e752b7b172);
windows_core::imp::interface_hierarchy!(ICcgDomainAuthCredentials, windows_core::IUnknown);
impl ICcgDomainAuthCredentials {
    pub unsafe fn GetPasswordCredentials<P0>(&self, plugininput: P0, domainname: *mut windows_core::PWSTR, username: *mut windows_core::PWSTR, password: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPasswordCredentials)(windows_core::Interface::as_raw(self), plugininput.param().abi(), domainname as _, username as _, password as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICcgDomainAuthCredentials_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPasswordCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ICcgDomainAuthCredentials_Impl: windows_core::IUnknownImpl {
    fn GetPasswordCredentials(&self, plugininput: &windows_core::PCWSTR, domainname: *mut windows_core::PWSTR, username: *mut windows_core::PWSTR, password: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl ICcgDomainAuthCredentials_Vtbl {
    pub const fn new<Identity: ICcgDomainAuthCredentials_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPasswordCredentials<Identity: ICcgDomainAuthCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugininput: windows_core::PCWSTR, domainname: *mut windows_core::PWSTR, username: *mut windows_core::PWSTR, password: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICcgDomainAuthCredentials_Impl::GetPasswordCredentials(this, core::mem::transmute(&plugininput), core::mem::transmute_copy(&domainname), core::mem::transmute_copy(&username), core::mem::transmute_copy(&password)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPasswordCredentials: GetPasswordCredentials::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICcgDomainAuthCredentials as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICcgDomainAuthCredentials {}
pub const ID_CAP_SLAPI: windows_core::PCWSTR = windows_core::w!("slapiQueryLicenseValue");
#[cfg(feature = "Win32_Security_Credentials")]
pub type IMPERSONATE_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type IMPORT_SECURITY_CONTEXT_FN_A = Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut SecBuffer, param2: *mut core::ffi::c_void, param3: *mut super::super::Credentials::SecHandle) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type IMPORT_SECURITY_CONTEXT_FN_W = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut SecBuffer, param2: *mut core::ffi::c_void, param3: *mut super::super::Credentials::SecHandle) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut super::super::Credentials::SecHandle, param2: *mut i8, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: *mut super::super::Credentials::SecHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut i64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type INITIALIZE_SECURITY_CONTEXT_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut super::super::Credentials::SecHandle, param2: *mut u16, param3: u32, param4: u32, param5: u32, param6: *mut SecBufferDesc, param7: u32, param8: *mut super::super::Credentials::SecHandle, param9: *mut SecBufferDesc, param10: *mut u32, param11: *mut i64) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type INIT_SECURITY_INTERFACE_A = Option<unsafe extern "system" fn() -> *mut SecurityFunctionTableA>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type INIT_SECURITY_INTERFACE_W = Option<unsafe extern "system" fn() -> *mut SecurityFunctionTableW>;
pub const ISC_REQ_ALLOCATE_MEMORY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(256u32);
pub const ISC_REQ_CALL_LEVEL: ISC_REQ_FLAGS = ISC_REQ_FLAGS(4096u32);
pub const ISC_REQ_CONFIDENTIALITY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(16u32);
pub const ISC_REQ_CONFIDENTIALITY_ONLY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1073741824u32);
pub const ISC_REQ_CONNECTION: ISC_REQ_FLAGS = ISC_REQ_FLAGS(2048u32);
pub const ISC_REQ_DATAGRAM: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1024u32);
pub const ISC_REQ_DEFERRED_CRED_VALIDATION: ISC_REQ_HIGH_FLAGS = ISC_REQ_HIGH_FLAGS(8589934592u64);
pub const ISC_REQ_DELEGATE: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1u32);
pub const ISC_REQ_EXTENDED_ERROR: ISC_REQ_FLAGS = ISC_REQ_FLAGS(16384u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ISC_REQ_FLAGS(pub u32);
impl ISC_REQ_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ISC_REQ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ISC_REQ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ISC_REQ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ISC_REQ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ISC_REQ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ISC_REQ_FORWARD_CREDENTIALS: ISC_REQ_FLAGS = ISC_REQ_FLAGS(4194304u32);
pub const ISC_REQ_FRAGMENT_SUPPLIED: ISC_REQ_FLAGS = ISC_REQ_FLAGS(8192u32);
pub const ISC_REQ_FRAGMENT_TO_FIT: ISC_REQ_FLAGS = ISC_REQ_FLAGS(2097152u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ISC_REQ_HIGH_FLAGS(pub u64);
impl ISC_REQ_HIGH_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ISC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ISC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ISC_REQ_HIGH_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ISC_REQ_HIGH_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ISC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ISC_REQ_IDENTIFY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(131072u32);
pub const ISC_REQ_INTEGRITY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(65536u32);
pub const ISC_REQ_MANUAL_CRED_VALIDATION: ISC_REQ_FLAGS = ISC_REQ_FLAGS(524288u32);
pub const ISC_REQ_MESSAGES: ISC_REQ_HIGH_FLAGS = ISC_REQ_HIGH_FLAGS(4294967296u64);
pub const ISC_REQ_MUTUAL_AUTH: ISC_REQ_FLAGS = ISC_REQ_FLAGS(2u32);
pub const ISC_REQ_NO_INTEGRITY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(8388608u32);
pub const ISC_REQ_NO_POST_HANDSHAKE_AUTH: ISC_REQ_HIGH_FLAGS = ISC_REQ_HIGH_FLAGS(17179869184u64);
pub const ISC_REQ_NULL_SESSION: ISC_REQ_FLAGS = ISC_REQ_FLAGS(262144u32);
pub const ISC_REQ_PROMPT_FOR_CREDS: ISC_REQ_FLAGS = ISC_REQ_FLAGS(64u32);
pub const ISC_REQ_REPLAY_DETECT: ISC_REQ_FLAGS = ISC_REQ_FLAGS(4u32);
pub const ISC_REQ_RESERVED1: ISC_REQ_FLAGS = ISC_REQ_FLAGS(1048576u32);
pub const ISC_REQ_SEQUENCE_DETECT: ISC_REQ_FLAGS = ISC_REQ_FLAGS(8u32);
pub const ISC_REQ_STREAM: ISC_REQ_FLAGS = ISC_REQ_FLAGS(32768u32);
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: ISC_REQ_FLAGS = ISC_REQ_FLAGS(536870912u32);
pub const ISC_REQ_USE_DCE_STYLE: ISC_REQ_FLAGS = ISC_REQ_FLAGS(512u32);
pub const ISC_REQ_USE_HTTP_STYLE: ISC_REQ_FLAGS = ISC_REQ_FLAGS(16777216u32);
pub const ISC_REQ_USE_SESSION_KEY: ISC_REQ_FLAGS = ISC_REQ_FLAGS(32u32);
pub const ISC_REQ_USE_SUPPLIED_CREDS: ISC_REQ_FLAGS = ISC_REQ_FLAGS(128u32);
pub const ISC_RET_ALLOCATED_MEMORY: u32 = 256u32;
pub const ISC_RET_CALL_LEVEL: u32 = 8192u32;
pub const ISC_RET_CONFIDENTIALITY: u32 = 16u32;
pub const ISC_RET_CONFIDENTIALITY_ONLY: u32 = 1073741824u32;
pub const ISC_RET_CONNECTION: u32 = 2048u32;
pub const ISC_RET_DATAGRAM: u32 = 1024u32;
pub const ISC_RET_DEFERRED_CRED_VALIDATION: u64 = 8589934592u64;
pub const ISC_RET_DELEGATE: u32 = 1u32;
pub const ISC_RET_EXTENDED_ERROR: u32 = 16384u32;
pub const ISC_RET_FORWARD_CREDENTIALS: u32 = 4194304u32;
pub const ISC_RET_FRAGMENT_ONLY: u32 = 2097152u32;
pub const ISC_RET_IDENTIFY: u32 = 131072u32;
pub const ISC_RET_INTEGRITY: u32 = 65536u32;
pub const ISC_RET_INTERMEDIATE_RETURN: u32 = 4096u32;
pub const ISC_RET_MANUAL_CRED_VALIDATION: u32 = 524288u32;
pub const ISC_RET_MESSAGES: u64 = 4294967296u64;
pub const ISC_RET_MUTUAL_AUTH: u32 = 2u32;
pub const ISC_RET_NO_ADDITIONAL_TOKEN: u32 = 33554432u32;
pub const ISC_RET_NO_POST_HANDSHAKE_AUTH: u64 = 17179869184u64;
pub const ISC_RET_NULL_SESSION: u32 = 262144u32;
pub const ISC_RET_REAUTHENTICATION: u32 = 134217728u32;
pub const ISC_RET_REPLAY_DETECT: u32 = 4u32;
pub const ISC_RET_RESERVED1: u32 = 1048576u32;
pub const ISC_RET_SEQUENCE_DETECT: u32 = 8u32;
pub const ISC_RET_STREAM: u32 = 32768u32;
pub const ISC_RET_USED_COLLECTED_CREDS: u32 = 64u32;
pub const ISC_RET_USED_DCE_STYLE: u32 = 512u32;
pub const ISC_RET_USED_HTTP_STYLE: u32 = 16777216u32;
pub const ISC_RET_USED_SUPPLIED_CREDS: u32 = 128u32;
pub const ISC_RET_USE_SESSION_KEY: u32 = 32u32;
pub const ISSP_LEVEL: u32 = 32u32;
pub const ISSP_MODE: u32 = 1u32;
pub const InvalidCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KDC_PROXY_CACHE_ENTRY_DATA {
    pub SinceLastUsed: u64,
    pub DomainName: LSA_UNICODE_STRING,
    pub ProxyServerName: LSA_UNICODE_STRING,
    pub ProxyServerVdir: LSA_UNICODE_STRING,
    pub ProxyServerPort: u16,
    pub LogonId: super::super::super::Foundation::LUID,
    pub CredUserName: LSA_UNICODE_STRING,
    pub CredDomainName: LSA_UNICODE_STRING,
    pub GlobalCache: bool,
}
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: u32 = 1u32;
pub const KDC_PROXY_SETTINGS_V1: u32 = 1u32;
pub const KERBEROS_REVISION: u32 = 6u32;
pub const KERBEROS_VERSION: u32 = 5u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_ADDRESS_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: LSA_UNICODE_STRING,
    pub KdcAddress: LSA_UNICODE_STRING,
    pub AddressType: KERB_ADDRESS_TYPE,
    pub DcFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub RealmName: LSA_UNICODE_STRING,
    pub KdcAddress: LSA_UNICODE_STRING,
    pub AddressType: KERB_ADDRESS_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_ADD_CREDENTIALS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub UserName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: KERB_REQUEST_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_ADD_CREDENTIALS_REQUEST_EX {
    pub Credentials: KERB_ADD_CREDENTIALS_REQUEST,
    pub PrincipalNameCount: u32,
    pub PrincipalNames: [LSA_UNICODE_STRING; 1],
}
impl Default for KERB_ADD_CREDENTIALS_REQUEST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_AUTH_DATA {
    pub Type: u32,
    pub Length: u32,
    pub Data: *mut u8,
}
impl Default for KERB_AUTH_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_BINDING_CACHE_ENTRY_DATA {
    pub DiscoveryTime: u64,
    pub RealmName: LSA_UNICODE_STRING,
    pub KdcAddress: LSA_UNICODE_STRING,
    pub AddressType: KERB_ADDRESS_TYPE,
    pub Flags: u32,
    pub DcFlags: u32,
    pub CacheFlags: u32,
    pub KdcName: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CERTIFICATE_HASHINFO {
    pub StoreNameLength: u16,
    pub HashLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CERTIFICATE_INFO {
    pub CertInfoSize: u32,
    pub InfoType: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_CERTIFICATE_INFO_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_CERTIFICATE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub DomainName: LSA_UNICODE_STRING,
    pub UserName: LSA_UNICODE_STRING,
    pub Pin: LSA_UNICODE_STRING,
    pub Flags: u32,
    pub CspDataLength: u32,
    pub CspData: *mut u8,
}
impl Default for KERB_CERTIFICATE_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
pub const KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_CERTIFICATE_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub CertificateLength: u32,
    pub Certificate: *mut u8,
}
impl Default for KERB_CERTIFICATE_S4U_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED: u32 = 4u32;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CERTIFICATE_UNLOCK_LOGON {
    pub Logon: KERB_CERTIFICATE_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CHANGEPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: LSA_UNICODE_STRING,
    pub AccountName: LSA_UNICODE_STRING,
    pub OldPassword: LSA_UNICODE_STRING,
    pub NewPassword: LSA_UNICODE_STRING,
    pub Impersonating: bool,
}
pub const KERB_CHECKSUM_CRC32: u32 = 1u32;
pub const KERB_CHECKSUM_DES_MAC: i32 = -133i32;
pub const KERB_CHECKSUM_DES_MAC_MD5: i32 = -134i32;
pub const KERB_CHECKSUM_HMAC_MD5: i32 = -138i32;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128: u32 = 15u32;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki: i32 = -150i32;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256: u32 = 16u32;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki: i32 = -151i32;
pub const KERB_CHECKSUM_KRB_DES_MAC: u32 = 4u32;
pub const KERB_CHECKSUM_KRB_DES_MAC_K: u32 = 5u32;
pub const KERB_CHECKSUM_LM: i32 = -130i32;
pub const KERB_CHECKSUM_MD25: i32 = -135i32;
pub const KERB_CHECKSUM_MD4: u32 = 2u32;
pub const KERB_CHECKSUM_MD5: u32 = 7u32;
pub const KERB_CHECKSUM_MD5_DES: u32 = 8u32;
pub const KERB_CHECKSUM_MD5_HMAC: i32 = -137i32;
pub const KERB_CHECKSUM_NONE: u32 = 0u32;
pub const KERB_CHECKSUM_RC4_MD5: i32 = -136i32;
pub const KERB_CHECKSUM_REAL_CRC32: i32 = -132i32;
pub const KERB_CHECKSUM_SHA1: i32 = -131i32;
pub const KERB_CHECKSUM_SHA1_NEW: u32 = 14u32;
pub const KERB_CHECKSUM_SHA256: i32 = -139i32;
pub const KERB_CHECKSUM_SHA384: i32 = -140i32;
pub const KERB_CHECKSUM_SHA512: i32 = -141i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_DATA {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    pub _bitfield: u32,
}
pub const KERB_CLOUD_KERBEROS_DEBUG_DATA_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Version: u32,
    pub Length: u32,
    pub Data: [u32; 1],
}
impl Default for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_CRYPTO_KEY {
    pub KeyType: KERB_CRYPTO_KEY_TYPE,
    pub Length: u32,
    pub Value: *mut u8,
}
impl Default for KERB_CRYPTO_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_CRYPTO_KEY32 {
    pub KeyType: i32,
    pub Length: u32,
    pub Offset: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_CRYPTO_KEY_TYPE(pub i32);
pub const KERB_DECRYPT_FLAG_DEFAULT_KEY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_DECRYPT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
    pub CryptoType: i32,
    pub KeyUsage: i32,
    pub Key: KERB_CRYPTO_KEY,
    pub EncryptedDataSize: u32,
    pub InitialVectorSize: u32,
    pub InitialVector: *mut u8,
    pub EncryptedData: *mut u8,
}
impl Default for KERB_DECRYPT_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_DECRYPT_RESPONSE {
    pub DecryptedData: [u8; 1],
}
impl Default for KERB_DECRYPT_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96: u32 = 17u32;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN: i32 = -148i32;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96: u32 = 18u32;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN: i32 = -149i32;
pub const KERB_ETYPE_DEFAULT: u32 = 0u32;
pub const KERB_ETYPE_DES3_CBC_MD5: u32 = 5u32;
pub const KERB_ETYPE_DES3_CBC_SHA1: u32 = 7u32;
pub const KERB_ETYPE_DES3_CBC_SHA1_KD: u32 = 16u32;
pub const KERB_ETYPE_DES_CBC_CRC: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(1i32);
pub const KERB_ETYPE_DES_CBC_MD4: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(2i32);
pub const KERB_ETYPE_DES_CBC_MD5: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(3i32);
pub const KERB_ETYPE_DES_CBC_MD5_NT: u32 = 20u32;
pub const KERB_ETYPE_DES_EDE3_CBC_ENV: u32 = 15u32;
pub const KERB_ETYPE_DES_PLAIN: i32 = -132i32;
pub const KERB_ETYPE_DSA_SHA1_CMS: u32 = 9u32;
pub const KERB_ETYPE_DSA_SIGN: u32 = 8u32;
pub const KERB_ETYPE_NULL: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(0i32);
pub const KERB_ETYPE_PKCS7_PUB: u32 = 13u32;
pub const KERB_ETYPE_RC2_CBC_ENV: u32 = 12u32;
pub const KERB_ETYPE_RC4_HMAC_NT: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(23i32);
pub const KERB_ETYPE_RC4_HMAC_NT_EXP: u32 = 24u32;
pub const KERB_ETYPE_RC4_HMAC_OLD: i32 = -133i32;
pub const KERB_ETYPE_RC4_HMAC_OLD_EXP: i32 = -135i32;
pub const KERB_ETYPE_RC4_LM: i32 = -130i32;
pub const KERB_ETYPE_RC4_MD4: KERB_CRYPTO_KEY_TYPE = KERB_CRYPTO_KEY_TYPE(-128i32);
pub const KERB_ETYPE_RC4_PLAIN: i32 = -140i32;
pub const KERB_ETYPE_RC4_PLAIN2: i32 = -129i32;
pub const KERB_ETYPE_RC4_PLAIN_EXP: i32 = -141i32;
pub const KERB_ETYPE_RC4_PLAIN_OLD: i32 = -134i32;
pub const KERB_ETYPE_RC4_PLAIN_OLD_EXP: i32 = -136i32;
pub const KERB_ETYPE_RC4_SHA: i32 = -131i32;
pub const KERB_ETYPE_RSA_ENV: u32 = 13u32;
pub const KERB_ETYPE_RSA_ES_OEAP_ENV: u32 = 14u32;
pub const KERB_ETYPE_RSA_MD5_CMS: u32 = 10u32;
pub const KERB_ETYPE_RSA_PRIV: u32 = 9u32;
pub const KERB_ETYPE_RSA_PUB: u32 = 10u32;
pub const KERB_ETYPE_RSA_PUB_MD5: u32 = 11u32;
pub const KERB_ETYPE_RSA_PUB_SHA1: u32 = 12u32;
pub const KERB_ETYPE_RSA_SHA1_CMS: u32 = 11u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_EXTERNAL_NAME {
    pub NameType: i16,
    pub NameCount: u16,
    pub Names: [LSA_UNICODE_STRING; 1],
}
impl Default for KERB_EXTERNAL_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_EXTERNAL_TICKET {
    pub ServiceName: *mut KERB_EXTERNAL_NAME,
    pub TargetName: *mut KERB_EXTERNAL_NAME,
    pub ClientName: *mut KERB_EXTERNAL_NAME,
    pub DomainName: LSA_UNICODE_STRING,
    pub TargetDomainName: LSA_UNICODE_STRING,
    pub AltTargetDomainName: LSA_UNICODE_STRING,
    pub SessionKey: KERB_CRYPTO_KEY,
    pub TicketFlags: KERB_TICKET_FLAGS,
    pub Flags: u32,
    pub KeyExpirationTime: i64,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewUntil: i64,
    pub TimeSkew: i64,
    pub EncodedTicketSize: u32,
    pub EncodedTicket: *mut u8,
}
impl Default for KERB_EXTERNAL_TICKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_INTERACTIVE_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub UserName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_INTERACTIVE_PROFILE {
    pub MessageType: KERB_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: LSA_UNICODE_STRING,
    pub HomeDirectory: LSA_UNICODE_STRING,
    pub FullName: LSA_UNICODE_STRING,
    pub ProfilePath: LSA_UNICODE_STRING,
    pub HomeDirectoryDrive: LSA_UNICODE_STRING,
    pub LogonServer: LSA_UNICODE_STRING,
    pub UserFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_INTERACTIVE_UNLOCK_LOGON {
    pub Logon: KERB_INTERACTIVE_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
pub const KERB_LOGON_FLAG_ALLOW_EXPIRED_TICKET: u32 = 1u32;
pub const KERB_LOGON_FLAG_REDIRECTED: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_LOGON_SUBMIT_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_NET_ADDRESS {
    pub Family: u32,
    pub Length: u32,
    pub Address: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_NET_ADDRESSES {
    pub Number: u32,
    pub Addresses: [KERB_NET_ADDRESS; 1],
}
impl Default for KERB_NET_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_PROFILE_BUFFER_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_PROTOCOL_MESSAGE_TYPE(pub i32);
pub const KERB_PURGE_ALL_TICKETS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_PURGE_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfPurged: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_PURGE_TKT_CACHE_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
    pub TicketTemplate: KERB_TICKET_CACHE_INFO_EX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_PURGE_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub ServerName: LSA_UNICODE_STRING,
    pub RealmName: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_QUERY_BINDING_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_BINDING_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: u32,
    pub Entries: *mut KERB_BINDING_CACHE_ENTRY_DATA,
}
impl Default for KERB_QUERY_BINDING_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub DomainName: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub ExtendedPolicies: u32,
    pub DsFlags: u32,
}
pub const KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE_FLAG_DAC_DISABLED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfEntries: u32,
    pub Entries: *mut KDC_PROXY_CACHE_ENTRY_DATA,
}
impl Default for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfCreds: u32,
    pub Creds: *mut KERB_S4U2PROXY_CRED,
}
impl Default for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX2; 1],
}
impl Default for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX3; 1],
}
impl Default for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO_EX; 1],
}
impl Default for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_QUERY_TKT_CACHE_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_QUERY_TKT_CACHE_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CountOfTickets: u32,
    pub Tickets: [KERB_TICKET_CACHE_INFO; 1],
}
impl Default for KERB_QUERY_TKT_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_REFRESH_POLICY_KDC: u32 = 2u32;
pub const KERB_REFRESH_POLICY_KERBEROS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_REFRESH_POLICY_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_REFRESH_POLICY_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
}
pub const KERB_REFRESH_SCCRED_GETTGT: u32 = 1u32;
pub const KERB_REFRESH_SCCRED_RELEASE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_REFRESH_SCCRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub CredentialBlob: LSA_UNICODE_STRING,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
}
pub const KERB_REQUEST_ADD_CREDENTIAL: KERB_REQUEST_FLAGS = KERB_REQUEST_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_REQUEST_FLAGS(pub u32);
pub const KERB_REQUEST_REMOVE_CREDENTIAL: KERB_REQUEST_FLAGS = KERB_REQUEST_FLAGS(4u32);
pub const KERB_REQUEST_REPLACE_CREDENTIAL: KERB_REQUEST_FLAGS = KERB_REQUEST_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_RETRIEVE_KEY_TAB_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub Flags: u32,
    pub UserName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_RETRIEVE_KEY_TAB_RESPONSE {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub KeyTabLength: u32,
    pub KeyTab: *mut u8,
}
impl Default for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_RETRIEVE_TICKET_AS_KERB_CRED: u32 = 8u32;
pub const KERB_RETRIEVE_TICKET_CACHE_TICKET: u32 = 32u32;
pub const KERB_RETRIEVE_TICKET_DEFAULT: u32 = 0u32;
pub const KERB_RETRIEVE_TICKET_DONT_USE_CACHE: u32 = 1u32;
pub const KERB_RETRIEVE_TICKET_MAX_LIFETIME: u32 = 64u32;
pub const KERB_RETRIEVE_TICKET_USE_CACHE_ONLY: u32 = 2u32;
pub const KERB_RETRIEVE_TICKET_USE_CREDHANDLE: u32 = 4u32;
pub const KERB_RETRIEVE_TICKET_WITH_SEC_CRED: u32 = 16u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Credentials")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_RETRIEVE_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub TargetName: LSA_UNICODE_STRING,
    pub TicketFlags: u32,
    pub CacheOptions: u32,
    pub EncryptionType: KERB_CRYPTO_KEY_TYPE,
    pub CredentialsHandle: super::super::Credentials::SecHandle,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_RETRIEVE_TKT_RESPONSE {
    pub Ticket: KERB_EXTERNAL_TICKET,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    pub ServerName: LSA_UNICODE_STRING,
    pub Flags: u32,
    pub LastStatus: super::super::super::Foundation::NTSTATUS,
    pub Expiry: i64,
}
pub const KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_S4U2PROXY_CRED {
    pub UserName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub Flags: u32,
    pub LastStatus: super::super::super::Foundation::NTSTATUS,
    pub Expiry: i64,
    pub CountOfEntries: u32,
    pub Entries: *mut KERB_S4U2PROXY_CACHE_ENTRY_INFO,
}
impl Default for KERB_S4U2PROXY_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KERB_S4U2PROXY_CRED_FLAG_NEGATIVE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_S4U_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub ClientUpn: LSA_UNICODE_STRING,
    pub ClientRealm: LSA_UNICODE_STRING,
}
pub const KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
pub const KERB_S4U_LOGON_FLAG_IDENTIFY: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Credentials")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_SETPASSWORD_EX_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub CredentialsHandle: super::super::Credentials::SecHandle,
    pub Flags: u32,
    pub AccountRealm: LSA_UNICODE_STRING,
    pub AccountName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
    pub ClientRealm: LSA_UNICODE_STRING,
    pub ClientName: LSA_UNICODE_STRING,
    pub Impersonating: bool,
    pub KdcAddress: LSA_UNICODE_STRING,
    pub KdcAddressType: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Credentials")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_SETPASSWORD_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub CredentialsHandle: super::super::Credentials::SecHandle,
    pub Flags: u32,
    pub DomainName: LSA_UNICODE_STRING,
    pub AccountName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
}
pub const KERB_SETPASS_USE_CREDHANDLE: u32 = 2u32;
pub const KERB_SETPASS_USE_LOGONID: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_SMART_CARD_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Pin: LSA_UNICODE_STRING,
    pub CspDataLength: u32,
    pub CspData: *mut u8,
}
impl Default for KERB_SMART_CARD_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_SMART_CARD_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub CertificateSize: u32,
    pub CertificateData: *mut u8,
}
impl Default for KERB_SMART_CARD_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_SMART_CARD_UNLOCK_LOGON {
    pub Logon: KERB_SMART_CARD_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_SUBMIT_TKT_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
    pub Key: KERB_CRYPTO_KEY32,
    pub KerbCredSize: u32,
    pub KerbCredOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TICKET_CACHE_INFO {
    pub ServerName: LSA_UNICODE_STRING,
    pub RealmName: LSA_UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: KERB_TICKET_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TICKET_CACHE_INFO_EX {
    pub ClientName: LSA_UNICODE_STRING,
    pub ClientRealm: LSA_UNICODE_STRING,
    pub ServerName: LSA_UNICODE_STRING,
    pub ServerRealm: LSA_UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TICKET_CACHE_INFO_EX2 {
    pub ClientName: LSA_UNICODE_STRING,
    pub ClientRealm: LSA_UNICODE_STRING,
    pub ServerName: LSA_UNICODE_STRING,
    pub ServerRealm: LSA_UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
    pub SessionKeyType: u32,
    pub BranchId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TICKET_CACHE_INFO_EX3 {
    pub ClientName: LSA_UNICODE_STRING,
    pub ClientRealm: LSA_UNICODE_STRING,
    pub ServerName: LSA_UNICODE_STRING,
    pub ServerRealm: LSA_UNICODE_STRING,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RenewTime: i64,
    pub EncryptionType: i32,
    pub TicketFlags: u32,
    pub SessionKeyType: u32,
    pub BranchId: u32,
    pub CacheFlags: u32,
    pub KdcCalled: LSA_UNICODE_STRING,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KERB_TICKET_FLAGS(pub u32);
impl KERB_TICKET_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for KERB_TICKET_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for KERB_TICKET_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for KERB_TICKET_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for KERB_TICKET_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for KERB_TICKET_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const KERB_TICKET_FLAGS_cname_in_pa_data: u32 = 262144u32;
pub const KERB_TICKET_FLAGS_enc_pa_rep: u32 = 65536u32;
pub const KERB_TICKET_FLAGS_forwardable: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(1073741824u32);
pub const KERB_TICKET_FLAGS_forwarded: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(536870912u32);
pub const KERB_TICKET_FLAGS_hw_authent: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(1048576u32);
pub const KERB_TICKET_FLAGS_initial: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(4194304u32);
pub const KERB_TICKET_FLAGS_invalid: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(16777216u32);
pub const KERB_TICKET_FLAGS_may_postdate: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(67108864u32);
pub const KERB_TICKET_FLAGS_name_canonicalize: u32 = 65536u32;
pub const KERB_TICKET_FLAGS_ok_as_delegate: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(262144u32);
pub const KERB_TICKET_FLAGS_postdated: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(33554432u32);
pub const KERB_TICKET_FLAGS_pre_authent: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(2097152u32);
pub const KERB_TICKET_FLAGS_proxiable: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(268435456u32);
pub const KERB_TICKET_FLAGS_proxy: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(134217728u32);
pub const KERB_TICKET_FLAGS_renewable: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(8388608u32);
pub const KERB_TICKET_FLAGS_reserved: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(2147483648u32);
pub const KERB_TICKET_FLAGS_reserved1: KERB_TICKET_FLAGS = KERB_TICKET_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KERB_TICKET_LOGON {
    pub MessageType: KERB_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub ServiceTicketLength: u32,
    pub TicketGrantingTicketLength: u32,
    pub ServiceTicket: *mut u8,
    pub TicketGrantingTicket: *mut u8,
}
impl Default for KERB_TICKET_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TICKET_PROFILE {
    pub Profile: KERB_INTERACTIVE_PROFILE,
    pub SessionKey: KERB_CRYPTO_KEY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TICKET_UNLOCK_LOGON {
    pub Logon: KERB_TICKET_LOGON,
    pub LogonId: super::super::super::Foundation::LUID,
}
pub const KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERB_TRANSFER_CRED_REQUEST {
    pub MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    pub OriginLogonId: super::super::super::Foundation::LUID,
    pub DestinationLogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
}
pub const KERB_TRANSFER_CRED_WITH_TICKETS: u32 = 1u32;
pub const KERB_USE_DEFAULT_TICKET_FLAGS: u32 = 0u32;
pub const KERB_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
pub const KERN_CONTEXT_CERT_INFO_V1: u32 = 0u32;
pub const KRB_ANONYMOUS_STRING: windows_core::PCWSTR = windows_core::w!("ANONYMOUS");
pub const KRB_NT_ENTERPRISE_PRINCIPAL: u32 = 10u32;
pub const KRB_NT_ENT_PRINCIPAL_AND_ID: i32 = -130i32;
pub const KRB_NT_MS_BRANCH_ID: i32 = -133i32;
pub const KRB_NT_MS_PRINCIPAL: i32 = -128i32;
pub const KRB_NT_MS_PRINCIPAL_AND_ID: i32 = -129i32;
pub const KRB_NT_PRINCIPAL: u32 = 1u32;
pub const KRB_NT_PRINCIPAL_AND_ID: i32 = -131i32;
pub const KRB_NT_SRV_HST: u32 = 3u32;
pub const KRB_NT_SRV_INST: u32 = 2u32;
pub const KRB_NT_SRV_INST_AND_ID: i32 = -132i32;
pub const KRB_NT_SRV_XHST: u32 = 4u32;
pub const KRB_NT_UID: u32 = 5u32;
pub const KRB_NT_UNKNOWN: u32 = 0u32;
pub const KRB_NT_WELLKNOWN: u32 = 11u32;
pub const KRB_NT_X500_PRINCIPAL: u32 = 6u32;
pub const KRB_WELLKNOWN_STRING: windows_core::PCWSTR = windows_core::w!("WELLKNOWN");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEC_CONTEXT_TYPE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSEC_LIST_ENTRY {
    pub List: super::super::super::System::Kernel::LIST_ENTRY,
    pub RefCount: i32,
    pub Signature: u32,
    pub OwningList: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for KSEC_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSecNonPaged: KSEC_CONTEXT_TYPE = KSEC_CONTEXT_TYPE(1i32);
pub const KSecPaged: KSEC_CONTEXT_TYPE = KSEC_CONTEXT_TYPE(0i32);
pub const KerbAddBindingCacheEntryExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(27i32);
pub const KerbAddBindingCacheEntryMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(10i32);
pub const KerbAddExtraCredentialsExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(22i32);
pub const KerbAddExtraCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(17i32);
pub const KerbCertificateLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(13i32);
pub const KerbCertificateS4ULogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(14i32);
pub const KerbCertificateUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(15i32);
pub const KerbChangeMachinePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(2i32);
pub const KerbChangePasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(7i32);
pub const KerbCleanupMachinePkinitCredsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(26i32);
pub const KerbDebugRequestMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(0i32);
pub const KerbDecryptDataMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(9i32);
pub const KerbInteractiveLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(2i32);
pub const KerbInteractiveProfile: KERB_PROFILE_BUFFER_TYPE = KERB_PROFILE_BUFFER_TYPE(2i32);
pub const KerbLuidLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(84i32);
pub const KerbNoElevationLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(83i32);
pub const KerbPinKdcMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(30i32);
pub const KerbPrintCloudKerberosDebugMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(36i32);
pub const KerbProxyLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(9i32);
pub const KerbPurgeBindingCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(29i32);
pub const KerbPurgeKdcProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(24i32);
pub const KerbPurgeTicketCacheExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(15i32);
pub const KerbPurgeTicketCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(6i32);
pub const KerbQueryBindingCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(28i32);
pub const KerbQueryDomainExtendedPoliciesMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(32i32);
pub const KerbQueryKdcProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(23i32);
pub const KerbQueryS4U2ProxyCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(33i32);
pub const KerbQuerySupplementalCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(18i32);
pub const KerbQueryTicketCacheEx2Message: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(20i32);
pub const KerbQueryTicketCacheEx3Message: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(25i32);
pub const KerbQueryTicketCacheExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(14i32);
pub const KerbQueryTicketCacheMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(1i32);
pub const KerbRefreshPolicyMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(35i32);
pub const KerbRefreshSmartcardCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(16i32);
pub const KerbRetrieveEncodedTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(8i32);
pub const KerbRetrieveKeyTabMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(34i32);
pub const KerbRetrieveTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(4i32);
pub const KerbS4ULogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(12i32);
pub const KerbSetPasswordExMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(12i32);
pub const KerbSetPasswordMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(11i32);
pub const KerbSmartCardLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(6i32);
pub const KerbSmartCardProfile: KERB_PROFILE_BUFFER_TYPE = KERB_PROFILE_BUFFER_TYPE(4i32);
pub const KerbSmartCardUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(8i32);
pub const KerbSubmitTicketMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(21i32);
pub const KerbTicketLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(10i32);
pub const KerbTicketProfile: KERB_PROFILE_BUFFER_TYPE = KERB_PROFILE_BUFFER_TYPE(6i32);
pub const KerbTicketUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(11i32);
pub const KerbTransferCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(19i32);
pub const KerbUnpinAllKdcsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(31i32);
pub const KerbUpdateAddressesMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(5i32);
pub const KerbVerifyCredentialsMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(13i32);
pub const KerbVerifyPacMessage: KERB_PROTOCOL_MESSAGE_TYPE = KERB_PROTOCOL_MESSAGE_TYPE(3i32);
pub const KerbWorkstationUnlockLogon: KERB_LOGON_SUBMIT_TYPE = KERB_LOGON_SUBMIT_TYPE(7i32);
pub type KspCompleteTokenFn = Option<unsafe extern "system" fn(contextid: usize, token: *const SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
pub type KspDeleteContextFn = Option<unsafe extern "system" fn(contextid: usize, lsacontextid: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
pub type KspGetTokenFn = Option<unsafe extern "system" fn(contextid: usize, impersonationtoken: *mut super::super::super::Foundation::HANDLE, rawtoken: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type KspInitContextFn = Option<unsafe extern "system" fn(contextid: usize, contextdata: *const SecBuffer, newcontextid: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Kernel")]
pub type KspInitPackageFn = Option<unsafe extern "system" fn(functiontable: *const SECPKG_KERNEL_FUNCTIONS) -> super::super::super::Foundation::NTSTATUS>;
pub type KspMakeSignatureFn = Option<unsafe extern "system" fn(contextid: usize, fqop: u32, message: *const SecBufferDesc, messageseqno: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type KspMapHandleFn = Option<unsafe extern "system" fn(contextid: usize, lsacontextid: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
pub type KspQueryAttributesFn = Option<unsafe extern "system" fn(contextid: usize, attribute: u32, buffer: *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type KspSealMessageFn = Option<unsafe extern "system" fn(contextid: usize, fqop: u32, message: *const SecBufferDesc, messageseqno: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type KspSerializeAuthDataFn = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: *mut u32, serializeddata: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type KspSetPagingModeFn = Option<unsafe extern "system" fn(pagingmode: bool) -> super::super::super::Foundation::NTSTATUS>;
pub type KspUnsealMessageFn = Option<unsafe extern "system" fn(contextid: usize, message: *const SecBufferDesc, messageseqno: u32, pfqop: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type KspVerifySignatureFn = Option<unsafe extern "system" fn(contextid: usize, message: *const SecBufferDesc, messageseqno: u32, pfqop: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub const LCRED_CRED_EXISTS: u32 = 1u32;
pub const LCRED_STATUS_NOCRED: u32 = 0u32;
pub const LCRED_STATUS_UNKNOWN_ISSUER: u32 = 2u32;
pub const LOGON_CACHED_ACCOUNT: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(4u32);
pub const LOGON_EXTRA_SIDS: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(32u32);
pub const LOGON_GRACE_LOGON: u32 = 16777216u32;
pub const LOGON_GUEST: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOGON_HOURS {
    pub UnitsPerWeek: u16,
    pub LogonHours: *mut u8,
}
impl Default for LOGON_HOURS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOGON_LM_V2: u32 = 4096u32;
pub const LOGON_MANAGED_SERVICE: u32 = 524288u32;
pub const LOGON_NOENCRYPTION: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(2u32);
pub const LOGON_NO_ELEVATION: u32 = 262144u32;
pub const LOGON_NO_OPTIMIZED: u32 = 131072u32;
pub const LOGON_NTLMV2_ENABLED: u32 = 256u32;
pub const LOGON_NTLM_V2: u32 = 8192u32;
pub const LOGON_NT_V2: u32 = 2048u32;
pub const LOGON_OPTIMIZED: u32 = 16384u32;
pub const LOGON_PKINIT: u32 = 65536u32;
pub const LOGON_PROFILE_PATH_RETURNED: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(1024u32);
pub const LOGON_RESOURCE_GROUPS: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(512u32);
pub const LOGON_SERVER_TRUST_ACCOUNT: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(128u32);
pub const LOGON_SUBAUTH_SESSION_KEY: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(64u32);
pub const LOGON_USED_LM_PASSWORD: MSV_SUB_AUTHENTICATION_FILTER = MSV_SUB_AUTHENTICATION_FILTER(8u32);
pub const LOGON_WINLOGON: u32 = 32768u32;
pub const LOOKUP_TRANSLATE_NAMES: u32 = 2048u32;
pub const LOOKUP_VIEW_LOCAL_INFORMATION: u32 = 1u32;
pub const LSAD_AES_BLOCK_SIZE: u32 = 16u32;
pub const LSAD_AES_CRYPT_SHA512_HASH_SIZE: u32 = 64u32;
pub const LSAD_AES_KEY_SIZE: u32 = 16u32;
pub const LSAD_AES_SALT_SIZE: u32 = 16u32;
pub const LSASETCAPS_RELOAD_FLAG: u32 = 1u32;
pub const LSASETCAPS_VALID_FLAG_MASK: u32 = 1u32;
pub const LSA_ADT_LEGACY_SECURITY_SOURCE_NAME: windows_core::PCWSTR = windows_core::w!("Security");
pub const LSA_ADT_SECURITY_SOURCE_NAME: windows_core::PCWSTR = windows_core::w!("Microsoft-Windows-Security-Auditing");
pub const LSA_AP_NAME_CALL_PACKAGE: windows_core::PCSTR = windows_core::s!("LsaApCallPackage\u{0}");
pub const LSA_AP_NAME_CALL_PACKAGE_PASSTHROUGH: windows_core::PCSTR = windows_core::s!("LsaApCallPackagePassthrough\u{0}");
pub const LSA_AP_NAME_CALL_PACKAGE_UNTRUSTED: windows_core::PCSTR = windows_core::s!("LsaApCallPackageUntrusted\u{0}");
pub const LSA_AP_NAME_INITIALIZE_PACKAGE: windows_core::PCSTR = windows_core::s!("LsaApInitializePackage\u{0}");
pub const LSA_AP_NAME_LOGON_TERMINATED: windows_core::PCSTR = windows_core::s!("LsaApLogonTerminated\u{0}");
pub const LSA_AP_NAME_LOGON_USER: windows_core::PCSTR = windows_core::s!("LsaApLogonUser\u{0}");
pub const LSA_AP_NAME_LOGON_USER_EX: windows_core::PCSTR = windows_core::s!("LsaApLogonUserEx\u{0}");
pub const LSA_AP_NAME_LOGON_USER_EX2: windows_core::PCSTR = windows_core::s!("LsaApLogonUserEx2\u{0}");
pub type LSA_AP_POST_LOGON_USER = Option<unsafe extern "system" fn(postlogonuserinfo: *const SECPKG_POST_LOGON_USER_INFO) -> super::super::super::Foundation::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_AUTH_INFORMATION {
    pub LastUpdateTime: i64,
    pub AuthType: LSA_AUTH_INFORMATION_AUTH_TYPE,
    pub AuthInfoLength: u32,
    pub AuthInfo: *mut u8,
}
impl Default for LSA_AUTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LSA_AUTH_INFORMATION_AUTH_TYPE(pub u32);
pub const LSA_CALL_LICENSE_SERVER: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct LSA_DISPATCH_TABLE {
    pub CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    pub DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    pub AddCredential: PLSA_ADD_CREDENTIAL,
    pub GetCredentials: PLSA_GET_CREDENTIALS,
    pub DeleteCredential: PLSA_DELETE_CREDENTIAL,
    pub AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    pub AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    pub FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    pub CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    pub CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_ENUMERATION_INFORMATION {
    pub Sid: super::super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_FOREST_TRUST_BINARY_DATA {
    pub Length: u32,
    pub Buffer: *mut u8,
}
impl Default for LSA_FOREST_TRUST_BINARY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_FOREST_TRUST_COLLISION_INFORMATION {
    pub RecordCount: u32,
    pub Entries: *mut *mut LSA_FOREST_TRUST_COLLISION_RECORD,
}
impl Default for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_FOREST_TRUST_COLLISION_RECORD {
    pub Index: u32,
    pub Type: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE,
    pub Flags: u32,
    pub Name: LSA_UNICODE_STRING,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LSA_FOREST_TRUST_COLLISION_RECORD_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_FOREST_TRUST_DOMAIN_INFO {
    pub Sid: super::super::PSID,
    pub DnsName: LSA_UNICODE_STRING,
    pub NetbiosName: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_FOREST_TRUST_INFORMATION {
    pub RecordCount: u32,
    pub Entries: *mut *mut LSA_FOREST_TRUST_RECORD,
}
impl Default for LSA_FOREST_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_FOREST_TRUST_INFORMATION2 {
    pub RecordCount: u32,
    pub Entries: *mut *mut LSA_FOREST_TRUST_RECORD2,
}
impl Default for LSA_FOREST_TRUST_INFORMATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_RECORD {
    pub Flags: u32,
    pub ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    pub Time: i64,
    pub ForestTrustData: LSA_FOREST_TRUST_RECORD_0,
}
impl Default for LSA_FOREST_TRUST_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LSA_FOREST_TRUST_RECORD_0 {
    pub TopLevelName: LSA_UNICODE_STRING,
    pub DomainInfo: LSA_FOREST_TRUST_DOMAIN_INFO,
    pub Data: LSA_FOREST_TRUST_BINARY_DATA,
}
impl Default for LSA_FOREST_TRUST_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LSA_FOREST_TRUST_RECORD2 {
    pub Flags: u32,
    pub ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    pub Time: i64,
    pub ForestTrustData: LSA_FOREST_TRUST_RECORD2_0,
}
impl Default for LSA_FOREST_TRUST_RECORD2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LSA_FOREST_TRUST_RECORD2_0 {
    pub TopLevelName: LSA_UNICODE_STRING,
    pub DomainInfo: LSA_FOREST_TRUST_DOMAIN_INFO,
    pub BinaryData: LSA_FOREST_TRUST_BINARY_DATA,
    pub ScannerInfo: LSA_FOREST_TRUST_SCANNER_INFO,
}
impl Default for LSA_FOREST_TRUST_RECORD2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LSA_FOREST_TRUST_RECORD_TYPE(pub i32);
pub const LSA_FOREST_TRUST_RECORD_TYPE_UNRECOGNIZED: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_FOREST_TRUST_SCANNER_INFO {
    pub DomainSid: super::super::PSID,
    pub DnsName: LSA_UNICODE_STRING,
    pub NetbiosName: LSA_UNICODE_STRING,
}
pub const LSA_FTRECORD_DISABLED_REASONS: i32 = 65535i32;
pub const LSA_GLOBAL_SECRET_PREFIX: windows_core::PCWSTR = windows_core::w!("G$");
pub const LSA_GLOBAL_SECRET_PREFIX_LENGTH: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LSA_HANDLE(pub isize);
impl LSA_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for LSA_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("advapi32.dll" "system" fn LsaClose(objecthandle : isize) -> i32);
            unsafe {
                LsaClose(self.0);
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_LAST_INTER_LOGON_INFO {
    pub LastSuccessfulLogon: i64,
    pub LastFailedLogon: i64,
    pub FailedAttemptCountSinceLastSuccessfulLogon: u32,
}
pub const LSA_LOCAL_SECRET_PREFIX: windows_core::PCWSTR = windows_core::w!("L$");
pub const LSA_LOCAL_SECRET_PREFIX_LENGTH: u32 = 2u32;
pub const LSA_LOOKUP_DISALLOW_CONNECTED_ACCOUNT_INTERNET_SID: u32 = 2147483648u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LSA_LOOKUP_DOMAIN_INFO_CLASS(pub i32);
pub const LSA_LOOKUP_ISOLATED_AS_LOCAL: u32 = 2147483648u32;
pub const LSA_LOOKUP_PREFER_INTERNET_NAMES: u32 = 1073741824u32;
pub const LSA_MACHINE_SECRET_PREFIX: windows_core::PCWSTR = windows_core::w!("M$");
pub const LSA_MAXIMUM_ENUMERATION_LENGTH: u32 = 32000u32;
pub const LSA_MAXIMUM_SID_COUNT: i32 = 256i32;
pub const LSA_MODE_INDIVIDUAL_ACCOUNTS: i32 = 2i32;
pub const LSA_MODE_LOG_FULL: i32 = 8i32;
pub const LSA_MODE_MANDATORY_ACCESS: i32 = 4i32;
pub const LSA_MODE_PASSWORD_PROTECTED: i32 = 1i32;
pub const LSA_NB_DISABLED_ADMIN: i32 = 4i32;
pub const LSA_NB_DISABLED_CONFLICT: i32 = 8i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::super::Foundation::HANDLE,
    pub ObjectName: *mut LSA_UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
}
impl Default for LSA_OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LSA_QUERY_CLIENT_PRELOGON_SESSION_ID: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_REFERENCED_DOMAIN_LIST {
    pub Entries: u32,
    pub Domains: *mut LSA_TRUST_INFORMATION,
}
impl Default for LSA_REFERENCED_DOMAIN_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LSA_SCANNER_INFO_ADMIN_ALL_FLAGS: i32 = 1i32;
pub const LSA_SCANNER_INFO_DISABLE_AUTH_TARGET_VALIDATION: i32 = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Security_Credentials", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct LSA_SECPKG_FUNCTION_TABLE {
    pub CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    pub DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    pub AddCredential: PLSA_ADD_CREDENTIAL,
    pub GetCredentials: PLSA_GET_CREDENTIALS,
    pub DeleteCredential: PLSA_DELETE_CREDENTIAL,
    pub AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    pub AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    pub FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    pub CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    pub CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
    pub ImpersonateClient: PLSA_IMPERSONATE_CLIENT,
    pub UnloadPackage: PLSA_UNLOAD_PACKAGE,
    pub DuplicateHandle: PLSA_DUPLICATE_HANDLE,
    pub SaveSupplementalCredentials: PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS,
    pub CreateThread: PLSA_CREATE_THREAD,
    pub GetClientInfo: PLSA_GET_CLIENT_INFO,
    pub RegisterNotification: PLSA_REGISTER_NOTIFICATION,
    pub CancelNotification: PLSA_CANCEL_NOTIFICATION,
    pub MapBuffer: PLSA_MAP_BUFFER,
    pub CreateToken: PLSA_CREATE_TOKEN,
    pub AuditLogon: PLSA_AUDIT_LOGON,
    pub CallPackage: PLSA_CALL_PACKAGE,
    pub FreeReturnBuffer: PLSA_FREE_LSA_HEAP,
    pub GetCallInfo: PLSA_GET_CALL_INFO,
    pub CallPackageEx: PLSA_CALL_PACKAGEEX,
    pub CreateSharedMemory: PLSA_CREATE_SHARED_MEMORY,
    pub AllocateSharedMemory: PLSA_ALLOCATE_SHARED_MEMORY,
    pub FreeSharedMemory: PLSA_FREE_SHARED_MEMORY,
    pub DeleteSharedMemory: PLSA_DELETE_SHARED_MEMORY,
    pub OpenSamUser: PLSA_OPEN_SAM_USER,
    pub GetUserCredentials: PLSA_GET_USER_CREDENTIALS,
    pub GetUserAuthData: PLSA_GET_USER_AUTH_DATA,
    pub CloseSamUser: PLSA_CLOSE_SAM_USER,
    pub ConvertAuthDataToToken: PLSA_CONVERT_AUTH_DATA_TO_TOKEN,
    pub ClientCallback: PLSA_CLIENT_CALLBACK,
    pub UpdateCredentials: PLSA_UPDATE_PRIMARY_CREDENTIALS,
    pub GetAuthDataForUser: PLSA_GET_AUTH_DATA_FOR_USER,
    pub CrackSingleName: PLSA_CRACK_SINGLE_NAME,
    pub AuditAccountLogon: PLSA_AUDIT_ACCOUNT_LOGON,
    pub CallPackagePassthrough: PLSA_CALL_PACKAGE_PASSTHROUGH,
    pub CrediRead: CredReadFn,
    pub CrediReadDomainCredentials: CredReadDomainCredentialsFn,
    pub CrediFreeCredentials: CredFreeCredentialsFn,
    pub LsaProtectMemory: PLSA_PROTECT_MEMORY,
    pub LsaUnprotectMemory: PLSA_PROTECT_MEMORY,
    pub OpenTokenByLogonId: PLSA_OPEN_TOKEN_BY_LOGON_ID,
    pub ExpandAuthDataForDomain: PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN,
    pub AllocatePrivateHeap: PLSA_ALLOCATE_PRIVATE_HEAP,
    pub FreePrivateHeap: PLSA_FREE_PRIVATE_HEAP,
    pub CreateTokenEx: PLSA_CREATE_TOKEN_EX,
    pub CrediWrite: CredWriteFn,
    pub CrediUnmarshalandDecodeString: CrediUnmarshalandDecodeStringFn,
    pub DummyFunction6: PLSA_PROTECT_MEMORY,
    pub GetExtendedCallFlags: PLSA_GET_EXTENDED_CALL_FLAGS,
    pub DuplicateTokenHandle: PLSA_DUPLICATE_HANDLE,
    pub GetServiceAccountPassword: PLSA_GET_SERVICE_ACCOUNT_PASSWORD,
    pub DummyFunction7: PLSA_PROTECT_MEMORY,
    pub AuditLogonEx: PLSA_AUDIT_LOGON_EX,
    pub CheckProtectedUserByToken: PLSA_CHECK_PROTECTED_USER_BY_TOKEN,
    pub QueryClientRequest: PLSA_QUERY_CLIENT_REQUEST,
    pub GetAppModeInfo: PLSA_GET_APP_MODE_INFO,
    pub SetAppModeInfo: PLSA_SET_APP_MODE_INFO,
    pub GetClientInfoEx: PLSA_GET_CLIENT_INFO_EX,
}
pub const LSA_SECRET_MAXIMUM_COUNT: i32 = 4096i32;
pub const LSA_SECRET_MAXIMUM_LENGTH: i32 = 512i32;
pub const LSA_SID_DISABLED_ADMIN: i32 = 1i32;
pub const LSA_SID_DISABLED_CONFLICT: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PSTR,
}
pub const LSA_TLN_DISABLED_ADMIN: i32 = 2i32;
pub const LSA_TLN_DISABLED_CONFLICT: i32 = 4i32;
pub const LSA_TLN_DISABLED_NEW: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_TOKEN_INFORMATION_NULL {
    pub ExpirationTime: i64,
    pub Groups: *mut super::super::TOKEN_GROUPS,
}
impl Default for LSA_TOKEN_INFORMATION_NULL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LSA_TOKEN_INFORMATION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_TOKEN_INFORMATION_V1 {
    pub ExpirationTime: i64,
    pub User: super::super::TOKEN_USER,
    pub Groups: *mut super::super::TOKEN_GROUPS,
    pub PrimaryGroup: super::super::TOKEN_PRIMARY_GROUP,
    pub Privileges: *mut super::super::TOKEN_PRIVILEGES,
    pub Owner: super::super::TOKEN_OWNER,
    pub DefaultDacl: super::super::TOKEN_DEFAULT_DACL,
}
impl Default for LSA_TOKEN_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LSA_TOKEN_INFORMATION_V3 {
    pub ExpirationTime: i64,
    pub User: super::super::TOKEN_USER,
    pub Groups: *mut super::super::TOKEN_GROUPS,
    pub PrimaryGroup: super::super::TOKEN_PRIMARY_GROUP,
    pub Privileges: *mut super::super::TOKEN_PRIVILEGES,
    pub Owner: super::super::TOKEN_OWNER,
    pub DefaultDacl: super::super::TOKEN_DEFAULT_DACL,
    pub UserClaims: super::super::TOKEN_USER_CLAIMS,
    pub DeviceClaims: super::super::TOKEN_DEVICE_CLAIMS,
    pub DeviceGroups: *mut super::super::TOKEN_GROUPS,
}
impl Default for LSA_TOKEN_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRANSLATED_NAME {
    pub Use: super::super::SID_NAME_USE,
    pub Name: LSA_UNICODE_STRING,
    pub DomainIndex: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRANSLATED_SID {
    pub Use: super::super::SID_NAME_USE,
    pub RelativeId: u32,
    pub DomainIndex: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRANSLATED_SID2 {
    pub Use: super::super::SID_NAME_USE,
    pub Sid: super::super::PSID,
    pub DomainIndex: i32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRUST_INFORMATION {
    pub Name: LSA_UNICODE_STRING,
    pub Sid: super::super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PWSTR,
}
pub const LocalUserCredKey: MSV1_0_CREDENTIAL_KEY_TYPE = MSV1_0_CREDENTIAL_KEY_TYPE(3i32);
pub const LsaTokenInformationNull: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(0i32);
pub const LsaTokenInformationV1: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(1i32);
pub const LsaTokenInformationV2: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(2i32);
pub const LsaTokenInformationV3: LSA_TOKEN_INFORMATION_TYPE = LSA_TOKEN_INFORMATION_TYPE(3i32);
#[cfg(feature = "Win32_Security_Credentials")]
pub type MAKE_SIGNATURE_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut SecBufferDesc, param3: u32) -> windows_core::HRESULT>;
pub const MAXIMUM_CAPES_PER_CAP: u32 = 127u32;
pub const MAX_CRED_SIZE: u32 = 1024u32;
pub const MAX_PROTOCOL_ID_SIZE: u32 = 255u32;
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: u32 = 4000u32;
pub const MAX_USER_RECORDS: u32 = 1000u32;
pub const MICROSOFT_KERBEROS_NAME: windows_core::PCWSTR = windows_core::w!("Kerberos");
pub const MICROSOFT_KERBEROS_NAME_A: windows_core::PCSTR = windows_core::s!("Kerberos");
pub const MICROSOFT_KERBEROS_NAME_W: windows_core::PCWSTR = windows_core::w!("Kerberos");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0(pub u32);
pub const MSV1_0_ALLOW_FORCE_GUEST: u32 = 8192u32;
pub const MSV1_0_ALLOW_MSVCHAPV2: u32 = 65536u32;
pub const MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(32u32);
pub const MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(2048u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_AVID(pub i32);
pub const MSV1_0_AV_FLAG_FORCE_GUEST: u32 = 1u32;
pub const MSV1_0_AV_FLAG_MIC_HANDSHAKE_MESSAGES: u32 = 2u32;
pub const MSV1_0_AV_FLAG_UNVERIFIED_TARGET: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_AV_PAIR {
    pub AvId: u16,
    pub AvLen: u16,
}
pub const MSV1_0_CHALLENGE_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_CHANGEPASSWORD_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: LSA_UNICODE_STRING,
    pub AccountName: LSA_UNICODE_STRING,
    pub OldPassword: LSA_UNICODE_STRING,
    pub NewPassword: LSA_UNICODE_STRING,
    pub Impersonating: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_CHANGEPASSWORD_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub PasswordInfoValid: bool,
    pub DomainPasswordInfo: DOMAIN_PASSWORD_INFORMATION,
}
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: u32 = 262144u32;
pub const MSV1_0_CLEARTEXT_PASSWORD_ALLOWED: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(2u32);
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: u32 = 16384u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_CREDENTIAL_KEY {
    pub Data: [u8; 20],
}
impl Default for MSV1_0_CREDENTIAL_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_CREDENTIAL_KEY_LENGTH: u32 = 20u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_CREDENTIAL_KEY_TYPE(pub i32);
pub const MSV1_0_CRED_CREDKEY_PRESENT: u32 = 8u32;
pub const MSV1_0_CRED_LM_PRESENT: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS = MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(1u32);
pub const MSV1_0_CRED_NT_PRESENT: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS = MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(2u32);
pub const MSV1_0_CRED_REMOVED: u32 = 4u32;
pub const MSV1_0_CRED_SHA_PRESENT: u32 = 16u32;
pub const MSV1_0_CRED_VERSION: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS = MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(0u32);
pub const MSV1_0_CRED_VERSION_ARSO: u32 = 4294901763u32;
pub const MSV1_0_CRED_VERSION_INVALID: u32 = 4294967295u32;
pub const MSV1_0_CRED_VERSION_IUM: u32 = 4294901761u32;
pub const MSV1_0_CRED_VERSION_REMOTE: u32 = 4294901762u32;
pub const MSV1_0_CRED_VERSION_RESERVED_1: u32 = 4294967294u32;
pub const MSV1_0_CRED_VERSION_V2: u32 = 2u32;
pub const MSV1_0_CRED_VERSION_V3: u32 = 4u32;
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: u32 = 4096u32;
pub const MSV1_0_DONT_TRY_GUEST_ACCOUNT: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(16u32);
pub const MSV1_0_GUEST_LOGON: MSV1_0 = MSV1_0(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_INTERACTIVE_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub UserName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_INTERACTIVE_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub LogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: LSA_UNICODE_STRING,
    pub HomeDirectory: LSA_UNICODE_STRING,
    pub FullName: LSA_UNICODE_STRING,
    pub ProfilePath: LSA_UNICODE_STRING,
    pub HomeDirectoryDrive: LSA_UNICODE_STRING,
    pub LogonServer: LSA_UNICODE_STRING,
    pub UserFlags: u32,
}
pub const MSV1_0_INTERNET_DOMAIN: u32 = 524288u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl Default for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_LM20_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub UserName: LSA_UNICODE_STRING,
    pub Workstation: LSA_UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub CaseSensitiveChallengeResponse: LSA_STRING,
    pub CaseInsensitiveChallengeResponse: LSA_STRING,
    pub ParameterControl: u32,
}
impl Default for MSV1_0_LM20_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_LM20_LOGON_PROFILE {
    pub MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    pub KickOffTime: i64,
    pub LogoffTime: i64,
    pub UserFlags: MSV_SUB_AUTHENTICATION_FILTER,
    pub UserSessionKey: [u8; 16],
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub LanmanSessionKey: [u8; 8],
    pub LogonServer: LSA_UNICODE_STRING,
    pub UserParameters: LSA_UNICODE_STRING,
}
impl Default for MSV1_0_LM20_LOGON_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_LOGON_SUBMIT_TYPE(pub i32);
pub const MSV1_0_MAX_AVL_SIZE: u32 = 64000u32;
pub const MSV1_0_MAX_NTLM3_LIFE: u32 = 1800u32;
pub const MSV1_0_MNS_LOGON: u32 = 16777216u32;
pub const MSV1_0_NTLM3_OWF_LENGTH: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_NTLM3_RESPONSE {
    pub Response: [u8; 16],
    pub RespType: u8,
    pub HiRespType: u8,
    pub Flags: u16,
    pub MsgWord: u32,
    pub TimeStamp: u64,
    pub ChallengeFromClient: [u8; 8],
    pub AvPairsOff: u32,
    pub Buffer: [u8; 1],
}
impl Default for MSV1_0_NTLM3_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: u32 = 16u32;
pub const MSV1_0_OWF_PASSWORD_LENGTH: u32 = 16u32;
pub const MSV1_0_PACKAGE_NAME: windows_core::PCSTR = windows_core::s!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
pub const MSV1_0_PACKAGE_NAMEW: windows_core::PCWSTR = windows_core::w!("MICROSOFT_AUTHENTICATION_PACKAGE_V1_0");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_PASSTHROUGH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub DomainName: LSA_UNICODE_STRING,
    pub PackageName: LSA_UNICODE_STRING,
    pub DataLength: u32,
    pub LogonData: *mut u8,
    pub Pad: u32,
}
impl Default for MSV1_0_PASSTHROUGH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_PASSTHROUGH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub Pad: u32,
    pub DataLength: u32,
    pub ValidationData: *mut u8,
}
impl Default for MSV1_0_PASSTHROUGH_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_PASSTHRU: MSV1_0 = MSV1_0(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_PROFILE_BUFFER_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_PROTOCOL_MESSAGE_TYPE(pub i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub EncryptedCredsSize: u32,
    pub EncryptedCreds: [u8; 1],
}
impl Default for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_RETURN_PASSWORD_EXPIRY: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(64u32);
pub const MSV1_0_RETURN_PROFILE_PATH: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(512u32);
pub const MSV1_0_RETURN_USER_PARAMETERS: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(8u32);
pub const MSV1_0_S4U2SELF: u32 = 131072u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_S4U_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
}
pub const MSV1_0_S4U_LOGON_FLAG_CHECK_LOGONHOURS: u32 = 2u32;
pub const MSV1_0_SHA_PASSWORD_LENGTH: u32 = 20u32;
pub const MSV1_0_SUBAUTHENTICATION_DLL: u32 = 4278190080u32;
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: u32 = 1048576u32;
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: u32 = 132u32;
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: u32 = 2u32;
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: u32 = 24u32;
pub const MSV1_0_SUBAUTHENTICATION_FLAGS: u32 = 4278190080u32;
pub const MSV1_0_SUBAUTHENTICATION_KEY: windows_core::PCSTR = windows_core::s!("SYSTEM\\CurrentControlSet\\Control\\Lsa\\MSV1_0");
pub const MSV1_0_SUBAUTHENTICATION_VALUE: windows_core::PCSTR = windows_core::s!("Auth");
pub const MSV1_0_SUBAUTH_ACCOUNT_DISABLED: u32 = 1u32;
pub const MSV1_0_SUBAUTH_ACCOUNT_EXPIRY: u32 = 16u32;
pub const MSV1_0_SUBAUTH_ACCOUNT_TYPE: u32 = 64u32;
pub const MSV1_0_SUBAUTH_LOCKOUT: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_SUBAUTH_LOGON {
    pub MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub UserName: LSA_UNICODE_STRING,
    pub Workstation: LSA_UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub AuthenticationInfo1: LSA_STRING,
    pub AuthenticationInfo2: LSA_STRING,
    pub ParameterControl: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL,
    pub SubAuthPackageId: u32,
}
impl Default for MSV1_0_SUBAUTH_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_SUBAUTH_LOGON_HOURS: u32 = 8u32;
pub const MSV1_0_SUBAUTH_PASSWORD: u32 = 2u32;
pub const MSV1_0_SUBAUTH_PASSWORD_EXPIRY: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_SUBAUTH_REQUEST {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthPackageId: u32,
    pub SubAuthInfoLength: u32,
    pub SubAuthSubmitBuffer: *mut u8,
}
impl Default for MSV1_0_SUBAUTH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_SUBAUTH_RESPONSE {
    pub MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub SubAuthInfoLength: u32,
    pub SubAuthReturnBuffer: *mut u8,
}
impl Default for MSV1_0_SUBAUTH_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_SUBAUTH_WORKSTATIONS: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    pub Version: u32,
    pub Flags: MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS,
    pub LmPassword: [u8; 16],
    pub NtPassword: [u8; 16],
}
impl Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    pub Version: u32,
    pub Flags: u32,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
}
impl Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    pub Version: u32,
    pub Flags: u32,
    pub CredentialKeyType: MSV1_0_CREDENTIAL_KEY_TYPE,
    pub NtPassword: [u8; 16],
    pub CredentialKey: MSV1_0_CREDENTIAL_KEY,
    pub ShaPassword: [u8; 20],
}
impl Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSV1_0_TRY_GUEST_ACCOUNT_ONLY: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(256u32);
pub const MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(1024u32);
pub const MSV1_0_UPDATE_LOGON_STATISTICS: MSV_SUBAUTH_LOGON_PARAMETER_CONTROL = MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(4u32);
pub const MSV1_0_USER_SESSION_KEY_LENGTH: u32 = 16u32;
pub const MSV1_0_USE_CLIENT_CHALLENGE: u32 = 128u32;
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: u32 = 32768u32;
#[repr(C)]
#[cfg(feature = "Win32_System_PasswordManagement")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_VALIDATION_INFO {
    pub LogoffTime: i64,
    pub KickoffTime: i64,
    pub LogonServer: LSA_UNICODE_STRING,
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub SessionKey: USER_SESSION_KEY,
    pub Authoritative: bool,
    pub UserFlags: u32,
    pub WhichFields: u32,
    pub UserId: u32,
}
pub const MSV1_0_VALIDATION_KICKOFF_TIME: u32 = 2u32;
pub const MSV1_0_VALIDATION_LOGOFF_TIME: u32 = 1u32;
pub const MSV1_0_VALIDATION_LOGON_DOMAIN: u32 = 8u32;
pub const MSV1_0_VALIDATION_LOGON_SERVER: u32 = 4u32;
pub const MSV1_0_VALIDATION_SESSION_KEY: u32 = 16u32;
pub const MSV1_0_VALIDATION_USER_FLAGS: u32 = 32u32;
pub const MSV1_0_VALIDATION_USER_ID: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV_SUBAUTH_LOGON_PARAMETER_CONTROL(pub u32);
impl MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV_SUB_AUTHENTICATION_FILTER(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS(pub u32);
impl MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MsV1_0CacheLogon: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(8i32);
pub const MsV1_0CacheLookup: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(11i32);
pub const MsV1_0CacheLookupEx: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(17i32);
pub const MsV1_0ChangeCachedPassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(6i32);
pub const MsV1_0ChangePassword: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(5i32);
pub const MsV1_0ClearCachedCredentials: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(14i32);
pub const MsV1_0ConfigLocalAliases: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(13i32);
pub const MsV1_0DecryptDpapiMasterKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(20i32);
pub const MsV1_0DeleteTbalSecrets: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(24i32);
pub const MsV1_0DeriveCredential: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(10i32);
pub const MsV1_0EnumerateUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(2i32);
pub const MsV1_0GenericPassthrough: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(7i32);
pub const MsV1_0GetCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(18i32);
pub const MsV1_0GetStrongCredentialKey: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(21i32);
pub const MsV1_0GetUserInfo: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(3i32);
pub const MsV1_0InteractiveLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(2i32);
pub const MsV1_0InteractiveProfile: MSV1_0_PROFILE_BUFFER_TYPE = MSV1_0_PROFILE_BUFFER_TYPE(2i32);
pub const MsV1_0Lm20ChallengeRequest: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(0i32);
pub const MsV1_0Lm20GetChallengeResponse: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(1i32);
pub const MsV1_0Lm20Logon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(3i32);
pub const MsV1_0Lm20LogonProfile: MSV1_0_PROFILE_BUFFER_TYPE = MSV1_0_PROFILE_BUFFER_TYPE(3i32);
pub const MsV1_0LookupToken: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(15i32);
pub const MsV1_0LuidLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(84i32);
pub const MsV1_0NetworkLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(4i32);
pub const MsV1_0NoElevationLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(83i32);
pub const MsV1_0ProvisionTbal: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(23i32);
pub const MsV1_0ReLogonUsers: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(4i32);
pub const MsV1_0S4ULogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(12i32);
pub const MsV1_0SetProcessOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(12i32);
pub const MsV1_0SetThreadOption: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(19i32);
pub const MsV1_0SmartCardProfile: MSV1_0_PROFILE_BUFFER_TYPE = MSV1_0_PROFILE_BUFFER_TYPE(4i32);
pub const MsV1_0SubAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(9i32);
pub const MsV1_0SubAuthLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(5i32);
pub const MsV1_0TransferCred: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(22i32);
pub const MsV1_0ValidateAuth: MSV1_0_PROTOCOL_MESSAGE_TYPE = MSV1_0_PROTOCOL_MESSAGE_TYPE(16i32);
pub const MsV1_0VirtualLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(82i32);
pub const MsV1_0WorkstationUnlockLogon: MSV1_0_LOGON_SUBMIT_TYPE = MSV1_0_LOGON_SUBMIT_TYPE(7i32);
pub const MsvAvChannelBindings: MSV1_0_AVID = MSV1_0_AVID(10i32);
pub const MsvAvDnsComputerName: MSV1_0_AVID = MSV1_0_AVID(3i32);
pub const MsvAvDnsDomainName: MSV1_0_AVID = MSV1_0_AVID(4i32);
pub const MsvAvDnsTreeName: MSV1_0_AVID = MSV1_0_AVID(5i32);
pub const MsvAvEOL: MSV1_0_AVID = MSV1_0_AVID(0i32);
pub const MsvAvFlags: MSV1_0_AVID = MSV1_0_AVID(6i32);
pub const MsvAvNbComputerName: MSV1_0_AVID = MSV1_0_AVID(1i32);
pub const MsvAvNbDomainName: MSV1_0_AVID = MSV1_0_AVID(2i32);
pub const MsvAvRestrictions: MSV1_0_AVID = MSV1_0_AVID(8i32);
pub const MsvAvTargetName: MSV1_0_AVID = MSV1_0_AVID(9i32);
pub const MsvAvTimestamp: MSV1_0_AVID = MSV1_0_AVID(7i32);
pub const NEGOSSP_NAME: windows_core::PCWSTR = windows_core::w!("Negotiate");
pub const NEGOSSP_NAME_A: windows_core::PCSTR = windows_core::s!("Negotiate");
pub const NEGOSSP_NAME_W: windows_core::PCWSTR = windows_core::w!("Negotiate");
pub const NEGOTIATE_ALLOW_NTLM: u32 = 268435456u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEGOTIATE_CALLER_NAME_REQUEST {
    pub MessageType: u32,
    pub LogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEGOTIATE_CALLER_NAME_RESPONSE {
    pub MessageType: u32,
    pub CallerName: windows_core::PWSTR,
}
pub const NEGOTIATE_MAX_PREFIX: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NEGOTIATE_MESSAGES(pub i32);
pub const NEGOTIATE_NEG_NTLM: u32 = 536870912u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NEGOTIATE_PACKAGE_PREFIX {
    pub PackageId: usize,
    pub PackageDataA: *mut core::ffi::c_void,
    pub PackageDataW: *mut core::ffi::c_void,
    pub PrefixLen: usize,
    pub Prefix: [u8; 32],
}
impl Default for NEGOTIATE_PACKAGE_PREFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEGOTIATE_PACKAGE_PREFIXES {
    pub MessageType: u32,
    pub PrefixCount: u32,
    pub Offset: u32,
    pub Pad: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NETLOGON_GENERIC_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub PackageName: LSA_UNICODE_STRING,
    pub DataLength: u32,
    pub LogonData: *mut u8,
}
impl Default for NETLOGON_GENERIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_PasswordManagement")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETLOGON_INTERACTIVE_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
    pub NtOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETLOGON_LOGON_IDENTITY_INFO {
    pub LogonDomainName: LSA_UNICODE_STRING,
    pub ParameterControl: u32,
    pub LogonId: i64,
    pub UserName: LSA_UNICODE_STRING,
    pub Workstation: LSA_UNICODE_STRING,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETLOGON_LOGON_INFO_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETLOGON_NETWORK_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmChallenge: CLEAR_BLOCK,
    pub NtChallengeResponse: LSA_STRING,
    pub LmChallengeResponse: LSA_STRING,
}
#[repr(C)]
#[cfg(feature = "Win32_System_PasswordManagement")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETLOGON_SERVICE_INFO {
    pub Identity: NETLOGON_LOGON_IDENTITY_INFO,
    pub LmOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
    pub NtOwfPassword: super::super::super::System::PasswordManagement::LM_OWF_PASSWORD,
}
pub const NGC_DATA_FLAG_IS_CLOUD_TRUST_CRED: u32 = 8u32;
pub const NGC_DATA_FLAG_IS_SMARTCARD_DATA: u32 = 4u32;
pub const NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: u32 = 1u32;
pub const NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: u32 = 2u32;
pub const NOTIFIER_FLAG_NEW_THREAD: u32 = 1u32;
pub const NOTIFIER_FLAG_ONE_SHOT: u32 = 2u32;
pub const NOTIFIER_FLAG_SECONDS: u32 = 2147483648u32;
pub const NOTIFIER_TYPE_HANDLE_WAIT: u32 = 2u32;
pub const NOTIFIER_TYPE_IMMEDIATE: u32 = 16u32;
pub const NOTIFIER_TYPE_INTERVAL: u32 = 1u32;
pub const NOTIFIER_TYPE_NOTIFY_EVENT: u32 = 4u32;
pub const NOTIFIER_TYPE_STATE_CHANGE: u32 = 3u32;
pub const NOTIFY_CLASS_DOMAIN_CHANGE: u32 = 3u32;
pub const NOTIFY_CLASS_PACKAGE_CHANGE: u32 = 1u32;
pub const NOTIFY_CLASS_REGISTRY_CHANGE: u32 = 4u32;
pub const NOTIFY_CLASS_ROLE_CHANGE: u32 = 2u32;
pub const NO_LONG_NAMES: u32 = 2u32;
pub const NTLMSP_NAME: windows_core::PCWSTR = windows_core::w!("NTLM");
pub const NTLMSP_NAME_A: windows_core::PCSTR = windows_core::s!("NTLM");
pub const NameCanonical: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(7i32);
pub const NameCanonicalEx: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(9i32);
pub const NameDisplay: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(3i32);
pub const NameDnsDomain: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(12i32);
pub const NameFullyQualifiedDN: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(1i32);
pub const NameGivenName: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(13i32);
pub const NameSamCompatible: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(2i32);
pub const NameServicePrincipal: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(10i32);
pub const NameSurname: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(14i32);
pub const NameUniqueId: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(6i32);
pub const NameUnknown: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(0i32);
pub const NameUserPrincipal: EXTENDED_NAME_FORMAT = EXTENDED_NAME_FORMAT(8i32);
pub const NegCallPackageMax: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(4i32);
pub const NegEnumPackagePrefixes: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(0i32);
pub const NegGetCallerName: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(1i32);
pub const NegMsgReserved1: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(3i32);
pub const NegTransferCredentials: NEGOTIATE_MESSAGES = NEGOTIATE_MESSAGES(2i32);
pub const NetlogonGenericInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(4i32);
pub const NetlogonInteractiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(1i32);
pub const NetlogonInteractiveTransitiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(5i32);
pub const NetlogonNetworkInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(2i32);
pub const NetlogonNetworkTransitiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(6i32);
pub const NetlogonServiceInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(3i32);
pub const NetlogonServiceTransitiveInformation: NETLOGON_LOGON_INFO_CLASS = NETLOGON_LOGON_INFO_CLASS(7i32);
pub const PCT1SP_NAME: windows_core::PCWSTR = windows_core::w!("Microsoft PCT 1.0");
pub const PCT1SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft PCT 1.0");
pub const PCT1SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft PCT 1.0");
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: u32 = 8u32;
pub const PER_USER_AUDIT_FAILURE_INCLUDE: u32 = 4u32;
pub const PER_USER_AUDIT_NONE: u32 = 16u32;
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: u32 = 2u32;
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: u32 = 1u32;
pub const PER_USER_POLICY_UNCHANGED: u32 = 0u32;
pub type PKSEC_CREATE_CONTEXT_LIST = Option<unsafe extern "system" fn(r#type: KSEC_CONTEXT_TYPE) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Kernel")]
pub type PKSEC_DEREFERENCE_LIST_ENTRY = Option<unsafe extern "system" fn(entry: *const KSEC_LIST_ENTRY, delete: *mut u8)>;
#[cfg(feature = "Win32_System_Kernel")]
pub type PKSEC_INSERT_LIST_ENTRY = Option<unsafe extern "system" fn(list: *const core::ffi::c_void, entry: *const KSEC_LIST_ENTRY)>;
pub type PKSEC_LOCATE_PKG_BY_ID = Option<unsafe extern "system" fn(packageid: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Kernel")]
pub type PKSEC_REFERENCE_LIST_ENTRY = Option<unsafe extern "system" fn(entry: *const KSEC_LIST_ENTRY, signature: u32, removenoref: bool) -> super::super::super::Foundation::NTSTATUS>;
pub type PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: *mut u32, serializeddata: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PKSEC_SERIALIZE_WINNT_AUTH_DATA = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: *mut u32, serializeddata: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PKU2U_CERTIFICATE_S4U_LOGON {
    pub MessageType: PKU2U_LOGON_SUBMIT_TYPE,
    pub Flags: u32,
    pub UserPrincipalName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub CertificateLength: u32,
    pub Certificate: *mut u8,
}
impl Default for PKU2U_CERTIFICATE_S4U_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PKU2U_CERT_BLOB {
    pub CertOffset: u32,
    pub CertLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PKU2U_CREDUI_CONTEXT {
    pub Version: u64,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub CertArrayCount: u16,
    pub CertArrayOffset: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PKU2U_LOGON_SUBMIT_TYPE(pub i32);
pub const PKU2U_PACKAGE_NAME: windows_core::PCWSTR = windows_core::w!("pku2u");
pub const PKU2U_PACKAGE_NAME_A: windows_core::PCSTR = windows_core::s!("pku2u");
pub const PKU2U_PACKAGE_NAME_W: windows_core::PCWSTR = windows_core::w!("pku2u");
pub type PLSA_ADD_CREDENTIAL = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, authenticationpackage: u32, primarykeyvalue: *const LSA_STRING, credentials: *const LSA_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_ALLOCATE_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, lengthrequired: u32, clientbaseaddress: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_ALLOCATE_LSA_HEAP = Option<unsafe extern "system" fn(length: u32) -> *mut core::ffi::c_void>;
pub type PLSA_ALLOCATE_PRIVATE_HEAP = Option<unsafe extern "system" fn(length: usize) -> *mut core::ffi::c_void>;
pub type PLSA_ALLOCATE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void, size: u32) -> *mut core::ffi::c_void>;
pub type PLSA_AP_CALL_PACKAGE = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AP_CALL_PACKAGE_PASSTHROUGH = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AP_INITIALIZE_PACKAGE = Option<unsafe extern "system" fn(authenticationpackageid: u32, lsadispatchtable: *const LSA_DISPATCH_TABLE, database: *const LSA_STRING, confidentiality: *const LSA_STRING, authenticationpackagename: *mut *mut LSA_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AP_LOGON_TERMINATED = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID)>;
pub type PLSA_AP_LOGON_USER = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, authenticationinformation: *const core::ffi::c_void, clientauthenticationbase: *const core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: *mut u32, logonid: *mut super::super::super::Foundation::LUID, substatus: *mut i32, tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut *mut LSA_UNICODE_STRING, authenticatingauthority: *mut *mut LSA_UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AP_LOGON_USER_EX = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, authenticationinformation: *const core::ffi::c_void, clientauthenticationbase: *const core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: *mut u32, logonid: *mut super::super::super::Foundation::LUID, substatus: *mut i32, tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut *mut LSA_UNICODE_STRING, authenticatingauthority: *mut *mut LSA_UNICODE_STRING, machinename: *mut *mut LSA_UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AP_LOGON_USER_EX2 =
    Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbuffersize: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebuffersize: *mut u32, logonid: *mut super::super::super::Foundation::LUID, substatus: *mut i32, tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut *mut LSA_UNICODE_STRING, authenticatingauthority: *mut *mut LSA_UNICODE_STRING, machinename: *mut *mut LSA_UNICODE_STRING, primarycredentials: *mut SECPKG_PRIMARY_CRED, supplementalcredentials: *mut *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AP_LOGON_USER_EX3 = Option<
    unsafe extern "system" fn(
        clientrequest: *const *const core::ffi::c_void,
        logontype: SECURITY_LOGON_TYPE,
        protocolsubmitbuffer: *const core::ffi::c_void,
        clientbufferbase: *const core::ffi::c_void,
        submitbuffersize: u32,
        surrogatelogon: *mut SECPKG_SURROGATE_LOGON,
        profilebuffer: *mut *mut core::ffi::c_void,
        profilebuffersize: *mut u32,
        logonid: *mut super::super::super::Foundation::LUID,
        substatus: *mut i32,
        tokeninformationtype: *mut LSA_TOKEN_INFORMATION_TYPE,
        tokeninformation: *mut *mut core::ffi::c_void,
        accountname: *mut *mut LSA_UNICODE_STRING,
        authenticatingauthority: *mut *mut LSA_UNICODE_STRING,
        machinename: *mut *mut LSA_UNICODE_STRING,
        primarycredentials: *mut SECPKG_PRIMARY_CRED,
        supplementalcredentials: *mut *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY,
    ) -> super::super::super::Foundation::NTSTATUS,
>;
pub type PLSA_AP_POST_LOGON_USER_SURROGATE = Option<
    unsafe extern "system" fn(
        clientrequest: *const *const core::ffi::c_void,
        logontype: SECURITY_LOGON_TYPE,
        protocolsubmitbuffer: *const core::ffi::c_void,
        clientbufferbase: *const core::ffi::c_void,
        submitbuffersize: u32,
        surrogatelogon: *const SECPKG_SURROGATE_LOGON,
        profilebuffer: *const core::ffi::c_void,
        profilebuffersize: u32,
        logonid: *const super::super::super::Foundation::LUID,
        status: super::super::super::Foundation::NTSTATUS,
        substatus: super::super::super::Foundation::NTSTATUS,
        tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE,
        tokeninformation: *const core::ffi::c_void,
        accountname: *const LSA_UNICODE_STRING,
        authenticatingauthority: *const LSA_UNICODE_STRING,
        machinename: *const LSA_UNICODE_STRING,
        primarycredentials: *const SECPKG_PRIMARY_CRED,
        supplementalcredentials: *const SECPKG_SUPPLEMENTAL_CRED_ARRAY,
    ) -> super::super::super::Foundation::NTSTATUS,
>;
pub type PLSA_AP_PRE_LOGON_USER_SURROGATE = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, logontype: SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbuffersize: u32, surrogatelogon: *mut SECPKG_SURROGATE_LOGON, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AUDIT_ACCOUNT_LOGON = Option<unsafe extern "system" fn(auditid: u32, success: bool, source: *const LSA_UNICODE_STRING, clientname: *const LSA_UNICODE_STRING, mappedname: *const LSA_UNICODE_STRING, status: super::super::super::Foundation::NTSTATUS) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_AUDIT_LOGON = Option<unsafe extern "system" fn(status: super::super::super::Foundation::NTSTATUS, substatus: super::super::super::Foundation::NTSTATUS, accountname: *const LSA_UNICODE_STRING, authenticatingauthority: *const LSA_UNICODE_STRING, workstationname: *const LSA_UNICODE_STRING, usersid: super::super::PSID, logontype: SECURITY_LOGON_TYPE, tokensource: *const super::super::TOKEN_SOURCE, logonid: *const super::super::super::Foundation::LUID)>;
pub type PLSA_AUDIT_LOGON_EX = Option<unsafe extern "system" fn(status: super::super::super::Foundation::NTSTATUS, substatus: super::super::super::Foundation::NTSTATUS, accountname: *const LSA_UNICODE_STRING, authenticatingauthority: *const LSA_UNICODE_STRING, workstationname: *const LSA_UNICODE_STRING, usersid: super::super::PSID, logontype: SECURITY_LOGON_TYPE, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokensource: *const super::super::TOKEN_SOURCE, logonid: *const super::super::super::Foundation::LUID)>;
pub type PLSA_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(argument1: usize, argument2: usize, inputbuffer: *mut SecBuffer, outputbuffer: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CALL_PACKAGE = Option<unsafe extern "system" fn(authenticationpackage: *const LSA_UNICODE_STRING, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CALL_PACKAGEEX = Option<unsafe extern "system" fn(authenticationpackage: *const LSA_UNICODE_STRING, clientbufferbase: *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CALL_PACKAGE_PASSTHROUGH = Option<unsafe extern "system" fn(authenticationpackage: *const LSA_UNICODE_STRING, clientbufferbase: *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CANCEL_NOTIFICATION = Option<unsafe extern "system" fn(notifyhandle: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CHECK_PROTECTED_USER_BY_TOKEN = Option<unsafe extern "system" fn(usertoken: super::super::super::Foundation::HANDLE, protecteduser: *mut bool) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CLIENT_CALLBACK = Option<unsafe extern "system" fn(callback: windows_core::PCSTR, argument1: usize, argument2: usize, input: *const SecBuffer, output: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CLOSE_SAM_USER = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CONVERT_AUTH_DATA_TO_TOKEN = Option<unsafe extern "system" fn(userauthdata: *const core::ffi::c_void, userauthdatasize: u32, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokensource: *const super::super::TOKEN_SOURCE, logontype: SECURITY_LOGON_TYPE, authorityname: *const LSA_UNICODE_STRING, token: *mut super::super::super::Foundation::HANDLE, logonid: *mut super::super::super::Foundation::LUID, accountname: *mut LSA_UNICODE_STRING, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_COPY_FROM_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, length: u32, buffertocopy: *mut core::ffi::c_void, clientbaseaddress: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_COPY_TO_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, length: u32, clientbaseaddress: *mut core::ffi::c_void, buffertocopy: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CRACK_SINGLE_NAME = Option<unsafe extern "system" fn(formatoffered: u32, performatgc: bool, nameinput: *const LSA_UNICODE_STRING, prefix: *const LSA_UNICODE_STRING, requestedformat: u32, crackedname: *mut LSA_UNICODE_STRING, dnsdomainname: *mut LSA_UNICODE_STRING, substatus: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CREATE_LOGON_SESSION = Option<unsafe extern "system" fn(logonid: *mut super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CREATE_SHARED_MEMORY = Option<unsafe extern "system" fn(maxsize: u32, initialsize: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Threading")]
pub type PLSA_CREATE_THREAD = Option<unsafe extern "system" fn(securityattributes: *const super::super::SECURITY_ATTRIBUTES, stacksize: u32, startfunction: super::super::super::System::Threading::LPTHREAD_START_ROUTINE, threadparameter: *const core::ffi::c_void, creationflags: u32, threadid: *mut u32) -> super::super::super::Foundation::HANDLE>;
pub type PLSA_CREATE_TOKEN = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, tokensource: *const super::super::TOKEN_SOURCE, logontype: SECURITY_LOGON_TYPE, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, tokengroups: *const super::super::TOKEN_GROUPS, accountname: *const LSA_UNICODE_STRING, authorityname: *const LSA_UNICODE_STRING, workstation: *const LSA_UNICODE_STRING, profilepath: *const LSA_UNICODE_STRING, token: *mut super::super::super::Foundation::HANDLE, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_CREATE_TOKEN_EX = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, tokensource: *const super::super::TOKEN_SOURCE, logontype: SECURITY_LOGON_TYPE, impersonationlevel: super::super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, tokengroups: *const super::super::TOKEN_GROUPS, workstation: *const LSA_UNICODE_STRING, profilepath: *const LSA_UNICODE_STRING, sessioninformation: *const core::ffi::c_void, sessioninformationtype: SECPKG_SESSIONINFO_TYPE, token: *mut super::super::super::Foundation::HANDLE, substatus: *mut i32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_DELETE_CREDENTIAL = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, authenticationpackage: u32, primarykeyvalue: *const LSA_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_DELETE_LOGON_SESSION = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_DELETE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void) -> bool>;
pub type PLSA_DUPLICATE_HANDLE = Option<unsafe extern "system" fn(sourcehandle: super::super::super::Foundation::HANDLE, destionationhandle: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN = Option<unsafe extern "system" fn(userauthdata: *const u8, userauthdatasize: u32, reserved: *const core::ffi::c_void, expandedauthdata: *mut *mut u8, expandedauthdatasize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_FREE_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, clientbaseaddress: *const core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_FREE_LSA_HEAP = Option<unsafe extern "system" fn(base: *const core::ffi::c_void)>;
pub type PLSA_FREE_PRIVATE_HEAP = Option<unsafe extern "system" fn(base: *const core::ffi::c_void)>;
pub type PLSA_FREE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void, memory: *mut core::ffi::c_void)>;
pub type PLSA_GET_APP_MODE_INFO = Option<unsafe extern "system" fn(userfunction: *mut u32, argument1: *mut usize, argument2: *mut usize, userdata: *mut SecBuffer, returntolsa: *mut bool) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_AUTH_DATA_FOR_USER = Option<unsafe extern "system" fn(name: *const SECURITY_STRING, nametype: SECPKG_NAME_TYPE, prefix: *const SECURITY_STRING, userauthdata: *mut *mut u8, userauthdatasize: *mut u32, userflatname: *mut LSA_UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_CALL_INFO = Option<unsafe extern "system" fn(info: *mut SECPKG_CALL_INFO) -> bool>;
pub type PLSA_GET_CLIENT_INFO = Option<unsafe extern "system" fn(clientinfo: *mut SECPKG_CLIENT_INFO) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_CLIENT_INFO_EX = Option<unsafe extern "system" fn(clientinfo: *mut SECPKG_CLIENT_INFO_EX, structsize: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_CREDENTIALS = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, authenticationpackage: u32, querycontext: *mut u32, retrieveallcredentials: bool, primarykeyvalue: *const LSA_STRING, primarykeylength: *mut u32, credentials: *const LSA_STRING) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_EXTENDED_CALL_FLAGS = Option<unsafe extern "system" fn(flags: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_SERVICE_ACCOUNT_PASSWORD = Option<unsafe extern "system" fn(accountname: *const LSA_UNICODE_STRING, domainname: *const LSA_UNICODE_STRING, credfetch: CRED_FETCH, filetimeexpiry: *mut super::super::super::Foundation::FILETIME, currentpassword: *mut LSA_UNICODE_STRING, previouspassword: *mut LSA_UNICODE_STRING, filetimecurrpwdvalidforoutbound: *mut super::super::super::Foundation::FILETIME) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_USER_AUTH_DATA = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void, userauthdata: *mut *mut u8, userauthdatasize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_GET_USER_CREDENTIALS = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void, primarycreds: *mut *mut core::ffi::c_void, primarycredssize: *mut u32, supplementalcreds: *mut *mut core::ffi::c_void, supplementalcredssize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_IMPERSONATE_CLIENT = Option<unsafe extern "system" fn() -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_LOCATE_PKG_BY_ID = Option<unsafe extern "system" fn(packgeid: u32) -> *mut core::ffi::c_void>;
pub type PLSA_MAP_BUFFER = Option<unsafe extern "system" fn(inputbuffer: *const SecBuffer, outputbuffer: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_OPEN_SAM_USER = Option<unsafe extern "system" fn(name: *const SECURITY_STRING, nametype: SECPKG_NAME_TYPE, prefix: *const SECURITY_STRING, allowguest: bool, reserved: u32, userhandle: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_OPEN_TOKEN_BY_LOGON_ID = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, rettokenhandle: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_PROTECT_MEMORY = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, buffersize: u32)>;
pub type PLSA_QUERY_CLIENT_REQUEST = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, querytype: u32, replybuffer: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_REDIRECTED_LOGON_CALLBACK = Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32, returnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE)>;
pub type PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS = Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, logonbuffer: *mut *mut u8, logonbufferlength: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_REDIRECTED_LOGON_GET_SID = Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, sid: *mut super::super::PSID) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS = Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, supplementalcredentials: *mut *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_REDIRECTED_LOGON_INIT = Option<unsafe extern "system" fn(redirectedlogonhandle: super::super::super::Foundation::HANDLE, packagename: *const LSA_UNICODE_STRING, sessionid: u32, logonid: *const super::super::super::Foundation::LUID) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_REGISTER_CALLBACK = Option<unsafe extern "system" fn(callbackid: u32, callback: PLSA_CALLBACK_FUNCTION) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Threading")]
pub type PLSA_REGISTER_NOTIFICATION = Option<unsafe extern "system" fn(startfunction: super::super::super::System::Threading::LPTHREAD_START_ROUTINE, parameter: *const core::ffi::c_void, notificationtype: u32, notificationclass: u32, notificationflags: u32, intervalminutes: u32, waitevent: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::HANDLE>;
pub type PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, supplementalcredsize: u32, supplementalcreds: *const core::ffi::c_void, synchronous: bool) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_SET_APP_MODE_INFO = Option<unsafe extern "system" fn(userfunction: u32, argument1: usize, argument2: usize, userdata: *const SecBuffer, returntolsa: bool) -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_UNLOAD_PACKAGE = Option<unsafe extern "system" fn() -> super::super::super::Foundation::NTSTATUS>;
pub type PLSA_UPDATE_PRIMARY_CREDENTIALS = Option<unsafe extern "system" fn(primarycredentials: *const SECPKG_PRIMARY_CRED, credentials: *const SECPKG_SUPPLEMENTAL_CRED_ARRAY) -> super::super::super::Foundation::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_ACCOUNT_DOMAIN_INFO {
    pub DomainName: LSA_UNICODE_STRING,
    pub DomainSid: super::super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLICY_AUDIT_CATEGORIES_INFO {
    pub MaximumCategoryCount: u32,
    pub SubCategoriesInfo: *mut POLICY_AUDIT_SUBCATEGORIES_INFO,
}
impl Default for POLICY_AUDIT_CATEGORIES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLICY_AUDIT_EVENTS_INFO {
    pub AuditingMode: bool,
    pub EventAuditingOptions: *mut u32,
    pub MaximumAuditEventCount: u32,
}
impl Default for POLICY_AUDIT_EVENTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_AUDIT_EVENT_FAILURE: i32 = 2i32;
pub const POLICY_AUDIT_EVENT_NONE: i32 = 4i32;
pub const POLICY_AUDIT_EVENT_SUCCESS: i32 = 1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICY_AUDIT_EVENT_TYPE(pub i32);
pub const POLICY_AUDIT_EVENT_UNCHANGED: i32 = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_AUDIT_FULL_QUERY_INFO {
    pub ShutDownOnFull: bool,
    pub LogIsFull: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_AUDIT_FULL_SET_INFO {
    pub ShutDownOnFull: bool,
}
pub const POLICY_AUDIT_LOG_ADMIN: i32 = 512i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_AUDIT_LOG_INFO {
    pub AuditLogPercentFull: u32,
    pub MaximumLogSize: u32,
    pub AuditRetentionPeriod: i64,
    pub AuditLogFullShutdownInProgress: bool,
    pub TimeToShutdown: i64,
    pub NextAuditRecordId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLICY_AUDIT_SID_ARRAY {
    pub UsersCount: u32,
    pub UserSidArray: *mut super::super::PSID,
}
impl Default for POLICY_AUDIT_SID_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLICY_AUDIT_SUBCATEGORIES_INFO {
    pub MaximumSubCategoryCount: u32,
    pub EventAuditingOptions: *mut u32,
}
impl Default for POLICY_AUDIT_SUBCATEGORIES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_CREATE_ACCOUNT: i32 = 16i32;
pub const POLICY_CREATE_PRIVILEGE: i32 = 64i32;
pub const POLICY_CREATE_SECRET: i32 = 32i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_DEFAULT_QUOTA_INFO {
    pub QuotaLimits: super::super::QUOTA_LIMITS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_DNS_DOMAIN_INFO {
    pub Name: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub DnsForestName: LSA_UNICODE_STRING,
    pub DomainGuid: windows_core::GUID,
    pub Sid: super::super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLICY_DOMAIN_EFS_INFO {
    pub InfoLength: u32,
    pub EfsBlob: *mut u8,
}
impl Default for POLICY_DOMAIN_EFS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICY_DOMAIN_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    pub AuthenticationOptions: u32,
    pub MaxServiceTicketAge: i64,
    pub MaxTicketAge: i64,
    pub MaxRenewAge: i64,
    pub MaxClockSkew: i64,
    pub Reserved: i64,
}
pub const POLICY_GET_PRIVATE_INFORMATION: i32 = 4i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICY_INFORMATION_CLASS(pub i32);
pub const POLICY_KERBEROS_VALIDATE_CLIENT: u32 = 128u32;
pub const POLICY_LOOKUP_NAMES: i32 = 2048i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICY_LSA_SERVER_ROLE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_LSA_SERVER_ROLE_INFO {
    pub LsaServerRole: POLICY_LSA_SERVER_ROLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_MACHINE_ACCT_INFO {
    pub Rid: u32,
    pub Sid: super::super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_MACHINE_ACCT_INFO2 {
    pub Rid: u32,
    pub Sid: super::super::PSID,
    pub ObjectGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_MODIFICATION_INFO {
    pub ModifiedId: i64,
    pub DatabaseCreationTime: i64,
}
pub const POLICY_NOTIFICATION: i32 = 4096i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICY_NOTIFICATION_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_PD_ACCOUNT_INFO {
    pub Name: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_PRIMARY_DOMAIN_INFO {
    pub Name: LSA_UNICODE_STRING,
    pub Sid: super::super::PSID,
}
pub const POLICY_QOS_ALLOW_LOCAL_ROOT_CERT_STORE: u32 = 32u32;
pub const POLICY_QOS_DHCP_SERVER_ALLOWED: u32 = 128u32;
pub const POLICY_QOS_INBOUND_CONFIDENTIALITY: u32 = 16u32;
pub const POLICY_QOS_INBOUND_INTEGRITY: u32 = 8u32;
pub const POLICY_QOS_OUTBOUND_CONFIDENTIALITY: u32 = 4u32;
pub const POLICY_QOS_OUTBOUND_INTEGRITY: u32 = 2u32;
pub const POLICY_QOS_RAS_SERVER_ALLOWED: u32 = 64u32;
pub const POLICY_QOS_SCHANNEL_REQUIRED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_REPLICA_SOURCE_INFO {
    pub ReplicaSource: LSA_UNICODE_STRING,
    pub ReplicaAccountName: LSA_UNICODE_STRING,
}
pub const POLICY_SERVER_ADMIN: i32 = 1024i32;
pub const POLICY_SET_AUDIT_REQUIREMENTS: i32 = 256i32;
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: i32 = 128i32;
pub const POLICY_TRUST_ADMIN: i32 = 8i32;
pub const POLICY_VIEW_AUDIT_INFORMATION: i32 = 2i32;
pub const POLICY_VIEW_LOCAL_INFORMATION: i32 = 1i32;
pub const PRIMARY_CRED_ARSO_LOGON: u32 = 2097152u32;
pub const PRIMARY_CRED_AUTH_ID: u32 = 512u32;
pub const PRIMARY_CRED_CACHED_INTERACTIVE_LOGON: u32 = 262144u32;
pub const PRIMARY_CRED_CACHED_LOGON: u32 = 8u32;
pub const PRIMARY_CRED_CLEAR_PASSWORD: u32 = 1u32;
pub const PRIMARY_CRED_DO_NOT_SPLIT: u32 = 1024u32;
pub const PRIMARY_CRED_ENCRYPTED_CREDGUARD_PASSWORD: u32 = 131072u32;
pub const PRIMARY_CRED_ENTERPRISE_INTERNET_USER: u32 = 65536u32;
pub const PRIMARY_CRED_EX: u32 = 4096u32;
pub const PRIMARY_CRED_FOR_PASSWORD_CHANGE: u32 = 8388608u32;
pub const PRIMARY_CRED_INTERACTIVE_FIDO_LOGON: u32 = 1048576u32;
pub const PRIMARY_CRED_INTERACTIVE_NGC_LOGON: u32 = 524288u32;
pub const PRIMARY_CRED_INTERACTIVE_SMARTCARD_LOGON: u32 = 64u32;
pub const PRIMARY_CRED_INTERNET_USER: u32 = 256u32;
pub const PRIMARY_CRED_LOGON_LUA: u32 = 32u32;
pub const PRIMARY_CRED_LOGON_NO_TCB: u32 = 16u32;
pub const PRIMARY_CRED_LOGON_PACKAGE_SHIFT: u32 = 24u32;
pub const PRIMARY_CRED_OWF_PASSWORD: u32 = 2u32;
pub const PRIMARY_CRED_PACKAGE_MASK: u32 = 4278190080u32;
pub const PRIMARY_CRED_PACKED_CREDS: u32 = 32768u32;
pub const PRIMARY_CRED_PROTECTED_USER: u32 = 2048u32;
pub const PRIMARY_CRED_REFRESH_NEEDED: u32 = 128u32;
pub const PRIMARY_CRED_RESTRICTED_TS: u32 = 16384u32;
pub const PRIMARY_CRED_SUPPLEMENTAL: u32 = 4194304u32;
pub const PRIMARY_CRED_TRANSFER: u32 = 8192u32;
pub const PRIMARY_CRED_UPDATE: u32 = 4u32;
pub type PSAM_CREDENTIAL_UPDATE_FREE_ROUTINE = Option<unsafe extern "system" fn(p: *const core::ffi::c_void)>;
pub type PSAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(clearpassword: *const LSA_UNICODE_STRING, oldcredentials: *const core::ffi::c_void, oldcredentialsize: u32, useraccountcontrol: u32, upn: *const LSA_UNICODE_STRING, username: *const LSA_UNICODE_STRING, netbiosdomainname: *const LSA_UNICODE_STRING, dnsdomainname: *const LSA_UNICODE_STRING, newcredentials: *mut *mut core::ffi::c_void, newcredentialsize: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type PSAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE = Option<unsafe extern "system" fn(table: *mut SAM_REGISTER_MAPPING_TABLE) -> super::super::super::Foundation::NTSTATUS>;
pub type PSAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE = Option<unsafe extern "system" fn(credentialname: *mut LSA_UNICODE_STRING) -> bool>;
pub type PSAM_INIT_NOTIFICATION_ROUTINE = Option<unsafe extern "system" fn() -> bool>;
pub type PSAM_PASSWORD_FILTER_ROUTINE = Option<unsafe extern "system" fn(accountname: *const LSA_UNICODE_STRING, fullname: *const LSA_UNICODE_STRING, password: *const LSA_UNICODE_STRING, setoperation: bool) -> bool>;
pub type PSAM_PASSWORD_NOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(username: *mut LSA_UNICODE_STRING, relativeid: u32, newpassword: *mut LSA_UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PctPublicKey {
    pub Type: u32,
    pub cbKey: u32,
    pub pKey: [u8; 1],
}
impl Default for PctPublicKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Pku2uCertificateS4ULogon: PKU2U_LOGON_SUBMIT_TYPE = PKU2U_LOGON_SUBMIT_TYPE(14i32);
pub const PolicyAccountDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(5i32);
pub const PolicyAuditEventsInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(2i32);
pub const PolicyAuditFullQueryInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(11i32);
pub const PolicyAuditFullSetInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(10i32);
pub const PolicyAuditLogInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(1i32);
pub const PolicyDefaultQuotaInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(8i32);
pub const PolicyDnsDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(12i32);
pub const PolicyDnsDomainInformationInt: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(13i32);
pub const PolicyDomainEfsInformation: POLICY_DOMAIN_INFORMATION_CLASS = POLICY_DOMAIN_INFORMATION_CLASS(2i32);
pub const PolicyDomainKerberosTicketInformation: POLICY_DOMAIN_INFORMATION_CLASS = POLICY_DOMAIN_INFORMATION_CLASS(3i32);
pub const PolicyLastEntry: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(17i32);
pub const PolicyLocalAccountDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(14i32);
pub const PolicyLsaServerRoleInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(6i32);
pub const PolicyMachineAccountInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(15i32);
pub const PolicyMachineAccountInformation2: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(16i32);
pub const PolicyModificationInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(9i32);
pub const PolicyNotifyAccountDomainInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(2i32);
pub const PolicyNotifyAuditEventsInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(1i32);
pub const PolicyNotifyDnsDomainInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(4i32);
pub const PolicyNotifyDomainEfsInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(5i32);
pub const PolicyNotifyDomainKerberosTicketInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(6i32);
pub const PolicyNotifyGlobalSaclInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(8i32);
pub const PolicyNotifyMachineAccountPasswordInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(7i32);
pub const PolicyNotifyMax: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(9i32);
pub const PolicyNotifyServerRoleInformation: POLICY_NOTIFICATION_INFORMATION_CLASS = POLICY_NOTIFICATION_INFORMATION_CLASS(3i32);
pub const PolicyPdAccountInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(4i32);
pub const PolicyPrimaryDomainInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(3i32);
pub const PolicyReplicaSourceInformation: POLICY_INFORMATION_CLASS = POLICY_INFORMATION_CLASS(7i32);
pub const PolicyServerRoleBackup: POLICY_LSA_SERVER_ROLE = POLICY_LSA_SERVER_ROLE(2i32);
pub const PolicyServerRolePrimary: POLICY_LSA_SERVER_ROLE = POLICY_LSA_SERVER_ROLE(3i32);
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_EX_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CONTEXT_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_CREDENTIALS_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type QUERY_SECURITY_CONTEXT_TOKEN_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type QUERY_SECURITY_PACKAGE_INFO_FN_A = Option<unsafe extern "system" fn(param0: *mut i8, param1: *mut *mut SecPkgInfoA) -> windows_core::HRESULT>;
pub type QUERY_SECURITY_PACKAGE_INFO_FN_W = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut *mut SecPkgInfoW) -> windows_core::HRESULT>;
pub const RCRED_CRED_EXISTS: u32 = 1u32;
pub const RCRED_STATUS_NOCRED: u32 = 0u32;
pub const RCRED_STATUS_UNKNOWN_ISSUER: u32 = 2u32;
#[cfg(feature = "Win32_Security_Credentials")]
pub type REVERT_SECURITY_CONTEXT_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle) -> windows_core::HRESULT>;
pub const RTL_ENCRYPT_MEMORY_SIZE: u32 = 8u32;
pub const RTL_ENCRYPT_OPTION_CROSS_PROCESS: u32 = 1u32;
pub const RTL_ENCRYPT_OPTION_FOR_SYSTEM: u32 = 4u32;
pub const RTL_ENCRYPT_OPTION_SAME_LOGON: u32 = 2u32;
pub const SAM_CREDENTIAL_UPDATE_FREE_ROUTINE: windows_core::PCSTR = windows_core::s!("CredentialUpdateFree");
pub const SAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE: windows_core::PCSTR = windows_core::s!("CredentialUpdateNotify");
pub const SAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE: windows_core::PCSTR = windows_core::s!("RegisterMappedEntrypoints");
pub const SAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE: windows_core::PCSTR = windows_core::s!("CredentialUpdateRegister");
pub const SAM_DAYS_PER_WEEK: u32 = 7u32;
pub const SAM_INIT_NOTIFICATION_ROUTINE: windows_core::PCSTR = windows_core::s!("InitializeChangeNotify");
pub const SAM_PASSWORD_CHANGE_NOTIFY_ROUTINE: windows_core::PCSTR = windows_core::s!("PasswordChangeNotify");
pub const SAM_PASSWORD_FILTER_ROUTINE: windows_core::PCSTR = windows_core::s!("PasswordFilter");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SAM_REGISTER_MAPPING_ELEMENT {
    pub Original: windows_core::PSTR,
    pub Mapped: windows_core::PSTR,
    pub Continuable: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAM_REGISTER_MAPPING_LIST {
    pub Count: u32,
    pub Elements: *mut SAM_REGISTER_MAPPING_ELEMENT,
}
impl Default for SAM_REGISTER_MAPPING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAM_REGISTER_MAPPING_TABLE {
    pub Count: u32,
    pub Lists: *mut SAM_REGISTER_MAPPING_LIST,
}
impl Default for SAM_REGISTER_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SASL_AUTHZID_STATE(pub i32);
pub const SASL_OPTION_AUTHZ_PROCESSING: u32 = 4u32;
pub const SASL_OPTION_AUTHZ_STRING: u32 = 3u32;
pub const SASL_OPTION_RECV_SIZE: u32 = 2u32;
pub const SASL_OPTION_SEND_SIZE: u32 = 1u32;
pub const SCHANNEL_ALERT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCHANNEL_ALERT_TOKEN {
    pub dwTokenType: u32,
    pub dwAlertType: SCHANNEL_ALERT_TOKEN_ALERT_TYPE,
    pub dwAlertNumber: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHANNEL_ALERT_TOKEN_ALERT_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CERT_HASH {
    pub dwLength: u32,
    pub dwFlags: u32,
    pub hProv: usize,
    pub ShaHash: [u8; 20],
}
impl Default for SCHANNEL_CERT_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CERT_HASH_STORE {
    pub dwLength: u32,
    pub dwFlags: u32,
    pub hProv: usize,
    pub ShaHash: [u8; 20],
    pub pwszStoreName: [u16; 128],
}
impl Default for SCHANNEL_CERT_HASH_STORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CLIENT_SIGNATURE {
    pub cbLength: u32,
    pub aiHash: super::super::Cryptography::ALG_ID,
    pub cbHash: u32,
    pub HashValue: [u8; 36],
    pub CertThumbprint: [u8; 20],
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SCHANNEL_CLIENT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCHANNEL_CRED {
    pub dwVersion: u32,
    pub cCreds: u32,
    pub paCred: *mut *mut super::super::Cryptography::CERT_CONTEXT,
    pub hRootStore: super::super::Cryptography::HCERTSTORE,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut super::super::Cryptography::ALG_ID,
    pub grbitEnabledProtocols: u32,
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
    pub dwSessionLifespan: u32,
    pub dwFlags: SCHANNEL_CRED_FLAGS,
    pub dwCredFormat: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SCHANNEL_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHANNEL_CRED_FLAGS(pub u32);
impl SCHANNEL_CRED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SCHANNEL_CRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SCHANNEL_CRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SCHANNEL_CRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SCHANNEL_CRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SCHANNEL_CRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SCHANNEL_CRED_VERSION: u32 = 4u32;
pub const SCHANNEL_NAME: windows_core::PCWSTR = windows_core::w!("Schannel");
pub const SCHANNEL_NAME_A: windows_core::PCSTR = windows_core::s!("Schannel");
pub const SCHANNEL_NAME_W: windows_core::PCWSTR = windows_core::w!("Schannel");
pub const SCHANNEL_RENEGOTIATE: u32 = 0u32;
pub const SCHANNEL_SECRET_PRIVKEY: u32 = 2u32;
pub const SCHANNEL_SECRET_TYPE_CAPI: u32 = 1u32;
pub const SCHANNEL_SESSION: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCHANNEL_SESSION_TOKEN {
    pub dwTokenType: u32,
    pub dwFlags: SCHANNEL_SESSION_TOKEN_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCHANNEL_SESSION_TOKEN_FLAGS(pub u32);
pub const SCHANNEL_SHUTDOWN: u32 = 1u32;
pub const SCH_ALLOW_NULL_ENCRYPTION: u32 = 33554432u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_CRED {
    pub dwVersion: u32,
    pub cCreds: u32,
    pub paSecret: *mut *mut core::ffi::c_void,
    pub paPublic: *mut *mut core::ffi::c_void,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
}
impl Default for SCH_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_CREDENTIALS {
    pub dwVersion: u32,
    pub dwCredFormat: u32,
    pub cCreds: u32,
    pub paCred: *mut *mut super::super::Cryptography::CERT_CONTEXT,
    pub hRootStore: super::super::Cryptography::HCERTSTORE,
    pub cMappers: u32,
    pub aphMappers: *mut *mut _HMAPPER,
    pub dwSessionLifespan: u32,
    pub dwFlags: u32,
    pub cTlsParameters: u32,
    pub pTlsParameters: *mut TLS_PARAMETERS,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SCH_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCH_CREDENTIALS_VERSION: u32 = 5u32;
pub const SCH_CRED_AUTO_CRED_VALIDATION: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(32u32);
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL: u32 = 32768u32;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(131072u32);
pub const SCH_CRED_CERT_CONTEXT: u32 = 3u32;
pub const SCH_CRED_DEFERRED_CRED_VALIDATION: u32 = 67108864u32;
pub const SCH_CRED_DISABLE_RECONNECTS: u32 = 128u32;
pub const SCH_CRED_FORMAT_CERT_CONTEXT: u32 = 0u32;
pub const SCH_CRED_FORMAT_CERT_HASH: u32 = 1u32;
pub const SCH_CRED_FORMAT_CERT_HASH_STORE: u32 = 2u32;
pub const SCH_CRED_IGNORE_NO_REVOCATION_CHECK: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(2048u32);
pub const SCH_CRED_IGNORE_REVOCATION_OFFLINE: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(4096u32);
pub const SCH_CRED_MANUAL_CRED_VALIDATION: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(8u32);
pub const SCH_CRED_MAX_STORE_NAME_SIZE: u32 = 128u32;
pub const SCH_CRED_MAX_SUPPORTED_ALGS: u32 = 256u32;
pub const SCH_CRED_MAX_SUPPORTED_ALPN_IDS: u32 = 16u32;
pub const SCH_CRED_MAX_SUPPORTED_CERTS: u32 = 100u32;
pub const SCH_CRED_MAX_SUPPORTED_CHAINING_MODES: u32 = 16u32;
pub const SCH_CRED_MAX_SUPPORTED_CRYPTO_SETTINGS: u32 = 16u32;
pub const SCH_CRED_MAX_SUPPORTED_PARAMETERS: u32 = 16u32;
pub const SCH_CRED_MEMORY_STORE_CERT: u32 = 65536u32;
pub const SCH_CRED_NO_DEFAULT_CREDS: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(16u32);
pub const SCH_CRED_NO_SERVERNAME_CHECK: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(4u32);
pub const SCH_CRED_NO_SYSTEM_MAPPER: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_CRED_PUBLIC_CERTCHAIN {
    pub dwType: u32,
    pub cbCertChain: u32,
    pub pCertChain: *mut u8,
}
impl Default for SCH_CRED_PUBLIC_CERTCHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCH_CRED_RESTRICTED_ROOTS: u32 = 8192u32;
pub const SCH_CRED_REVOCATION_CHECK_CACHE_ONLY: u32 = 16384u32;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(512u32);
pub const SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(1024u32);
pub const SCH_CRED_REVOCATION_CHECK_END_CERT: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(256u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCH_CRED_SECRET_CAPI {
    pub dwType: u32,
    pub hProv: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_CRED_SECRET_PRIVKEY {
    pub dwType: u32,
    pub pPrivateKey: *mut u8,
    pub cbPrivateKey: u32,
    pub pszPassword: windows_core::PSTR,
}
impl Default for SCH_CRED_SECRET_PRIVKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCH_CRED_SNI_CREDENTIAL: u32 = 524288u32;
pub const SCH_CRED_SNI_ENABLE_OCSP: u32 = 1048576u32;
pub const SCH_CRED_USE_DEFAULT_CREDS: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(64u32);
pub const SCH_CRED_V1: u32 = 1u32;
pub const SCH_CRED_V2: u32 = 2u32;
pub const SCH_CRED_V3: u32 = 3u32;
pub const SCH_CRED_VERSION: u32 = 2u32;
pub const SCH_CRED_X509_CAPI: u32 = 2u32;
pub const SCH_CRED_X509_CERTCHAIN: u32 = 1u32;
pub const SCH_DISABLE_RECONNECTS: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(128u32);
pub const SCH_EXTENSIONS_OPTIONS_NONE: SchGetExtensionsOptions = SchGetExtensionsOptions(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCH_EXTENSION_DATA {
    pub ExtensionType: u16,
    pub pExtData: *const u8,
    pub cbExtData: u32,
}
impl Default for SCH_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCH_MACHINE_CERT_HASH: u32 = 1u32;
pub const SCH_MAX_EXT_SUBSCRIPTIONS: u32 = 2u32;
pub const SCH_NO_RECORD_HEADER: SchGetExtensionsOptions = SchGetExtensionsOptions(1i32);
pub const SCH_SEND_AUX_RECORD: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(2097152u32);
pub const SCH_SEND_ROOT_CERT: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(262144u32);
pub const SCH_USE_DTLS_ONLY: u32 = 16777216u32;
pub const SCH_USE_PRESHAREDKEY_ONLY: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(8388608u32);
pub const SCH_USE_STRONG_CRYPTO: SCHANNEL_CRED_FLAGS = SCHANNEL_CRED_FLAGS(4194304u32);
pub const SECBUFFER_ALERT: u32 = 17u32;
pub const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18u32;
pub const SECBUFFER_ATTRMASK: u32 = 4026531840u32;
pub const SECBUFFER_CERTIFICATE_REQUEST_CONTEXT: u32 = 29u32;
pub const SECBUFFER_CHANGE_PASS_RESPONSE: u32 = 15u32;
pub const SECBUFFER_CHANNEL_BINDINGS: u32 = 14u32;
pub const SECBUFFER_CHANNEL_BINDINGS_RESULT: u32 = 30u32;
pub const SECBUFFER_DATA: u32 = 1u32;
pub const SECBUFFER_DTLS_MTU: u32 = 24u32;
pub const SECBUFFER_EMPTY: u32 = 0u32;
pub const SECBUFFER_EXTRA: u32 = 5u32;
pub const SECBUFFER_FLAGS: u32 = 27u32;
pub const SECBUFFER_KERNEL_MAP: u32 = 536870912u32;
pub const SECBUFFER_MECHLIST: u32 = 11u32;
pub const SECBUFFER_MECHLIST_SIGNATURE: u32 = 12u32;
pub const SECBUFFER_MISSING: u32 = 4u32;
pub const SECBUFFER_NEGOTIATION_INFO: u32 = 8u32;
pub const SECBUFFER_PADDING: u32 = 9u32;
pub const SECBUFFER_PKG_PARAMS: u32 = 3u32;
pub const SECBUFFER_PRESHARED_KEY: u32 = 22u32;
pub const SECBUFFER_PRESHARED_KEY_IDENTITY: u32 = 23u32;
pub const SECBUFFER_READONLY: u32 = 2147483648u32;
pub const SECBUFFER_READONLY_WITH_CHECKSUM: u32 = 268435456u32;
pub const SECBUFFER_RESERVED: u32 = 1610612736u32;
pub const SECBUFFER_SEND_GENERIC_TLS_EXTENSION: u32 = 25u32;
pub const SECBUFFER_SRTP_MASTER_KEY_IDENTIFIER: u32 = 20u32;
pub const SECBUFFER_SRTP_PROTECTION_PROFILES: u32 = 19u32;
pub const SECBUFFER_STREAM: u32 = 10u32;
pub const SECBUFFER_STREAM_HEADER: u32 = 7u32;
pub const SECBUFFER_STREAM_TRAILER: u32 = 6u32;
pub const SECBUFFER_SUBSCRIBE_GENERIC_TLS_EXTENSION: u32 = 26u32;
pub const SECBUFFER_TARGET: u32 = 13u32;
pub const SECBUFFER_TARGET_HOST: u32 = 16u32;
pub const SECBUFFER_TOKEN: u32 = 2u32;
pub const SECBUFFER_TOKEN_BINDING: u32 = 21u32;
pub const SECBUFFER_TRAFFIC_SECRETS: u32 = 28u32;
pub const SECBUFFER_UNMAPPED: u32 = 1073741824u32;
pub const SECBUFFER_VERSION: u32 = 0u32;
pub const SECPKGCONTEXT_CIPHERINFO_V1: u32 = 1u32;
pub const SECPKGCONTEXT_CONNECTION_INFO_EX_V1: u32 = 1u32;
pub const SECPKG_ANSI_ATTRIBUTE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_APP_MODE_INFO {
    pub UserFunction: u32,
    pub Argument1: usize,
    pub Argument2: usize,
    pub UserData: SecBuffer,
    pub ReturnToLsa: bool,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_ATTR(pub u32);
pub const SECPKG_ATTR_ACCESS_TOKEN: SECPKG_ATTR = SECPKG_ATTR(18u32);
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 35u32;
pub const SECPKG_ATTR_APP_DATA: SECPKG_ATTR = SECPKG_ATTR(94u32);
pub const SECPKG_ATTR_AUTHENTICATION_ID: u32 = 20u32;
pub const SECPKG_ATTR_AUTHORITY: SECPKG_ATTR = SECPKG_ATTR(6u32);
pub const SECPKG_ATTR_CC_POLICY_RESULT: u32 = 97u32;
pub const SECPKG_ATTR_CERT_CHECK_RESULT: u32 = 113u32;
pub const SECPKG_ATTR_CERT_CHECK_RESULT_INPROC: u32 = 114u32;
pub const SECPKG_ATTR_CERT_TRUST_STATUS: SECPKG_ATTR = SECPKG_ATTR(2147483780u32);
pub const SECPKG_ATTR_CIPHER_INFO: u32 = 100u32;
pub const SECPKG_ATTR_CIPHER_STRENGTHS: u32 = 87u32;
pub const SECPKG_ATTR_CLIENT_CERT_POLICY: u32 = 96u32;
pub const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: SECPKG_ATTR = SECPKG_ATTR(27u32);
pub const SECPKG_ATTR_CONNECTION_INFO: SECPKG_ATTR = SECPKG_ATTR(90u32);
pub const SECPKG_ATTR_CONNECTION_INFO_EX: u32 = 110u32;
pub const SECPKG_ATTR_CONTEXT_DELETED: u32 = 33u32;
pub const SECPKG_ATTR_CREDENTIAL_NAME: u32 = 16u32;
pub const SECPKG_ATTR_CREDS: SECPKG_ATTR = SECPKG_ATTR(2147483776u32);
pub const SECPKG_ATTR_CREDS_2: SECPKG_ATTR = SECPKG_ATTR(2147483782u32);
pub const SECPKG_ATTR_C_ACCESS_TOKEN: SECPKG_ATTR = SECPKG_ATTR(2147483666u32);
pub const SECPKG_ATTR_C_FULL_ACCESS_TOKEN: SECPKG_ATTR = SECPKG_ATTR(2147483778u32);
pub const SECPKG_ATTR_DCE_INFO: SECPKG_ATTR = SECPKG_ATTR(3u32);
pub const SECPKG_ATTR_DTLS_MTU: SECPKG_ATTR = SECPKG_ATTR(34u32);
pub const SECPKG_ATTR_EAP_KEY_BLOCK: SECPKG_ATTR = SECPKG_ATTR(91u32);
pub const SECPKG_ATTR_EAP_PRF_INFO: SECPKG_ATTR = SECPKG_ATTR(101u32);
pub const SECPKG_ATTR_EARLY_START: SECPKG_ATTR = SECPKG_ATTR(105u32);
pub const SECPKG_ATTR_ENDPOINT_BINDINGS: SECPKG_ATTR = SECPKG_ATTR(26u32);
pub const SECPKG_ATTR_FLAGS: SECPKG_ATTR = SECPKG_ATTR(14u32);
pub const SECPKG_ATTR_ISSUER_LIST: u32 = 80u32;
pub const SECPKG_ATTR_ISSUER_LIST_EX: SECPKG_ATTR = SECPKG_ATTR(89u32);
pub const SECPKG_ATTR_IS_LOOPBACK: u32 = 37u32;
pub const SECPKG_ATTR_KEYING_MATERIAL: u32 = 107u32;
pub const SECPKG_ATTR_KEYING_MATERIAL_INFO: SECPKG_ATTR = SECPKG_ATTR(106u32);
pub const SECPKG_ATTR_KEYING_MATERIAL_INPROC: u32 = 112u32;
pub const SECPKG_ATTR_KEYING_MATERIAL_TOKEN_BINDING: u32 = 111u32;
pub const SECPKG_ATTR_KEY_INFO: SECPKG_ATTR = SECPKG_ATTR(5u32);
pub const SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS: SECPKG_ATTR = SECPKG_ATTR(30u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_ATTR_LCT_STATUS(pub i32);
pub const SECPKG_ATTR_LIFESPAN: SECPKG_ATTR = SECPKG_ATTR(2u32);
pub const SECPKG_ATTR_LOCAL_CERT_CONTEXT: SECPKG_ATTR = SECPKG_ATTR(84u32);
pub const SECPKG_ATTR_LOCAL_CERT_INFO: u32 = 99u32;
pub const SECPKG_ATTR_LOCAL_CRED: SECPKG_ATTR = SECPKG_ATTR(82u32);
pub const SECPKG_ATTR_LOGOFF_TIME: u32 = 21u32;
pub const SECPKG_ATTR_MAPPED_CRED_ATTR: u32 = 92u32;
pub const SECPKG_ATTR_NAMES: SECPKG_ATTR = SECPKG_ATTR(1u32);
pub const SECPKG_ATTR_NATIVE_NAMES: SECPKG_ATTR = SECPKG_ATTR(13u32);
pub const SECPKG_ATTR_NEGOTIATED_TLS_EXTENSIONS: u32 = 36u32;
pub const SECPKG_ATTR_NEGOTIATION_INFO: SECPKG_ATTR = SECPKG_ATTR(12u32);
pub const SECPKG_ATTR_NEGOTIATION_PACKAGE: SECPKG_ATTR = SECPKG_ATTR(2147483777u32);
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: u32 = 1u32;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: u32 = 2u32;
pub const SECPKG_ATTR_NEGO_KEYS: u32 = 22u32;
pub const SECPKG_ATTR_NEGO_PKG_INFO: u32 = 31u32;
pub const SECPKG_ATTR_NEGO_STATUS: u32 = 32u32;
pub const SECPKG_ATTR_PACKAGE_INFO: SECPKG_ATTR = SECPKG_ATTR(10u32);
pub const SECPKG_ATTR_PASSWORD_EXPIRY: SECPKG_ATTR = SECPKG_ATTR(8u32);
pub const SECPKG_ATTR_PROMPTING_NEEDED: u32 = 24u32;
pub const SECPKG_ATTR_PROTO_INFO: u32 = 7u32;
pub const SECPKG_ATTR_REMOTE_CERTIFICATES: u32 = 95u32;
pub const SECPKG_ATTR_REMOTE_CERT_CHAIN: u32 = 103u32;
pub const SECPKG_ATTR_REMOTE_CERT_CONTEXT: SECPKG_ATTR = SECPKG_ATTR(83u32);
pub const SECPKG_ATTR_REMOTE_CRED: u32 = 81u32;
pub const SECPKG_ATTR_ROOT_STORE: SECPKG_ATTR = SECPKG_ATTR(85u32);
pub const SECPKG_ATTR_SASL_CONTEXT: u32 = 65536u32;
pub const SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT: u32 = 117u32;
pub const SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT_INPROC: u32 = 116u32;
pub const SECPKG_ATTR_SERVER_AUTH_FLAGS: SECPKG_ATTR = SECPKG_ATTR(2147483779u32);
pub const SECPKG_ATTR_SESSION_INFO: SECPKG_ATTR = SECPKG_ATTR(93u32);
pub const SECPKG_ATTR_SESSION_KEY: SECPKG_ATTR = SECPKG_ATTR(9u32);
pub const SECPKG_ATTR_SESSION_TICKET_KEYS: u32 = 115u32;
pub const SECPKG_ATTR_SIZES: SECPKG_ATTR = SECPKG_ATTR(0u32);
pub const SECPKG_ATTR_SRTP_PARAMETERS: u32 = 108u32;
pub const SECPKG_ATTR_STREAM_SIZES: SECPKG_ATTR = SECPKG_ATTR(4u32);
pub const SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES: SECPKG_ATTR = SECPKG_ATTR(124u32);
pub const SECPKG_ATTR_SUPPORTED_ALGS: u32 = 86u32;
pub const SECPKG_ATTR_SUPPORTED_PROTOCOLS: u32 = 88u32;
pub const SECPKG_ATTR_SUPPORTED_SIGNATURES: SECPKG_ATTR = SECPKG_ATTR(102u32);
pub const SECPKG_ATTR_TARGET: u32 = 19u32;
pub const SECPKG_ATTR_TARGET_INFORMATION: SECPKG_ATTR = SECPKG_ATTR(17u32);
pub const SECPKG_ATTR_THUNK_ALL: u32 = 65536u32;
pub const SECPKG_ATTR_TOKEN_BINDING: u32 = 109u32;
pub const SECPKG_ATTR_UI_INFO: u32 = 104u32;
pub const SECPKG_ATTR_UNIQUE_BINDINGS: SECPKG_ATTR = SECPKG_ATTR(25u32);
pub const SECPKG_ATTR_USER_FLAGS: u32 = 11u32;
pub const SECPKG_ATTR_USE_NCRYPT: u32 = 98u32;
pub const SECPKG_ATTR_USE_VALIDATED: u32 = 15u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_BYTE_VECTOR {
    pub ByteArrayOffset: u32,
    pub ByteArrayLength: u16,
}
pub const SECPKG_CALLFLAGS_APPCONTAINER: u32 = 1u32;
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: u32 = 2u32;
pub const SECPKG_CALLFLAGS_APPCONTAINER_UPNCAPABLE: u32 = 8u32;
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: u32 = 4u32;
pub const SECPKG_CALL_ANSI: u32 = 2u32;
pub const SECPKG_CALL_ASYNC_UPDATE: u32 = 4096u32;
pub const SECPKG_CALL_BUFFER_MARSHAL: u32 = 65536u32;
pub const SECPKG_CALL_CLEANUP: u32 = 32u32;
pub const SECPKG_CALL_CLOUDAP_CONNECT: u32 = 262144u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_CALL_INFO {
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub Attributes: u32,
    pub CallCount: u32,
    pub MechOid: *mut core::ffi::c_void,
}
impl Default for SECPKG_CALL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_CALL_IN_PROC: u32 = 16u32;
pub const SECPKG_CALL_IS_TCB: u32 = 512u32;
pub const SECPKG_CALL_KERNEL_MODE: u32 = 1u32;
pub const SECPKG_CALL_NEGO: u32 = 16384u32;
pub const SECPKG_CALL_NEGO_EXTENDER: u32 = 32768u32;
pub const SECPKG_CALL_NETWORK_ONLY: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_MESSAGE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    pub MessageType: u32,
    pub Flags: u32,
    pub DomainName: LSA_UNICODE_STRING,
    pub DcName: LSA_UNICODE_STRING,
    pub DcFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    pub MessageType: u32,
    pub OriginLogonId: super::super::super::Foundation::LUID,
    pub DestinationLogonId: super::super::super::Foundation::LUID,
    pub Flags: u32,
}
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_CLEANUP_CREDENTIALS: u32 = 2u32;
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_OPTIMISTIC_LOGON: u32 = 1u32;
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_TO_SSO_SESSION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    pub MessageType: u32,
    pub Flags: u32,
}
pub const SECPKG_CALL_PROCESS_TERM: u32 = 256u32;
pub const SECPKG_CALL_RECURSIVE: u32 = 8u32;
pub const SECPKG_CALL_SYSTEM_PROC: u32 = 8192u32;
pub const SECPKG_CALL_THREAD_TERM: u32 = 128u32;
pub const SECPKG_CALL_UNLOCK: u32 = 131072u32;
pub const SECPKG_CALL_URGENT: u32 = 4u32;
pub const SECPKG_CALL_WINLOGON: u32 = 2048u32;
pub const SECPKG_CALL_WOWA32: u32 = 262144u32;
pub const SECPKG_CALL_WOWCLIENT: u32 = 64u32;
pub const SECPKG_CALL_WOWX86: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_CLIENT_INFO {
    pub LogonId: super::super::super::Foundation::LUID,
    pub ProcessID: u32,
    pub ThreadID: u32,
    pub HasTcbPrivilege: bool,
    pub Impersonating: bool,
    pub Restricted: bool,
    pub ClientFlags: u8,
    pub ImpersonationLevel: super::super::SECURITY_IMPERSONATION_LEVEL,
    pub ClientToken: super::super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_CLIENT_INFO_EX {
    pub LogonId: super::super::super::Foundation::LUID,
    pub ProcessID: u32,
    pub ThreadID: u32,
    pub HasTcbPrivilege: bool,
    pub Impersonating: bool,
    pub Restricted: bool,
    pub ClientFlags: u8,
    pub ImpersonationLevel: super::super::SECURITY_IMPERSONATION_LEVEL,
    pub ClientToken: super::super::super::Foundation::HANDLE,
    pub IdentificationLogonId: super::super::super::Foundation::LUID,
    pub IdentificationToken: super::super::super::Foundation::HANDLE,
}
pub const SECPKG_CLIENT_PROCESS_TERMINATED: u32 = 1u32;
pub const SECPKG_CLIENT_THREAD_TERMINATED: u32 = 2u32;
pub const SECPKG_CONTEXT_EXPORT_DELETE_OLD: EXPORT_SECURITY_CONTEXT_FLAGS = EXPORT_SECURITY_CONTEXT_FLAGS(2u32);
pub const SECPKG_CONTEXT_EXPORT_RESET_NEW: EXPORT_SECURITY_CONTEXT_FLAGS = EXPORT_SECURITY_CONTEXT_FLAGS(1u32);
pub const SECPKG_CONTEXT_EXPORT_TO_KERNEL: EXPORT_SECURITY_CONTEXT_FLAGS = EXPORT_SECURITY_CONTEXT_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_CONTEXT_THUNKS {
    pub InfoLevelCount: u32,
    pub Levels: [u32; 1],
}
impl Default for SECPKG_CONTEXT_THUNKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CRED(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_CREDENTIAL {
    pub Version: u64,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub ClientProcess: u32,
    pub ClientThread: u32,
    pub LogonId: super::super::super::Foundation::LUID,
    pub ClientToken: super::super::super::Foundation::HANDLE,
    pub SessionId: u32,
    pub ModifiedId: super::super::super::Foundation::LUID,
    pub fCredentials: u32,
    pub Flags: u32,
    pub PrincipalName: SECPKG_BYTE_VECTOR,
    pub PackageList: SECPKG_BYTE_VECTOR,
    pub MarshaledSuppliedCreds: SECPKG_BYTE_VECTOR,
}
pub const SECPKG_CREDENTIAL_ATTRIBUTE: u32 = 0u32;
pub const SECPKG_CREDENTIAL_FLAGS_CALLER_HAS_TCB: u32 = 1u32;
pub const SECPKG_CREDENTIAL_FLAGS_CREDMAN_CRED: u32 = 2u32;
pub const SECPKG_CREDENTIAL_VERSION: u32 = 201u32;
pub const SECPKG_CRED_ATTR_CERT: u32 = 4u32;
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: u32 = 3u32;
pub const SECPKG_CRED_ATTR_NAMES: u32 = 1u32;
pub const SECPKG_CRED_ATTR_PAC_BYPASS: u32 = 5u32;
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: u32 = 2u32;
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: u32 = 16u32;
pub const SECPKG_CRED_BOTH: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CRED_CLASS(pub i32);
pub const SECPKG_CRED_DEFAULT: u32 = 4u32;
pub const SECPKG_CRED_INBOUND: SECPKG_CRED = SECPKG_CRED(1u32);
pub const SECPKG_CRED_OUTBOUND: SECPKG_CRED = SECPKG_CRED(2u32);
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: u32 = 32u32;
pub const SECPKG_CRED_RESERVED: u32 = 4026531840u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_DLL_FUNCTIONS {
    pub AllocateHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeHeap: PLSA_FREE_LSA_HEAP,
    pub RegisterCallback: PLSA_REGISTER_CALLBACK,
    pub LocatePackageById: PLSA_LOCATE_PKG_BY_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_EVENT_NOTIFY {
    pub EventClass: u32,
    pub Reserved: u32,
    pub EventDataSize: u32,
    pub EventData: *mut core::ffi::c_void,
    pub PackageParameter: *mut core::ffi::c_void,
}
impl Default for SECPKG_EVENT_NOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_EVENT_PACKAGE_CHANGE {
    pub ChangeType: SECPKG_PACKAGE_CHANGE_TYPE,
    pub PackageId: usize,
    pub PackageName: SECURITY_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_EVENT_ROLE_CHANGE {
    pub PreviousRole: u32,
    pub NewRole: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECPKG_EXTENDED_INFORMATION {
    pub Class: SECPKG_EXTENDED_INFORMATION_CLASS,
    pub Info: SECPKG_EXTENDED_INFORMATION_0,
}
impl Default for SECPKG_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SECPKG_EXTENDED_INFORMATION_0 {
    pub GssInfo: SECPKG_GSS_INFO,
    pub ContextThunks: SECPKG_CONTEXT_THUNKS,
    pub MutualAuthLevel: SECPKG_MUTUAL_AUTH_LEVEL,
    pub WowClientDll: SECPKG_WOW_CLIENT_DLL,
    pub ExtraOids: SECPKG_EXTRA_OIDS,
    pub Nego2Info: SECPKG_NEGO2_INFO,
}
impl Default for SECPKG_EXTENDED_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_EXTENDED_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_EXTRA_OIDS {
    pub OidCount: u32,
    pub Oids: [SECPKG_SERIALIZED_OID; 1],
}
impl Default for SECPKG_EXTRA_OIDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: u32 = 512u32;
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: u32 = 8388608u32;
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: u32 = 4194304u32;
pub const SECPKG_FLAG_APPLY_LOOPBACK: u32 = 33554432u32;
pub const SECPKG_FLAG_ASCII_BUFFERS: u32 = 16384u32;
pub const SECPKG_FLAG_CLIENT_ONLY: u32 = 64u32;
pub const SECPKG_FLAG_CONNECTION: u32 = 16u32;
pub const SECPKG_FLAG_CREDENTIAL_ISOLATION_ENABLED: u32 = 16777216u32;
pub const SECPKG_FLAG_DATAGRAM: u32 = 8u32;
pub const SECPKG_FLAG_DELEGATION: u32 = 131072u32;
pub const SECPKG_FLAG_EXTENDED_ERROR: u32 = 128u32;
pub const SECPKG_FLAG_FRAGMENT: u32 = 32768u32;
pub const SECPKG_FLAG_GSS_COMPATIBLE: u32 = 4096u32;
pub const SECPKG_FLAG_IMPERSONATION: u32 = 256u32;
pub const SECPKG_FLAG_INTEGRITY: u32 = 1u32;
pub const SECPKG_FLAG_LOGON: u32 = 8192u32;
pub const SECPKG_FLAG_MULTI_REQUIRED: u32 = 32u32;
pub const SECPKG_FLAG_MUTUAL_AUTH: u32 = 65536u32;
pub const SECPKG_FLAG_NEGOTIABLE: u32 = 2048u32;
pub const SECPKG_FLAG_NEGOTIABLE2: u32 = 2097152u32;
pub const SECPKG_FLAG_NEGO_EXTENDER: u32 = 1048576u32;
pub const SECPKG_FLAG_PRIVACY: u32 = 2u32;
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: u32 = 262144u32;
pub const SECPKG_FLAG_RESTRICTED_TOKENS: u32 = 524288u32;
pub const SECPKG_FLAG_STREAM: u32 = 1024u32;
pub const SECPKG_FLAG_TOKEN_ONLY: u32 = 4u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Security_Credentials", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_FUNCTION_TABLE {
    pub InitializePackage: PLSA_AP_INITIALIZE_PACKAGE,
    pub LogonUserA: PLSA_AP_LOGON_USER,
    pub CallPackage: PLSA_AP_CALL_PACKAGE,
    pub LogonTerminated: PLSA_AP_LOGON_TERMINATED,
    pub CallPackageUntrusted: PLSA_AP_CALL_PACKAGE,
    pub CallPackagePassthrough: PLSA_AP_CALL_PACKAGE_PASSTHROUGH,
    pub LogonUserExA: PLSA_AP_LOGON_USER_EX,
    pub LogonUserEx2: PLSA_AP_LOGON_USER_EX2,
    pub Initialize: SpInitializeFn,
    pub Shutdown: SpShutdownFn,
    pub GetInfo: SpGetInfoFn,
    pub AcceptCredentials: SpAcceptCredentialsFn,
    pub AcquireCredentialsHandleA: SpAcquireCredentialsHandleFn,
    pub QueryCredentialsAttributesA: SpQueryCredentialsAttributesFn,
    pub FreeCredentialsHandle: SpFreeCredentialsHandleFn,
    pub SaveCredentials: SpSaveCredentialsFn,
    pub GetCredentials: SpGetCredentialsFn,
    pub DeleteCredentials: SpDeleteCredentialsFn,
    pub InitLsaModeContext: SpInitLsaModeContextFn,
    pub AcceptLsaModeContext: SpAcceptLsaModeContextFn,
    pub DeleteContext: SpDeleteContextFn,
    pub ApplyControlToken: SpApplyControlTokenFn,
    pub GetUserInfo: SpGetUserInfoFn,
    pub GetExtendedInformation: SpGetExtendedInformationFn,
    pub QueryContextAttributesA: SpQueryContextAttributesFn,
    pub AddCredentialsA: SpAddCredentialsFn,
    pub SetExtendedInformation: SpSetExtendedInformationFn,
    pub SetContextAttributesA: SpSetContextAttributesFn,
    pub SetCredentialsAttributesA: SpSetCredentialsAttributesFn,
    pub ChangeAccountPasswordA: SpChangeAccountPasswordFn,
    pub QueryMetaData: SpQueryMetaDataFn,
    pub ExchangeMetaData: SpExchangeMetaDataFn,
    pub GetCredUIContext: SpGetCredUIContextFn,
    pub UpdateCredentials: SpUpdateCredentialsFn,
    pub ValidateTargetInfo: SpValidateTargetInfoFn,
    pub PostLogonUser: LSA_AP_POST_LOGON_USER,
    pub GetRemoteCredGuardLogonBuffer: SpGetRemoteCredGuardLogonBufferFn,
    pub GetRemoteCredGuardSupplementalCreds: SpGetRemoteCredGuardSupplementalCredsFn,
    pub GetTbalSupplementalCreds: SpGetTbalSupplementalCredsFn,
    pub LogonUserEx3: PLSA_AP_LOGON_USER_EX3,
    pub PreLogonUserSurrogate: PLSA_AP_PRE_LOGON_USER_SURROGATE,
    pub PostLogonUserSurrogate: PLSA_AP_POST_LOGON_USER_SURROGATE,
    pub ExtractTargetInfo: SpExtractTargetInfoFn,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_GSS_INFO {
    pub EncodedIdLength: u32,
    pub EncodedId: [u8; 4],
}
impl Default for SECPKG_GSS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_ID_NONE: u32 = 65535u32;
pub const SECPKG_INTERFACE_VERSION: u32 = 65536u32;
pub const SECPKG_INTERFACE_VERSION_10: u32 = 33554432u32;
pub const SECPKG_INTERFACE_VERSION_11: u32 = 67108864u32;
pub const SECPKG_INTERFACE_VERSION_2: u32 = 131072u32;
pub const SECPKG_INTERFACE_VERSION_3: u32 = 262144u32;
pub const SECPKG_INTERFACE_VERSION_4: u32 = 524288u32;
pub const SECPKG_INTERFACE_VERSION_5: u32 = 1048576u32;
pub const SECPKG_INTERFACE_VERSION_6: u32 = 2097152u32;
pub const SECPKG_INTERFACE_VERSION_7: u32 = 4194304u32;
pub const SECPKG_INTERFACE_VERSION_8: u32 = 8388608u32;
pub const SECPKG_INTERFACE_VERSION_9: u32 = 16777216u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_KERNEL_FUNCTIONS {
    pub AllocateHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeHeap: PLSA_FREE_LSA_HEAP,
    pub CreateContextList: PKSEC_CREATE_CONTEXT_LIST,
    pub InsertListEntry: PKSEC_INSERT_LIST_ENTRY,
    pub ReferenceListEntry: PKSEC_REFERENCE_LIST_ENTRY,
    pub DereferenceListEntry: PKSEC_DEREFERENCE_LIST_ENTRY,
    pub SerializeWinntAuthData: PKSEC_SERIALIZE_WINNT_AUTH_DATA,
    pub SerializeSchannelAuthData: PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA,
    pub LocatePackageById: PKSEC_LOCATE_PKG_BY_ID,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_KERNEL_FUNCTION_TABLE {
    pub Initialize: KspInitPackageFn,
    pub DeleteContext: KspDeleteContextFn,
    pub InitContext: KspInitContextFn,
    pub MapHandle: KspMapHandleFn,
    pub Sign: KspMakeSignatureFn,
    pub Verify: KspVerifySignatureFn,
    pub Seal: KspSealMessageFn,
    pub Unseal: KspUnsealMessageFn,
    pub GetToken: KspGetTokenFn,
    pub QueryAttributes: KspQueryAttributesFn,
    pub CompleteToken: KspCompleteTokenFn,
    pub ExportContext: SpExportSecurityContextFn,
    pub ImportContext: SpImportSecurityContextFn,
    pub SetPackagePagingMode: KspSetPagingModeFn,
    pub SerializeAuthData: KspSerializeAuthDataFn,
}
pub const SECPKG_LSAMODEINIT_NAME: windows_core::PCSTR = windows_core::s!("SpLsaModeInitialize");
pub const SECPKG_MAX_OID_LENGTH: u32 = 32u32;
pub const SECPKG_MSVAV_FLAGS_VALID: u32 = 1u32;
pub const SECPKG_MSVAV_TIMESTAMP_VALID: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_MUTUAL_AUTH_LEVEL {
    pub MutualAuthLevel: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_NAME_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_NEGO2_INFO {
    pub AuthScheme: [u8; 16],
    pub PackageFlags: u32,
}
impl Default for SECPKG_NEGO2_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_NEGOTIATION_COMPLETE: u32 = 0u32;
pub const SECPKG_NEGOTIATION_DIRECT: u32 = 3u32;
pub const SECPKG_NEGOTIATION_IN_PROGRESS: u32 = 2u32;
pub const SECPKG_NEGOTIATION_OPTIMISTIC: u32 = 1u32;
pub const SECPKG_NEGOTIATION_TRY_MULTICRED: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_NTLM_TARGETINFO {
    pub Flags: u32,
    pub MsvAvNbComputerName: windows_core::PWSTR,
    pub MsvAvNbDomainName: windows_core::PWSTR,
    pub MsvAvDnsComputerName: windows_core::PWSTR,
    pub MsvAvDnsDomainName: windows_core::PWSTR,
    pub MsvAvDnsTreeName: windows_core::PWSTR,
    pub MsvAvFlags: u32,
    pub MsvAvTimestamp: super::super::super::Foundation::FILETIME,
    pub MsvAvTargetName: windows_core::PWSTR,
}
pub const SECPKG_OPTIONS_PERMANENT: u32 = 1u32;
pub const SECPKG_OPTIONS_TYPE_LSA: SECURITY_PACKAGE_OPTIONS_TYPE = SECURITY_PACKAGE_OPTIONS_TYPE(1u32);
pub const SECPKG_OPTIONS_TYPE_SSPI: SECURITY_PACKAGE_OPTIONS_TYPE = SECURITY_PACKAGE_OPTIONS_TYPE(2u32);
pub const SECPKG_OPTIONS_TYPE_UNKNOWN: SECURITY_PACKAGE_OPTIONS_TYPE = SECURITY_PACKAGE_OPTIONS_TYPE(0u32);
pub const SECPKG_PACKAGE_CHANGE_LOAD: SECPKG_PACKAGE_CHANGE_TYPE = SECPKG_PACKAGE_CHANGE_TYPE(0u32);
pub const SECPKG_PACKAGE_CHANGE_SELECT: SECPKG_PACKAGE_CHANGE_TYPE = SECPKG_PACKAGE_CHANGE_TYPE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_PACKAGE_CHANGE_TYPE(pub u32);
pub const SECPKG_PACKAGE_CHANGE_UNLOAD: SECPKG_PACKAGE_CHANGE_TYPE = SECPKG_PACKAGE_CHANGE_TYPE(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_PARAMETERS {
    pub Version: u32,
    pub MachineState: u32,
    pub SetupMode: u32,
    pub DomainSid: super::super::PSID,
    pub DomainName: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub DomainGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_POST_LOGON_USER_INFO {
    pub Flags: u32,
    pub LogonId: super::super::super::Foundation::LUID,
    pub LinkedLogonId: super::super::super::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_PRIMARY_CRED {
    pub LogonId: super::super::super::Foundation::LUID,
    pub DownlevelName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
    pub OldPassword: LSA_UNICODE_STRING,
    pub UserSid: super::super::PSID,
    pub Flags: u32,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub Upn: LSA_UNICODE_STRING,
    pub LogonServer: LSA_UNICODE_STRING,
    pub Spare1: LSA_UNICODE_STRING,
    pub Spare2: LSA_UNICODE_STRING,
    pub Spare3: LSA_UNICODE_STRING,
    pub Spare4: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_PRIMARY_CRED_EX {
    pub LogonId: super::super::super::Foundation::LUID,
    pub DownlevelName: LSA_UNICODE_STRING,
    pub DomainName: LSA_UNICODE_STRING,
    pub Password: LSA_UNICODE_STRING,
    pub OldPassword: LSA_UNICODE_STRING,
    pub UserSid: super::super::PSID,
    pub Flags: u32,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub Upn: LSA_UNICODE_STRING,
    pub LogonServer: LSA_UNICODE_STRING,
    pub Spare1: LSA_UNICODE_STRING,
    pub Spare2: LSA_UNICODE_STRING,
    pub Spare3: LSA_UNICODE_STRING,
    pub Spare4: LSA_UNICODE_STRING,
    pub PackageId: usize,
    pub PrevLogonId: super::super::super::Foundation::LUID,
    pub FlagsEx: u32,
}
pub const SECPKG_PRIMARY_CRED_EX_FLAGS_EX_DELEGATION_TOKEN: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_REDIRECTED_LOGON_BUFFER {
    pub RedirectedLogonGuid: windows_core::GUID,
    pub RedirectedLogonHandle: super::super::super::Foundation::HANDLE,
    pub Init: PLSA_REDIRECTED_LOGON_INIT,
    pub Callback: PLSA_REDIRECTED_LOGON_CALLBACK,
    pub CleanupCallback: PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK,
    pub GetLogonCreds: PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS,
    pub GetSupplementalCreds: PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS,
    pub GetRedirectedLogonSid: PLSA_REDIRECTED_LOGON_GET_SID,
}
pub const SECPKG_REDIRECTED_LOGON_GUID_INITIALIZER: windows_core::GUID = windows_core::GUID::from_u128(0xc2be5457_82eb_483e_ae4e_7468ef14d509);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_SERIALIZED_OID {
    pub OidLength: u32,
    pub OidAttributes: u32,
    pub OidValue: [u8; 32],
}
impl Default for SECPKG_SERIALIZED_OID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_SESSIONINFO_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_SHORT_VECTOR {
    pub ShortArrayOffset: u32,
    pub ShortArrayCount: u16,
}
pub const SECPKG_STATE_CRED_ISOLATION_ENABLED: u32 = 32u32;
pub const SECPKG_STATE_DOMAIN_CONTROLLER: u32 = 4u32;
pub const SECPKG_STATE_ENCRYPTION_PERMITTED: u32 = 1u32;
pub const SECPKG_STATE_RESERVED_1: u32 = 2147483648u32;
pub const SECPKG_STATE_STANDALONE: u32 = 16u32;
pub const SECPKG_STATE_STRONG_ENCRYPTION_PERMITTED: u32 = 2u32;
pub const SECPKG_STATE_WORKSTATION: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_SUPPLEMENTAL_CRED {
    pub PackageName: LSA_UNICODE_STRING,
    pub CredentialSize: u32,
    pub Credentials: *mut u8,
}
impl Default for SECPKG_SUPPLEMENTAL_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    pub CredentialCount: u32,
    pub Credentials: [SECPKG_SUPPLEMENTAL_CRED; 1],
}
impl Default for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_SUPPLIED_CREDENTIAL {
    pub cbHeaderLength: u16,
    pub cbStructureLength: u16,
    pub UserName: SECPKG_SHORT_VECTOR,
    pub DomainName: SECPKG_SHORT_VECTOR,
    pub PackedCredentials: SECPKG_BYTE_VECTOR,
    pub CredFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_SURROGATE_LOGON {
    pub Version: u32,
    pub SurrogateLogonID: super::super::super::Foundation::LUID,
    pub EntryCount: u32,
    pub Entries: *mut SECPKG_SURROGATE_LOGON_ENTRY,
}
impl Default for SECPKG_SURROGATE_LOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECPKG_SURROGATE_LOGON_ENTRY {
    pub Type: windows_core::GUID,
    pub Data: *mut core::ffi::c_void,
}
impl Default for SECPKG_SURROGATE_LOGON_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_SURROGATE_LOGON_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_TARGETINFO {
    pub DomainSid: super::super::PSID,
    pub ComputerName: windows_core::PCWSTR,
}
pub const SECPKG_UNICODE_ATTRIBUTE: u32 = 2147483648u32;
pub const SECPKG_USERMODEINIT_NAME: windows_core::PCSTR = windows_core::s!("SpUserModeInitialize");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_USER_FUNCTION_TABLE {
    pub InstanceInit: SpInstanceInitFn,
    pub InitUserModeContext: SpInitUserModeContextFn,
    pub MakeSignature: SpMakeSignatureFn,
    pub VerifySignature: SpVerifySignatureFn,
    pub SealMessage: SpSealMessageFn,
    pub UnsealMessage: SpUnsealMessageFn,
    pub GetContextToken: SpGetContextTokenFn,
    pub QueryContextAttributesA: SpQueryContextAttributesFn,
    pub CompleteAuthToken: SpCompleteAuthTokenFn,
    pub DeleteUserModeContext: SpDeleteContextFn,
    pub FormatCredentials: SpFormatCredentialsFn,
    pub MarshallSupplementalCreds: SpMarshallSupplementalCredsFn,
    pub ExportContext: SpExportSecurityContextFn,
    pub ImportContext: SpImportSecurityContextFn,
    pub MarshalAttributeData: SpMarshalAttributeDataFn,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECPKG_WOW_CLIENT_DLL {
    pub WowClientDllPath: SECURITY_STRING,
}
pub const SECQOP_WRAP_NO_ENCRYPT: u32 = 2147483649u32;
pub const SECQOP_WRAP_OOB_DATA: u32 = 1073741824u32;
pub const SECRET_QUERY_VALUE: i32 = 2i32;
pub const SECRET_SET_VALUE: i32 = 1i32;
pub const SECURITY_ENTRYPOINT: windows_core::PCWSTR = windows_core::w!("INITSECURITYINTERFACEA");
pub const SECURITY_ENTRYPOINT16: windows_core::PCSTR = windows_core::s!("INITSECURITYINTERFACEA");
pub const SECURITY_ENTRYPOINT_ANSI: windows_core::PCWSTR = windows_core::w!("InitSecurityInterfaceW");
pub const SECURITY_ENTRYPOINT_ANSIA: windows_core::PCSTR = windows_core::s!("InitSecurityInterfaceA");
pub const SECURITY_ENTRYPOINT_ANSIW: windows_core::PCSTR = windows_core::s!("InitSecurityInterfaceW");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_LOGON_SESSION_DATA {
    pub Size: u32,
    pub LogonId: super::super::super::Foundation::LUID,
    pub UserName: LSA_UNICODE_STRING,
    pub LogonDomain: LSA_UNICODE_STRING,
    pub AuthenticationPackage: LSA_UNICODE_STRING,
    pub LogonType: u32,
    pub Session: u32,
    pub Sid: super::super::PSID,
    pub LogonTime: i64,
    pub LogonServer: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub Upn: LSA_UNICODE_STRING,
    pub UserFlags: u32,
    pub LastLogonInfo: LSA_LAST_INTER_LOGON_INFO,
    pub LogonScript: LSA_UNICODE_STRING,
    pub ProfilePath: LSA_UNICODE_STRING,
    pub HomeDirectory: LSA_UNICODE_STRING,
    pub HomeDirectoryDrive: LSA_UNICODE_STRING,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_LOGON_TYPE(pub i32);
impl SECURITY_LOGON_TYPE {
    pub const UndefinedLogonType: Self = Self(0i32);
    pub const Interactive: Self = Self(2i32);
    pub const Network: Self = Self(3i32);
    pub const Batch: Self = Self(4i32);
    pub const Service: Self = Self(5i32);
    pub const Proxy: Self = Self(6i32);
    pub const Unlock: Self = Self(7i32);
    pub const NetworkCleartext: Self = Self(8i32);
    pub const NewCredentials: Self = Self(9i32);
    pub const RemoteInteractive: Self = Self(10i32);
    pub const CachedInteractive: Self = Self(11i32);
    pub const CachedRemoteInteractive: Self = Self(12i32);
    pub const CachedUnlock: Self = Self(13i32);
}
pub const SECURITY_NATIVE_DREP: u32 = 16u32;
pub const SECURITY_NETWORK_DREP: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECURITY_PACKAGE_OPTIONS {
    pub Size: u32,
    pub Type: SECURITY_PACKAGE_OPTIONS_TYPE,
    pub Flags: u32,
    pub SignatureSize: u32,
    pub Signature: *mut core::ffi::c_void,
}
impl Default for SECURITY_PACKAGE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_PACKAGE_OPTIONS_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECURITY_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: *mut u16,
}
impl Default for SECURITY_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1u32;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_2: u32 = 2u32;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_3: u32 = 3u32;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_4: u32 = 4u32;
pub const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_5: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_USER_DATA {
    pub UserName: SECURITY_STRING,
    pub LogonDomainName: SECURITY_STRING,
    pub LogonServer: SECURITY_STRING,
    pub pSid: super::super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_APPLICATION_PROTOCOLS {
    pub ProtocolListsSize: u32,
    pub ProtocolLists: [SEC_APPLICATION_PROTOCOL_LIST; 1],
}
impl Default for SEC_APPLICATION_PROTOCOLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_APPLICATION_PROTOCOL_LIST {
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolListSize: u16,
    pub ProtocolList: [u8; 1],
}
impl Default for SEC_APPLICATION_PROTOCOL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_CERTIFICATE_REQUEST_CONTEXT {
    pub cbCertificateRequestContext: u8,
    pub rgCertificateRequestContext: [u8; 1],
}
impl Default for SEC_CERTIFICATE_REQUEST_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_CHANNEL_BINDINGS {
    pub dwInitiatorAddrType: u32,
    pub cbInitiatorLength: u32,
    pub dwInitiatorOffset: u32,
    pub dwAcceptorAddrType: u32,
    pub cbAcceptorLength: u32,
    pub dwAcceptorOffset: u32,
    pub cbApplicationDataLength: u32,
    pub dwApplicationDataOffset: u32,
}
pub const SEC_CHANNEL_BINDINGS_AUDIT_BINDINGS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_CHANNEL_BINDINGS_EX {
    pub magicNumber: u32,
    pub flags: u32,
    pub cbHeaderLength: u32,
    pub cbStructureLength: u32,
    pub dwInitiatorAddrType: u32,
    pub cbInitiatorLength: u32,
    pub dwInitiatorOffset: u32,
    pub dwAcceptorAddrType: u32,
    pub cbAcceptorLength: u32,
    pub dwAcceptorOffset: u32,
    pub cbApplicationDataLength: u32,
    pub dwApplicationDataOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_CHANNEL_BINDINGS_RESULT {
    pub flags: u32,
}
pub const SEC_CHANNEL_BINDINGS_RESULT_ABSENT: u32 = 2u32;
pub const SEC_CHANNEL_BINDINGS_RESULT_CLIENT_SUPPORT: u32 = 1u32;
pub const SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISMATCH: u32 = 4u32;
pub const SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISSING: u32 = 8u32;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID_MATCHED: u32 = 16u32;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID_MISSING: u32 = 64u32;
pub const SEC_CHANNEL_BINDINGS_RESULT_VALID_PROXY: u32 = 32u32;
pub const SEC_CHANNEL_BINDINGS_VALID_FLAGS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_DTLS_MTU {
    pub PathMTU: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_FLAGS {
    pub Flags: u64,
}
pub type SEC_GET_KEY_FN = Option<unsafe extern "system" fn(arg: *mut core::ffi::c_void, principal: *mut core::ffi::c_void, keyver: u32, key: *mut *mut core::ffi::c_void, status: *mut windows_core::HRESULT)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_NEGOTIATION_INFO {
    pub Size: u32,
    pub NameLength: u32,
    pub Name: *mut u16,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for SEC_NEGOTIATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_PRESHAREDKEY {
    pub KeySize: u16,
    pub Key: [u8; 1],
}
impl Default for SEC_PRESHAREDKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_PRESHAREDKEY_IDENTITY {
    pub KeyIdentitySize: u16,
    pub KeyIdentity: [u8; 1],
}
impl Default for SEC_PRESHAREDKEY_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_SRTP_MASTER_KEY_IDENTIFIER {
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: [u8; 1],
}
impl Default for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_SRTP_PROTECTION_PROFILES {
    pub ProfilesSize: u16,
    pub ProfilesList: [u16; 1],
}
impl Default for SEC_SRTP_PROTECTION_PROFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_TOKEN_BINDING {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: [u8; 1],
}
impl Default for SEC_TOKEN_BINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_TRAFFIC_SECRETS {
    pub SymmetricAlgId: [u16; 64],
    pub ChainingMode: [u16; 64],
    pub HashAlgId: [u16; 64],
    pub KeySize: u16,
    pub IvSize: u16,
    pub MsgSequenceStart: u16,
    pub MsgSequenceEnd: u16,
    pub TrafficSecretType: SEC_TRAFFIC_SECRET_TYPE,
    pub TrafficSecretSize: u16,
    pub TrafficSecret: [u8; 1],
}
impl Default for SEC_TRAFFIC_SECRETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEC_TRAFFIC_SECRET_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY32 {
    pub User: u32,
    pub UserLength: u32,
    pub Domain: u32,
    pub DomainLength: u32,
    pub Password: u32,
    pub PasswordLength: u32,
    pub Flags: u32,
}
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_FOR_SYSTEM: u32 = 4u32;
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_LOGON: u32 = 1u32;
pub const SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_PROCESS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_EX2 {
    pub Version: u32,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub UserOffset: u32,
    pub UserLength: u16,
    pub DomainOffset: u32,
    pub DomainLength: u16,
    pub PackedCredentialsOffset: u32,
    pub PackedCredentialsLength: u16,
    pub Flags: u32,
    pub PackageListOffset: u32,
    pub PackageListLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_EX32 {
    pub Version: u32,
    pub Length: u32,
    pub User: u32,
    pub UserLength: u32,
    pub Domain: u32,
    pub DomainLength: u32,
    pub Password: u32,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: u32,
    pub PackageListLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_EXA {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u8,
    pub PackageListLength: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_EXW {
    pub Version: u32,
    pub Length: u32,
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: *mut u16,
    pub PackageListLength: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_ID_PROVIDER: u32 = 524288u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_DOMAIN: u32 = 262144u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_USER: u32 = 131072u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_PROCESS_ENCRYPTED: u32 = 16u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_RESERVED: u32 = 65536u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 268435456u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 2147483648u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_NO_CHECKBOX: u32 = 536870912u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 2147483648u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_CHECKED: u32 = 1073741824u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_USE_MASK: u32 = 4278190080u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_ENCRYPTED: u32 = 128u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_PROTECTED: u32 = 32u32;
pub const SEC_WINNT_AUTH_IDENTITY_FLAGS_USER_PROTECTED: u32 = 64u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Rpc")]
#[derive(Clone, Copy)]
pub union SEC_WINNT_AUTH_IDENTITY_INFO {
    pub AuthIdExw: SEC_WINNT_AUTH_IDENTITY_EXW,
    pub AuthIdExa: SEC_WINNT_AUTH_IDENTITY_EXA,
    pub AuthId_a: super::super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_A,
    pub AuthId_w: super::super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W,
    pub AuthIdEx2: SEC_WINNT_AUTH_IDENTITY_EX2,
}
#[cfg(feature = "Win32_System_Rpc")]
impl Default for SEC_WINNT_AUTH_IDENTITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_MARSHALLED: u32 = 4u32;
pub const SEC_WINNT_AUTH_IDENTITY_ONLY: u32 = 8u32;
pub const SEC_WINNT_AUTH_IDENTITY_VERSION: u32 = 512u32;
pub const SEC_WINNT_AUTH_IDENTITY_VERSION_2: u32 = 513u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEND_GENERIC_TLS_EXTENSION {
    pub ExtensionType: u16,
    pub HandshakeType: u16,
    pub Flags: u32,
    pub BufferSize: u16,
    pub Buffer: [u8; 1],
}
impl Default for SEND_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SESSION_TICKET_INFO_V0: u32 = 0u32;
pub const SESSION_TICKET_INFO_VERSION: u32 = 0u32;
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CONTEXT_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CONTEXT_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_A = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Security_Credentials")]
pub type SET_CREDENTIALS_ATTRIBUTES_FN_W = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: u32, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_ADT_ACCESS_REASON {
    pub AccessMask: u32,
    pub AccessReasons: [u32; 32],
    pub ObjectTypeIndex: u32,
    pub AccessGranted: u32,
    pub SecurityDescriptor: super::super::PSECURITY_DESCRIPTOR,
}
impl Default for SE_ADT_ACCESS_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_ADT_CLAIMS {
    pub Length: u32,
    pub Claims: *mut core::ffi::c_void,
}
impl Default for SE_ADT_CLAIMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SE_ADT_OBJECT_ONLY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SE_ADT_OBJECT_TYPE {
    pub ObjectType: windows_core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
pub const SE_ADT_PARAMETERS_SELF_RELATIVE: u32 = 1u32;
pub const SE_ADT_PARAMETERS_SEND_TO_LSA: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_ADT_PARAMETER_ARRAY {
    pub CategoryId: u32,
    pub AuditId: u32,
    pub ParameterCount: u32,
    pub Length: u32,
    pub FlatSubCategoryId: u16,
    pub Type: u16,
    pub Flags: u32,
    pub Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; 32],
}
impl Default for SE_ADT_PARAMETER_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_ADT_PARAMETER_ARRAY_ENTRY {
    pub Type: SE_ADT_PARAMETER_TYPE,
    pub Length: u32,
    pub Data: [usize; 2],
    pub Address: *mut core::ffi::c_void,
}
impl Default for SE_ADT_PARAMETER_ARRAY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_ADT_PARAMETER_ARRAY_EX {
    pub CategoryId: u32,
    pub AuditId: u32,
    pub Version: u32,
    pub ParameterCount: u32,
    pub Length: u32,
    pub FlatSubCategoryId: u16,
    pub Type: u16,
    pub Flags: u32,
    pub Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; 32],
}
impl Default for SE_ADT_PARAMETER_ARRAY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SE_ADT_PARAMETER_EXTENSIBLE_AUDIT: u32 = 4u32;
pub const SE_ADT_PARAMETER_GENERIC_AUDIT: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SE_ADT_PARAMETER_TYPE(pub i32);
pub const SE_ADT_PARAMETER_WRITE_SYNCHRONOUS: u32 = 16u32;
pub const SE_ADT_POLICY_AUDIT_EVENT_TYPE_EX_BEGIN: u32 = 100u32;
pub const SE_BATCH_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeBatchLogonRight");
pub const SE_DENY_BATCH_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeDenyBatchLogonRight");
pub const SE_DENY_INTERACTIVE_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeDenyInteractiveLogonRight");
pub const SE_DENY_NETWORK_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeDenyNetworkLogonRight");
pub const SE_DENY_REMOTE_INTERACTIVE_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeDenyRemoteInteractiveLogonRight");
pub const SE_DENY_SERVICE_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeDenyServiceLogonRight");
pub const SE_INTERACTIVE_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeInteractiveLogonRight");
pub const SE_MAX_AUDIT_PARAMETERS: u32 = 32u32;
pub const SE_MAX_GENERIC_AUDIT_PARAMETERS: u32 = 28u32;
pub const SE_NETWORK_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeNetworkLogonRight");
pub const SE_REMOTE_INTERACTIVE_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeRemoteInteractiveLogonRight");
pub const SE_SERVICE_LOGON_NAME: windows_core::PCWSTR = windows_core::w!("SeServiceLogonRight");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SLDATATYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SLIDTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SLLICENSINGSTATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SLREFERRALTYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SL_ACTIVATION_INFO_HEADER {
    pub cbSize: u32,
    pub r#type: SL_ACTIVATION_TYPE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SL_ACTIVATION_TYPE(pub i32);
pub const SL_ACTIVATION_TYPE_ACTIVE_DIRECTORY: SL_ACTIVATION_TYPE = SL_ACTIVATION_TYPE(1i32);
pub const SL_ACTIVATION_TYPE_DEFAULT: SL_ACTIVATION_TYPE = SL_ACTIVATION_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SL_AD_ACTIVATION_INFO {
    pub header: SL_ACTIVATION_INFO_HEADER,
    pub pwszProductKey: windows_core::PCWSTR,
    pub pwszActivationObjectName: windows_core::PCWSTR,
}
pub const SL_CLIENTAPI_ZONE: u32 = 61440u32;
pub const SL_DATA_BINARY: SLDATATYPE = SLDATATYPE(3u32);
pub const SL_DATA_DWORD: SLDATATYPE = SLDATATYPE(4u32);
pub const SL_DATA_MULTI_SZ: SLDATATYPE = SLDATATYPE(7u32);
pub const SL_DATA_NONE: SLDATATYPE = SLDATATYPE(0u32);
pub const SL_DATA_SUM: SLDATATYPE = SLDATATYPE(100u32);
pub const SL_DATA_SZ: SLDATATYPE = SLDATATYPE(1u32);
pub const SL_DEFAULT_MIGRATION_ENCRYPTOR_URI: windows_core::PCWSTR = windows_core::w!("msft:spp/migrationencryptor/tokenact/1.0");
pub const SL_EVENT_LICENSING_STATE_CHANGED: windows_core::PCWSTR = windows_core::w!("msft:rm/event/licensingstatechanged");
pub const SL_EVENT_POLICY_CHANGED: windows_core::PCWSTR = windows_core::w!("msft:rm/event/policychanged");
pub const SL_EVENT_USER_NOTIFICATION: windows_core::PCWSTR = windows_core::w!("msft:rm/event/usernotification");
pub const SL_E_ACTIVATION_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC004E028_u32 as _);
pub const SL_E_APPLICATION_POLICIES_MISSING: windows_core::HRESULT = windows_core::HRESULT(0xC004F072_u32 as _);
pub const SL_E_APPLICATION_POLICIES_NOT_LOADED: windows_core::HRESULT = windows_core::HRESULT(0xC004F073_u32 as _);
pub const SL_E_AUTHN_CANT_VERIFY: windows_core::HRESULT = windows_core::HRESULT(0xC004F07A_u32 as _);
pub const SL_E_AUTHN_CHALLENGE_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0xC004F079_u32 as _);
pub const SL_E_AUTHN_MISMATCHED_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F078_u32 as _);
pub const SL_E_AUTHN_WRONG_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC004F077_u32 as _);
pub const SL_E_BASE_SKU_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F055_u32 as _);
pub const SL_E_BIOS_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F215_u32 as _);
pub const SL_E_BLOCKED_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F051_u32 as _);
pub const SL_E_CHPA_ACTCONFIG_ID_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C009_u32 as _);
pub const SL_E_CHPA_BINDING_MAPPING_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C006_u32 as _);
pub const SL_E_CHPA_BINDING_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C005_u32 as _);
pub const SL_E_CHPA_BUSINESS_RULE_INPUT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C700_u32 as _);
pub const SL_E_CHPA_DATABASE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004C013_u32 as _);
pub const SL_E_CHPA_DIGITALMARKER_BINDING_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0xC004C052_u32 as _);
pub const SL_E_CHPA_DIGITALMARKER_INVALID_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004C051_u32 as _);
pub const SL_E_CHPA_DMAK_EXTENSION_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0xC004C021_u32 as _);
pub const SL_E_CHPA_DMAK_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0xC004C020_u32 as _);
pub const SL_E_CHPA_DYNAMICALLY_BLOCKED_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004C060_u32 as _);
pub const SL_E_CHPA_FAILED_TO_DELETE_PRODUCTKEY_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004C757_u32 as _);
pub const SL_E_CHPA_FAILED_TO_DELETE_PRODUCT_KEY_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0xC004C75C_u32 as _);
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCTKEY_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004C756_u32 as _);
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCT_KEY_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0xC004C75A_u32 as _);
pub const SL_E_CHPA_FAILED_TO_INSERT_PRODUCT_KEY_RECORD: windows_core::HRESULT = windows_core::HRESULT(0xC004C780_u32 as _);
pub const SL_E_CHPA_FAILED_TO_PROCESS_PRODUCT_KEY_BINDINGS_XML: windows_core::HRESULT = windows_core::HRESULT(0xC004C758_u32 as _);
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCTKEY_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004C755_u32 as _);
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCT_KEY_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0xC004C75B_u32 as _);
pub const SL_E_CHPA_FAILED_TO_UPDATE_PRODUCT_KEY_RECORD: windows_core::HRESULT = windows_core::HRESULT(0xC004C781_u32 as _);
pub const SL_E_CHPA_GENERAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004C050_u32 as _);
pub const SL_E_CHPA_INVALID_ACTCONFIG_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004C00D_u32 as _);
pub const SL_E_CHPA_INVALID_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0xC004C014_u32 as _);
pub const SL_E_CHPA_INVALID_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004C002_u32 as _);
pub const SL_E_CHPA_INVALID_BINDING_URI: windows_core::HRESULT = windows_core::HRESULT(0xC004C011_u32 as _);
pub const SL_E_CHPA_INVALID_PRODUCT_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004C00B_u32 as _);
pub const SL_E_CHPA_INVALID_PRODUCT_DATA_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004C00A_u32 as _);
pub const SL_E_CHPA_INVALID_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004C004_u32 as _);
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_CHAR: windows_core::HRESULT = windows_core::HRESULT(0xC004C010_u32 as _);
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0xC004C00F_u32 as _);
pub const SL_E_CHPA_INVALID_PRODUCT_KEY_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC004C00E_u32 as _);
pub const SL_E_CHPA_MAXIMUM_UNLOCK_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0xC004C008_u32 as _);
pub const SL_E_CHPA_MSCH_RESPONSE_NOT_AVAILABLE_VGA: windows_core::HRESULT = windows_core::HRESULT(0xC004C3FF_u32 as _);
pub const SL_E_CHPA_NETWORK_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004C012_u32 as _);
pub const SL_E_CHPA_NO_RULES_TO_ACTIVATE: windows_core::HRESULT = windows_core::HRESULT(0xC004C04F_u32 as _);
pub const SL_E_CHPA_NULL_VALUE_FOR_PROPERTY_NAME_OR_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004C750_u32 as _);
pub const SL_E_CHPA_OEM_SLP_COA0: windows_core::HRESULT = windows_core::HRESULT(0xC004C016_u32 as _);
pub const SL_E_CHPA_OVERRIDE_REQUEST_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C023_u32 as _);
pub const SL_E_CHPA_PRODUCT_KEY_BEING_USED: windows_core::HRESULT = windows_core::HRESULT(0xC004C770_u32 as _);
pub const SL_E_CHPA_PRODUCT_KEY_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0xC004C003_u32 as _);
pub const SL_E_CHPA_PRODUCT_KEY_BLOCKED_IPLOCATION: windows_core::HRESULT = windows_core::HRESULT(0xC004C017_u32 as _);
pub const SL_E_CHPA_PRODUCT_KEY_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0xC004C001_u32 as _);
pub const SL_E_CHPA_REISSUANCE_LIMIT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C022_u32 as _);
pub const SL_E_CHPA_RESPONSE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004C015_u32 as _);
pub const SL_E_CHPA_SYSTEM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004C00C_u32 as _);
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_AFTER_END_DATE: windows_core::HRESULT = windows_core::HRESULT(0xC004C031_u32 as _);
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_BEFORE_START_DATE: windows_core::HRESULT = windows_core::HRESULT(0xC004C030_u32 as _);
pub const SL_E_CHPA_TIMEBASED_ACTIVATION_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004C032_u32 as _);
pub const SL_E_CHPA_TIMEBASED_PRODUCT_KEY_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0xC004C033_u32 as _);
pub const SL_E_CHPA_UNKNOWN_PRODUCT_KEY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC004C764_u32 as _);
pub const SL_E_CHPA_UNKNOWN_PROPERTY_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004C752_u32 as _);
pub const SL_E_CHPA_UNKNOWN_PROPERTY_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC004C751_u32 as _);
pub const SL_E_CHPA_UNSUPPORTED_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004C007_u32 as _);
pub const SL_E_CIDIID_INVALID_CHECK_DIGITS: windows_core::HRESULT = windows_core::HRESULT(0xC004F04D_u32 as _);
pub const SL_E_CIDIID_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004F02C_u32 as _);
pub const SL_E_CIDIID_INVALID_DATA_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC004F02F_u32 as _);
pub const SL_E_CIDIID_INVALID_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC004F02D_u32 as _);
pub const SL_E_CIDIID_MISMATCHED: windows_core::HRESULT = windows_core::HRESULT(0xC004F031_u32 as _);
pub const SL_E_CIDIID_MISMATCHED_PKEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F07E_u32 as _);
pub const SL_E_CIDIID_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F07F_u32 as _);
pub const SL_E_CIDIID_NOT_DEPOSITED: windows_core::HRESULT = windows_core::HRESULT(0xC004F030_u32 as _);
pub const SL_E_CIDIID_VERSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC004F02E_u32 as _);
pub const SL_E_DATATYPE_MISMATCHED: windows_core::HRESULT = windows_core::HRESULT(0xC004F01E_u32 as _);
pub const SL_E_DECRYPTION_LICENSES_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F01C_u32 as _);
pub const SL_E_DEPENDENT_PROPERTY_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0xC004F066_u32 as _);
pub const SL_E_DOWNLEVEL_SETUP_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F214_u32 as _);
pub const SL_E_DUPLICATE_POLICY: windows_core::HRESULT = windows_core::HRESULT(0xC004F052_u32 as _);
pub const SL_E_EDITION_MISMATCHED: windows_core::HRESULT = windows_core::HRESULT(0xC004F210_u32 as _);
pub const SL_E_ENGINE_DETECTED_EXPLOIT: windows_core::HRESULT = windows_core::HRESULT(0xC004C4B1_u32 as _);
pub const SL_E_EUL_CONSUMPTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004E015_u32 as _);
pub const SL_E_EUL_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F034_u32 as _);
pub const SL_E_EVALUATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004E003_u32 as _);
pub const SL_E_EVENT_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC004F01B_u32 as _);
pub const SL_E_EVENT_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC004F01A_u32 as _);
pub const SL_E_EXTERNAL_SIGNATURE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F006_u32 as _);
pub const SL_E_GRACE_TIME_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004F009_u32 as _);
pub const SL_E_HEALTH_CHECK_FAILED_MUI_FILES: windows_core::HRESULT = windows_core::HRESULT(0xC004C4AE_u32 as _);
pub const SL_E_HEALTH_CHECK_FAILED_NEUTRAL_FILES: windows_core::HRESULT = windows_core::HRESULT(0xC004C4AD_u32 as _);
pub const SL_E_HWID_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0xC004F211_u32 as _);
pub const SL_E_HWID_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004E01B_u32 as _);
pub const SL_E_IA_ID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004FD03_u32 as _);
pub const SL_E_IA_INVALID_VIRTUALIZATION_PLATFORM: windows_core::HRESULT = windows_core::HRESULT(0xC004FD01_u32 as _);
pub const SL_E_IA_MACHINE_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004FD04_u32 as _);
pub const SL_E_IA_PARENT_PARTITION_NOT_ACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0xC004FD02_u32 as _);
pub const SL_E_IA_THROTTLE_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0xC004FD00_u32 as _);
pub const SL_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004F001_u32 as _);
pub const SL_E_INVALID_AD_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004C4AF_u32 as _);
pub const SL_E_INVALID_BINDING_BLOB: windows_core::HRESULT = windows_core::HRESULT(0xC004F032_u32 as _);
pub const SL_E_INVALID_CLIENT_TOKEN: windows_core::HRESULT = windows_core::HRESULT(0xC004C328_u32 as _);
pub const SL_E_INVALID_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0xC004E001_u32 as _);
pub const SL_E_INVALID_CONTEXT_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004E024_u32 as _);
pub const SL_E_INVALID_EVENT_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004F019_u32 as _);
pub const SL_E_INVALID_FILE_HASH: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A1_u32 as _);
pub const SL_E_INVALID_GUID: windows_core::HRESULT = windows_core::HRESULT(0xC004E006_u32 as _);
pub const SL_E_INVALID_HASH: windows_core::HRESULT = windows_core::HRESULT(0xC004E025_u32 as _);
pub const SL_E_INVALID_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0xC004F01F_u32 as _);
pub const SL_E_INVALID_LICENSE_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A8_u32 as _);
pub const SL_E_INVALID_LICENSE_STATE_BREACH_GRACE: windows_core::HRESULT = windows_core::HRESULT(0xC004C291_u32 as _);
pub const SL_E_INVALID_LICENSE_STATE_BREACH_GRACE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004C292_u32 as _);
pub const SL_E_INVALID_OEM_OR_VOLUME_BINDING_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A7_u32 as _);
pub const SL_E_INVALID_OFFLINE_BLOB: windows_core::HRESULT = windows_core::HRESULT(0xC004C329_u32 as _);
pub const SL_E_INVALID_OSVERSION_TEMPLATEID: windows_core::HRESULT = windows_core::HRESULT(0xC004C32B_u32 as _);
pub const SL_E_INVALID_OS_FOR_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004C401_u32 as _);
pub const SL_E_INVALID_PACKAGE: windows_core::HRESULT = windows_core::HRESULT(0xC004F020_u32 as _);
pub const SL_E_INVALID_PACKAGE_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC004F060_u32 as _);
pub const SL_E_INVALID_PKEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F010_u32 as _);
pub const SL_E_INVALID_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004F050_u32 as _);
pub const SL_E_INVALID_PRODUCT_KEY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC004F07D_u32 as _);
pub const SL_E_INVALID_RSDP_COUNT: windows_core::HRESULT = windows_core::HRESULT(0xC004C4B0_u32 as _);
pub const SL_E_INVALID_RULESET_RULE: windows_core::HRESULT = windows_core::HRESULT(0xC004E023_u32 as _);
pub const SL_E_INVALID_RUNNING_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC004F029_u32 as _);
pub const SL_E_INVALID_TEMPLATE_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004C2F6_u32 as _);
pub const SL_E_INVALID_TOKEN_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004C4AC_u32 as _);
pub const SL_E_INVALID_USE_OF_ADD_ON_PKEY: windows_core::HRESULT = windows_core::HRESULT(0x8004E026_u32 as _);
pub const SL_E_INVALID_XML_BLOB: windows_core::HRESULT = windows_core::HRESULT(0xC004C2FA_u32 as _);
pub const SL_E_IP_LOCATION_FALIED: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A9_u32 as _);
pub const SL_E_ISSUANCE_LICENSE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F062_u32 as _);
pub const SL_E_LICENSE_AUTHORIZATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004F022_u32 as _);
pub const SL_E_LICENSE_DECRYPTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004F023_u32 as _);
pub const SL_E_LICENSE_FILE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F011_u32 as _);
pub const SL_E_LICENSE_INVALID_ADDON_INFO: windows_core::HRESULT = windows_core::HRESULT(0xC004E01A_u32 as _);
pub const SL_E_LICENSE_MANAGEMENT_DATA_DUPLICATED: windows_core::HRESULT = windows_core::HRESULT(0xC004F054_u32 as _);
pub const SL_E_LICENSE_MANAGEMENT_DATA_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F04F_u32 as _);
pub const SL_E_LICENSE_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F080_u32 as _);
pub const SL_E_LICENSE_SERVER_URL_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F018_u32 as _);
pub const SL_E_LICENSE_SIGNATURE_VERIFICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004F01D_u32 as _);
pub const SL_E_LUA_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0xC004F025_u32 as _);
pub const SL_E_MISMATCHED_APPID: windows_core::HRESULT = windows_core::HRESULT(0xC004F00A_u32 as _);
pub const SL_E_MISMATCHED_KEY_TYPES: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A4_u32 as _);
pub const SL_E_MISMATCHED_PID: windows_core::HRESULT = windows_core::HRESULT(0xC004F005_u32 as _);
pub const SL_E_MISMATCHED_PKEY_RANGE: windows_core::HRESULT = windows_core::HRESULT(0xC004F004_u32 as _);
pub const SL_E_MISMATCHED_PRODUCT_SKU: windows_core::HRESULT = windows_core::HRESULT(0xC004F069_u32 as _);
pub const SL_E_MISMATCHED_SECURITY_PROCESSOR: windows_core::HRESULT = windows_core::HRESULT(0xC004F00E_u32 as _);
pub const SL_E_MISSING_OVERRIDE_ONLY_ATTRIBUTE: windows_core::HRESULT = windows_core::HRESULT(0xC004F053_u32 as _);
pub const SL_E_NONGENUINE_GRACE_TIME_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004F064_u32 as _);
pub const SL_E_NONGENUINE_GRACE_TIME_EXPIRED_2: windows_core::HRESULT = windows_core::HRESULT(0xC004F067_u32 as _);
pub const SL_E_NON_GENUINE_STATUS_LAST: windows_core::HRESULT = windows_core::HRESULT(0xC004C600_u32 as _);
pub const SL_E_NOTIFICATION_BREACH_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0xC004C531_u32 as _);
pub const SL_E_NOTIFICATION_GRACE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004C532_u32 as _);
pub const SL_E_NOTIFICATION_OTHER_REASONS: windows_core::HRESULT = windows_core::HRESULT(0xC004C533_u32 as _);
pub const SL_E_NOT_ACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0xC004E005_u32 as _);
pub const SL_E_NOT_EVALUATED: windows_core::HRESULT = windows_core::HRESULT(0xC004E004_u32 as _);
pub const SL_E_NOT_GENUINE: windows_core::HRESULT = windows_core::HRESULT(0xC004F200_u32 as _);
pub const SL_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC004F016_u32 as _);
pub const SL_E_NO_PID_CONFIG_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004F00B_u32 as _);
pub const SL_E_NO_PRODUCT_KEY_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F213_u32 as _);
pub const SL_E_OEM_KEY_EDITION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004F212_u32 as _);
pub const SL_E_OFFLINE_GENUINE_BLOB_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C32D_u32 as _);
pub const SL_E_OFFLINE_GENUINE_BLOB_REVOKED: windows_core::HRESULT = windows_core::HRESULT(0xC004C32C_u32 as _);
pub const SL_E_OFFLINE_VALIDATION_BLOB_PARAM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C32A_u32 as _);
pub const SL_E_OPERATION_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC004F06A_u32 as _);
pub const SL_E_OUT_OF_TOLERANCE: windows_core::HRESULT = windows_core::HRESULT(0xC004F00F_u32 as _);
pub const SL_E_PKEY_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004E019_u32 as _);
pub const SL_E_PKEY_INVALID_ALGORITHM: windows_core::HRESULT = windows_core::HRESULT(0xC004E018_u32 as _);
pub const SL_E_PKEY_INVALID_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0xC004E016_u32 as _);
pub const SL_E_PKEY_INVALID_KEYCHANGE1: windows_core::HRESULT = windows_core::HRESULT(0xC004E01C_u32 as _);
pub const SL_E_PKEY_INVALID_KEYCHANGE2: windows_core::HRESULT = windows_core::HRESULT(0xC004E01D_u32 as _);
pub const SL_E_PKEY_INVALID_KEYCHANGE3: windows_core::HRESULT = windows_core::HRESULT(0xC004E01E_u32 as _);
pub const SL_E_PKEY_INVALID_UNIQUEID: windows_core::HRESULT = windows_core::HRESULT(0xC004E017_u32 as _);
pub const SL_E_PKEY_INVALID_UPGRADE: windows_core::HRESULT = windows_core::HRESULT(0xC004F061_u32 as _);
pub const SL_E_PKEY_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F014_u32 as _);
pub const SL_E_PLUGIN_INVALID_MANIFEST: windows_core::HRESULT = windows_core::HRESULT(0xC004F071_u32 as _);
pub const SL_E_PLUGIN_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC004F076_u32 as _);
pub const SL_E_POLICY_CACHE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0xC004F028_u32 as _);
pub const SL_E_POLICY_OTHERINFO_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004E020_u32 as _);
pub const SL_E_PRODUCT_KEY_INSTALLATION_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC004F033_u32 as _);
pub const SL_E_PRODUCT_SKU_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F015_u32 as _);
pub const SL_E_PRODUCT_UNIQUENESS_GROUP_ID_INVALID: windows_core::HRESULT = windows_core::HRESULT(0xC004E021_u32 as _);
pub const SL_E_PROXY_KEY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F026_u32 as _);
pub const SL_E_PROXY_POLICY_NOT_UPDATED: windows_core::HRESULT = windows_core::HRESULT(0xC004F047_u32 as _);
pub const SL_E_PUBLISHING_LICENSE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F017_u32 as _);
pub const SL_E_RAC_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F007_u32 as _);
pub const SL_E_RIGHT_NOT_CONSUMED: windows_core::HRESULT = windows_core::HRESULT(0xC004F002_u32 as _);
pub const SL_E_RIGHT_NOT_GRANTED: windows_core::HRESULT = windows_core::HRESULT(0xC004F013_u32 as _);
pub const SL_E_SECURE_STORE_ID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004E022_u32 as _);
pub const SL_E_SERVICE_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0xC004F07B_u32 as _);
pub const SL_E_SERVICE_STOPPING: windows_core::HRESULT = windows_core::HRESULT(0xC004F075_u32 as _);
pub const SL_E_SFS_BAD_TOKEN_EXT: windows_core::HRESULT = windows_core::HRESULT(0x8004E105_u32 as _);
pub const SL_E_SFS_BAD_TOKEN_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8004E104_u32 as _);
pub const SL_E_SFS_DUPLICATE_TOKEN_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8004E106_u32 as _);
pub const SL_E_SFS_FILE_READ_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004E109_u32 as _);
pub const SL_E_SFS_FILE_WRITE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004E10A_u32 as _);
pub const SL_E_SFS_INVALID_FD_TABLE: windows_core::HRESULT = windows_core::HRESULT(0x8004E102_u32 as _);
pub const SL_E_SFS_INVALID_FILE_POSITION: windows_core::HRESULT = windows_core::HRESULT(0x8004E10B_u32 as _);
pub const SL_E_SFS_INVALID_FS_HEADER: windows_core::HRESULT = windows_core::HRESULT(0x8004E10D_u32 as _);
pub const SL_E_SFS_INVALID_FS_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8004E101_u32 as _);
pub const SL_E_SFS_INVALID_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x8004E103_u32 as _);
pub const SL_E_SFS_INVALID_TOKEN_DATA_HASH: windows_core::HRESULT = windows_core::HRESULT(0x8004E108_u32 as _);
pub const SL_E_SFS_INVALID_TOKEN_DESCRIPTOR: windows_core::HRESULT = windows_core::HRESULT(0x8004E10E_u32 as _);
pub const SL_E_SFS_NO_ACTIVE_TRANSACTION: windows_core::HRESULT = windows_core::HRESULT(0x8004E10C_u32 as _);
pub const SL_E_SFS_TOKEN_SIZE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8004E107_u32 as _);
pub const SL_E_SLP_BAD_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0xC004F059_u32 as _);
pub const SL_E_SLP_INVALID_MARKER_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC004F07C_u32 as _);
pub const SL_E_SLP_MISSING_ACPI_SLIC: windows_core::HRESULT = windows_core::HRESULT(0xC004F057_u32 as _);
pub const SL_E_SLP_MISSING_SLP_MARKER: windows_core::HRESULT = windows_core::HRESULT(0xC004F058_u32 as _);
pub const SL_E_SLP_NOT_SIGNED: windows_core::HRESULT = windows_core::HRESULT(0xC004F02A_u32 as _);
pub const SL_E_SLP_OEM_CERT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0xC004F063_u32 as _);
pub const SL_E_SOFTMOD_EXPLOIT_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0xC004C4AB_u32 as _);
pub const SL_E_SPC_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F008_u32 as _);
pub const SL_E_SRV_AUTHORIZATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004B005_u32 as _);
pub const SL_E_SRV_BUSINESS_TOKEN_ENTRY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004B010_u32 as _);
pub const SL_E_SRV_CLIENT_CLOCK_OUT_OF_SYNC: windows_core::HRESULT = windows_core::HRESULT(0xC004B011_u32 as _);
pub const SL_E_SRV_GENERAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC004B100_u32 as _);
pub const SL_E_SRV_INVALID_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004B006_u32 as _);
pub const SL_E_SRV_INVALID_LICENSE_STRUCTURE: windows_core::HRESULT = windows_core::HRESULT(0xC004B004_u32 as _);
pub const SL_E_SRV_INVALID_PAYLOAD: windows_core::HRESULT = windows_core::HRESULT(0xC004B008_u32 as _);
pub const SL_E_SRV_INVALID_PRODUCT_KEY_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0xC004B002_u32 as _);
pub const SL_E_SRV_INVALID_PUBLISH_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0xC004B001_u32 as _);
pub const SL_E_SRV_INVALID_RIGHTS_ACCOUNT_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0xC004B003_u32 as _);
pub const SL_E_SRV_INVALID_SECURITY_PROCESSOR_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0xC004B009_u32 as _);
pub const SL_E_SRV_SERVER_PONG: windows_core::HRESULT = windows_core::HRESULT(0xC004B007_u32 as _);
pub const SL_E_STORE_UPGRADE_TOKEN_NOT_AUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0xC004E02E_u32 as _);
pub const SL_E_STORE_UPGRADE_TOKEN_NOT_PRS_SIGNED: windows_core::HRESULT = windows_core::HRESULT(0xC004E02C_u32 as _);
pub const SL_E_STORE_UPGRADE_TOKEN_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004E029_u32 as _);
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_EDITION: windows_core::HRESULT = windows_core::HRESULT(0xC004E02A_u32 as _);
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_PID: windows_core::HRESULT = windows_core::HRESULT(0xC004E02B_u32 as _);
pub const SL_E_STORE_UPGRADE_TOKEN_WRONG_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC004E02D_u32 as _);
pub const SL_E_TAMPER_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0xC004F027_u32 as _);
pub const SL_E_TAMPER_RECOVERY_REQUIRES_ACTIVATION: windows_core::HRESULT = windows_core::HRESULT(0xC004FE00_u32 as _);
pub const SL_E_TKA_CERT_CNG_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F313_u32 as _);
pub const SL_E_TKA_CERT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F305_u32 as _);
pub const SL_E_TKA_CHALLENGE_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004F301_u32 as _);
pub const SL_E_TKA_CHALLENGE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004F309_u32 as _);
pub const SL_E_TKA_CRITERIA_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004F30F_u32 as _);
pub const SL_E_TKA_FAILED_GRANT_PARSING: windows_core::HRESULT = windows_core::HRESULT(0xC004F30C_u32 as _);
pub const SL_E_TKA_GRANT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F304_u32 as _);
pub const SL_E_TKA_INVALID_BLOB: windows_core::HRESULT = windows_core::HRESULT(0xC004F307_u32 as _);
pub const SL_E_TKA_INVALID_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0xC004F30A_u32 as _);
pub const SL_E_TKA_INVALID_CERT_CHAIN: windows_core::HRESULT = windows_core::HRESULT(0xC004F303_u32 as _);
pub const SL_E_TKA_INVALID_SKU_ID: windows_core::HRESULT = windows_core::HRESULT(0xC004F306_u32 as _);
pub const SL_E_TKA_INVALID_SMARTCARD: windows_core::HRESULT = windows_core::HRESULT(0xC004F30B_u32 as _);
pub const SL_E_TKA_INVALID_THUMBPRINT: windows_core::HRESULT = windows_core::HRESULT(0xC004F30D_u32 as _);
pub const SL_E_TKA_SILENT_ACTIVATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC004F302_u32 as _);
pub const SL_E_TKA_SOFT_CERT_DISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC004F311_u32 as _);
pub const SL_E_TKA_SOFT_CERT_INVALID: windows_core::HRESULT = windows_core::HRESULT(0xC004F312_u32 as _);
pub const SL_E_TKA_TAMPERED_CERT_CHAIN: windows_core::HRESULT = windows_core::HRESULT(0xC004F308_u32 as _);
pub const SL_E_TKA_THUMBPRINT_CERT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F30E_u32 as _);
pub const SL_E_TKA_TPID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004F310_u32 as _);
pub const SL_E_TOKEN_STORE_INVALID_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC004E002_u32 as _);
pub const SL_E_TOKSTO_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0xC004E00A_u32 as _);
pub const SL_E_TOKSTO_CANT_ACQUIRE_MUTEX: windows_core::HRESULT = windows_core::HRESULT(0xC004E013_u32 as _);
pub const SL_E_TOKSTO_CANT_CREATE_FILE: windows_core::HRESULT = windows_core::HRESULT(0xC004E00C_u32 as _);
pub const SL_E_TOKSTO_CANT_CREATE_MUTEX: windows_core::HRESULT = windows_core::HRESULT(0xC004E012_u32 as _);
pub const SL_E_TOKSTO_CANT_PARSE_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0xC004E00F_u32 as _);
pub const SL_E_TOKSTO_CANT_READ_FILE: windows_core::HRESULT = windows_core::HRESULT(0xC004E00E_u32 as _);
pub const SL_E_TOKSTO_CANT_WRITE_TO_FILE: windows_core::HRESULT = windows_core::HRESULT(0xC004E00D_u32 as _);
pub const SL_E_TOKSTO_INVALID_FILE: windows_core::HRESULT = windows_core::HRESULT(0xC004E011_u32 as _);
pub const SL_E_TOKSTO_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0xC004E009_u32 as _);
pub const SL_E_TOKSTO_NO_ID_SET: windows_core::HRESULT = windows_core::HRESULT(0xC004E00B_u32 as _);
pub const SL_E_TOKSTO_NO_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0xC004E008_u32 as _);
pub const SL_E_TOKSTO_NO_TOKEN_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004E014_u32 as _);
pub const SL_E_TOKSTO_PROPERTY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004E010_u32 as _);
pub const SL_E_TOKSTO_TOKEN_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004E007_u32 as _);
pub const SL_E_USE_LICENSE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F003_u32 as _);
pub const SL_E_VALIDATION_BLOB_PARAM_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004C327_u32 as _);
pub const SL_E_VALIDATION_BLOCKED_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A2_u32 as _);
pub const SL_E_VALIDATION_INVALID_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0xC004C4A5_u32 as _);
pub const SL_E_VALIDITY_PERIOD_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004FC07_u32 as _);
pub const SL_E_VALIDITY_TIME_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004F021_u32 as _);
pub const SL_E_VALUE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F012_u32 as _);
pub const SL_E_VL_AD_AO_NAME_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0xC004F082_u32 as _);
pub const SL_E_VL_AD_AO_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F081_u32 as _);
pub const SL_E_VL_AD_SCHEMA_VERSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC004F083_u32 as _);
pub const SL_E_VL_BINDING_SERVICE_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0xC004F039_u32 as _);
pub const SL_E_VL_BINDING_SERVICE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004F074_u32 as _);
pub const SL_E_VL_INFO_PRODUCT_USER_RIGHT: windows_core::HRESULT = windows_core::HRESULT(0x4004F040_u32 as _);
pub const SL_E_VL_INVALID_TIMESTAMP: windows_core::HRESULT = windows_core::HRESULT(0xC004F06C_u32 as _);
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_ID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004F042_u32 as _);
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_NOT_ACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0xC004F041_u32 as _);
pub const SL_E_VL_KEY_MANAGEMENT_SERVICE_VM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC004F06B_u32 as _);
pub const SL_E_VL_MACHINE_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004F056_u32 as _);
pub const SL_E_VL_NOT_ENOUGH_COUNT: windows_core::HRESULT = windows_core::HRESULT(0xC004F038_u32 as _);
pub const SL_E_VL_NOT_WINDOWS_SLP: windows_core::HRESULT = windows_core::HRESULT(0xC004F035_u32 as _);
pub const SL_E_WINDOWS_INVALID_LICENSE_STATE: windows_core::HRESULT = windows_core::HRESULT(0xC004F024_u32 as _);
pub const SL_E_WINDOWS_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004E027_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SL_GENUINE_STATE(pub i32);
pub const SL_GEN_STATE_INVALID_LICENSE: SL_GENUINE_STATE = SL_GENUINE_STATE(1i32);
pub const SL_GEN_STATE_IS_GENUINE: SL_GENUINE_STATE = SL_GENUINE_STATE(0i32);
pub const SL_GEN_STATE_LAST: SL_GENUINE_STATE = SL_GENUINE_STATE(4i32);
pub const SL_GEN_STATE_OFFLINE: SL_GENUINE_STATE = SL_GENUINE_STATE(3i32);
pub const SL_GEN_STATE_TAMPERED: SL_GENUINE_STATE = SL_GENUINE_STATE(2i32);
pub const SL_ID_ALL_LICENSES: SLIDTYPE = SLIDTYPE(5i32);
pub const SL_ID_ALL_LICENSE_FILES: SLIDTYPE = SLIDTYPE(6i32);
pub const SL_ID_APPLICATION: SLIDTYPE = SLIDTYPE(0i32);
pub const SL_ID_LAST: SLIDTYPE = SLIDTYPE(8i32);
pub const SL_ID_LICENSE: SLIDTYPE = SLIDTYPE(3i32);
pub const SL_ID_LICENSE_FILE: SLIDTYPE = SLIDTYPE(2i32);
pub const SL_ID_PKEY: SLIDTYPE = SLIDTYPE(4i32);
pub const SL_ID_PRODUCT_SKU: SLIDTYPE = SLIDTYPE(1i32);
pub const SL_ID_STORE_TOKEN: SLIDTYPE = SLIDTYPE(7i32);
pub const SL_INFO_KEY_ACTIVE_PLUGINS: windows_core::PCWSTR = windows_core::w!("ActivePlugins");
pub const SL_INFO_KEY_AUTHOR: windows_core::PCWSTR = windows_core::w!("Author");
pub const SL_INFO_KEY_BIOS_OA2_MINOR_VERSION: windows_core::PCWSTR = windows_core::w!("BiosOA2MinorVersion");
pub const SL_INFO_KEY_BIOS_PKEY: windows_core::PCWSTR = windows_core::w!("BiosProductKey");
pub const SL_INFO_KEY_BIOS_PKEY_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("BiosProductKeyDescription");
pub const SL_INFO_KEY_BIOS_PKEY_PKPN: windows_core::PCWSTR = windows_core::w!("BiosProductKeyPkPn");
pub const SL_INFO_KEY_BIOS_SLIC_STATE: windows_core::PCWSTR = windows_core::w!("BiosSlicState");
pub const SL_INFO_KEY_CHANNEL: windows_core::PCWSTR = windows_core::w!("Channel");
pub const SL_INFO_KEY_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const SL_INFO_KEY_DIGITAL_PID: windows_core::PCWSTR = windows_core::w!("DigitalPID");
pub const SL_INFO_KEY_DIGITAL_PID2: windows_core::PCWSTR = windows_core::w!("DigitalPID2");
pub const SL_INFO_KEY_IS_KMS: windows_core::PCWSTR = windows_core::w!("IsKeyManagementService");
pub const SL_INFO_KEY_IS_PRS: windows_core::PCWSTR = windows_core::w!("IsPRS");
pub const SL_INFO_KEY_KMS_CURRENT_COUNT: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceCurrentCount");
pub const SL_INFO_KEY_KMS_FAILED_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceFailedRequests");
pub const SL_INFO_KEY_KMS_LICENSED_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceLicensedRequests");
pub const SL_INFO_KEY_KMS_NON_GENUINE_GRACE_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceNonGenuineGraceRequests");
pub const SL_INFO_KEY_KMS_NOTIFICATION_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceNotificationRequests");
pub const SL_INFO_KEY_KMS_OOB_GRACE_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceOOBGraceRequests");
pub const SL_INFO_KEY_KMS_OOT_GRACE_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceOOTGraceRequests");
pub const SL_INFO_KEY_KMS_REQUIRED_CLIENT_COUNT: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceRequiredClientCount");
pub const SL_INFO_KEY_KMS_TOTAL_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceTotalRequests");
pub const SL_INFO_KEY_KMS_UNLICENSED_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceUnlicensedRequests");
pub const SL_INFO_KEY_LICENSE_TYPE: windows_core::PCWSTR = windows_core::w!("LicenseType");
pub const SL_INFO_KEY_LICENSOR_URL: windows_core::PCWSTR = windows_core::w!("LicensorUrl");
pub const SL_INFO_KEY_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const SL_INFO_KEY_PARTIAL_PRODUCT_KEY: windows_core::PCWSTR = windows_core::w!("PartialProductKey");
pub const SL_INFO_KEY_PRODUCT_KEY_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("PKCURL");
pub const SL_INFO_KEY_PRODUCT_SKU_ID: windows_core::PCWSTR = windows_core::w!("ProductSkuId");
pub const SL_INFO_KEY_RIGHT_ACCOUNT_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("RACURL");
pub const SL_INFO_KEY_SECURE_PROCESSOR_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("SPCURL");
pub const SL_INFO_KEY_SECURE_STORE_ID: windows_core::PCWSTR = windows_core::w!("SecureStoreId");
pub const SL_INFO_KEY_SYSTEM_STATE: windows_core::PCWSTR = windows_core::w!("SystemState");
pub const SL_INFO_KEY_USE_LICENSE_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("EULURL");
pub const SL_INFO_KEY_VERSION: windows_core::PCWSTR = windows_core::w!("Version");
pub const SL_INTERNAL_ZONE: u32 = 57344u32;
pub const SL_I_NONGENUINE_GRACE_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x4004F065_u32 as _);
pub const SL_I_NONGENUINE_GRACE_PERIOD_2: windows_core::HRESULT = windows_core::HRESULT(0x4004F068_u32 as _);
pub const SL_I_OOB_GRACE_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x4004F00C_u32 as _);
pub const SL_I_OOT_GRACE_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x4004F00D_u32 as _);
pub const SL_I_PERPETUAL_OOB_GRACE_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x4004FC05_u32 as _);
pub const SL_I_STORE_BASED_ACTIVATION: windows_core::HRESULT = windows_core::HRESULT(0x4004F401_u32 as _);
pub const SL_I_TIMEBASED_EXTENDED_GRACE_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x4004FC06_u32 as _);
pub const SL_I_TIMEBASED_VALIDITY_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x4004FC04_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SL_LICENSING_STATUS {
    pub SkuId: windows_core::GUID,
    pub eStatus: SLLICENSINGSTATUS,
    pub dwGraceTime: u32,
    pub dwTotalGraceDays: u32,
    pub hrReason: windows_core::HRESULT,
    pub qwValidityExpiration: u64,
}
pub const SL_LICENSING_STATUS_IN_GRACE_PERIOD: SLLICENSINGSTATUS = SLLICENSINGSTATUS(2i32);
pub const SL_LICENSING_STATUS_LAST: SLLICENSINGSTATUS = SLLICENSINGSTATUS(4i32);
pub const SL_LICENSING_STATUS_LICENSED: SLLICENSINGSTATUS = SLLICENSINGSTATUS(1i32);
pub const SL_LICENSING_STATUS_NOTIFICATION: SLLICENSINGSTATUS = SLLICENSINGSTATUS(3i32);
pub const SL_LICENSING_STATUS_UNLICENSED: SLLICENSINGSTATUS = SLLICENSINGSTATUS(0i32);
pub const SL_MDOLLAR_ZONE: u32 = 40960u32;
pub const SL_MSCH_ZONE: u32 = 49152u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SL_NONGENUINE_UI_OPTIONS {
    pub cbSize: u32,
    pub pComponentId: *const windows_core::GUID,
    pub hResultUI: windows_core::HRESULT,
}
impl Default for SL_NONGENUINE_UI_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_PKEY_DETECT: windows_core::PCWSTR = windows_core::w!("msft:rm/algorithm/pkey/detect");
pub const SL_PKEY_MS2005: windows_core::PCWSTR = windows_core::w!("msft:rm/algorithm/pkey/2005");
pub const SL_PKEY_MS2009: windows_core::PCWSTR = windows_core::w!("msft:rm/algorithm/pkey/2009");
pub const SL_POLICY_EVALUATION_MODE_ENABLED: windows_core::PCWSTR = windows_core::w!("Security-SPP-EvaluationModeEnabled");
pub const SL_PROP_ACTIVATION_VALIDATION_IN_PROGRESS: windows_core::PCWSTR = windows_core::w!("SL_ACTIVATION_VALIDATION_IN_PROGRESS");
pub const SL_PROP_BRT_COMMIT: windows_core::PCWSTR = windows_core::w!("SL_BRT_COMMIT");
pub const SL_PROP_BRT_DATA: windows_core::PCWSTR = windows_core::w!("SL_BRT_DATA");
pub const SL_PROP_GENUINE_RESULT: windows_core::PCWSTR = windows_core::w!("SL_GENUINE_RESULT");
pub const SL_PROP_GET_GENUINE_AUTHZ: windows_core::PCWSTR = windows_core::w!("SL_GET_GENUINE_AUTHZ");
pub const SL_PROP_GET_GENUINE_SERVER_AUTHZ: windows_core::PCWSTR = windows_core::w!("SL_GET_GENUINE_SERVER_AUTHZ");
pub const SL_PROP_LAST_ACT_ATTEMPT_HRESULT: windows_core::PCWSTR = windows_core::w!("SL_LAST_ACT_ATTEMPT_HRESULT");
pub const SL_PROP_LAST_ACT_ATTEMPT_SERVER_FLAGS: windows_core::PCWSTR = windows_core::w!("SL_LAST_ACT_ATTEMPT_SERVER_FLAGS");
pub const SL_PROP_LAST_ACT_ATTEMPT_TIME: windows_core::PCWSTR = windows_core::w!("SL_LAST_ACT_ATTEMPT_TIME");
pub const SL_PROP_NONGENUINE_GRACE_FLAG: windows_core::PCWSTR = windows_core::w!("SL_NONGENUINE_GRACE_FLAG");
pub const SL_REARM_REBOOT_REQUIRED: u32 = 1u32;
pub const SL_REFERRALTYPE_APPID: SLREFERRALTYPE = SLREFERRALTYPE(1i32);
pub const SL_REFERRALTYPE_BEST_MATCH: SLREFERRALTYPE = SLREFERRALTYPE(4i32);
pub const SL_REFERRALTYPE_OVERRIDE_APPID: SLREFERRALTYPE = SLREFERRALTYPE(3i32);
pub const SL_REFERRALTYPE_OVERRIDE_SKUID: SLREFERRALTYPE = SLREFERRALTYPE(2i32);
pub const SL_REFERRALTYPE_SKUID: SLREFERRALTYPE = SLREFERRALTYPE(0i32);
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_CHECK_DIGITS: windows_core::HRESULT = windows_core::HRESULT(0x803FA090_u32 as _);
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0x803FA08E_u32 as _);
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_DATA_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x803FA08F_u32 as _);
pub const SL_REMAPPING_MDOLLAR_CIDIID_INVALID_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x803FA08D_u32 as _);
pub const SL_REMAPPING_MDOLLAR_DIGITALMARKER_BINDING_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x803FA0D4_u32 as _);
pub const SL_REMAPPING_MDOLLAR_DIGITALMARKER_INVALID_BINDING: windows_core::HRESULT = windows_core::HRESULT(0x803FA0D3_u32 as _);
pub const SL_REMAPPING_MDOLLAR_DMAK_EXTENSION_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x803FA080_u32 as _);
pub const SL_REMAPPING_MDOLLAR_DMAK_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x803FA07F_u32 as _);
pub const SL_REMAPPING_MDOLLAR_DMAK_OVERRIDE_LIMIT_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x803FA0D6_u32 as _);
pub const SL_REMAPPING_MDOLLAR_FREE_OFFER_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x803FA400_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_ACTCONFIG_ID: windows_core::HRESULT = windows_core::HRESULT(0x803FA076_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0x803FA07D_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_BINDING: windows_core::HRESULT = windows_core::HRESULT(0x803FA066_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_BINDING_URI: windows_core::HRESULT = windows_core::HRESULT(0x803FA07A_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_DATA: windows_core::HRESULT = windows_core::HRESULT(0x803FA074_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_DATA_ID: windows_core::HRESULT = windows_core::HRESULT(0x803FA073_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0x803FA068_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x803FA078_u32 as _);
pub const SL_REMAPPING_MDOLLAR_INVALID_PRODUCT_KEY_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x803FA077_u32 as _);
pub const SL_REMAPPING_MDOLLAR_MAXIMUM_UNLOCK_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x803FA071_u32 as _);
pub const SL_REMAPPING_MDOLLAR_NO_RULES_TO_ACTIVATE: windows_core::HRESULT = windows_core::HRESULT(0x803FA0C8_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OEM_SLP_COA0: windows_core::HRESULT = windows_core::HRESULT(0x803FA083_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_DEVICE_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803FABC3_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_DEVICE_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x803FABBE_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_DONOR_HWID_NO_ENTITLEMENT: windows_core::HRESULT = windows_core::HRESULT(0x803FABB8_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_GENERIC_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x803FABB9_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_GP_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x803FABBF_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_HARDWARE_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803FABC0_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_LICENSE_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803FABC2_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_LICENSE_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x803FABBD_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_NOT_ADMIN: windows_core::HRESULT = windows_core::HRESULT(0x803FABBB_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_NO_ASSOCIATION: windows_core::HRESULT = windows_core::HRESULT(0x803FABBA_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_USER_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803FABC1_u32 as _);
pub const SL_REMAPPING_MDOLLAR_OSR_USER_THROTTLED: windows_core::HRESULT = windows_core::HRESULT(0x803FABBC_u32 as _);
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x803FA067_u32 as _);
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_BLOCKED_IPLOCATION: windows_core::HRESULT = windows_core::HRESULT(0x803FA0CB_u32 as _);
pub const SL_REMAPPING_MDOLLAR_PRODUCT_KEY_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x803FA065_u32 as _);
pub const SL_REMAPPING_MDOLLAR_ROT_OVERRIDE_LIMIT_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x803FA0D5_u32 as _);
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_AFTER_END_DATE: windows_core::HRESULT = windows_core::HRESULT(0x803FA098_u32 as _);
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_BEFORE_START_DATE: windows_core::HRESULT = windows_core::HRESULT(0x803FA097_u32 as _);
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_ACTIVATION_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x803FA099_u32 as _);
pub const SL_REMAPPING_MDOLLAR_TIMEBASED_PRODUCT_KEY_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x803FA09A_u32 as _);
pub const SL_REMAPPING_MDOLLAR_UNSUPPORTED_PRODUCT_KEY: windows_core::HRESULT = windows_core::HRESULT(0x803FA06C_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_BAD_GET_INFO_QUERY: windows_core::HRESULT = windows_core::HRESULT(0xC004D012_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_HANDLE_NOT_COMMITED: windows_core::HRESULT = windows_core::HRESULT(0xC004D081_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_INVALID_ALGORITHM_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC004D009_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0xC004D02C_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_INVALID_KEY_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC004D055_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_INVALID_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0xC004D000_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_NO_AES_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0xC004D073_u32 as _);
pub const SL_REMAPPING_SP_PUB_API_TOO_MANY_LOADED_ENVIRONMENTS: windows_core::HRESULT = windows_core::HRESULT(0xC004D00C_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_HASH_FINALIZED: windows_core::HRESULT = windows_core::HRESULT(0xC004D209_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_BLOCK: windows_core::HRESULT = windows_core::HRESULT(0xC004D20F_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_BLOCKLENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC004D202_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_CIPHER: windows_core::HRESULT = windows_core::HRESULT(0xC004D203_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_CIPHERMODE: windows_core::HRESULT = windows_core::HRESULT(0xC004D204_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0xC004D210_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_KEYLENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC004D201_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_PADDING: windows_core::HRESULT = windows_core::HRESULT(0xC004D211_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0xC004D20E_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_INVALID_SIGNATURELENGTH: windows_core::HRESULT = windows_core::HRESULT(0xC004D20D_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_KEY_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC004D20A_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_KEY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004D20B_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_NOT_BLOCK_ALIGNED: windows_core::HRESULT = windows_core::HRESULT(0xC004D20C_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_ATTRIBUTEID: windows_core::HRESULT = windows_core::HRESULT(0xC004D208_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_HASHID: windows_core::HRESULT = windows_core::HRESULT(0xC004D207_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_KEYID: windows_core::HRESULT = windows_core::HRESULT(0xC004D206_u32 as _);
pub const SL_REMAPPING_SP_PUB_CRYPTO_UNKNOWN_PROVIDERID: windows_core::HRESULT = windows_core::HRESULT(0xC004D205_u32 as _);
pub const SL_REMAPPING_SP_PUB_GENERAL_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0xC004D101_u32 as _);
pub const SL_REMAPPING_SP_PUB_KM_CACHE_IDENTICAL: windows_core::HRESULT = windows_core::HRESULT(0x4004D601_u32 as _);
pub const SL_REMAPPING_SP_PUB_KM_CACHE_POLICY_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x4004D602_u32 as _);
pub const SL_REMAPPING_SP_PUB_KM_CACHE_TAMPER: windows_core::HRESULT = windows_core::HRESULT(0xC004D501_u32 as _);
pub const SL_REMAPPING_SP_PUB_KM_CACHE_TAMPER_RESTORE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC004D502_u32 as _);
pub const SL_REMAPPING_SP_PUB_PROXY_SOFT_TAMPER: windows_core::HRESULT = windows_core::HRESULT(0xC004D702_u32 as _);
pub const SL_REMAPPING_SP_PUB_TAMPER_MODULE_AUTHENTICATION: windows_core::HRESULT = windows_core::HRESULT(0xC004D401_u32 as _);
pub const SL_REMAPPING_SP_PUB_TAMPER_SECURITY_PROCESSOR_PATCHED: windows_core::HRESULT = windows_core::HRESULT(0xC004D402_u32 as _);
pub const SL_REMAPPING_SP_PUB_TIMER_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC004D30A_u32 as _);
pub const SL_REMAPPING_SP_PUB_TIMER_EXPIRED: windows_core::HRESULT = windows_core::HRESULT(0xC004D30C_u32 as _);
pub const SL_REMAPPING_SP_PUB_TIMER_NAME_SIZE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0xC004D30D_u32 as _);
pub const SL_REMAPPING_SP_PUB_TIMER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004D30B_u32 as _);
pub const SL_REMAPPING_SP_PUB_TIMER_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0xC004D311_u32 as _);
pub const SL_REMAPPING_SP_PUB_TRUSTED_TIME_OK: windows_core::HRESULT = windows_core::HRESULT(0x4004D30F_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0xC004D314_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ATTRIBUTE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004D313_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ATTRIBUTE_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0xC004D312_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_DATA_SIZE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0xC004D308_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC004D305_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004D304_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_KEY_SIZE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0xC004D306_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_ENTRY_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0xC004D310_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_FULL: windows_core::HRESULT = windows_core::HRESULT(0xC004D30E_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_INVALID_HW_BINDING: windows_core::HRESULT = windows_core::HRESULT(0xC004D309_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_MAX_REARM_REACHED: windows_core::HRESULT = windows_core::HRESULT(0xC004D307_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_NAMESPACE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0xC004D316_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_NAMESPACE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC004D315_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_REARMED: windows_core::HRESULT = windows_core::HRESULT(0xC004D302_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_RECREATED: windows_core::HRESULT = windows_core::HRESULT(0xC004D303_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED: windows_core::HRESULT = windows_core::HRESULT(0xC004D301_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_BREADCRUMB_GENERATION: windows_core::HRESULT = windows_core::HRESULT(0xC004D318_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_BREADCRUMB_LOAD_INVALID: windows_core::HRESULT = windows_core::HRESULT(0xC004D317_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_DATA_BREADCRUMB_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004D31B_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_DATA_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0xC004D31C_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004D319_u32 as _);
pub const SL_REMAPPING_SP_PUB_TS_TAMPERED_NO_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004D31A_u32 as _);
pub const SL_REMAPPING_SP_STATUS_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC004D105_u32 as _);
pub const SL_REMAPPING_SP_STATUS_DEBUGGER_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x8004D10B_u32 as _);
pub const SL_REMAPPING_SP_STATUS_GENERIC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC004D103_u32 as _);
pub const SL_REMAPPING_SP_STATUS_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0xC004D107_u32 as _);
pub const SL_REMAPPING_SP_STATUS_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0xC004D104_u32 as _);
pub const SL_REMAPPING_SP_STATUS_INVALIDDATA: windows_core::HRESULT = windows_core::HRESULT(0xC004D108_u32 as _);
pub const SL_REMAPPING_SP_STATUS_INVALID_SPAPI_CALL: windows_core::HRESULT = windows_core::HRESULT(0xC004D109_u32 as _);
pub const SL_REMAPPING_SP_STATUS_INVALID_SPAPI_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC004D10A_u32 as _);
pub const SL_REMAPPING_SP_STATUS_NO_MORE_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC004D10C_u32 as _);
pub const SL_REMAPPING_SP_STATUS_PUSHKEY_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0xC004D701_u32 as _);
pub const SL_REMAPPING_SP_STATUS_SYSTEM_TIME_SKEWED: windows_core::HRESULT = windows_core::HRESULT(0x8004D102_u32 as _);
pub const SL_SERVER_ZONE: u32 = 45056u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SL_SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl Default for SL_SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_SYSTEM_STATE_REBOOT_POLICY_FOUND: u32 = 1u32;
pub const SL_SYSTEM_STATE_TAMPERED: u32 = 2u32;
pub const SPP_MIGRATION_GATHER_ACTIVATED_WINDOWS_STATE: u32 = 2u32;
pub const SPP_MIGRATION_GATHER_ALL: u32 = 4294967295u32;
pub const SPP_MIGRATION_GATHER_MIGRATABLE_APPS: u32 = 1u32;
pub const SP_ACCEPT_CREDENTIALS_NAME: windows_core::PCSTR = windows_core::s!("SpAcceptCredentials\u{0}");
pub const SP_PROT_ALL: u32 = 4294967295u32;
pub const SP_PROT_DTLS1_0_CLIENT: u32 = 131072u32;
pub const SP_PROT_DTLS1_0_SERVER: u32 = 65536u32;
pub const SP_PROT_DTLS1_2_CLIENT: u32 = 524288u32;
pub const SP_PROT_DTLS1_2_SERVER: u32 = 262144u32;
pub const SP_PROT_DTLS_CLIENT: u32 = 131072u32;
pub const SP_PROT_DTLS_SERVER: u32 = 65536u32;
pub const SP_PROT_NONE: u32 = 0u32;
pub const SP_PROT_PCT1_CLIENT: u32 = 2u32;
pub const SP_PROT_PCT1_SERVER: u32 = 1u32;
pub const SP_PROT_SSL2_CLIENT: u32 = 8u32;
pub const SP_PROT_SSL2_SERVER: u32 = 4u32;
pub const SP_PROT_SSL3_CLIENT: u32 = 32u32;
pub const SP_PROT_SSL3_SERVER: u32 = 16u32;
pub const SP_PROT_TLS1_0_CLIENT: u32 = 128u32;
pub const SP_PROT_TLS1_0_SERVER: u32 = 64u32;
pub const SP_PROT_TLS1_1_CLIENT: u32 = 512u32;
pub const SP_PROT_TLS1_1_SERVER: u32 = 256u32;
pub const SP_PROT_TLS1_2_CLIENT: u32 = 2048u32;
pub const SP_PROT_TLS1_2_SERVER: u32 = 1024u32;
pub const SP_PROT_TLS1_3PLUS_CLIENT: u32 = 8192u32;
pub const SP_PROT_TLS1_3PLUS_SERVER: u32 = 4096u32;
pub const SP_PROT_TLS1_3_CLIENT: u32 = 8192u32;
pub const SP_PROT_TLS1_3_SERVER: u32 = 4096u32;
pub const SP_PROT_TLS1_CLIENT: u32 = 128u32;
pub const SP_PROT_TLS1_SERVER: u32 = 64u32;
pub const SP_PROT_UNI_CLIENT: u32 = 2147483648u32;
pub const SP_PROT_UNI_SERVER: u32 = 1073741824u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_SECURITY_DESCRIPTOR {
    pub Length: u32,
    pub SecurityDescriptor: *mut u8,
}
impl Default for SR_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL2SP_NAME: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 2.0");
pub const SSL2SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft SSL 2.0");
pub const SSL2SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 2.0");
pub const SSL3SP_NAME: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 3.0");
pub const SSL3SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft SSL 3.0");
pub const SSL3SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft SSL 3.0");
#[cfg(feature = "Win32_Security_Cryptography")]
pub type SSL_CRACK_CERTIFICATE_FN = Option<unsafe extern "system" fn(pbcertificate: *mut u8, cbcertificate: u32, verifysignature: windows_core::BOOL, ppcertificate: *mut *mut X509Certificate) -> windows_core::BOOL>;
pub const SSL_CRACK_CERTIFICATE_NAME: windows_core::PCWSTR = windows_core::w!("SslCrackCertificate");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SSL_CREDENTIAL_CERTIFICATE {
    pub cbPrivateKey: u32,
    pub pPrivateKey: *mut u8,
    pub cbCertificate: u32,
    pub pCertificate: *mut u8,
    pub pszPassword: windows_core::PSTR,
}
impl Default for SSL_CREDENTIAL_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SSL_EMPTY_CACHE_FN_A = Option<unsafe extern "system" fn(psztargetname: windows_core::PCSTR, dwflags: u32) -> windows_core::BOOL>;
pub type SSL_EMPTY_CACHE_FN_W = Option<unsafe extern "system" fn(psztargetname: windows_core::PCWSTR, dwflags: u32) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography")]
pub type SSL_FREE_CERTIFICATE_FN = Option<unsafe extern "system" fn(pcertificate: *mut X509Certificate)>;
pub const SSL_FREE_CERTIFICATE_NAME: windows_core::PCWSTR = windows_core::w!("SslFreeCertificate");
pub const SSL_SESSION_DISABLE_RECONNECTS: SCHANNEL_SESSION_TOKEN_FLAGS = SCHANNEL_SESSION_TOKEN_FLAGS(2u32);
pub const SSL_SESSION_ENABLE_RECONNECTS: SCHANNEL_SESSION_TOKEN_FLAGS = SCHANNEL_SESSION_TOKEN_FLAGS(1u32);
pub const SSL_SESSION_RECONNECT: u32 = 1u32;
pub const SSPIPFC_CREDPROV_DO_NOT_LOAD: u32 = 4u32;
pub const SSPIPFC_CREDPROV_DO_NOT_SAVE: u32 = 1u32;
pub const SSPIPFC_NO_CHECKBOX: u32 = 2u32;
pub const SSPIPFC_SAVE_CRED_BY_CALLER: u32 = 1u32;
pub const SSPIPFC_USE_CREDUIBROKER: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SUBSCRIBE_GENERIC_TLS_EXTENSION {
    pub Flags: u32,
    pub SubscriptionsCount: u32,
    pub Subscriptions: [TLS_EXTENSION_SUBSCRIPTION; 1],
}
impl Default for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SZ_ALG_MAX_SIZE: u32 = 64u32;
pub const Sasl_AuthZIDForbidden: SASL_AUTHZID_STATE = SASL_AUTHZID_STATE(0i32);
pub const Sasl_AuthZIDProcessed: SASL_AUTHZID_STATE = SASL_AUTHZID_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchGetExtensionsOptions(pub i32);
impl SchGetExtensionsOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SchGetExtensionsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SchGetExtensionsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SchGetExtensionsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SchGetExtensionsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SchGetExtensionsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SeAdtParmTypeAccessMask: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(7i32);
pub const SeAdtParmTypeAccessReason: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(29i32);
pub const SeAdtParmTypeClaims: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(32i32);
pub const SeAdtParmTypeDateTime: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(22i32);
pub const SeAdtParmTypeDuration: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(18i32);
pub const SeAdtParmTypeFileSpec: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(2i32);
pub const SeAdtParmTypeGuid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(13i32);
pub const SeAdtParmTypeHexInt64: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(15i32);
pub const SeAdtParmTypeHexUlong: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(10i32);
pub const SeAdtParmTypeLogonHours: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(25i32);
pub const SeAdtParmTypeLogonId: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(5i32);
pub const SeAdtParmTypeLogonIdAsSid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(33i32);
pub const SeAdtParmTypeLogonIdEx: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(35i32);
pub const SeAdtParmTypeLogonIdNoSid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(26i32);
pub const SeAdtParmTypeLuid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(14i32);
pub const SeAdtParmTypeMessage: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(21i32);
pub const SeAdtParmTypeMultiSzString: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(34i32);
pub const SeAdtParmTypeNoLogonId: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(6i32);
pub const SeAdtParmTypeNoUac: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(20i32);
pub const SeAdtParmTypeNone: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(0i32);
pub const SeAdtParmTypeObjectTypes: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(9i32);
pub const SeAdtParmTypePrivs: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(8i32);
pub const SeAdtParmTypePtr: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(11i32);
pub const SeAdtParmTypeResourceAttribute: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(31i32);
pub const SeAdtParmTypeSD: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(24i32);
pub const SeAdtParmTypeSid: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(4i32);
pub const SeAdtParmTypeSidList: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(17i32);
pub const SeAdtParmTypeSockAddr: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(23i32);
pub const SeAdtParmTypeSockAddrNoPort: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(28i32);
pub const SeAdtParmTypeStagingReason: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(30i32);
pub const SeAdtParmTypeString: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(1i32);
pub const SeAdtParmTypeStringList: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(16i32);
pub const SeAdtParmTypeTime: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(12i32);
pub const SeAdtParmTypeUlong: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(3i32);
pub const SeAdtParmTypeUlongNoConv: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(27i32);
pub const SeAdtParmTypeUserAccountControl: SE_ADT_PARAMETER_TYPE = SE_ADT_PARAMETER_TYPE(19i32);
pub const SecApplicationProtocolNegotiationExt_ALPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(2i32);
pub const SecApplicationProtocolNegotiationExt_NPN: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(1i32);
pub const SecApplicationProtocolNegotiationExt_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT(0i32);
pub const SecApplicationProtocolNegotiationStatus_None: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(0i32);
pub const SecApplicationProtocolNegotiationStatus_SelectedClientOnly: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(2i32);
pub const SecApplicationProtocolNegotiationStatus_Success: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS = SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut core::ffi::c_void,
}
impl Default for SecBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: *mut SecBuffer,
}
impl Default for SecBufferDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecDelegationType(pub i32);
pub const SecDirectory: SecDelegationType = SecDelegationType(3i32);
pub const SecFull: SecDelegationType = SecDelegationType(0i32);
pub const SecNameAlternateId: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(1i32);
pub const SecNameDN: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(3i32);
pub const SecNameFlat: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(2i32);
pub const SecNameSPN: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(4i32);
pub const SecNameSamCompatible: SECPKG_NAME_TYPE = SECPKG_NAME_TYPE(0i32);
pub const SecObject: SecDelegationType = SecDelegationType(4i32);
pub const SecPkgAttrLastClientTokenMaybe: SECPKG_ATTR_LCT_STATUS = SECPKG_ATTR_LCT_STATUS(2i32);
pub const SecPkgAttrLastClientTokenNo: SECPKG_ATTR_LCT_STATUS = SECPKG_ATTR_LCT_STATUS(1i32);
pub const SecPkgAttrLastClientTokenYes: SECPKG_ATTR_LCT_STATUS = SECPKG_ATTR_LCT_STATUS(0i32);
pub const SecPkgCallPackageMaxMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1026i32);
pub const SecPkgCallPackageMinMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1024i32);
pub const SecPkgCallPackagePinDcMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1024i32);
pub const SecPkgCallPackageTransferCredMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1026i32);
pub const SecPkgCallPackageUnpinAllDcsMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = SECPKG_CALL_PACKAGE_MESSAGE_TYPE(1025i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_AccessToken {
    pub AccessToken: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_AccessToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ApplicationProtocol {
    pub ProtoNegoStatus: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS,
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolIdSize: u8,
    pub ProtocolId: [u8; 255],
}
impl Default for SecPkgContext_ApplicationProtocol {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_AuthorityA {
    pub sAuthorityName: *mut i8,
}
impl Default for SecPkgContext_AuthorityA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_AuthorityW {
    pub sAuthorityName: *mut u16,
}
impl Default for SecPkgContext_AuthorityW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_AuthzID {
    pub AuthzIDLength: u32,
    pub AuthzID: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_Bindings {
    pub BindingsLength: u32,
    pub Bindings: *mut SEC_CHANNEL_BINDINGS,
}
impl Default for SecPkgContext_Bindings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_CertInfo {
    pub dwVersion: u32,
    pub cbSubjectName: u32,
    pub pwszSubjectName: windows_core::PWSTR,
    pub cbIssuerName: u32,
    pub pwszIssuerName: windows_core::PWSTR,
    pub dwKeySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_CertificateValidationResult {
    pub dwChainErrorStatus: u32,
    pub hrVerifyChainStatus: windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_Certificates {
    pub cCertificates: u32,
    pub cbCertificateChain: u32,
    pub pbCertificateChain: *mut u8,
}
impl Default for SecPkgContext_Certificates {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_CipherInfo {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub dwCipherSuite: u32,
    pub dwBaseCipherSuite: u32,
    pub szCipherSuite: [u16; 64],
    pub szCipher: [u16; 64],
    pub dwCipherLen: u32,
    pub dwCipherBlockLen: u32,
    pub szHash: [u16; 64],
    pub dwHashLen: u32,
    pub szExchange: [u16; 64],
    pub dwMinExchangeLen: u32,
    pub dwMaxExchangeLen: u32,
    pub szCertificate: [u16; 64],
    pub dwKeyType: u32,
}
impl Default for SecPkgContext_CipherInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_ClientCertPolicyResult {
    pub dwPolicyResult: windows_core::HRESULT,
    pub guidPolicyId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ClientSpecifiedTarget {
    pub sTargetName: *mut u16,
}
impl Default for SecPkgContext_ClientSpecifiedTarget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_ConnectionInfo {
    pub dwProtocol: u32,
    pub aiCipher: super::super::Cryptography::ALG_ID,
    pub dwCipherStrength: u32,
    pub aiHash: super::super::Cryptography::ALG_ID,
    pub dwHashStrength: u32,
    pub aiExch: super::super::Cryptography::ALG_ID,
    pub dwExchStrength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ConnectionInfoEx {
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub szCipher: [u16; 64],
    pub dwCipherStrength: u32,
    pub szHash: [u16; 64],
    pub dwHashStrength: u32,
    pub szExchange: [u16; 64],
    pub dwExchStrength: u32,
}
impl Default for SecPkgContext_ConnectionInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_CredInfo {
    pub CredClass: SECPKG_CRED_CLASS,
    pub IsPromptingNeeded: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_CredentialNameA {
    pub CredentialType: u32,
    pub sCredentialName: *mut i8,
}
impl Default for SecPkgContext_CredentialNameA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_CredentialNameW {
    pub CredentialType: u32,
    pub sCredentialName: *mut u16,
}
impl Default for SecPkgContext_CredentialNameW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_DceInfo {
    pub AuthzSvc: u32,
    pub pPac: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_DceInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_EapKeyBlock {
    pub rgbKeys: [u8; 128],
    pub rgbIVs: [u8; 64],
}
impl Default for SecPkgContext_EapKeyBlock {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_EapPrfInfo {
    pub dwVersion: u32,
    pub cbPrfData: u32,
    pub pbPrfData: *mut u8,
}
impl Default for SecPkgContext_EapPrfInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_EarlyStart {
    pub dwEarlyStartFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_Flags {
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_IssuerListInfoEx {
    pub aIssuers: *mut super::super::Cryptography::CRYPT_INTEGER_BLOB,
    pub cIssuers: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SecPkgContext_IssuerListInfoEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_KeyInfoA {
    pub sSignatureAlgorithmName: *mut i8,
    pub sEncryptAlgorithmName: *mut i8,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl Default for SecPkgContext_KeyInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_KeyInfoW {
    pub sSignatureAlgorithmName: *mut u16,
    pub sEncryptAlgorithmName: *mut u16,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
    pub EncryptAlgorithm: u32,
}
impl Default for SecPkgContext_KeyInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_KeyingMaterial {
    pub cbKeyingMaterial: u32,
    pub pbKeyingMaterial: *mut u8,
}
impl Default for SecPkgContext_KeyingMaterial {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_KeyingMaterialInfo {
    pub cbLabel: u16,
    pub pszLabel: windows_core::PSTR,
    pub cbContextValue: u16,
    pub pbContextValue: *mut u8,
    pub cbKeyingMaterial: u32,
}
impl Default for SecPkgContext_KeyingMaterialInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_KeyingMaterial_Inproc {
    pub cbLabel: u16,
    pub pszLabel: windows_core::PSTR,
    pub cbContextValue: u16,
    pub pbContextValue: *mut u8,
    pub cbKeyingMaterial: u32,
    pub pbKeyingMaterial: *mut u8,
}
impl Default for SecPkgContext_KeyingMaterial_Inproc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_LastClientTokenStatus {
    pub LastClientTokenStatus: SECPKG_ATTR_LCT_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_Lifespan {
    pub tsStart: i64,
    pub tsExpiry: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_LocalCredentialInfo {
    pub cbCertificateChain: u32,
    pub pbCertificateChain: *mut u8,
    pub cCertificates: u32,
    pub fFlags: u32,
    pub dwBits: u32,
}
impl Default for SecPkgContext_LocalCredentialInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_LogoffTime {
    pub tsLogoffTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_MappedCredAttr {
    pub dwAttribute: u32,
    pub pvBuffer: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_MappedCredAttr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NamesA {
    pub sUserName: *mut i8,
}
impl Default for SecPkgContext_NamesA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NamesW {
    pub sUserName: *mut u16,
}
impl Default for SecPkgContext_NamesW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NativeNamesA {
    pub sClientName: *mut i8,
    pub sServerName: *mut i8,
}
impl Default for SecPkgContext_NativeNamesA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NativeNamesW {
    pub sClientName: *mut u16,
    pub sServerName: *mut u16,
}
impl Default for SecPkgContext_NativeNamesW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NegoKeys {
    pub KeyType: u32,
    pub KeyLength: u16,
    pub KeyValue: *mut u8,
    pub VerifyKeyType: u32,
    pub VerifyKeyLength: u16,
    pub VerifyKeyValue: *mut u8,
}
impl Default for SecPkgContext_NegoKeys {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_NegoPackageInfo {
    pub PackageMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_NegoStatus {
    pub LastStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NegotiatedTlsExtensions {
    pub ExtensionsCount: u32,
    pub Extensions: *mut u16,
}
impl Default for SecPkgContext_NegotiatedTlsExtensions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NegotiationInfoA {
    pub PackageInfo: *mut SecPkgInfoA,
    pub NegotiationState: u32,
}
impl Default for SecPkgContext_NegotiationInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_NegotiationInfoW {
    pub PackageInfo: *mut SecPkgInfoW,
    pub NegotiationState: u32,
}
impl Default for SecPkgContext_NegotiationInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_PackageInfoA {
    pub PackageInfo: *mut SecPkgInfoA,
}
impl Default for SecPkgContext_PackageInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_PackageInfoW {
    pub PackageInfo: *mut SecPkgInfoW,
}
impl Default for SecPkgContext_PackageInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_PasswordExpiry {
    pub tsPasswordExpires: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ProtoInfoA {
    pub sProtocolName: *mut i8,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl Default for SecPkgContext_ProtoInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ProtoInfoW {
    pub sProtocolName: *mut u16,
    pub majorVersion: u32,
    pub minorVersion: u32,
}
impl Default for SecPkgContext_ProtoInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_RemoteCredentialInfo {
    pub cbCertificateChain: u32,
    pub pbCertificateChain: *mut u8,
    pub cCertificates: u32,
    pub fFlags: u32,
    pub dwBits: u32,
}
impl Default for SecPkgContext_RemoteCredentialInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SaslContext {
    pub SaslContext: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_SaslContext {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SessionAppData {
    pub dwFlags: u32,
    pub cbAppData: u32,
    pub pbAppData: *mut u8,
}
impl Default for SecPkgContext_SessionAppData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SessionInfo {
    pub dwFlags: u32,
    pub cbSessionId: u32,
    pub rgbSessionId: [u8; 32],
}
impl Default for SecPkgContext_SessionInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SessionKey {
    pub SessionKeyLength: u32,
    pub SessionKey: *mut u8,
}
impl Default for SecPkgContext_SessionKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_Sizes {
    pub cbMaxToken: u32,
    pub cbMaxSignature: u32,
    pub cbBlockSize: u32,
    pub cbSecurityTrailer: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SrtpParameters {
    pub ProtectionProfile: u16,
    pub MasterKeyIdentifierSize: u8,
    pub MasterKeyIdentifier: *mut u8,
}
impl Default for SecPkgContext_SrtpParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_StreamSizes {
    pub cbHeader: u32,
    pub cbTrailer: u32,
    pub cbMaximumMessage: u32,
    pub cBuffers: u32,
    pub cbBlockSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SubjectAttributes {
    pub AttributeInfo: *mut core::ffi::c_void,
}
impl Default for SecPkgContext_SubjectAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_SupportedSignatures {
    pub cSignatureAndHashAlgorithms: u16,
    pub pSignatureAndHashAlgorithms: *mut u16,
}
impl Default for SecPkgContext_SupportedSignatures {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_Target {
    pub TargetLength: u32,
    pub Target: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_TargetInformation {
    pub MarshalledTargetInfoLength: u32,
    pub MarshalledTargetInfo: *mut u8,
}
impl Default for SecPkgContext_TargetInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_TokenBinding {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub KeyParametersSize: u16,
    pub KeyParameters: *mut u8,
}
impl Default for SecPkgContext_TokenBinding {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_UiInfo {
    pub hParentWindow: super::super::super::Foundation::HWND,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgContext_UserFlags {
    pub UserFlags: u32,
}
pub const SecPkgCredClass_Ephemeral: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(10i32);
pub const SecPkgCredClass_Explicit: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(40i32);
pub const SecPkgCredClass_None: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(0i32);
pub const SecPkgCredClass_PersistedGeneric: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(20i32);
pub const SecPkgCredClass_PersistedSpecific: SECPKG_CRED_CLASS = SECPKG_CRED_CLASS(30i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCred_CipherStrengths {
    pub dwMinimumCipherStrength: u32,
    pub dwMaximumCipherStrength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCred_ClientCertPolicy {
    pub dwFlags: u32,
    pub guidPolicyId: windows_core::GUID,
    pub dwCertFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub fCheckRevocationFreshnessTime: windows_core::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub fOmitUsageCheck: windows_core::BOOL,
    pub pwszSslCtlStoreName: windows_core::PWSTR,
    pub pwszSslCtlIdentifier: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCred_SessionTicketKey {
    pub TicketInfoVersion: u32,
    pub KeyId: [u8; 16],
    pub KeyingMaterial: [u8; 64],
    pub KeyingMaterialSize: u8,
}
impl Default for SecPkgCred_SessionTicketKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCred_SessionTicketKeys {
    pub cSessionTicketKeys: u32,
    pub pSessionTicketKeys: *mut SecPkgCred_SessionTicketKey,
}
impl Default for SecPkgCred_SessionTicketKeys {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCred_SupportedAlgs {
    pub cSupportedAlgs: u32,
    pub palgSupportedAlgs: *mut super::super::Cryptography::ALG_ID,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SecPkgCred_SupportedAlgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCred_SupportedProtocols {
    pub grbitProtocol: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCredentials_Cert {
    pub EncodedCertSize: u32,
    pub EncodedCert: *mut u8,
}
impl Default for SecPkgCredentials_Cert {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SecPkgCredentials_KdcProxySettingsW {
    pub Version: u32,
    pub Flags: u32,
    pub ProxyServerOffset: u16,
    pub ProxyServerLength: u16,
    pub ClientTlsCredOffset: u16,
    pub ClientTlsCredLength: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCredentials_NamesA {
    pub sUserName: *mut i8,
}
impl Default for SecPkgCredentials_NamesA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCredentials_NamesW {
    pub sUserName: *mut u16,
}
impl Default for SecPkgCredentials_NamesW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCredentials_SSIProviderA {
    pub sProviderName: *mut i8,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: windows_core::PSTR,
}
impl Default for SecPkgCredentials_SSIProviderA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgCredentials_SSIProviderW {
    pub sProviderName: *mut u16,
    pub ProviderInfoLength: u32,
    pub ProviderInfo: windows_core::PSTR,
}
impl Default for SecPkgCredentials_SSIProviderW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgInfoA {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut i8,
    pub Comment: *mut i8,
}
impl Default for SecPkgInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgInfoW {
    pub fCapabilities: u32,
    pub wVersion: u16,
    pub wRPCID: u16,
    pub cbMaxToken: u32,
    pub Name: *mut u16,
    pub Comment: *mut u16,
}
impl Default for SecPkgInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SecService: SecDelegationType = SecDelegationType(1i32);
pub const SecSessionPrimaryCred: SECPKG_SESSIONINFO_TYPE = SECPKG_SESSIONINFO_TYPE(0i32);
pub const SecTrafficSecret_Client: SEC_TRAFFIC_SECRET_TYPE = SEC_TRAFFIC_SECRET_TYPE(1i32);
pub const SecTrafficSecret_None: SEC_TRAFFIC_SECRET_TYPE = SEC_TRAFFIC_SECRET_TYPE(0i32);
pub const SecTrafficSecret_Server: SEC_TRAFFIC_SECRET_TYPE = SEC_TRAFFIC_SECRET_TYPE(2i32);
pub const SecTree: SecDelegationType = SecDelegationType(2i32);
pub const SecpkgContextThunks: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(2i32);
pub const SecpkgExtraOids: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(5i32);
pub const SecpkgGssInfo: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(1i32);
pub const SecpkgMaxInfo: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(6i32);
pub const SecpkgMutualAuthLevel: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(3i32);
pub const SecpkgNego2Info: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(7i32);
pub const SecpkgWowClientDll: SECPKG_EXTENDED_INFORMATION_CLASS = SECPKG_EXTENDED_INFORMATION_CLASS(4i32);
#[repr(C)]
#[cfg(feature = "Win32_Security_Credentials")]
#[derive(Clone, Copy, Debug)]
pub struct SecurityFunctionTableA {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesA: ENUMERATE_SECURITY_PACKAGES_FN_A,
    pub QueryCredentialsAttributesA: QUERY_CREDENTIALS_ATTRIBUTES_FN_A,
    pub AcquireCredentialsHandleA: ACQUIRE_CREDENTIALS_HANDLE_FN_A,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut core::ffi::c_void,
    pub InitializeSecurityContextA: INITIALIZE_SECURITY_CONTEXT_FN_A,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesA: QUERY_CONTEXT_ATTRIBUTES_FN_A,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoA: QUERY_SECURITY_PACKAGE_INFO_FN_A,
    pub Reserved3: *mut core::ffi::c_void,
    pub Reserved4: *mut core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextA: IMPORT_SECURITY_CONTEXT_FN_A,
    pub AddCredentialsA: ADD_CREDENTIALS_FN_A,
    pub Reserved8: *mut core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesA: SET_CONTEXT_ATTRIBUTES_FN_A,
    pub SetCredentialsAttributesA: SET_CREDENTIALS_ATTRIBUTES_FN_A,
    pub ChangeAccountPasswordA: CHANGE_PASSWORD_FN_A,
    pub QueryContextAttributesExA: QUERY_CONTEXT_ATTRIBUTES_EX_FN_A,
    pub QueryCredentialsAttributesExA: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A,
}
#[cfg(feature = "Win32_Security_Credentials")]
impl Default for SecurityFunctionTableA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Credentials")]
#[derive(Clone, Copy, Debug)]
pub struct SecurityFunctionTableW {
    pub dwVersion: u32,
    pub EnumerateSecurityPackagesW: ENUMERATE_SECURITY_PACKAGES_FN_W,
    pub QueryCredentialsAttributesW: QUERY_CREDENTIALS_ATTRIBUTES_FN_W,
    pub AcquireCredentialsHandleW: ACQUIRE_CREDENTIALS_HANDLE_FN_W,
    pub FreeCredentialsHandle: FREE_CREDENTIALS_HANDLE_FN,
    pub Reserved2: *mut core::ffi::c_void,
    pub InitializeSecurityContextW: INITIALIZE_SECURITY_CONTEXT_FN_W,
    pub AcceptSecurityContext: ACCEPT_SECURITY_CONTEXT_FN,
    pub CompleteAuthToken: COMPLETE_AUTH_TOKEN_FN,
    pub DeleteSecurityContext: DELETE_SECURITY_CONTEXT_FN,
    pub ApplyControlToken: APPLY_CONTROL_TOKEN_FN,
    pub QueryContextAttributesW: QUERY_CONTEXT_ATTRIBUTES_FN_W,
    pub ImpersonateSecurityContext: IMPERSONATE_SECURITY_CONTEXT_FN,
    pub RevertSecurityContext: REVERT_SECURITY_CONTEXT_FN,
    pub MakeSignature: MAKE_SIGNATURE_FN,
    pub VerifySignature: VERIFY_SIGNATURE_FN,
    pub FreeContextBuffer: FREE_CONTEXT_BUFFER_FN,
    pub QuerySecurityPackageInfoW: QUERY_SECURITY_PACKAGE_INFO_FN_W,
    pub Reserved3: *mut core::ffi::c_void,
    pub Reserved4: *mut core::ffi::c_void,
    pub ExportSecurityContext: EXPORT_SECURITY_CONTEXT_FN,
    pub ImportSecurityContextW: IMPORT_SECURITY_CONTEXT_FN_W,
    pub AddCredentialsW: ADD_CREDENTIALS_FN_W,
    pub Reserved8: *mut core::ffi::c_void,
    pub QuerySecurityContextToken: QUERY_SECURITY_CONTEXT_TOKEN_FN,
    pub EncryptMessage: ENCRYPT_MESSAGE_FN,
    pub DecryptMessage: DECRYPT_MESSAGE_FN,
    pub SetContextAttributesW: SET_CONTEXT_ATTRIBUTES_FN_W,
    pub SetCredentialsAttributesW: SET_CREDENTIALS_ATTRIBUTES_FN_W,
    pub ChangeAccountPasswordW: CHANGE_PASSWORD_FN_W,
    pub QueryContextAttributesExW: QUERY_CONTEXT_ATTRIBUTES_EX_FN_W,
    pub QueryCredentialsAttributesExW: QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W,
}
#[cfg(feature = "Win32_Security_Credentials")]
impl Default for SecurityFunctionTableW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SpAcceptCredentialsFn = Option<unsafe extern "system" fn(logontype: SECURITY_LOGON_TYPE, accountname: *const LSA_UNICODE_STRING, primarycredentials: *const SECPKG_PRIMARY_CRED, supplementalcredentials: *const SECPKG_SUPPLEMENTAL_CRED) -> super::super::super::Foundation::NTSTATUS>;
pub type SpAcceptLsaModeContextFn = Option<unsafe extern "system" fn(credentialhandle: usize, contexthandle: usize, inputbuffer: *const SecBufferDesc, contextrequirements: u32, targetdatarep: u32, newcontexthandle: *mut usize, outputbuffer: *mut SecBufferDesc, contextattributes: *mut u32, expirationtime: *mut i64, mappedcontext: *mut bool, contextdata: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type SpAcquireCredentialsHandleFn = Option<unsafe extern "system" fn(principalname: *const LSA_UNICODE_STRING, credentialuseflags: u32, logonid: *const super::super::super::Foundation::LUID, authorizationdata: *const core::ffi::c_void, getkeyfunciton: *const core::ffi::c_void, getkeyargument: *const core::ffi::c_void, credentialhandle: *mut usize, expirationtime: *mut i64) -> super::super::super::Foundation::NTSTATUS>;
pub type SpAddCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: usize, principalname: *const LSA_UNICODE_STRING, package: *const LSA_UNICODE_STRING, credentialuseflags: u32, authorizationdata: *const core::ffi::c_void, getkeyfunciton: *const core::ffi::c_void, getkeyargument: *const core::ffi::c_void, expirationtime: *mut i64) -> super::super::super::Foundation::NTSTATUS>;
pub type SpApplyControlTokenFn = Option<unsafe extern "system" fn(contexthandle: usize, controltoken: *const SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
pub type SpChangeAccountPasswordFn = Option<unsafe extern "system" fn(pdomainname: *const LSA_UNICODE_STRING, paccountname: *const LSA_UNICODE_STRING, poldpassword: *const LSA_UNICODE_STRING, pnewpassword: *const LSA_UNICODE_STRING, impersonating: bool, poutput: *mut SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
pub type SpCompleteAuthTokenFn = Option<unsafe extern "system" fn(contexthandle: usize, inputbuffer: *const SecBufferDesc) -> super::super::super::Foundation::NTSTATUS>;
pub type SpDeleteContextFn = Option<unsafe extern "system" fn(contexthandle: usize) -> super::super::super::Foundation::NTSTATUS>;
pub type SpDeleteCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: usize, key: *const SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type SpExchangeMetaDataFn = Option<unsafe extern "system" fn(credentialhandle: usize, targetname: *const LSA_UNICODE_STRING, contextrequirements: u32, metadatalength: u32, metadata: *const u8, contexthandle: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
pub type SpExportSecurityContextFn = Option<unsafe extern "system" fn(phcontext: usize, fflags: u32, ppackedcontext: *mut SecBuffer, ptoken: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
pub type SpExtractTargetInfoFn = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, ppvtargetinfo: *mut *mut core::ffi::c_void, pcbtargetinfo: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpFormatCredentialsFn = Option<unsafe extern "system" fn(credentials: *const SecBuffer, formattedcredentials: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type SpFreeCredentialsHandleFn = Option<unsafe extern "system" fn(credentialhandle: usize) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetContextTokenFn = Option<unsafe extern "system" fn(contexthandle: usize, impersonationtoken: *mut super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetCredUIContextFn = Option<unsafe extern "system" fn(contexthandle: usize, credtype: *const windows_core::GUID, flatcreduicontextlength: *mut u32, flatcreduicontext: *mut *mut u8) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: usize, credentials: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetExtendedInformationFn = Option<unsafe extern "system" fn(class: SECPKG_EXTENDED_INFORMATION_CLASS, ppinformation: *mut *mut SECPKG_EXTENDED_INFORMATION) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetInfoFn = Option<unsafe extern "system" fn(packageinfo: *mut SecPkgInfoA) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetRemoteCredGuardLogonBufferFn = Option<unsafe extern "system" fn(credhandle: usize, contexthandle: usize, targetname: *const LSA_UNICODE_STRING, redirectedlogonhandle: *mut super::super::super::Foundation::HANDLE, callback: *mut PLSA_REDIRECTED_LOGON_CALLBACK, cleanupcallback: *mut PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK, logonbuffersize: *mut u32, logonbuffer: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetRemoteCredGuardSupplementalCredsFn = Option<unsafe extern "system" fn(credhandle: usize, targetname: *const LSA_UNICODE_STRING, redirectedlogonhandle: *mut super::super::super::Foundation::HANDLE, callback: *mut PLSA_REDIRECTED_LOGON_CALLBACK, cleanupcallback: *mut PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK, supplementalcredssize: *mut u32, supplementalcreds: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetTbalSupplementalCredsFn = Option<unsafe extern "system" fn(logonid: super::super::super::Foundation::LUID, supplementalcredssize: *mut u32, supplementalcreds: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type SpGetUserInfoFn = Option<unsafe extern "system" fn(logonid: *const super::super::super::Foundation::LUID, flags: u32, userdata: *mut *mut SECURITY_USER_DATA) -> super::super::super::Foundation::NTSTATUS>;
pub type SpImportSecurityContextFn = Option<unsafe extern "system" fn(ppackedcontext: *const SecBuffer, token: super::super::super::Foundation::HANDLE, phcontext: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
pub type SpInitLsaModeContextFn = Option<unsafe extern "system" fn(credentialhandle: usize, contexthandle: usize, targetname: *const LSA_UNICODE_STRING, contextrequirements: u32, targetdatarep: u32, inputbuffers: *const SecBufferDesc, newcontexthandle: *mut usize, outputbuffers: *mut SecBufferDesc, contextattributes: *mut u32, expirationtime: *mut i64, mappedcontext: *mut bool, contextdata: *mut SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type SpInitUserModeContextFn = Option<unsafe extern "system" fn(contexthandle: usize, packedcontext: *const SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(all(feature = "Win32_Security_Credentials", feature = "Win32_System_Threading"))]
pub type SpInitializeFn = Option<unsafe extern "system" fn(packageid: usize, parameters: *const SECPKG_PARAMETERS, functiontable: *const LSA_SECPKG_FUNCTION_TABLE) -> super::super::super::Foundation::NTSTATUS>;
pub type SpInstanceInitFn = Option<unsafe extern "system" fn(version: u32, functiontable: *const SECPKG_DLL_FUNCTIONS, userfunctions: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(all(feature = "Win32_Security_Credentials", feature = "Win32_System_Threading"))]
pub type SpLsaModeInitializeFn = Option<unsafe extern "system" fn(lsaversion: u32, packageversion: *mut u32, pptables: *mut *mut SECPKG_FUNCTION_TABLE, pctables: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpMakeSignatureFn = Option<unsafe extern "system" fn(contexthandle: usize, qualityofprotection: u32, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpMarshalAttributeDataFn = Option<unsafe extern "system" fn(attributeinfo: u32, attribute: u32, attributedatasize: u32, attributedata: *const u8, marshaledattributedatasize: *mut u32, marshaledattributedata: *mut *mut u8) -> super::super::super::Foundation::NTSTATUS>;
pub type SpMarshallSupplementalCredsFn = Option<unsafe extern "system" fn(credentialsize: u32, credentials: *const u8, marshalledcredsize: *mut u32, marshalledcreds: *mut *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type SpQueryContextAttributesFn = Option<unsafe extern "system" fn(contexthandle: usize, contextattribute: u32, buffer: *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type SpQueryCredentialsAttributesFn = Option<unsafe extern "system" fn(credentialhandle: usize, credentialattribute: u32, buffer: *mut core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS>;
pub type SpQueryMetaDataFn = Option<unsafe extern "system" fn(credentialhandle: usize, targetname: *const LSA_UNICODE_STRING, contextrequirements: u32, metadatalength: *mut u32, metadata: *mut *mut u8, contexthandle: *mut usize) -> super::super::super::Foundation::NTSTATUS>;
pub type SpSaveCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: usize, credentials: *const SecBuffer) -> super::super::super::Foundation::NTSTATUS>;
pub type SpSealMessageFn = Option<unsafe extern "system" fn(contexthandle: usize, qualityofprotection: u32, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpSetContextAttributesFn = Option<unsafe extern "system" fn(contexthandle: usize, contextattribute: u32, buffer: *const core::ffi::c_void, buffersize: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpSetCredentialsAttributesFn = Option<unsafe extern "system" fn(credentialhandle: usize, credentialattribute: u32, buffer: *const core::ffi::c_void, buffersize: u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpSetExtendedInformationFn = Option<unsafe extern "system" fn(class: SECPKG_EXTENDED_INFORMATION_CLASS, info: *const SECPKG_EXTENDED_INFORMATION) -> super::super::super::Foundation::NTSTATUS>;
pub type SpShutdownFn = Option<unsafe extern "system" fn() -> super::super::super::Foundation::NTSTATUS>;
pub type SpUnsealMessageFn = Option<unsafe extern "system" fn(contexthandle: usize, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32, qualityofprotection: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpUpdateCredentialsFn = Option<unsafe extern "system" fn(contexthandle: usize, credtype: *const windows_core::GUID, flatcreduicontextlength: u32, flatcreduicontext: *const u8) -> super::super::super::Foundation::NTSTATUS>;
pub type SpUserModeInitializeFn = Option<unsafe extern "system" fn(lsaversion: u32, packageversion: *mut u32, pptables: *mut *mut SECPKG_USER_FUNCTION_TABLE, pctables: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
pub type SpValidateTargetInfoFn = Option<unsafe extern "system" fn(clientrequest: *const *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, targetinfo: *const SECPKG_TARGETINFO) -> super::super::super::Foundation::NTSTATUS>;
pub type SpVerifySignatureFn = Option<unsafe extern "system" fn(contexthandle: usize, messagebuffers: *const SecBufferDesc, messagesequencenumber: u32, qualityofprotection: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Security_Cryptography")]
pub type SslDeserializeCertificateStoreFn = Option<unsafe extern "system" fn(serializedcertificatestore: super::super::Cryptography::CRYPT_INTEGER_BLOB, ppcertcontext: *mut *mut super::super::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT>;
pub type SslGetExtensionsFn = Option<unsafe extern "system" fn(clienthello: *const u8, clienthellobytesize: u32, genericextensions: *mut SCH_EXTENSION_DATA, genericextensionscount: u8, bytestoread: *mut u32, flags: SchGetExtensionsOptions) -> windows_core::HRESULT>;
pub type SslGetServerIdentityFn = Option<unsafe extern "system" fn(clienthello: *const u8, clienthellosize: u32, serveridentity: *mut *mut u8, serveridentitysize: *mut u32, flags: u32) -> windows_core::HRESULT>;
pub const TLS1SP_NAME: windows_core::PCWSTR = windows_core::w!("Microsoft TLS 1.0");
pub const TLS1SP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft TLS 1.0");
pub const TLS1SP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft TLS 1.0");
pub const TLS1_ALERT_ACCESS_DENIED: u32 = 49u32;
pub const TLS1_ALERT_BAD_CERTIFICATE: u32 = 42u32;
pub const TLS1_ALERT_BAD_RECORD_MAC: u32 = 20u32;
pub const TLS1_ALERT_CERTIFICATE_EXPIRED: u32 = 45u32;
pub const TLS1_ALERT_CERTIFICATE_REVOKED: u32 = 44u32;
pub const TLS1_ALERT_CERTIFICATE_UNKNOWN: u32 = 46u32;
pub const TLS1_ALERT_CLOSE_NOTIFY: u32 = 0u32;
pub const TLS1_ALERT_DECODE_ERROR: u32 = 50u32;
pub const TLS1_ALERT_DECOMPRESSION_FAIL: u32 = 30u32;
pub const TLS1_ALERT_DECRYPTION_FAILED: u32 = 21u32;
pub const TLS1_ALERT_DECRYPT_ERROR: u32 = 51u32;
pub const TLS1_ALERT_EXPORT_RESTRICTION: u32 = 60u32;
pub const TLS1_ALERT_FATAL: SCHANNEL_ALERT_TOKEN_ALERT_TYPE = SCHANNEL_ALERT_TOKEN_ALERT_TYPE(2u32);
pub const TLS1_ALERT_HANDSHAKE_FAILURE: u32 = 40u32;
pub const TLS1_ALERT_ILLEGAL_PARAMETER: u32 = 47u32;
pub const TLS1_ALERT_INSUFFIENT_SECURITY: u32 = 71u32;
pub const TLS1_ALERT_INTERNAL_ERROR: u32 = 80u32;
pub const TLS1_ALERT_NO_APP_PROTOCOL: u32 = 120u32;
pub const TLS1_ALERT_NO_RENEGOTIATION: u32 = 100u32;
pub const TLS1_ALERT_PROTOCOL_VERSION: u32 = 70u32;
pub const TLS1_ALERT_RECORD_OVERFLOW: u32 = 22u32;
pub const TLS1_ALERT_UNEXPECTED_MESSAGE: u32 = 10u32;
pub const TLS1_ALERT_UNKNOWN_CA: u32 = 48u32;
pub const TLS1_ALERT_UNKNOWN_PSK_IDENTITY: u32 = 115u32;
pub const TLS1_ALERT_UNSUPPORTED_CERT: u32 = 43u32;
pub const TLS1_ALERT_UNSUPPORTED_EXT: u32 = 110u32;
pub const TLS1_ALERT_USER_CANCELED: u32 = 90u32;
pub const TLS1_ALERT_WARNING: SCHANNEL_ALERT_TOKEN_ALERT_TYPE = SCHANNEL_ALERT_TOKEN_ALERT_TYPE(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TLS_EXTENSION_SUBSCRIPTION {
    pub ExtensionType: u16,
    pub HandshakeType: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TLS_PARAMETERS {
    pub cAlpnIds: u32,
    pub rgstrAlpnIds: *mut LSA_UNICODE_STRING,
    pub grbitDisabledProtocols: u32,
    pub cDisabledCrypto: u32,
    pub pDisabledCrypto: *mut CRYPTO_SETTINGS,
    pub dwFlags: u32,
}
impl Default for TLS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TLS_PARAMS_OPTIONAL: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOKENBINDING_EXTENSION_FORMAT(pub i32);
pub const TOKENBINDING_EXTENSION_FORMAT_UNDEFINED: TOKENBINDING_EXTENSION_FORMAT = TOKENBINDING_EXTENSION_FORMAT(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKENBINDING_IDENTIFIER {
    pub keyType: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOKENBINDING_KEY_PARAMETERS_TYPE(pub i32);
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_ANYEXISTING: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(255i32);
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_ECDSAP256: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(2i32);
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_RSA2048_PKCS: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(0i32);
pub const TOKENBINDING_KEY_PARAMETERS_TYPE_RSA2048_PSS: TOKENBINDING_KEY_PARAMETERS_TYPE = TOKENBINDING_KEY_PARAMETERS_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKENBINDING_KEY_TYPES {
    pub keyCount: u32,
    pub keyType: *mut TOKENBINDING_KEY_PARAMETERS_TYPE,
}
impl Default for TOKENBINDING_KEY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKENBINDING_RESULT_DATA {
    pub bindingType: TOKENBINDING_TYPE,
    pub identifierSize: u32,
    pub identifierData: *mut TOKENBINDING_IDENTIFIER,
    pub extensionFormat: TOKENBINDING_EXTENSION_FORMAT,
    pub extensionSize: u32,
    pub extensionData: *mut core::ffi::c_void,
}
impl Default for TOKENBINDING_RESULT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKENBINDING_RESULT_LIST {
    pub resultCount: u32,
    pub resultData: *mut TOKENBINDING_RESULT_DATA,
}
impl Default for TOKENBINDING_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOKENBINDING_TYPE(pub i32);
pub const TOKENBINDING_TYPE_PROVIDED: TOKENBINDING_TYPE = TOKENBINDING_TYPE(0i32);
pub const TOKENBINDING_TYPE_REFERRED: TOKENBINDING_TYPE = TOKENBINDING_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTED_CONTROLLERS_INFO {
    pub Entries: u32,
    pub Names: *mut LSA_UNICODE_STRING,
}
impl Default for TRUSTED_CONTROLLERS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTED_DOMAIN_AUTH_INFORMATION {
    pub IncomingAuthInfos: u32,
    pub IncomingAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
    pub IncomingPreviousAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
    pub OutgoingAuthInfos: u32,
    pub OutgoingAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
    pub OutgoingPreviousAuthenticationInformation: *mut LSA_AUTH_INFORMATION,
}
impl Default for TRUSTED_DOMAIN_AUTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_DOMAIN_FULL_INFORMATION2 {
    pub Information: TRUSTED_DOMAIN_INFORMATION_EX2,
    pub PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    pub AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_DOMAIN_INFORMATION_EX {
    pub Name: LSA_UNICODE_STRING,
    pub FlatName: LSA_UNICODE_STRING,
    pub Sid: super::super::PSID,
    pub TrustDirection: TRUSTED_DOMAIN_TRUST_DIRECTION,
    pub TrustType: TRUSTED_DOMAIN_TRUST_TYPE,
    pub TrustAttributes: TRUSTED_DOMAIN_TRUST_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTED_DOMAIN_INFORMATION_EX2 {
    pub Name: LSA_UNICODE_STRING,
    pub FlatName: LSA_UNICODE_STRING,
    pub Sid: super::super::PSID,
    pub TrustDirection: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub ForestTrustLength: u32,
    pub ForestTrustInfo: *mut u8,
}
impl Default for TRUSTED_DOMAIN_INFORMATION_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_DOMAIN_NAME_INFO {
    pub Name: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    pub SupportedEncryptionTypes: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUSTED_DOMAIN_TRUST_ATTRIBUTES(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUSTED_DOMAIN_TRUST_DIRECTION(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUSTED_DOMAIN_TRUST_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRUSTED_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_PASSWORD_INFO {
    pub Password: LSA_UNICODE_STRING,
    pub OldPassword: LSA_UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTED_POSIX_OFFSET_INFO {
    pub Offset: u32,
}
pub const TRUSTED_QUERY_AUTH: i32 = 64i32;
pub const TRUSTED_QUERY_CONTROLLERS: i32 = 2i32;
pub const TRUSTED_QUERY_DOMAIN_NAME: i32 = 1i32;
pub const TRUSTED_QUERY_POSIX: i32 = 8i32;
pub const TRUSTED_SET_AUTH: i32 = 32i32;
pub const TRUSTED_SET_CONTROLLERS: i32 = 4i32;
pub const TRUSTED_SET_POSIX: i32 = 16i32;
pub const TRUST_ATTRIBUTES_USER: u32 = 4278190080u32;
pub const TRUST_ATTRIBUTES_VALID: u32 = 4278386687u32;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(16u32);
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_ENABLE_TGT_DELEGATION: u32 = 2048u32;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION: u32 = 512u32;
pub const TRUST_ATTRIBUTE_DISABLE_AUTH_TARGET_VALIDATION: u32 = 4096u32;
pub const TRUST_ATTRIBUTE_FILTER_SIDS: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(4u32);
pub const TRUST_ATTRIBUTE_FOREST_TRANSITIVE: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(8u32);
pub const TRUST_ATTRIBUTE_NON_TRANSITIVE: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(1u32);
pub const TRUST_ATTRIBUTE_PIM_TRUST: u32 = 1024u32;
pub const TRUST_ATTRIBUTE_QUARANTINED_DOMAIN: u32 = 4u32;
pub const TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(64u32);
pub const TRUST_ATTRIBUTE_TREE_PARENT: u32 = 4194304u32;
pub const TRUST_ATTRIBUTE_TREE_ROOT: u32 = 8388608u32;
pub const TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS: u32 = 256u32;
pub const TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION: u32 = 128u32;
pub const TRUST_ATTRIBUTE_UPLEVEL_ONLY: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(2u32);
pub const TRUST_ATTRIBUTE_WITHIN_FOREST: TRUSTED_DOMAIN_TRUST_ATTRIBUTES = TRUSTED_DOMAIN_TRUST_ATTRIBUTES(32u32);
pub const TRUST_AUTH_TYPE_CLEAR: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(2u32);
pub const TRUST_AUTH_TYPE_NONE: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(0u32);
pub const TRUST_AUTH_TYPE_NT4OWF: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(1u32);
pub const TRUST_AUTH_TYPE_VERSION: LSA_AUTH_INFORMATION_AUTH_TYPE = LSA_AUTH_INFORMATION_AUTH_TYPE(3u32);
pub const TRUST_DIRECTION_BIDIRECTIONAL: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(3u32);
pub const TRUST_DIRECTION_DISABLED: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(0u32);
pub const TRUST_DIRECTION_INBOUND: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(1u32);
pub const TRUST_DIRECTION_OUTBOUND: TRUSTED_DOMAIN_TRUST_DIRECTION = TRUSTED_DOMAIN_TRUST_DIRECTION(2u32);
pub const TRUST_TYPE_AAD: u32 = 5u32;
pub const TRUST_TYPE_DCE: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(4u32);
pub const TRUST_TYPE_DOWNLEVEL: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(1u32);
pub const TRUST_TYPE_MIT: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(3u32);
pub const TRUST_TYPE_UPLEVEL: TRUSTED_DOMAIN_TRUST_TYPE = TRUSTED_DOMAIN_TRUST_TYPE(2u32);
pub const TlsHashAlgorithm_Md5: eTlsHashAlgorithm = eTlsHashAlgorithm(1i32);
pub const TlsHashAlgorithm_None: eTlsHashAlgorithm = eTlsHashAlgorithm(0i32);
pub const TlsHashAlgorithm_Sha1: eTlsHashAlgorithm = eTlsHashAlgorithm(2i32);
pub const TlsHashAlgorithm_Sha224: eTlsHashAlgorithm = eTlsHashAlgorithm(3i32);
pub const TlsHashAlgorithm_Sha256: eTlsHashAlgorithm = eTlsHashAlgorithm(4i32);
pub const TlsHashAlgorithm_Sha384: eTlsHashAlgorithm = eTlsHashAlgorithm(5i32);
pub const TlsHashAlgorithm_Sha512: eTlsHashAlgorithm = eTlsHashAlgorithm(6i32);
pub const TlsParametersCngAlgUsageCertSig: eTlsAlgorithmUsage = eTlsAlgorithmUsage(4i32);
pub const TlsParametersCngAlgUsageCipher: eTlsAlgorithmUsage = eTlsAlgorithmUsage(2i32);
pub const TlsParametersCngAlgUsageDigest: eTlsAlgorithmUsage = eTlsAlgorithmUsage(3i32);
pub const TlsParametersCngAlgUsageKeyExchange: eTlsAlgorithmUsage = eTlsAlgorithmUsage(0i32);
pub const TlsParametersCngAlgUsageSignature: eTlsAlgorithmUsage = eTlsAlgorithmUsage(1i32);
pub const TlsSignatureAlgorithm_Anonymous: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(0i32);
pub const TlsSignatureAlgorithm_Dsa: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(2i32);
pub const TlsSignatureAlgorithm_Ecdsa: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(3i32);
pub const TlsSignatureAlgorithm_Rsa: eTlsSignatureAlgorithm = eTlsSignatureAlgorithm(1i32);
pub const TrustedControllersInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(2i32);
pub const TrustedDomainAuthInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(7i32);
pub const TrustedDomainAuthInformationInternal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(9i32);
pub const TrustedDomainAuthInformationInternalAes: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(14i32);
pub const TrustedDomainFullInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(8i32);
pub const TrustedDomainFullInformation2Internal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(12i32);
pub const TrustedDomainFullInformationInternal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(10i32);
pub const TrustedDomainFullInformationInternalAes: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(15i32);
pub const TrustedDomainInformationBasic: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(5i32);
pub const TrustedDomainInformationEx: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(6i32);
pub const TrustedDomainInformationEx2Internal: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(11i32);
pub const TrustedDomainNameInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(1i32);
pub const TrustedDomainSupportedEncryptionTypes: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(13i32);
pub const TrustedPasswordInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(4i32);
pub const TrustedPosixOffsetInformation: TRUSTED_INFORMATION_CLASS = TRUSTED_INFORMATION_CLASS(3i32);
pub const UNDERSTANDS_LONG_NAMES: u32 = 1u32;
pub const UNISP_NAME: windows_core::PCWSTR = windows_core::w!("Microsoft Unified Security Protocol Provider");
pub const UNISP_NAME_A: windows_core::PCSTR = windows_core::s!("Microsoft Unified Security Protocol Provider");
pub const UNISP_NAME_W: windows_core::PCWSTR = windows_core::w!("Microsoft Unified Security Protocol Provider");
pub const UNISP_RPC_ID: u32 = 14u32;
pub const USER_ACCOUNT_AUTO_LOCKED: u32 = 1024u32;
pub const USER_ACCOUNT_DISABLED: u32 = 1u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct USER_ALL_INFORMATION {
    pub LastLogon: i64,
    pub LastLogoff: i64,
    pub PasswordLastSet: i64,
    pub AccountExpires: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub UserName: LSA_UNICODE_STRING,
    pub FullName: LSA_UNICODE_STRING,
    pub HomeDirectory: LSA_UNICODE_STRING,
    pub HomeDirectoryDrive: LSA_UNICODE_STRING,
    pub ScriptPath: LSA_UNICODE_STRING,
    pub ProfilePath: LSA_UNICODE_STRING,
    pub AdminComment: LSA_UNICODE_STRING,
    pub WorkStations: LSA_UNICODE_STRING,
    pub UserComment: LSA_UNICODE_STRING,
    pub Parameters: LSA_UNICODE_STRING,
    pub LmPassword: LSA_UNICODE_STRING,
    pub NtPassword: LSA_UNICODE_STRING,
    pub PrivateData: LSA_UNICODE_STRING,
    pub SecurityDescriptor: SR_SECURITY_DESCRIPTOR,
    pub UserId: u32,
    pub PrimaryGroupId: u32,
    pub UserAccountControl: u32,
    pub WhichFields: u32,
    pub LogonHours: LOGON_HOURS,
    pub BadPasswordCount: u16,
    pub LogonCount: u16,
    pub CountryCode: u16,
    pub CodePage: u16,
    pub LmPasswordPresent: bool,
    pub NtPasswordPresent: bool,
    pub PasswordExpired: bool,
    pub PrivateDataSensitive: bool,
}
pub const USER_ALL_PARAMETERS: u32 = 2097152u32;
pub const USER_DONT_EXPIRE_PASSWORD: u32 = 512u32;
pub const USER_DONT_REQUIRE_PREAUTH: u32 = 65536u32;
pub const USER_ENCRYPTED_TEXT_PASSWORD_ALLOWED: u32 = 2048u32;
pub const USER_HOME_DIRECTORY_REQUIRED: u32 = 2u32;
pub const USER_INTERDOMAIN_TRUST_ACCOUNT: u32 = 64u32;
pub const USER_MNS_LOGON_ACCOUNT: u32 = 32u32;
pub const USER_NORMAL_ACCOUNT: u32 = 16u32;
pub const USER_NOT_DELEGATED: u32 = 16384u32;
pub const USER_NO_AUTH_DATA_REQUIRED: u32 = 524288u32;
pub const USER_PARTIAL_SECRETS_ACCOUNT: u32 = 1048576u32;
pub const USER_PASSWORD_EXPIRED: u32 = 131072u32;
pub const USER_PASSWORD_NOT_REQUIRED: u32 = 4u32;
pub const USER_SERVER_TRUST_ACCOUNT: u32 = 256u32;
#[repr(C)]
#[cfg(feature = "Win32_System_PasswordManagement")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USER_SESSION_KEY {
    pub data: [super::super::super::System::PasswordManagement::CYPHER_BLOCK; 2],
}
#[cfg(feature = "Win32_System_PasswordManagement")]
impl Default for USER_SESSION_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USER_SMARTCARD_REQUIRED: u32 = 4096u32;
pub const USER_TEMP_DUPLICATE_ACCOUNT: u32 = 8u32;
pub const USER_TRUSTED_FOR_DELEGATION: u32 = 8192u32;
pub const USER_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: u32 = 262144u32;
pub const USER_USE_AES_KEYS: u32 = 2097152u32;
pub const USER_USE_DES_KEY_ONLY: u32 = 32768u32;
pub const USER_WORKSTATION_TRUST_ACCOUNT: u32 = 128u32;
#[cfg(feature = "Win32_Security_Credentials")]
pub type VERIFY_SIGNATURE_FN = Option<unsafe extern "system" fn(param0: *mut super::super::Credentials::SecHandle, param1: *mut SecBufferDesc, param2: u32, param3: *mut u32) -> windows_core::HRESULT>;
pub const WDIGEST_SP_NAME: windows_core::PCWSTR = windows_core::w!("WDigest");
pub const WDIGEST_SP_NAME_A: windows_core::PCSTR = windows_core::s!("WDigest");
pub const WDIGEST_SP_NAME_W: windows_core::PCWSTR = windows_core::w!("WDigest");
pub const WINDOWS_SLID: windows_core::GUID = windows_core::GUID::from_u128(0x55c92734_d682_4d71_983e_d6ec3f16059f);
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct X509Certificate {
    pub Version: u32,
    pub SerialNumber: [u32; 4],
    pub SignatureAlgorithm: super::super::Cryptography::ALG_ID,
    pub ValidFrom: super::super::super::Foundation::FILETIME,
    pub ValidUntil: super::super::super::Foundation::FILETIME,
    pub pszIssuer: windows_core::PSTR,
    pub pszSubject: windows_core::PSTR,
    pub pPublicKey: *mut PctPublicKey,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for X509Certificate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const _FACILITY_WINDOWS_STORE: u32 = 63u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct _HMAPPER(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct eTlsAlgorithmUsage(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct eTlsHashAlgorithm(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct eTlsSignatureAlgorithm(pub i32);
