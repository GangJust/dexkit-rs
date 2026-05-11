use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::gen_flatbuffers::dexkit::fb::{
    FBOpCodeMatchType, FBOpCodesMatcher, FBOpCodesMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::enums::OpCodeMatchType;
use crate::query::matchers::IntRange;

pub struct OpCodesMatcher {
    op_codes: Option<Vec<i16>>,
    match_type: OpCodeMatchType,
    range_matcher: Option<IntRange>,
}

impl Default for OpCodesMatcher {
    fn default() -> Self {
        OpCodesMatcher {
            op_codes: None,
            match_type: OpCodeMatchType::Contains,
            range_matcher: None,
        }
    }
}

impl<'a> BaseQuery<'a, WIPOffset<FBOpCodesMatcher<'a>>> for OpCodesMatcher {
    fn inner_build(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<FBOpCodesMatcher<'a>> {
        let op_codes = self.op_codes.as_ref().map(|codes| {
            let codes_slice: &[i16] = codes;
            fbb.create_vector(codes_slice)
        });
        let match_type: FBOpCodeMatchType = self.match_type.into();
        let op_code_count = self.range_matcher.as_ref().map(|r| r.inner_build(fbb));

        FBOpCodesMatcher::create(
            fbb,
            &FBOpCodesMatcherArgs {
                op_codes,
                match_type,
                op_code_count,
            },
        )
    }
}

impl OpCodesMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OpCodesMatcher {
    pub fn op_codes<I>(mut self, op_codes: I) -> Self
    where
        I: IntoIterator<Item = i16>,
    {
        self.op_codes = Some(op_codes.into_iter().collect());
        self
    }

    pub fn match_type(mut self, match_type: OpCodeMatchType) -> Self {
        self.match_type = match_type;
        self
    }

    pub fn range(mut self, range: IntRange) -> Self {
        self.range_matcher = Some(range);
        self
    }
}

impl OpCodesMatcher {
    pub fn add_op_code(mut self, op_code: i16) -> Self {
        self.op_codes.get_or_insert_with(Vec::new).push(op_code);
        self
    }
}

impl OpCodesMatcher {
    pub fn count(mut self, count: u32) -> Self {
        self.range_matcher = Some(IntRange::exactly(count));
        self
    }

    pub fn count_range(mut self, min: u32, max: u32) -> Self {
        self.range_matcher = Some(IntRange::range(min, max));
        self
    }

    pub fn count_min(mut self, min: u32) -> Self {
        self.range_matcher = Some(IntRange::at_least(min));
        self
    }

    pub fn count_max(mut self, max: u32) -> Self {
        self.range_matcher = Some(IntRange::at_most(max));
        self
    }
}
