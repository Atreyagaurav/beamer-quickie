mod imp;

use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, ResponseType, SignalListItemFactory};
use gtk::{prelude::*, ListItem};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::pdfparse;
use crate::slide::{SlideData, SlideObject};
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
        self.imp().current_slide_content.replace("Test".to_string());
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
		let slides: Vec<SlideData> = window.slides().iter::<SlideObject>().map(|s| s.unwrap().content().to_string()).map(|s| SlideData::new(true, 0, 0, s, None)).collect();
		let beamer = BeamerContents::new(preamble, slides, vec![], vec![]);
		println!("{}", beamer.to_string());
                    }));

        self.imp()
            .txt_browse
            .connect_changed(clone!(@weak self as window => move |text| {
                if let Ok(bc) =
                    BeamerContents::load(text.text())
                {

		    window.imp().preamble.replace(bc.preamble().to_string());
		    let pdffile = PathBuf::from(text.text()).with_extension("pdf");
		    let pages: Vec<i32> = (0..crate::pdfparse::pdf_pages_count(&pdffile)).map(|i| i+1).collect();
		    let scanner = crate::synctex::Scanner::from_output(&pdffile, None);
		    let lines = scanner.get_lines(&pages);
                    window.slides().remove_all();
                    bc.slides().enumerate().for_each(|(_, s)| {
			let sob = SlideObject::new(s);

			let page = lines.iter().enumerate().filter_map(|(i, (_, l))| {
			    let s = sob.linestart();
			    let e = sob.lineend();
			    if (s..=e).contains(l) {
				Some(i)
			    }else{
				None
			    }
			}).last();
			if let Some(page) = page {
			    sob.set_image(pdfparse::get_thumbnail(&pdffile, page));
			}
			window.slides().append(&sob)
                    });
                }
            }));

        self.imp()
            .btn_save_new
            .connect_clicked(clone!(@weak self as window => move |_| {
            window.open_editor(&window.imp().current_slide_content);
                }));

        // TEMP for testing
        self.imp()
            .txt_browse
            .set_text("/home/gaurav/work/presentations/ms-thesis/slides.tex");
    }

    pub fn open_editor(&self, content: &Rc<RefCell<String>>) {
        //     let editor = gtk::Dialog::builder()
        //         .default_height(500)
        //         .default_width(500)
        //         .title("Edit Frame")
        //         .build();

        //     let scroll = gtk::ScrolledWindow::builder()
        //         .hexpand(true)
        //         .vexpand(true)
        //         .build();
        //     let text = gtk::TextView::builder().hexpand(true).vexpand(true).build();
        //     let btn_accept = gtk::Button::builder().label("Accept").build();
        //     let btn_cancel = gtk::Button::builder().label("Cancel").build();
        //     text.buffer().set_text(&content.borrow());
        //     scroll.set_child(Some(&text));
        //     editor.set_child(Some(&scroll));
        //     editor.add_action_widget(&btn_accept, ResponseType::Accept);
        //     editor.add_action_widget(&btn_accept, ResponseType::Cancel);

        //     editor.set_transient_for(Some(self));
        //     editor.set_modal(false);

        //     editor.connect_close_request(clone!(@weak self as window => @default-panic,  move |_| {
        //     window.set_sensitive(true);
        //     glib::Propagation::Proceed
        //     }));
        //     // self.set_sensitive(false);
        //     editor.connect_response(
        //         clone!(@weak self as window, @weak text, @weak content => move |_, res|{
        //             match res {
        //             ResponseType::Accept => {
        //                 let buf = text.buffer();
        //             content.replace(buf.text(&buf.start_iter(), &buf.end_iter(), true)
        //                     .to_string());
        //             }
        //             _ => (),
        //             }
        //         println!("{}", window.imp().current_slide_content.borrow());
        //             }),
        //     );
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
