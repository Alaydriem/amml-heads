/// Accepts only the build whose layouts the mod mirrors.
pub struct BuildGate;

impl BuildGate {
    // sizeof(mce::Mesh) in 26.60; 26.51 has 0x278
    const MESH_SIZE_2660: u64 = 0x260;

    pub fn accepts(mesh_size: u64) -> bool {
        mesh_size == Self::MESH_SIZE_2660
    }
}
