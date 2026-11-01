fn seasons_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::serene_seasons_horizon as m;
    if !game.native_on(m::ID) { return; }
    if command == "season" || command == "season calendar" {
        let days = game.native_setting(m::ID, "days_per_season").and_then(|v| v.parse().ok()).unwrap_or(24);
        let offset = game.native_setting(m::ID, "offset_days").and_then(|v| v.parse().ok()).unwrap_or(0);
        let calendar = m::Calendar::at(game.world.day_time, days, offset);
        game.native_chat(&calendar.summary());
        *ran = true;
    } else if let Some(name) = command.strip_prefix("season set ") {
        let index = match name.trim().to_ascii_lowercase().as_str() {
            "spring" => Some(0), "summer" => Some(1), "autumn" | "fall" => Some(2), "winter" => Some(3), _ => None,
        };
        if let Some(index) = index {
            let days = game.native_setting(m::ID, "days_per_season").and_then(|v| v.parse().ok()).unwrap_or(24);
            let day = game.world.day_time.div_euclid(24_000);
            let year_days = i64::from(days) * 4;
            let year_start = day.div_euclid(year_days) * year_days;
            let target_start = year_start + i64::from(index * days);
            let offset = target_start - day;
            game.native_set_setting(m::ID, "offset_days", &offset.to_string());
            game.native_chat("Serene Seasons: calendar offset updated.");
            *ran = true;
        }
    }
}
