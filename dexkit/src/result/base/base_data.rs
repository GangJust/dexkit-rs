use crate::DexkitBridge;

pub(crate) trait BaseData {
    fn get_bradge(&self) -> &DexkitBridge;

    fn get_dex_id(&self) -> u32;

    fn get_id(&self) -> u32;

    fn get_mine_encode_id(&self) -> i64 {
        Self::get_encode_id(self.get_dex_id(), self.get_id())
    }

    fn get_encode_id(dex_id: u32, id: u32) -> i64 {
        ((dex_id as i64) << 32) | (id as i64)
    }
}
