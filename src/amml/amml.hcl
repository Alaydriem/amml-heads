namespace    = "alaydriem"
name         = "heads"
entry        = "alaydriem_heads.dll"
api_version  = 3

uses {
  feature "faces" {
    event "network.player_list" {
      version  = 1
      required = true
    }
    event "network.player_skin" {
      version  = 1
      required = true
    }
    event "ui.locator_bar_icon" {
      version  = 1
      required = true
    }
    event "world.left" {
      version  = 1
      required = true
    }
    event "ui.textures_reloaded" {
      version  = 1
      required = true
    }
    area "ui.textures" {
      version  = 1
      required = true
    }
    area "network.skin" {
      version  = 1
      required = true
    }
  }
  feature "settings" {
    event "client.tick" {
      version  = 1
      required = true
    }
    event "registry.provider_ready" {
      version  = 1
      required = true
    }
    event "registry.provider_gone" {
      version  = 1
      required = true
    }
    event "mods.loaded" {
      version  = 1
      required = true
    }
    event "input.lease_suspended" {
      version  = 1
      required = false
    }
    event "input.lease_resumed" {
      version  = 1
      required = false
    }
    event "input.lease_lost" {
      version  = 1
      required = false
    }
    event "input.injected_released" {
      version  = 2
      required = false
    }
    event "overlay.screenshot_taken" {
      version  = 1
      required = false
    }
    event "overlay.screenshot_written" {
      version  = 1
      required = false
    }
    area "input.lease" {
      version  = 1
      required = false
    }
  }
  feature "chat" {
    event "ui.chat_message" {
      version  = 1
      required = true
    }
    area "ui.inline_icons" {
      version  = 1
      required = true
    }
  }
  feature "persona" {
    area "network.skin_animated" {
      version  = 1
      required = true
    }
  }
  feature "geometry" {
    area "network.skin_geometry" {
      version  = 1
      required = true
    }
  }
  feature "diagnostics" {
    area "network.skin_details" {
      version  = 1
      required = true
    }
  }
}

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
