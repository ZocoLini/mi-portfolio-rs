// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables
// TODO: Make the skills panes autogenerables
// TODO: Make the works autogenerables

use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

pub trait DynGenerable {
    type Data;

    fn r#type(&self) -> String;

    async fn data(&self, resource_id: &str) -> Self::Data
    where
        for<'a> Self::Data: Deserialize<'a>,
    {
        let url = format!(
            "{}/dyn_data/{}/{}.json",
            env!("CARGO_MANIFEST_DIR"),
            self.r#type(),
            resource_id
        );
        
        serde_json::from_str(&url).unwrap()
    }
}

pub fn load_data() -> HashMap<String, Vec<String>> {
    let mut data = HashMap::new();
    
    let dyn_data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dyn_data");
    
    for entry in std::fs::read_dir(dyn_data).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        data.insert(path.file_name().unwrap().to_str().unwrap().to_string(), vec![]);
        
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            
            data.get_mut(path.file_name().unwrap().to_str().unwrap()).unwrap().push(file_name);
        }
    }
    
    data
}
