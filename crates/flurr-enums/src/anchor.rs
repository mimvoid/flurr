bitflags::bitflags! {
    pub struct Anchor: u8 {
        const TOP = 0x01;
        const RIGHT = 0x02;
        const BOTTOM = 0x04;
        const LEFT = 0x08;
    }
}

