mod imp;

use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};
use std::path::PathBuf;

use crate::slide::SlideObject;
use crate::slide_row::SlideRow;
use crate::texparse::BeamerContents;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn slides(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .slides
            .borrow()
            .clone()
            .expect("Could not get slides.")
    }

    fn setup_preamble(&self) {
        self.imp().preamble.replace("".to_string());
    }

    fn setup_slides(&self) {
        // Create new model
        let model = gio::ListStore::new::<SlideObject>();

        // Get state and set model
        self.imp().slides.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.slides()));
        self.imp().lv_slides.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        self.imp()
            .btn_browse
            .connect_clicked(clone!(@weak self as window => move |_| {
            let dialog = gtk::FileDialog::builder()
                .title("Beamer LaTeX File")
                .accept_label("Open")
                .initial_folder(&gio::File::for_path(window.imp().txt_browse.text()))
                .build();

            dialog.open(Some(&window), gio::Cancellable::NONE,clone!(@weak window => move |file| {
                if let Ok(file) = file {
                let filename = file.path().expect("Couldn't get file path");
                let name = filename.to_string_lossy();
                window.imp().txt_browse.set_text(&name);
                }}));
            }));

        self.imp()
            .btn_save
            .connect_clicked(clone!(@weak self as window => move |_| {
                let preamble =  window.imp().preamble.borrow().clone();

            //     let slides = window.slides().iter::<ListItem>().map(|s| s.unwrap()
            //             .item()
            //             .and_downcast::<SlideObject>()
            //             .expect("The item has to be an `SlideObject`.").content()).collect();
            //     let beamer = BeamerContents::new(preamble, slides, vec![], vec![]);
            // println!("{}", beamer.to_string());
                        }));

        self.imp()
            .txt_browse
            .connect_changed(clone!(@weak self as window => move |text| {
                    if let Ok(bc) =
                BeamerContents::load(text.text())
                {

            window.imp().preamble.replace(bc.preamble().to_string());
            let pdffile = PathBuf::from(text.text()).with_extension("pdf");
            let pages = crate::pdfparse::frames_pages(&pdffile);
            window.slides().remove_all();
            bc.slides().enumerate().for_each(|(i, s)| {
            window.slides().append(
                &SlideObject::new(
                s.to_string(),
                pages.get(i).map(|p| PathBuf::from(format!("temp/p-{}.png", p)))
                )
            )
            });
            }
            }));

        // TEMP for testing
        self.imp()
            .txt_browse
            .set_text("/home/gaurav/work/presentations/ms-thesis/slides.tex");
    }

    // fn new_slide(&self) {
    //     // Get content from entry and clear it
    //     let buffer = self.imp().entry.buffer();
    //     let content = buffer.text().to_string();
    //     if content.is_empty() {
    //         return;
    //     }
    //     buffer.set_text("");

    //     // Add new slide to model
    //     let slide = SlideObject::new(false, content);
    //     self.slides().append(&slide);
    // }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `SlideRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `SlideRow`
            let slide_row = SlideRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&slide_row));
        });

        // Tell factory how to bind `SlideRow` to a `SlideObject`
        factory.connect_bind(move |_, list_item| {
            // Get `SlideObject` from `ListItem`
            let slide_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<SlideObject>()
                .expect("The item has to be an `SlideObject`.");

            // Get `SlideRow` from `ListItem`
            let slide_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<SlideRow>()
                .expect("The child has to be a `SlideRow`.");

            slide_row.bind(&slide_object);
        });

        // Tell factory how to unbind `SlideRow` from `SlideObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `SlideRow` from `ListItem`
            let slide_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<SlideRow>()
                .expect("The child has to be a `SlideRow`.");

            slide_row.unbind();
        });

        // Set the factory of the list view
        self.imp().lv_slides.set_factory(Some(&factory));
    }
}
