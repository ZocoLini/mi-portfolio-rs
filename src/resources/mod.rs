pub fn get_icon(id: &str) -> String
{
    format!("static/img/icon/{}", id)
}

pub fn get_work_icon(id: &str) -> String
{
    get_work_img(id, "icon.png")
}

pub fn get_work_img(id: &str, img_id: &str) -> String
{
    format!("static/img/works/{}/{}", id, img_id)
}

pub fn get_translation_file(locale: &str) -> String
{
    format!("static/langs/{}.properties", locale.replace("-", "_"))
}

pub fn get_data_json(id: &str) -> String
{
    format!("static/data/{}.json", id)
}