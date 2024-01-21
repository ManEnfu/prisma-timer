use gtk::glib;

pub struct TemplateCallbacks {}

#[gtk::template_callbacks(functions)]
impl TemplateCallbacks {
    #[template_callback]
    pub fn string_not_empty(string: Option<&str>) -> bool {
        !string.unwrap_or_default().is_empty()
    }

    #[template_callback]
    pub fn and_boolean(a: bool, b: bool) -> bool {
        a && b
    }

    #[template_callback]
    pub fn or_boolean(a: bool, b: bool) -> bool {
        a || b
    }

    #[template_callback]
    pub fn invert_boolean(a: bool) -> bool {
        !a
    }

    #[template_callback]
    pub fn object_is_some(obj: Option<glib::Object>) -> bool {
        obj.is_some()
    }
}
