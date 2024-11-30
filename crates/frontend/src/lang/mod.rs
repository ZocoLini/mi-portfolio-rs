use web_sys::window;

pub trait MultiLang {
    fn translate(self) -> Self;
}

impl MultiLang for String {
    fn translate(self) -> Self {
        get_string(self)
    }
}

fn get_string(_id: String) -> String
{
    get_locale().unwrap()
}

fn get_locale() -> Option<String> {
    let window = window()?;
    let navigator = window.navigator();
    navigator.language()
}