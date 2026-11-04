use amml_sdk::amml_abi::AmmlLogLevel;
use amml_sdk::{Api, HookHandle, SdkError};

use crate::traits::GameHost;
use crate::types::HookSpec;

// the AMML loader as the host the mod installs into
pub struct AmmlHost<'a> {
    api: &'a Api,
    module: &'static str,
}

impl<'a> AmmlHost<'a> {
    pub fn new(api: &'a Api, module: &'static str) -> Self {
        Self { api, module }
    }
}

impl GameHost for AmmlHost<'_> {
    fn resolve(&self, id: &str) -> Result<u64, SdkError> {
        self.api.resolve(id)
    }

    #[allow(unsafe_code)]
    fn read_pointer(&self, address: u64) -> Result<u64, SdkError> {
        if address == 0 {
            return Err(SdkError::Mod("null vtable address".to_string()));
        }
        // SAFETY: the caller passes a vtable entry in the game image plus a slot of that class, so the word is
        // readable
        Ok(unsafe { (address as *const u64).read_unaligned() })
    }

    fn hook(&self, target: u64, spec: &HookSpec) -> Result<HookHandle, SdkError> {
        // SAFETY: `target` is the derived start of the game function `spec.entry` names (or the body its vtable
        // slot holds), and `spec.detour` is the detour written for that function's signature; HeadsMod::hooks
        // pairs each entry with its detour. Each original slot belongs to one hook only.
        #[allow(unsafe_code)]
        unsafe {
            self.api.hook(target, spec.detour, 0, spec.original)
        }
    }

    fn unhook(&self, hook: HookHandle) -> Result<(), SdkError> {
        self.api.unhook(hook)
    }

    fn warn(&self, message: &str) {
        self.api.log(self.module, AmmlLogLevel::WARN, message);
    }

    fn info(&self, message: &str) {
        self.api.log(self.module, AmmlLogLevel::INFO, message);
    }

    fn error(&self, message: &str) {
        self.api.log(self.module, AmmlLogLevel::ERROR, message);
    }
}
