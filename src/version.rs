pub const VERSION: Version = Version {
    major: 0,
    minor: 0,
    patch: 0,
    unpublished_v: 1,
};

pub const VERSION_STR: &str = "0.0.0-0";

#[derive(Copy, Clone, Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub unpublished_v: u8,
}

impl Version {
    pub const fn protocol_v(&self) -> u64 {
        ((self.major as u64) << 24)
            | ((self.minor as u64) << 16)
            | ((self.patch as u64) << 8)
            | ((self.unpublished_v as u64) << 0)
    }
}
