use std::mem;
use windows::Win32::{Foundation::*, Security::*, System::Threading::*};

pub fn enable_debug_priv() -> windows::core::Result<()> {
    unsafe {
        let mut token = HANDLE::default();
        let mut luid = LUID::default();
        
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )?;

        LookupPrivilegeValueW(None, SE_DEBUG_NAME, &mut luid)?;

        let tkp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        AdjustTokenPrivileges(
            token,
            false,
            Some(&tkp),
            mem::size_of::<TOKEN_PRIVILEGES>() as _,
            None,
            None,
        )?;

        Ok(())
    }
}
