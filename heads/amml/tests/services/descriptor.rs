use alaydriem_heads::services::HeadsDescriptor;
use amm_core::services::DescriptorChecker;
use amm_core::services::settings::SchemaChecker;
use amm_core::structs::descriptor::{Capability, ClaimMode, Descriptor};
use amm_core::structs::settings::HclDocument;
use heads_core::config::{HeadsDocument, HeadsSettings};

fn descriptor() -> Descriptor {
    DescriptorChecker::new(128, 256).parse(HeadsDescriptor::TEXT).unwrap()
}

#[test]
fn descriptor_parses_and_lists_its_fields() {
    let descriptor = descriptor();

    assert_eq!(descriptor.namespace, "alaydriem");
    assert_eq!(descriptor.name, "heads");
    assert_eq!(descriptor.id(), "@alaydriem/heads");
    let claims: Vec<_> = descriptor.claims.iter().map(|claim| (claim.capability, claim.mode)).collect();
    assert_eq!(claims, [(Capability::Hud, ClaimMode::Shared)]);
    let fields: Vec<_> =
        descriptor.settings.iter().flat_map(|section| section.fields.iter().map(|field| field.path.as_str())).collect();
    assert_eq!(fields, ["heads.enabled", "heads.scale", "heads.chat"]);
}

#[test]
fn schema_defaults_match_core_defaults() {
    let checked = SchemaChecker::check(&descriptor().settings, HclDocument::empty());

    let document = hcl::from_str::<HeadsDocument>(&checked.document.text()).unwrap();

    assert_eq!(document.heads, HeadsSettings::default());
}
