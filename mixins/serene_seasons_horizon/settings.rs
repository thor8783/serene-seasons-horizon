fn seasons_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::serene_seasons_horizon as m;
    if mod_id != m::ID { return; }
    let days = game.native_setting(m::ID, "days_per_season").and_then(|v| v.parse().ok()).unwrap_or(24);
    buttons.push(m::season_length_label(days));
}
