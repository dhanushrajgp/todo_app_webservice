use super::super::enums::TaskStatus;
use serde::Serialize;
#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}


#[cfg(test)]
mod base_test{
    use super::Base;
    use super::TaskStatus;

    #[test]
    fn new(){
        let expected_status = TaskStatus::DONE;
        let expected_title = String::from("title");

        let new_base_struct = Base{
            title: expected_title.clone(),
            status:TaskStatus::DONE
        };

        assert_eq!(expected_title,new_base_struct.title);
        assert_eq!(expected_status,new_base_struct.status);
    }
}