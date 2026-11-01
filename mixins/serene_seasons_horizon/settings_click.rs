fn seasons_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::serene_seasons_horizon as m;
    if mod_id != m::ID || button != 0 { return; }
    let days = game.native_setting(m::ID, "days_per_season").and_then(|v| v.parse().ok()).unwrap_or(24);
    let next = match days { 16 => 24, 24 => 32, _ => 16 };
    game.native_set_setting(m::ID, "days_per_season", &next.to_string());
}
