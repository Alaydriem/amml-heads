namespace   = "alaydriem"
name        = "heads"
entry       = "alaydriem_heads.dll"
api_version = 2
entries = [
  "ClientInstance.update",
  "ClientInstance.getLocalPlayer",
  "ClientInstance.getTextureGroup",
  "ClientExitLevel.hook",
  "LegacyClientNetworkHandler.vtable",
  "Mesh.size",
  "core.operator_new",
  "core.operator_delete",
  "HudLocatorBarRenderer.vtable",
  "HudLocatorBarRenderer.render",
  "HudLocatorBarRenderer.drawIcon",
  "LocalPlayer.mClientLocatorBar",
  "ClientEntityWaypoint.vtable",
  "ClientNetworkHandler.handle.PlayerListPacket",
  "ClientNetworkHandler.handle.PlayerSkinPacket",
  "TextureGroup.getTexture",
  "TextureGroup.uploadTexture",
  "TextureGroup.unloadTexture",
  "ResourceLocation.ctor",
  "ImageBuffer.ctorFromDescription",
  "PlayerListAddEntry.size",
  "PlayerListPacket.variantIndex",
  "PlayerListAddEntry.mSkin",
  "SerializedSkinImpl.mSkinImage",
  "JsonValue.toStyledString",
  "SerializedSkinImpl.mGeometryData",
  "GuiMessage.ctor",
  "BitmapFont.vtable",
  "BitmapFont._getCharWidth",
  "PlayerListAddEntry.mName",
  "PlayerListAddEntry.mXUID",
  "ClientInstance.getFontHandle",
  "FontHandle.dtor",
  "FontHandle.size",
  "FontHandle.mDefaultFont",
  "PlayerListPacket.mEntries",
  "PlayerListAddEntry.mUUID",
  "PlayerListAddEntry.mId",
  "PlayerSkinPacket.mUUID",
  "PlayerSkinPacket.mSkin",
  "SerializedSkinImpl.mResourcePatch",
  "SerializedSkinImpl.mDefaultGeometryName",
  "SerializedSkinImpl.mSkinAnimatedImages",
  "SerializedSkinImpl.mIsPremium",
  "SerializedSkinImpl.mIsPersona",
  "AnimatedImageData.size",
  "AnimatedImageData.mType",
  "AnimatedImageData.mImage",
  "ClientLocatorBar.listHead",
  "ClientLocatorBar.nodeWaypoint",
  "ClientWaypoint.mColor",
  "ClientWaypoint.mColorEngaged",
  "ClientWaypoint.mVisible",
  "ClientEntityWaypoint.mActorId",
  "TextureGroup.textureGroupBase",
  "ImageBuffer.size",
  "ImageBuffer.mDeleter",
  "ImageBuffer.mData",
  "ImageBuffer.mSize",
  "ResourceLocation.size",
]

listing {
  title       = "Heads"
  summary     = "Player skin faces on the locator bar and in chat."
  description = <<-EOT
    View player heads in chat and in the locator bar. Works with both skin files and creator skins.
  EOT
  authors     = ["Charles R. Portwood II"]
  license     = "BSD-3-Clause"
  source      = "https://github.com/alaydriem/amml-heads"
  homepage    = "https://github.com/alaydriem/amml-heads#readme"
  tags        = ["skins", "locator-bar", "chat", "client", "visual"]
  icon        = "icon.png"
}
