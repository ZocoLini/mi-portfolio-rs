

pub enum PaneType {
    Primary,
    Secondary,
    Tertiary,
}

impl PaneType {
    fn as_bkg_color(&self) -> String {
        match self {
            PaneType::Primary => String::from("var(--color-primary-bkg-pane)"),
            PaneType::Secondary => String::from("var(--color-secondary-bkg-pane)"),
            PaneType::Tertiary => String::from("var(--color-tertiary-bkg-pane)"),
        }
    }
}

pub struct PaneStyle {
    pane_type: PaneType,
    padding: u8,
    margin: u8,
    border_radius: u8,
}

impl PaneStyle {
    pub fn new(r#type: PaneType) -> Self {
        PaneStyle {
            pane_type: r#type,
            padding: 10,
            margin: 5,
            border_radius: 20,
        }
    }

    pub fn css(&self) -> String {
        format!(
            "padding: {}px;
             margin: {}px;
             border-radius: {}px;
             background-color: {};",
            self.padding,
            self.margin,
            self.border_radius,
            self.pane_type.as_bkg_color()
        )
    }
}
