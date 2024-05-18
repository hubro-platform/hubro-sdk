use bytes::Bytes;

enum Actions {
    RegisterComplete,
    QuestionnaireAssigned
}

pub struct Client {}

impl crate::signals::Client {
    pub fn process_data(data: Bytes) {

    }
}