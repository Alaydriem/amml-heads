use alaydriem_heads::HeadsMod;
use amm_client::test_support::FakeTable;
use amm_core::structs::event::ModEvent;
use amml_api::amml_abi::{
    ChatMessageAdded, ClientTick, LocatorBarIconDraw, PlayerListUpdate, PlayerSkinChanged, TexturesReloaded, WorldLeft,
};
use amml_api::testing::{FakeChat, FakeGame, FakeIconEdit, FakePlayerEntry, FakePlayerList, FakeSkin};
use amml_api::{ActorUniqueId, Area, ChatKind, PlayerListAction, Uuid};

// the heads mod enabled on a fake game that serves every event and area of its `USES`
pub struct HeadsRig {
    pub game: FakeGame,
}

impl HeadsRig {
    pub const STEVE: Uuid = Uuid([0x11; 16]);
    pub const ALEX: Uuid = Uuid([0x22; 16]);

    // the mod manager is provided, with the default settings
    pub fn new() -> Self {
        Self::with_config(Some(""))
    }

    // the mod manager is provided and reads `config` as the config file; `None` is no manager
    pub fn with_config(config: Option<&str>) -> Self {
        let game = Self::served_game();
        if let Some(config) = config {
            FakeTable::provide(&game.ctx()).unwrap();
            FakeTable::set_config(config);
        }
        game.load::<HeadsMod>().unwrap();
        game.enable::<HeadsMod>().unwrap();
        Self { game }
    }

    // a fake game that serves all of the mod's rows, but has not loaded the mod
    pub fn served_game() -> FakeGame {
        let game = FakeTable::game();
        game.offer::<PlayerListUpdate>();
        game.offer::<PlayerSkinChanged>();
        game.offer::<LocatorBarIconDraw>();
        game.offer::<ChatMessageAdded>();
        game.offer::<WorldLeft>();
        game.offer::<TexturesReloaded>();
        game.offer::<ClientTick>();
        for area in [
            Area::UI_TEXTURES,
            Area::NETWORK_SKIN,
            Area::NETWORK_SKIN_ANIMATED,
            Area::NETWORK_SKIN_GEOMETRY,
            Area::NETWORK_SKIN_DETAILS,
            Area::UI_INLINE_ICONS,
        ] {
            game.offer_area(area);
        }
        game.set_bitmap_font(true);
        game
    }

    pub fn entry(uuid: Uuid, actor_id: i64, name: &str, skin: Option<FakeSkin>) -> FakePlayerEntry {
        FakePlayerEntry { uuid, actor_id, name: name.to_string(), xuid: format!("xuid-{name}"), skin }
    }

    pub fn tick(&self) {
        self.game.raise_with_client(&mut ClientTick { header: FakeGame::header::<ClientTick>() });
    }

    pub fn add(&self, entries: Vec<FakePlayerEntry>) {
        self.game.raise_player_list(&FakePlayerList { action: PlayerListAction::Add, entries });
    }

    pub fn remove(&self, uuid: Uuid) {
        let entry = Self::entry(uuid, 0, "", None);
        self.game.raise_player_list(&FakePlayerList { action: PlayerListAction::Remove, entries: vec![entry] });
    }

    // what the game draws for the locator bar icon of `actor` once its texture is ready
    pub fn icon(&self, actor: i64) -> FakeIconEdit {
        for (texture, _, _) in self.game.textures().uploads() {
            self.game.textures().set_ready(texture, true);
        }
        self.game.raise_locator_icon(Some(ActorUniqueId(actor)))
    }

    // the text and the filtered text of a line `name` said
    pub fn say(&self, name: &str, text: &str) -> (Option<String>, Option<String>) {
        self.game.raise_chat(&FakeChat {
            kind: ChatKind(1),
            username: name.to_string(),
            xuid: format!("xuid-{name}"),
            text: format!("<{name}> {text}"),
            filtered_text: Some(format!("<{name}> {}", "*".repeat(text.len()))),
        })
    }

    pub fn settings_changed(&self, config: &str) {
        FakeTable::set_config(config);
        FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });
        self.tick();
    }

    // true when the log holds `line` (level, message and fields as the fake writes them)
    pub fn logged(&self, line: &str) -> bool {
        self.game.logs().iter().any(|logged| logged == line)
    }

    // the number of log lines that start with `prefix`
    pub fn logged_count(&self, prefix: &str) -> usize {
        self.game.logs().iter().filter(|logged| logged.starts_with(prefix)).count()
    }
}
