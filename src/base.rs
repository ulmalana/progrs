#[derive(Debug)]
pub enum Status {
    Ongoing,
    Finished,
    Ignored,
}

#[derive(Debug)]
pub struct Record {
    pub id: u32,
    pub todo_name: String,
    pub start_date: String,
    pub end_date: String,
    pub notes: String,
    pub status: Status,
}
