use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

use crate::api::loaders::ImageLoader;
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

    pub fn bind(&self, worker: Worker, heading: &str, body: &str, image_url: &str) {
        let widget = self.imp();

        widget.heading.set_text(heading);
        widget.body.set_text(body);

        println!("image_url: {}", image_url);

        let url = image_url.to_string();

        let image = widget.image.clone();
        worker.send_local_task(async move {
            let loader = ImageLoader::new();
            let pixbuf = loader.from_url(url).await;
            image.set_from_pixbuf(pixbuf.as_ref());
        });
    }
}
