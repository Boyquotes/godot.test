use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Action {
    status: u8,
    position: [f64; 2],
    speed: [f64; 2],
    back: [f64; 2],
    hp: i32,
    mp: i32,
    atn: i32,
    int: i32,
}

impl Action {
    pub fn new(
        status: u8,
        position: [f64; 2],
        speed: [f64; 2],
        back: [f64; 2],
        hp: i32,
        mp: i32,
        atn: i32,
        int: i32,
    ) -> Self {
        Self {
            status,
            position,
            speed,
            back,
            hp,
            mp,
            atn,
            int,
        }
    }
}
