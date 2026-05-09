use crate::BridgeCore;

pub(crate) trait BaseData {
    fn bridge(&self) -> &BridgeCore;

    fn dex_id(&self) -> u32;

    fn id(&self) -> u32;

    fn get_mine_encode_id(&self) -> i64 {
        Self::get_encode_id(self.dex_id(), self.id())
    }

    fn get_encode_id(dex_id: u32, id: u32) -> i64 {
        ((dex_id as i64) << 32) | (id as i64)
    }
}
