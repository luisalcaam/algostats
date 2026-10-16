use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CodeforcesApiResponse<T>{
    status : String,
    pub result : T,
}
