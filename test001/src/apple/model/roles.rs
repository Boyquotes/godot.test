use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    pub _id: ObjectId,
    pub user_id: Option<ObjectId>,
    pub room_key: Option<i32>,
    pub stats: Option<stats::Stats>,
    pub action: Option<action::Action>,
}
impl Roles {
    pub fn new(user_id: Option<ObjectId>, room_key: Option<i32>) -> Self {
        Self {
            _id: ObjectId::new(),
            user_id,
            room_key,
            stats: Some(stats::Stats::default()),
            action: Some(action::Action::default()),
        }
    }
}
