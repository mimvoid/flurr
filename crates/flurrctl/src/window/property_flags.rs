use heck::ToShoutySnakeCase;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PropertyFlags: u16 {
        const VISIBLE = 0x01;
        const NAMESPACE = 0x02;
        const LAYER = 0x04;
        const KEYBOARD_MODE = 0x08;
        const ANCHOR = 0x10;
        const EXCLUSION = 0x20;
        const AUTO_EXCLUSION = 0x40;
        const MARGIN_TOP = 0x80;
        const MARGIN_RIGHT = 0x100;
        const MARGIN_BOTTOM = 0x200;
        const MARGIN_LEFT = 0x400;
        const UNLOCKED = 0x800;
    }
}

impl PropertyFlags {
    pub fn parse_names(prop_names: &[String]) -> Self {
        let mut flags = Self::empty();

        for name in prop_names {
            match Self::from_name(name.to_shouty_snake_case().as_str()) {
                Some(flag) => flags.insert(flag),
                _ => log::warn!("Skipping unknown property: {}", name),
            }
        }

        flags
    }
}

impl std::fmt::Display for PropertyFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}
