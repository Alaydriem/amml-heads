namespace    = "alaydriem"
name         = "heads"
display_name = "Heads"
version      = "26.62.0"
author       = "Alaydriem"
config_path  = "config.hcl"

claim "Hud" {
  mode   = "Shared"
  reason = "Replaces the dots on the vanilla locator bar with player faces"
}

section "heads" {
  title = "Heads"

  field "heads.enabled" {
    label   = "Show player faces"
    help    = "Draws each player's face on the locator bar instead of the dot"
    kind    = "bool"
    default = true
    apply   = "Live"
  }

  field "heads.scale" {
    label   = "Face size"
    help    = "How much larger than the dot the face is drawn"
    kind    = { float = { min = 1.0, max = 2.0, step = 0.25 } }
    default = 1.0
    apply   = "Live"
  }

  field "heads.chat" {
    label   = "Show faces in chat"
    help    = "Draws each player's face before their name in chat"
    kind    = "bool"
    default = true
    apply   = "Live"
  }
}
