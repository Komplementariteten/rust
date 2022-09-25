use crate::pswdb::field::RecordField;

#[derive(Debug, Clone)]
pub struct DbRecord {
    pub(crate) Fields: Vec<RecordField>
}
