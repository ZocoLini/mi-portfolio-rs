// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables
// TODO: Make the skills panes autogenerables
// TODO: Make the works autogenerables

use gloo_net::http::Request;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

pub trait DynGenerable {
    type Data;

    fn r#type(&self) -> String;
    fn resouce_id(&self) -> String;
    
    async fn data(&self) -> Self::Data
    where
        for<'a> Self::Data: Deserialize<'a>,
    {
        let fetched_data = Request::get(
            &format!("resources/dyn_data/{}/{}.json", self.r#type(), self.resouce_id())
        ) // Ruta relativa al directorio dist
            .send()
            .await
            .expect("Failed to fetch JSON");

        serde_json::from_value::<Self::Data>(
            fetched_data.json().await.expect("Failed to parse JSON"),
        )
        .expect("Failed to deserialize DynGenerable object")
    }
}

pub fn load_data() -> HashMap<String, Vec<String>> {
    let mut data = HashMap::new();

    let dyn_data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dyn_data");

    for entry in std::fs::read_dir(dyn_data).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        data.insert(
            path.file_name().unwrap().to_str().unwrap().to_string(),
            vec![],
        );

        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

            data.get_mut(path.file_name().unwrap().to_str().unwrap())
                .unwrap()
                .push(file_name);
        }
    }

    data
}
