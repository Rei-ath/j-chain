// use crate::{candidate::Candidate, voter::Voter};
pub enum Val {
    Voter,
    Candidate,
}

pub struct User {
    user_info:Option<Val>,
    user_type:String,
}

impl User {
    pub fn new(user_info: Option<Val>,user_type:String)->Self{
        Self{
            user_info,
            user_type
        }
    }
}