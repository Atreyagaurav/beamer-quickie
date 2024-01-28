mod imp;

use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};
use itertools::Itertools;
use sourceview5::gtk::prelude::TextBufferExt;
use sourceview5::gtk::prelude::TextViewExt;
use std::iter::Iterator;

use crate::slide::{SlideData, SlideObject};
use crate::slide_row::SlideRow;
use crate::texparse::{self, BeamerContents};

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
            let mut dialog = gtk::FileDialog::builder()
                        .title("Beamer LaTeX File")
                    .accept_label("Open");
		let txt = window.imp().txt_browse.text();
		if !txt.is_empty() {
		    dialog = dialog.initial_file(&gio::File::for_path(txt));
		};

            dialog.build().open(Some(&window), gio::Cancellable::NONE,clone!(@weak window => move |file| {
                        if let Ok(file) = file {
                let filename = file.path().expect("Couldn't get file path");
                let name = filename.to_string_lossy();
                window.imp().txt_browse.set_text(&name);
                        }}));
                }));

        self.imp()
            .cb_selectall
            .connect_active_notify(clone!(@weak self as window => move |sall| {
            let include = sall.is_active();
                window.slides()
                .iter::<SlideObject>()
                .for_each(|s| s.unwrap().set_include(include));
                            }));

        self.imp()
            .btn_preview
            .connect_clicked(clone!(@weak self as window => move |_| {
            let text = window.get_contents();
            window.imp().tv_frame.buffer().set_text(&text);
                        }));

        self.imp()
            .btn_graphics
            .connect_clicked(clone!(@weak self as window => move |_| {
                let (dirs, files) = window.get_graphics();
                let text = format!("Graphics Directories:\n\n{}\n\nGraphics:\n\n{}\n", dirs.join("\n"), files.join("\n"));
                window.imp().tv_frame.buffer().set_text(&text);
            }));

        self.imp()
            .btn_copy
            .connect_clicked(clone!(@weak self as window => move |_| {
            let display = gdk::Display::default().unwrap();
            let tb = window.imp().tv_frame.buffer();
                let text = tb.text(&tb.start_iter(), &tb.end_iter(), true);
                let clipboard = display.clipboard();
                    clipboard.set_text(&text);
                                    }));

        self.imp()
            .txt_browse
            .connect_changed(clone!(@weak self as window => move |text| {
                if let Ok(bc) =
                    BeamerContents::load(text.text())
                {
		    window.imp().tv_frame.buffer().set_text(&bc.to_string());
		    window.imp().preamble.replace(bc.preamble().to_string());
                    window.slides().remove_all();
                    bc.slides().chain(bc.appendix()).chain(bc.unused()).enumerate().for_each(|(_, s)| {
			let sob = SlideObject::new(s);
			window.slides().append(&sob);
                    });
                }
            }));

        self.imp()
            .tv_frame
            .buffer()
            .connect_changed(clone!(@weak self as window => move |_| {
            let tb = window.imp().tv_frame.buffer();
                    let mut prev = tb.start_iter();
                let mut point = tb.start_iter();
                while point.forward_char() {
                    if prev.char() == '\\' {
                        window.format_frametitle(&tb, &mut point);
            }
                            prev = point;
                }
                            }));
    }

    fn format_frametitle(&self, tb: &gtk::TextBuffer, point: &mut gtk::TextIter) {
        let prev = *point;
        match point.char() {
            '%' | '\\' => (),
            _ => {
                while point.forward_char() && point.char().is_ascii_alphabetic() {}
                let cmd = tb.text(&prev, point, true);
                match cmd.as_str() {
                    "begin" => {
                        let temp = *point;
                        while point.forward_char() && point.char() != '}' {}
                        point.forward_char();
                        if tb.text(&temp, point, true) == "{frame}" && point.char() != '\n' {
                            let mut sol = prev;
                            sol.backward_char();
                            if point.char() != '{' {
                                while point.forward_char() && point.char() != ']' {}
                                point.forward_char();
                            }
                            let sob = *point;
                            while (point.char() != '%' && point.char() != '\n')
                                && point.forward_char()
                            {}
                            tb.apply_tag_by_name("tag_frametitle", &sob, point);
                        }
                        *point = temp;
                    }
                    "frametitle" => {
                        if point.char() != '{' {
                            while point.forward_char() && point.char() != ']' {}
                            point.forward_char();
                        }
                        let sob = *point;
                        while (point.char() != '%' && point.char() != '\n') && point.forward_char()
                        {
                        }
                        tb.apply_tag_by_name("tag_frametitle", &sob, point);
                    }
                    _ => (),
                }
            }
        }
    }

    fn get_slidedatas(&self) -> Vec<SlideData> {
        self.slides()
            .iter::<SlideObject>()
            .filter(|s| s.as_ref().unwrap().include())
            .map(|s| {
                (
                    s.as_ref().unwrap().content().to_string(),
                    s.as_ref().unwrap().slidetype(),
                )
            })
            .map(|(s, t)| SlideData::new(true, 0, 0, s, None, t))
            .collect()
    }

    fn get_contents(&self) -> String {
        let slides = self.get_slidedatas();
        if self.imp().cb_slidesonly.is_active() {
            slides.iter().map(|s| &s.content).join("\n\n")
        } else {
            let preamble = self.imp().preamble.borrow().clone();
            BeamerContents::from_slides(preamble, slides).to_string()
        }
    }

    fn get_graphics(&self) -> (Vec<String>, Vec<String>) {
        let graphics_paths = if self.imp().cb_slidesonly.is_active() {
            vec![".".to_string()]
        } else {
            let preamble = self.imp().preamble.borrow().clone();
            texparse::get_graphics_dir(&preamble)
                .iter()
                .map(|s| s.to_string())
                .collect()
        };
        let slides = self.get_slidedatas();
        let graphics = slides
            .iter()
            .map(|s| &s.content)
            .flat_map(|c| texparse::get_graphics(c))
            .map(|s| s.to_string())
            .collect();
        (graphics_paths, graphics)
    }

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
