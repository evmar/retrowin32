/// x86 segment descriptor, with fields broken out.
/// Intel manual volume 3 section 3.4.5.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct SegmentDescriptor {
    pub base: u32,
    pub limit: u32,

    /// G flag
    pub granularity: bool,
    /// DB flag
    pub db: bool,
    /// L flag
    pub long: bool,
    /// AVL flag
    pub available: bool,
    /// P flag
    pub present: bool,
    /// DPL flag
    pub dpl: u8,
    /// S flag
    pub system: bool,
    /// Intel manual 3.4.5.1 Code-and Data-Segment Descriptor Types
    pub type_: u8,
}

impl SegmentDescriptor {
    #[allow(unused)]
    pub fn decode(raw: u64) -> Self {
        let hi = (raw >> 32) as u32;
        let lo = raw as u32;

        let base = (hi & 0xFF00_0000) | ((hi & 0x0000_00FF) << 16) | ((lo & 0xFFFF_0000) >> 16);
        let limit = (hi & 0x000F_0000) | (lo & 0x0000_FFFF);
        let granularity = (hi & (1 << 23)) != 0;
        let db = (hi & (1 << 22)) != 0;
        let long = (hi & (1 << 21)) != 0;
        let available = (hi & (1 << 20)) != 0;
        let present = (hi & (1 << 15)) != 0;
        let dpl = ((hi >> 13) & 0xb11) as u8;
        let system = (hi & (1 << 12)) != 0;
        let type_ = ((hi & 0x0000_0F00) >> 8) as u8;

        SegmentDescriptor {
            base,
            limit,
            granularity,
            db,
            long,
            available,
            present,
            dpl,
            system,
            type_,
        }
    }

    pub fn encode(&self) -> u64 {
        let hi = (self.base & 0xFF00_0000)
            | (self.granularity as u32) << 23
            | (self.db as u32) << 22
            | (self.long as u32) << 21
            | (self.available as u32) << 20
            | (self.limit & 0x000F_0000)
            | (self.present as u32) << 15
            | (self.dpl as u32 & 0b11) << 13
            | (self.system as u32) << 12
            | (self.type_ as u32 & 0b1111) << 8
            | (self.base & 0x00FF_0000) >> 16;
        let lo = ((self.base & 0x0000_FFFF) << 16) | (self.limit & 0x0000_FFFF);
        ((hi as u64) << 32) | (lo as u64)
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.encode() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentDescriptor;

    #[test]
    fn empty() {
        assert_eq!(SegmentDescriptor::default().encode(), 0);
    }

    #[test]
    fn full() {
        let full = SegmentDescriptor {
            base: 0xFFFF_FFFF,
            limit: 0xFFFF_FFFF,
            granularity: true,
            db: true,
            long: true,
            available: true,
            present: true,
            dpl: 3,
            system: true,
            type_: 0b1111,
        };
        assert_eq!(full.encode(), 0xFFFF_FFFF_FFFF_FFFF);
    }
}
