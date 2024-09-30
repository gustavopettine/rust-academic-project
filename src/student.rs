#[derive(Debug)]
pub enum Status {
    Active,
    Enrolled,
    Graduated,
}

pub struct Student {
    pub name: String,
    pub id: u32,
    pub status: Status,
}
