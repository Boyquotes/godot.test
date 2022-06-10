use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub class: Option<String>,
    pub name: Option<String>,
    pub max_hp: i32,
    pub max_mp: i32,
    pub phy: i32,
    pub spi: i32,
    pub agile: i32,
    pub speed: i32,
    pub sight: i32,
    pub lucky: i32,
    pub weight: i32,
    pub lv: i32,
    pub exp: i32,
}

impl Stats {
    pub fn new(
        class: Option<String>,
        name: Option<String>,
        max_hp: i32,
        max_mp: i32,
        phy: i32,
        spi: i32,
        agile: i32,
        speed: i32,
        sight: i32,
        lucky: i32,
        weight: i32,
        lv: i32,
        exp: i32,
    ) -> Self {
        Self {
            class,
            name,
            max_hp,
            max_mp,
            phy,
            spi,
            agile,
            speed,
            sight,
            lucky,
            weight,
            lv,
            exp,
        }
    }
}
