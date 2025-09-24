use crate::gen_flatbuffers::dexkit::schema::{
    OpCodeMatchType as FBOpCodeMatchType, OpCodesMatcher as FBOpCodesMatcher,
    OpCodesMatcherArgs as FBOpCodesMatcherArgs,
};
use crate::query::base::BaseQuery;
use crate::query::enums::OpCodeMatchType;
use crate::query::matchers::base::IntRange;

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

impl<'a> BaseQuery<'a, flatbuffers::WIPOffset<FBOpCodesMatcher<'a>>> for OpCodesMatcher {
    fn inner_build(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<FBOpCodesMatcher<'a>> {
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
