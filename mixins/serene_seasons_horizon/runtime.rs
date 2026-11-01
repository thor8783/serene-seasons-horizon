pub const ID: &str = "serene_seasons_horizon";
pub const NAME: &str = "Serene Seasons: Horizon";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "A configurable four-season calendar with sub-seasons.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Season { Spring, Summer, Autumn, Winter }

impl Season {
    pub fn label(self) -> &'static str {
        match self { Self::Spring => "Spring", Self::Summer => "Summer", Self::Autumn => "Autumn", Self::Winter => "Winter" }
    }
    pub fn from_index(index: i64) -> Self {
        match index.rem_euclid(4) { 0 => Self::Spring, 1 => Self::Summer, 2 => Self::Autumn, _ => Self::Winter }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calendar {
    pub year: i64,
    pub season: Season,
    pub sub_season: u8,
    pub day_in_season: u32,
    pub days_per_season: u32,
}

impl Calendar {
    pub fn at(day_time: i64, days_per_season: u32, offset_days: i64) -> Self {
        let days_per_season = days_per_season.clamp(4, 96);
        let year_days = i64::from(days_per_season) * 4;
        let world_day = day_time.div_euclid(24_000).saturating_add(offset_days);
        let year = world_day.div_euclid(year_days);
        let day_of_year = world_day.rem_euclid(year_days);
        let season_index = day_of_year.div_euclid(i64::from(days_per_season));
        let day_in_season = day_of_year.rem_euclid(i64::from(days_per_season)) as u32;
        let sub_season = ((day_in_season * 3) / days_per_season).min(2) as u8 + 1;
        Self { year, season: Season::from_index(season_index), sub_season, day_in_season: day_in_season + 1, days_per_season }
    }
    pub fn summary(self) -> String {
        format!("Year {}, {} {}, day {}/{}", self.year + 1, self.season.label(), self.sub_season, self.day_in_season, self.days_per_season)
    }
}

/// Temperature offset for systems that consume the season calendar.
pub fn temperature_offset(season: Season) -> f32 {
    match season { Season::Spring => 0.0, Season::Summer => 8.0, Season::Autumn => -2.0, Season::Winter => -12.0 }
}

/// Relative crop growth for a crop tagged to one season, or a year-round crop.
pub fn crop_fertility(season: Season, crop_season: Option<Season>, year_round: bool) -> f32 {
    if year_round { return 1.0; }
    match crop_season {
        Some(crop) if crop == season => 1.0,
        Some(_) => 0.0,
        None => 0.5,
    }
}

pub fn season_length_label(days: u32) -> String { format!("Season length: {days} days") }
