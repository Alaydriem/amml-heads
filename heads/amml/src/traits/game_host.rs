use amml_sdk::{HookHandle, SdkError};

use crate::types::HookSpec;

// what installing the mod needs from the loader: entry values, vtable reads, hooks and one log line per refusal
pub trait GameHost {
    fn resolve(&self, id: &str) -> Result<u64, SdkError>;

    // the code pointer or word stored at `address`, an entry of a vtable in the game image
    fn read_pointer(&self, address: u64) -> Result<u64, SdkError>;

    // installs `spec.detour` on `target`, the resolved target of `spec`
    fn hook(&self, target: u64, spec: &HookSpec) -> Result<HookHandle, SdkError>;

    fn unhook(&self, hook: HookHandle) -> Result<(), SdkError>;

    fn warn(&self, message: &str);

    fn info(&self, message: &str);
}
