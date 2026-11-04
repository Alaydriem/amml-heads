<div align="center">

<img src="./assets/icon.png" alt="Heads" width="256">

# Alaydriem's Heads

[![Release](https://img.shields.io/github/v/release/alaydriem/heads?include_prereleases)](https://github.com/alaydriem/heads/releases)
[![Build](https://img.shields.io/github/actions/workflow/status/alaydriem/heads/build.yml?branch=master&label=build)](https://github.com/alaydriem/heads/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/alaydriem/heads)](./LICENSE)
![Minecraft Bedrock](https://img.shields.io/badge/Minecraft_Bedrock-1.26.60-62B47A)
[![AMML](https://img.shields.io/badge/AMML-mod-4A90D9)](https://github.com/alaydriem/amml)

</div>

Player heads for Minecraft Bedrock. Each player shows as their skin face (face and hat layer) on the locator bar instead of a coloured dot, and next to their messages in chat.

Client-side only. The faces come from the skins your game already gets from the server. The mod sends nothing.

## Features

- Locator bar: each player's skin face instead of the dot, in any world and on any server
- Chat: the player's face next to their messages
- A skin change shows at once
- The game draws the face on the locator bar itself, so position, fade, sorting and the up and down arrows stay vanilla
- Locator bar waypoints that are not players keep the vanilla icon

## Installation

Requires:

- the **AMML loader**, which loads the mod into the game. The loader also carries the recipes that find the game
  addresses the mod uses (in its `recipes/`). An older loader does not have them and the mod stays off. Update the
  loader first.
- [Alaydriem's Mod Manager](https://github.com/alaydriem/modmanager) (ABI 2.x), for the settings. Without it the mod
  runs with its defaults.

Download the zip from the [latest release](https://github.com/alaydriem/heads/releases/latest) and extract it.
It holds the folder `@alaydriem/heads`, with `alaydriem_heads.dll`, `alaydriem_heads.pdb` and
`amml.hcl`. Put that folder in the AMML mods folder of the game, so that the files are in
`mods/amml/@alaydriem/heads/`. Then enable the mod in the mod manager.

| Heads | Minecraft Bedrock | Mod Manager |
| --- | --- | --- |
| 26.60.0 | 1.26.60 | ABI 2.x |

## Configuration

Settings are in the mod manager panel and saved to `mods/amml/@alaydriem/heads/config/config.hcl`.

| Setting | Default | Description |
| --- | --- | --- |
| `heads.enabled` | `true` | Show player faces instead of dots |
| `heads.scale` | `1.0` | Face size compared to the dot (1–2, steps of 0.25) |

## FAQ

**A player still shows a dot.** The face shows when the game has that player's skin. Players not in the player list,
and script or location waypoints, keep the vanilla icon.

**A face looks wrong.** Persona and custom geometry skins whose face is not at the classic place on the skin show
whatever is at that place.

## Building

Requires Windows x64, [mise](https://mise.jdx.dev), rustup and [cargo-nextest](https://nexte.st). No sibling checkout is needed: cargo fetches `amml` and `amml-modmanager` at the release tag named in `Cargo.toml`.

```powershell
mise trust
mise run preflight
mise run package
mise run test
```

`mise run package` writes `dist/@alaydriem/heads` and checks that it holds exactly the dll, the pdb and `amml.hcl`.

## License

BSD 3-Clause. Copyright (c) 2026 Charles R. Portwood II. See [LICENSE](./LICENSE).
