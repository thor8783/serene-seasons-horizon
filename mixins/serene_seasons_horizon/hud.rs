fn seasons_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::serene_seasons_horizon as m;
    if !game.native_on(m::ID) || !game.native_playing() { return; }
    use horizon_client::extras::Corner;
    let corner = Corner::TopLeft;
    if !game.native_corner_shown(corner) { return; }
    let days = game.native_setting(m::ID, "days_per_season").and_then(|v| v.parse().ok()).unwrap_or(24);
    let offset = game.native_setting(m::ID, "offset_days").and_then(|v| v.parse().ok()).unwrap_or(0);
    let calendar = m::Calendar::at(game.world.day_time, days, offset);
    let text = calendar.summary();
    let width = game.native_text_width(&text) + 8.0;
    let (x, y) = corners.place(corner, width, 14.0);
    game.native_fill(batch, [x, y, width, 14.0], 0x7800_0000);
    game.native_text(batch, &text, [x + 4.0, y + 3.0], 0xFFFFFF, true);
}
