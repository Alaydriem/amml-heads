use amml_sdk::{HookHandle, SdkError};

use super::HeadsRuntime;
use crate::traits::GameHost;
use crate::types::HookSpec;

// installs the mod hooks: every target must resolve before any hook goes in
pub struct HookInstaller;

impl HookInstaller {
    // the code address each spec hooks; a slot target is read from its vtable
    pub fn resolve<H: GameHost>(host: &H, specs: &[HookSpec]) -> Result<Vec<u64>, SdkError> {
        specs.iter().map(|spec| Self::target(host, spec)).collect()
    }

    // installs every spec; on a failure the hooks this call put in come out again and earlier hooks stay
    pub fn install<H: GameHost>(
        host: &H,
        runtime: &HeadsRuntime,
        specs: &[HookSpec],
        targets: &[u64],
    ) -> Result<(), SdkError> {
        let before = runtime.hook_count();
        for (spec, target) in specs.iter().zip(targets) {
            let kept = match host.hook(*target, spec) {
                Ok(hook) => Self::keep(host, runtime, hook),
                Err(error) => Err(error),
            };
            if let Err(error) = kept {
                Self::remove_since(host, runtime, before);
                return Err(error);
            }
        }
        Ok(())
    }

    pub fn remove_all<H: GameHost>(host: &H, runtime: &HeadsRuntime) {
        Self::remove_since(host, runtime, 0);
    }

    fn remove_since<H: GameHost>(host: &H, runtime: &HeadsRuntime, count: usize) {
        for hook in runtime.take_hooks_since(count) {
            if let Err(error) = host.unhook(hook) {
                host.warn(&format!("unhook failed: {error}"));
            }
        }
    }

    fn target<H: GameHost>(host: &H, spec: &HookSpec) -> Result<u64, SdkError> {
        let value = host.resolve(spec.entry)?;
        let Some(table) = spec.table else { return Ok(value) };
        let base = host.resolve(table)?;
        let code = host.read_pointer(base + value * 8)?;
        if code == 0 {
            return Err(SdkError::Mod(format!("slot {value} of {table} is empty")));
        }
        Ok(code)
    }

    // a hook the runtime cannot keep is removed again, so no installed hook is left without a handle
    fn keep<H: GameHost>(host: &H, runtime: &HeadsRuntime, hook: HookHandle) -> Result<(), SdkError> {
        if runtime.keep_hook(hook) {
            return Ok(());
        }
        let _ = host.unhook(hook);
        Err(SdkError::Mod(format!("more than {} hooks", HeadsRuntime::HOOK_CAPACITY)))
    }
}
