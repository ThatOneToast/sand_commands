use super::nbt::block_entity_format::BlockEntityComponents;


pub struct BlockEntityData {
    pub data: BlockEntityComponents
}

impl ToString for BlockEntityData {
    fn to_string(&self) -> String {
        let data = self.data.to_string();
        format!("block_entity_data:{data}")
    }
}
