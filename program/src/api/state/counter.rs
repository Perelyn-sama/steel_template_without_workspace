use steel::*;

use super::SteelTemplateWithoutWorkspaceAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(SteelTemplateWithoutWorkspaceAccount, Counter);
