use adw::subclass::prelude::*;
use gtk::prelude::BoxExt;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

use crate::api::loaders::ImageLoader;
use crate::api::GameMod;
use crate::dispatch::Worker;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/mnts/ModManager/ui/components/card.ui")]
    pub struct Card {
        #[template_child]
        pub heading: TemplateChild<gtk::Label>,

        #[template_child]
        pub body: TemplateChild<gtk::Label>,

        #[template_child]
        pub image: TemplateChild<gtk::Image>,

        #[template_child]
        pub footer: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Card {
        const NAME: &'static str = "Card";
        type Type = super::Card;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Card {}
    impl WidgetImpl for Card {}
    impl BoxImpl for Card {}
}

glib::wrapper! {
    pub struct Card(ObjectSubclass<imp::Card>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Card {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn set_heading(&self, heading: &str) {
        let widget = self.imp();
        widget.heading.set_text(heading);
    }

    fn set_body(&self, body: &str) {
        let widget = self.imp();
        widget.body.set_text(body);
    }

    fn set_image_from_url(&self, worker: Worker, url: String) {
        let card = self.imp();
        let image = card.image.clone();

        worker.send_local_task(async move {
            let loader = ImageLoader::new();
            let pixbuf = loader.from_url(url).await;
            image.set_from_pixbuf(pixbuf.as_ref());
        });
    }

    pub fn new_for_game_mod(worker: Worker, game_mod: &GameMod) -> Self {
        let card = Self::new();
        let widget = card.imp();

        card.set_image_from_url(worker, game_mod.logo.url.to_string());
        card.set_heading(&game_mod.name);
        card.set_body(&game_mod.summary);

        let categories = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        categories.append(&gtk::Label::new(Some("Categories: ")));

        for category in &game_mod.categories {
            let button = gtk::Button::builder()
                .label(&category.name)
                .css_classes(["pill", "flat"])
                .build();
            categories.append(&button);
        }
        widget.footer.append(&categories);

        card
    }
}
