pub fn get_icon_src(id: &str) -> String
{
    format!("resources/img/icon/{}", id)
}

pub fn get_work_icon_src(id: &str) -> String
{
    format!("resources/img/works/{}/icon.png", id)
}

pub fn get_work_image_src(work_id: &str, img_id: &str) -> String
{
    format!("resources/img/works/{}/{}", work_id, img_id)
}

pub fn get_translations_src(locale: &str) -> String
{
    format!("resources/langs/{}.properties", locale.replace("-", "_"))
}

pub fn get_dyn_data_src(id: &str) -> String
{
    format!("resources/dyn-data/{}.json", id)
}