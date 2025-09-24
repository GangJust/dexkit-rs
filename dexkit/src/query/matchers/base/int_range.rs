use crate::gen_flatbuffers::dexkit::schema::{
    IntRange as FBIntRange, IntRangeArgs as FBIntRangeArgs,
};
use crate::query::base::BaseQuery;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct IntRange {
    min: i32,
    max: i32,
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
                min: self.min,
                max: self.max,
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
    pub fn set_min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    pub fn set_max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }
}
