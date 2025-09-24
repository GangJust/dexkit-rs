pub trait BaseData {
    fn get_dex_id(&self) -> u32;

    fn get_id(&self) -> u32;

    fn get_encode_id(&self) -> i64 {
        ((self.get_dex_id() as i64) << 32) | (self.get_id() as i64)
    }
}
