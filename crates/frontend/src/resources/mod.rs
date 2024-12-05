pub fn get_icon_src(id: &str) -> String
{
    format!("/resources/img/icon/{}", id)
}

pub fn get_work_icon_src(id: &str) -> String
{
    format!("/resources/img/works/{}/icon.png", id)
}