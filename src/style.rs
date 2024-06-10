use gtk::{style_context_add_provider_for_display, CssProvider};

const WINDOW_PADDING: i32 = 4;
const LABEL_PADDING: i32 = 4;
const FONT_SIZE: i32 = 14;
pub const APP_HEIGHT: i32 = WINDOW_PADDING + LABEL_PADDING + FONT_SIZE;

const GLOBAL_CSS: &str = "
    window {
        padding: 4px 8px;
        font-size: 14px;
        background-color: black;
    }

    label {
        padding-bottom: 4px;
    }

    .green {
        color: #0f0;
    }

    .red {
        color: #f00;
    }
";

pub fn init_global_css() {
    let css_provider = CssProvider::new();
    css_provider.load_from_string(GLOBAL_CSS);
    let display = gdk::Display::default().unwrap();
    style_context_add_provider_for_display(&display, &css_provider, u32::MAX)
}
