use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{FBIntRange, FBIntRangeArgs};
use crate::query::base::BaseQuery;

pub struct IntRange {
    min: u32,
    max: u32,
}

impl Default for IntRange {
    fn default() -> Self {
        IntRange {
            min: 0,
            max: 2147483647,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBIntRange<'a>>> for IntRange {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBIntRange<'a>> {
        let root = FBIntRange::create(
            fbb,
            &FBIntRangeArgs {
                min: self.min as i32,
                max: self.max as i32,
            },
        );
        root
    }
}

impl IntRange {
    pub fn create() -> Self {
        Self::default()
    }

    // base
    pub fn set_min(mut self, min: u32) -> Self {
        self.min = min;
        self
    }

    pub fn set_max(mut self, max: u32) -> Self {
        self.max = max;
        self
    }

    // extend
    pub fn exactly(count: u32) -> Self {
        IntRange {
            min: count,
            max: count,
        }
    }

    pub fn range(min: u32, max: u32) -> Self {
        IntRange { min, max }
    }

    pub fn at_least(count: u32) -> Self {
        IntRange {
            min: count,
            max: 2147483647,
        }
    }

    pub fn at_most(count: u32) -> Self {
        IntRange { min: 0, max: count }
    }
}
