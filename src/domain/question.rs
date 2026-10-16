use serde::Deserialize;

#[derive(Deserialize,Debug, Eq, PartialEq)]
pub struct Question {
    pub contestId : ContestId,
    pub problemsetName : Option<String>,
    pub index : String,
    pub name : String,
    pub rating : Option<u32>,
    pub tags : Vec<String>
}

#[derive(Deserialize,Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContestId(u32);


