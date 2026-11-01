// the descriptor the manager registers the mod from; it names the claim and the settings schema
pub struct HeadsDescriptor;

impl HeadsDescriptor {
    pub const TEXT: &'static str = include_str!("../../descriptor.hcl");
}
