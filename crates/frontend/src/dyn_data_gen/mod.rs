// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables
// TODO: Make the skills panes autogenerables
// TODO: Make the works autogenerables

use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use yew::Html;

pub trait DynGenerable {
    type Data;

    fn r#type(&self) -> String;

    fn data(&self) -> Self::Data
    where
        for<'a> Self::Data: Deserialize<'a>,
    {
        serde_json::from_reader(
            File::open(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("dyn_data")
                    .join(self.r#type())
                    .join("test.json"),
            )
            .unwrap(),
        )
        .expect("Cannot deserialize data struct")
    }

    fn generate(&self) -> Html;
}
