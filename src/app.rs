use gtk::{prelude::*, ApplicationWindow};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::style::{init_global_css, APP_HEIGHT};

const ANCHORS: [(Edge, bool); 4] = [
    (Edge::Left, true),
    (Edge::Right, true),
    (Edge::Top, false),
    (Edge::Bottom, true),
];

pub struct App {
    window: ApplicationWindow,
}

impl App {
    pub fn new(window: ApplicationWindow) -> Self {
        App { window }
    }

    pub fn init(&self) {
        init_global_css();
        self.init_layer_shell();
        self.build_ui();
    }

    pub fn present(&self) {
        self.window.present()
    }

    fn init_layer_shell(&self) {
        self.window.init_layer_shell();
        self.window.set_layer(Layer::Top);
        self.window.set_margin_bottom(0);
        self.window.set_margin_top(0);
        self.window.set_margin_end(0);
        self.window.set_margin_start(0);
        self.window.set_exclusive_zone(APP_HEIGHT);

        for (anchor, state) in ANCHORS {
            self.window.set_anchor(anchor, state);
        }
    }

    fn build_ui(&self) {
        let div = gtk::Box::new(gtk::Orientation::Horizontal, 24);
        div.set_halign(gtk::Align::Center);

        let label1 = gtk::Label::new(Some("BTC 71,512(▴34.12)"));
        label1.set_css_classes(&["green"]);

        let label2 = gtk::Label::new(Some("ETH 4,215(▾11.5)"));
        label2.set_css_classes(&["red"]);

        div.append(&label1);
        div.append(&label2);
        self.window.set_child(Some(&div));
    }
}
