use heads_core::structs::HeadUv;

/// One skin the packet observer read, with how the extraction went.
pub struct SkinEvent<'a> {
    /// The packet kind: "PlayerList.Add" or "PlayerSkin".
    pub kind: &'static str,
    pub uuid: [u8; 16],
    pub actor: Option<i64>,
    /// The address of the `SerializedSkinRef` the skin was read through.
    pub skin_ref: usize,
    /// "animated_face" when the face came from the persona animated face image, "geometry" when the head came from
    /// the skin geometry, "classic" otherwise.
    pub head_source: &'static str,
    /// The head rectangles the face was cut from.
    pub head: HeadUv,
    /// The raw geometry JSON, when it was read.
    pub geometry_json: Option<&'a str>,
    /// "Ok", the `FaceError` variant, or "SkinNotReadable" when no view could be made.
    pub outcome: String,
}
