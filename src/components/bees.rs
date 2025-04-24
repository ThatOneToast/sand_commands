use super::{nbt::entity_format::EntityData, Component};

#[derive(Clone, Debug)]
pub struct Bee {
    pub entity_data: EntityData,
    pub min_ticks_in_hive: u32,
    pub ticks_in_hive: u32,
}

impl ToString for Bee {
    fn to_string(&self) -> String {
        let data = self.entity_data.to_string();
        let mtih = self.min_ticks_in_hive;
        let tih = self.ticks_in_hive;
        format!("{{entity_data:{data},min_ticks_in_hive:{mtih},ticks_in_hive:{tih}}}")
    }
}

impl Component for Bee {}

#[derive(Clone, Debug)]
pub struct Bees(pub Vec<Bee>);

impl ToString for Bees {
    fn to_string(&self) -> String {
        let bees = self
            .0
            .iter()
            .map(|bee| bee.to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("{{bees:{bees}}}")
    }
}
