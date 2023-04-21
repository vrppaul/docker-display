use std::collections::HashMap;

#[derive(Debug)]
pub struct Container {
    pub id: String,
    pub image: String,
    pub name: String,
}

pub type ContainersHashMap = HashMap<Option<String>, HashMap<String, Container>>;
