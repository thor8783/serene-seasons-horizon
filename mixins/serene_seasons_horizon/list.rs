fn seasons_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::serene_seasons_horizon as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
