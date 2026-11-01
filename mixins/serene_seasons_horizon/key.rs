fn calendar_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::serene_seasons_horizon as m;
    if !game.native_on(m::ID) || !pressed || key != "key.mouse.right" { return; }

    let is_clock = game.inventory
        .get(game.selected)
        .and_then(Option::as_ref)
        .is_some_and(|stack| stack.item.as_ref() == "minecraft:clock");
    if !is_clock { return; }

    let days = game.native_setting(m::ID, "days_per_season").and_then(|v| v.parse().ok()).unwrap_or(24);
    let offset = game.native_setting(m::ID, "offset_days").and_then(|v| v.parse().ok()).unwrap_or(0);
    let calendar = m::Calendar::at(game.world.day_time, days, offset);
    game.native_chat(&calendar.summary());
}
