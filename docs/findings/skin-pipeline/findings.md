# Skin pipeline findings (26.60)

Tags: [H] header, [W] bedrock-protocol-rs wire code, [I] inference. L60 = C:\Users\charl\projects\LeviLamina (26.60, commit a2bad50d01 "chore: update to 26.60"), L51 = C:\Users\charl\projects\LeviLamina-2651 (26.51.9). Nothing was verified in a running client.

## Short answer
- Best skin source for any remote player, also far away: Level::getPlayerList() -> PlayerListEntry.mSkin (a SerializedSkinRef). It needs no loaded actor. [H+I]
- Waypoint key: ServerWaypoint::Payload::mActorID (optional<ActorUniqueID>). Match it to PlayerListEntry.mId. The waypoint handle UUID is a waypoint id, not the player UUID. [H+I]
- Weak points: (a) SerializedSkinRef has no accessor in the header. (b) The client waypoint classes are empty stubs. (c) The mod must make its own texture.

## 1. Network
- PlayerListPacket : ll::PayloadPacket<PlayerListPacketPayload> (src/mc/network/packet/PlayerListPacket.h:19). Payload: vector<variant<RemoveEntry, AddEntry>> mEntries (PlayerListPacketPayload.h:95-99). [H]
- AddEntry (PlayerListPacketPayload.h:31-43): mAction, mUUID, mId (ActorUniqueID), mName, mXUID, mPlayFabId, mPlatformOnlineId, mBuildPlatform, mSkin (SerializedSkinRef, 16 B shared_ptr), mIsTeacher, mIsHost, mIsSubClient, mColor. RemoveEntry: mAction, mUUID (:85-86). AddEntry::toPlayerListEntry() is client-only (:62). [H]
- PlayerListEntry (src/mc/world/actor/player/PlayerListEntry.h:21-32): mId, mUUID, mName, mXUID, mPlayFabId, mPlatformOnlineId, mBuildPlatform, mSkin, mColor, mIsTeacher, mIsHost, mIsSubClient. [H]
- PlayerSkinPacketPayload.h:13-16: mUUID, mSkin, mLocalizedNewSkinName, mLocalizedOldSkinName. No ActorUniqueID and no name. Key is the UUID. [H]
- AddPlayerPacketPayload: mUuid, mName, mRuntimeId, mPlatformOnlineId, pos/vel/rot, item, game type, synched data, abilities, mLinks, mPassengerOfBlockData, mDeviceId, mBuildPlatform. No skin, no ActorUniqueID. [H]
- Client handlers: src-client/mc/client/network/LegacyClientNetworkHandler.h:153 (AddPlayerPacket), :211 (PlayerListPacket), :213 (PlayerSkinPacket). ClientNetworkHandler.h:503 (LocatorBarPacket; thunk :849). [H]
- src/mc/world/level/PlayerListManager.h: addPlayerEntry(uuid, PlayerListEntry&&) (client only), tryGetPlayerEntry(uuid), pubsub mOnPlayerListEntryAdded / mOnPlayerListEntryRemoved. Good cache hooks. [H]
- Wire [W] (bedrock-protocol-rs/bedrock-protocol/.claude/worktrees/version-26.60.0/src/protocol/types/generated/...): PlayerListAddEntry and PlayerSkinPacket carry the same fields. SkinImage on the wire = u32 LE width, u32 LE height, varuint length + raw bytes (skin_image.rs). The image is raw RGBA, not PNG.

## 2. SerializedSkin layout
- No class SerializedSkin in L60. SerializedSkinRef = shared_ptr<Bedrock::Application::ThreadOwner<SerializedSkinImpl,0>> mSkinImpl (src/mc/world/actor/player/SerializedSkinRef.h:34-35). ThreadOwner is an empty declaration (src/mc/util/ThreadOwner.h:7-8). The offset of SerializedSkinImpl inside it is unknown; find it by a runtime test (mId must read as a valid string) or by disassembly of a SerializedSkinRef accessor. [H+I]
- SerializedSkinImpl.h:28-50 in order: mId, mFullId, mResourcePatch, mDefaultGeometryName (strings 8/32); mSkinImage, mCapeImage (SkinImage 8/48 each); mSkinAnimatedImages (vector 8/24); mGeometryData (Json::Value 8/16); mGeometryDataMinEngineVersion (8/32); mGeometryDataMutable (8/16); mAnimationData, mCapeId (strings); mPersonaPieces (vector); mArmSizeType (1/1); mPieceTintColors (unordered_map 8/64); mSkinColor (mce::Color 4/16); mIsTrustedSkin (1/1); bools mIsPremium, mIsPersona, mIsPersonaCapeOnClassicSkin, mIsPrimaryUser, mOverridesPlayerAppearance; mProfileHash (string). Offsets are not printed; compute them from (align,size) with normal padding. [H+I]
- SkinImage : mce::Image, no extra members (SkinImage.h:8). mce::Image (src/mc/deps/core/image/Image.h:21-27): imageFormat (mce::ImageFormat, 4), mWidth, mHeight, mDepth (uint), mUsage (1), mAlphaUsage (1), mImageBytes (mce::Blob 8/24). mce::Blob (src/mc/deps/core/container/Blob.h): unique_ptr<uchar[],Deleter> mBlob (8/16, Deleter holds a function pointer), then uint64 mSize. Pixel pointer position inside the unique_ptr is to be confirmed. [H; pointer position I]
- ImageFormat.h: Unknown 0, R8Unorm 1, RG8Unorm 2, RGB8Unorm 3, RGBA8Unorm 4, RGBA16Float 5. Expect RGBA8Unorm: 4 bytes per pixel, stride = width*4. Check imageFormat and mSize == w*h*4 first. mce::Image::clone() is client-available (Image.h:34). SerializedSkinRef::isValid() is at SerializedSkinRef.h:76. [H, expectation I]
- Persona flag: mIsPersona. Geometry: mResourcePatch (JSON), mDefaultGeometryName, mGeometryData. Animated: AnimatedImageData (AnimatedImageData.h:19-22: mType, mAnimationExpression, mImage, mFrames). Cape: mCapeImage, mCapeId. [H]
- Face extraction [I]: face 8x8 at (8,8) and hat 8x8 at (40,8) on a 64-wide reference. Scale s = width/64 (64x32 and 64x64: s=1, 128x128: s=2, 256x256: s=4). Blend hat over face. Persona skins are baked into a 64x64 or 128x128 image; the head is usually in the same place, but custom geometry (mResourcePatch/mGeometryData) can move the UV. For non-humanoid geometry, read the head cube uv from mGeometryData or keep the vanilla dot. Needs tests with real persona and custom-geometry skins.

## 3. Where the client keeps skins
- Player list: ILevel::getPlayerList() (src/mc/world/level/ILevel.h:1035, Level.h:1310), unordered_map<mce::UUID,PlayerListEntry> const&; also PlayerListManager::mPlayerList. Filled from the packet; independent of actors. [H] MOST RELIABLE for far players. [I]
- Actor: Player::mSkin = unique_ptr<SerializedSkinRef> (src/mc/world/actor/player/Player.h:299), Player::updateSkin(skin, int) (:978). Only when the actor exists; no getSkin in the header. [H]
- Render: ClientSkin (src-client/mc/client/components/ClientSkin.h:23) holds ClientSkinInfoData (8/328). ClientSkinInfoData : SkinInfoData (mSkin, mDefaultMeshName). It has BedrockTexture mSkinTexture, mSkinAnimatedTextures, mCapeTexture, mLodSkinImage, mLodSkinTexture, mLodSkinProfileHash, mValid. GPU textures for rendered entities only. [H]
- SkinRepository (src/mc/client/persona/SkinRepository.h): local user's packs. Not useful. PlayerSkinLoadedClientEvent { WeakEntityRef mPlayer }. [H]
- Keep fresh: subscribe to mOnPlayerListEntryAdded/Removed or hook handle(PlayerSkinPacket) (key = UUID).

## 4. UI path for a player face
- No ready face renderer exists. Nothing named PersonaHead, PlayerFace or ChatHead in L60 or L51. [H]
- Renderers in src-client/mc/client/gui/controls/renderers/ that touch a player or skin: PaperDollRenderer (3D doll, local selected skin), LivePlayerRenderer (3D model), ActorPortraitRenderer (3D portrait by ActorUniqueID), ProfileImageRenderer (gamerpic: shared_ptr<mce::Image> -> mce::ClientTexture; NOT the skin; the closest precedent for image -> texture -> draw), SkinImages.h:14-15 (two mce::Image const*). None is a face quad.
- Own texture recipe [H signatures, I recipe]: (1) cg::ImageBuffer(mce::Image const&/&&) (src/mc/deps/core_graphics/ImageBuffer.h). (2) mce::TextureGroup::uploadTexture(ResourceLocation const&, cg::ImageBuffer) -> BedrockTexture& (src-client/mc/client/renderer/TextureGroup.h); also updateTextureInPlace, addEmptyTexture, unloadTexture. (3) ResourceLocation(Core::PathView, ResourceFileSystem) with a virtual path like "skin_face/<uuid>" (src/mc/deps/core/resource/ResourceLocation.h:62-68); the safe ResourceFileSystem value is not in the header. (4) mce::TexturePtr(shared_ptr<TextureGroupBase>, ResourceLocation) or MinecraftUIRenderContext::getTexture; the context has mTextureGroup (MinecraftUIRenderContext.h:99). (5) MinecraftUIRenderContext::drawImage(mce::ClientTexture const&, vec2 pos, vec2 size, vec2 uv, vec2 uvSize, bool) (:163-170) then flushImages. No TexturePtr -> ClientTexture conversion in the header; reverse a caller of drawImage.
- Remote-skin code in the game: ClientSkinSystem::createOrUpdate(..., SerializedSkinRef const&, mce::Image const*, mce::Image const*, bool, std::function) and ClientSkinInfoData::updateSkin(SerializedSkinRef const&, mce::Image const* skin, mce::Image const* cape). The game gets mce::Image const* from the SerializedSkinRef; the accessor is not in the header.

## 5. Waypoint key
- LocatorBarPacketPayload.h:12: vector<LocatorBarWaypointPayload> mWaypoints. LocatorBarWaypointPayload.h:14-16: ServerWaypoint::Payload mPayload, WaypointGroup::WaypointHandle mHandle, ServerWaypointGroup::Action mAction (None 0, Add 1, Remove 2, Update 3). [H]
- WaypointHandle = only mce::UUID mUUID (WaypointGroup.h:29): the waypoint's own id. [H; not-player-UUID is I]
- ServerWaypoint::Payload (ServerWaypoint.h:38-49): mUpdateFlag, mIsVisible, mWorldPosition, mTexturePath, mIconSize, mColor, mClientPositionAuthority, optional<ActorUniqueID> mActorID. Update flags: Position 1, Visibility 2, TexturePath 4, IconSize 8, Color 16, ClientPositionAuthority 32 (:27-35). mActorID has no flag bit, so it is probably sent on Add. [H+I]
- Server: ServerEntityWaypoint has ActorUniqueID const mActorID + WeakEntityRef; ServerPlayerWaypoint : ServerEntityWaypoint; Waypoint::tryGetActorID() is virtual. VanillaWaypointManager::update(Player&, ServerLocatorBar&, bool) makes them; textures come from PLAYER_DOT_0..3_TEXTURE_PATH. [H]
- Client: ClientWaypoint, ClientEntityWaypoint, ClientWaypointGroup, ClientLocatorBar are EMPTY stubs. LocalPlayer::mClientLocatorBar is 8/72 (src-client/mc/client/player/LocalPlayer.h:178). Option: hook ClientNetworkHandler::handle(LocatorBarPacket) and build a map handle UUID -> mActorID on Add/Remove/Update. [H+I]
- Wire [W]: the waypoint Payload in bedrock-protocol-rs has an extra texture_id: Option<u32> (codec v975+) that the L60 header lacks. Check with a real capture. [I]
- Mapping: mActorID -> entry.mId in getPlayerList() -> entry.mSkin. Fallbacks: ILevel::getPlayer(ActorUniqueID) (ILevel.h:382) and getPlayer(mce::UUID) (:380) -> Player::mSkin. If mActorID is empty or not in the list, keep the vanilla dot. [H+I]

## 6. Diff 26.51 -> 26.60
Identical: PlayerListPacket, PlayerListPacketType, PlayerSkinPacket, PlayerSkinPacketPayload, PlayerListManager, SkinRepository, SkinInfoData, BedrockTexture, SkinImages, ImageFormat, ImageBuffer, LocatorBarPacket, LocatorBarPacketPayload, LocatorBarWaypointPayload, Waypoint, WaypointGroup, ServerEntityWaypoint, ServerPlayerWaypoint, ServerWaypointGroup, VanillaWaypointManager, ServerLocatorBar, ClientWaypoint, ClientLocatorBar, PlayerSkinLoadedClientEvent, VanillaWaypointManagerConstants. [H]
Layout changes [H]:
1. SerializedSkinImpl: 26.51 had std::string mPlayFabId right after mId; 26.60 removed it. Everything after mId shifts by 32 B. Do not reuse 26.51 offsets.
2. PlayerListEntry (:25): ADDED mPlayFabId (8/32) after mXUID. Later fields shift by 32.
3. PlayerListPacketPayload::AddEntry (:36): ADDED mPlayFabId after mXUID.
4. AddPlayerPacketPayload: ADDED optional<PassengerOfBlockArguments> mPassengerOfBlockData (4/40) between mLinks and mDeviceId.
5. ClientSkinInfoData: ADDED string mLodSkinProfileHash; ClientSkin::mSkinInfoData 296 -> 328.
6. HudLocatorBarRenderer::mMeshes 3792 -> 3648 (mce::Mesh 632 -> 608 B); later members move.
7. ProfileImageRenderer: mProfileImageOptions 72 -> 104; mTexture (mce::ClientTexture) 24 -> 16. Do not hardcode ClientTexture size.
8. Non-layout: TexturePtr default ctor no longer exported. TextureGroup::shouldLoadPBRResources and ResourceLocation::getFileSystemName removed. ClientTexture lost its $vftable.

## Risks
1. SerializedSkinImpl offset inside ThreadOwner is unknown; confirm by disassembly or runtime test; never reuse 26.51 offsets.
2. Capture waypoint data in the LocatorBarPacket handler, or read it from the renderer's own state (client classes are stubs).
3. Face coordinates may be wrong for persona skins with custom geometry.
4. TexturePtr -> ClientTexture for drawImage is not in headers.
5. Copy the 8x8 face (plus hat) once into a mod-owned buffer; upload on the render thread. [I]

Task Completed
