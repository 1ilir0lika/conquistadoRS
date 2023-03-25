pub mod player{
 use crate::oggetti::regno::regno::Regno;
 #[derive(Clone)]
    pub struct Player{
       pub regno:Regno,
       pub id:u8,
       pub nome:String,
    }
}