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
    "hola".to_string()
}