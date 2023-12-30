use adw::subclass::prelude::*;
use glib::{clone, closure_local, RustClosure};
use gtk::prelude::*;
use gtk::{gio, glib};
use panel::subclass::prelude::*;
use std::cell::RefCell;

use super::*;

// var builder = new Gtk.Builder.from_resource ("/com/raggesilver/BlackBox/gtk/tab-menu.ui");
// this.tab_view.menu_model = builder.get_object ("tab-menu") as GLib.Menu;

// this.layout_box.append (this.header_bar_revealer);
// this.layout_box.append (this.tab_view);

// this.overlay = new Gtk.Overlay ();
// this.overlay.child = this.layout_box;

// this.content = this.overlay;

// this.set_name ("blackbox-main-window");

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Terms/gtk/window.ui")]
pub struct Window {
    pub init_args: RefCell<TerminalInitArgs>,

    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TermsWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
// impl WorkspaceImpl for Window {}

impl Window {
    fn setup_widgets(&self) {
        let session = Session::new(self.init_args.borrow().clone());
        self.tab_view.append(&session);

        session.connect_close(clone!(@weak self as this => move |session: &Session| {
                                this.tab_view.close_page(&this.tab_view.page(session));

                                if this.tab_view.n_pages() == 0 {
                                        this.obj().close();
                                }
        }));

        self.connect_signals();
    }

    fn connect_signals(&self) {}

    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args;
    }
}

// use super::{app_header::AppHeaderModel, session::SessionModel};

// pub struct Window {
//     app_header_controller: AsyncController<AppHeaderModel>,
//     session_factory: AsyncFactoryVecDeque<SessionModel>,
//     // services_sidebar_controller: AsyncController<ServicesSidebarModel>,
//     // task_list_sidebar_controller: AsyncController<TaskListSidebarModel>,
//     // content_controller: AsyncController<ContentModel>,
//     // about_dialog: Controller<AboutDialog>,
// }

// new_action_group!(pub(crate) WindowActionGroup, "win");
// new_stateless_action!(pub(crate) AboutAction, WindowActionGroup, "about");
// new_stateless_action!(pub(crate) QuitAction, WindowActionGroup, "quit");

// #[derive(Debug)]
// pub enum WindowInput {
//     NewSession,
//     Quit,
// }

// #[relm4::factory(pub async)]
// impl AsyncFactoryComponent for Window {
//     type Init = TermsArgs;
//     type Input = WindowInput;
//     type Output = ();
//     type CommandOutput = ();
//     type ParentWidget = adw::Application;

//     view! {
//         #[root]
//         adw::ApplicationWindow {
//             set_widget_name: "Window-main-window",
//             set_size_request: (200, 300),
//             // connect_close_request[sender] => move |_| {
//             //     sender.input(WindowInput::Quit);
//             //     Propagation::Stop
//             // },

//             // #[wrap(Some)]
//             // set_help_overlay: shortcuts = &gtk::Builder::from_resource(
//             // 		"/com/github/vhdirk/Window/ui/gtk/help-overlay.ui"
//             // ).object::<gtk::ShortcutsWindow>("help_overlay").unwrap() -> gtk::ShortcutsWindow {
//             // 	set_transient_for: Some(&root),
//             // 	set_application: Some(&main_adw_application()),
//             // },

//             add_css_class?: if PROFILE == "Devel" {
//                 Some("devel")
//             } else {
//                 None
//             },

//             gtk::Overlay {
//                 #[name = "layout_box"]
//                 gtk::Box{
//                     set_orientation: gtk::Orientation::Vertical,

//                     #[name = "header_revealer"]
//                     gtk::Revealer {
//                         set_transition_duration: HEADER_BAR_REVEALER_DURATION_MS,
//                         set_valign: gtk::Align::Start,

//                         // TODO
//                         set_reveal_child: true,

//                     },

//                     #[name = "tab_view"]
//                     adw::TabView {
//                         set_shortcuts: adw::TabViewShortcuts::NONE,

//                     }

//                 }
//             },
//         }
//     }

//     fn init_widgets(
//         &mut self,
//         index: &DynamicIndex,
//         root: &Self::Root,
//         returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
//         sender: AsyncFactorySender<Self>,
//     ) -> Self::Widgets {
//         let widgets = view_output!();

//         widgets.header_revealer.set_child(Some(self.app_header_controller.widget()));

//         // self.about_dialog.model().
//         widgets
//     }

//     // Initialize the component.
//     async fn init_model(init: Self::Init, _index: &DynamicIndex, sender: AsyncFactorySender<Self>) -> Self {

//         // let about_dialog = ComponentBuilder::default()
//         //     .launch(adw::ApplicationWindow::default())
//         //     .detach();

//         Self {
//             app_header_controller: AppHeaderModel::builder()
//                 .launch(())
//                 .forward(sender.input_sender(), |message| match message {
//                     // ServicesSidebarOutput::ServiceSelected(service) => {
//                     // 	WindowInput::ServiceSelected(service)
//                     // },
//                     // ServicesSidebarOutput::ServiceDisabled(service) => {
//                     // 	WindowInput::ServiceDisabled(service)
//                     // },
//                 }),
//             session_factory: AsyncFactoryVecDeque::builder()
//                 .launch(adw::TabView::default())
//                 .forward(sender.input_sender(), |message| match message {

//                 }),
//             // 	task_list_sidebar_controller: TaskListSidebarModel::builder()
//             // 		.launch(Service::Computer)
//             // 		.forward(sender.input_sender(), |message| match message {
//             // 			TaskListSidebarOutput::SelectList(list, service) => {
//             // 				WindowInput::ListSelected(list, service)
//             // 			},
//             // 			TaskListSidebarOutput::CleanContent => WindowInput::CleanContent,
//             // 		}),
//             // 	content_controller: ContentModel::builder().launch(None).detach(),
//             // about_dialog,
//         }
//     }

//     async fn update(&mut self, message: Self::Input, _sender: AsyncFactorySender<Self>) {
//         match message {
//             WindowInput::NewSession => (),
//             WindowInput::Quit => {
//                 // self.
//             }
//         }
//     }

//     // async fn init(
//     //     init: Self::Init,
//     //     root: Self::Root,
//     //     sender: AsyncComponentSender<Self>,
//     // ) -> AsyncComponentParts<Self> {

//     //     let about_dialog = ComponentBuilder::default()
//     //         .launch(root.upcast_ref::<gtk::Window>().clone())
//     //         .detach();

//     //     let widgets = view_output!();

//     //     let mut model = Window {
//     //         app_header_controller: AppHeaderModel::builder()
//     //             .launch(())
//     //             .forward(sender.input_sender(), |message| match message {
//     //                 // ServicesSidebarOutput::ServiceSelected(service) => {
//     //                 // 	WindowInput::ServiceSelected(service)
//     //                 // },
//     //                 // ServicesSidebarOutput::ServiceDisabled(service) => {
//     //                 // 	WindowInput::ServiceDisabled(service)
//     //                 // },
//     //             }),
//     //         session_factory: AsyncFactoryVecDeque::builder()
//     //             .launch(widgets.tab_view.clone())
//     //             .forward(sender.input_sender(), |message| match message {

//     //             }),
//     //         // 	task_list_sidebar_controller: TaskListSidebarModel::builder()
//     //         // 		.launch(Service::Computer)
//     //         // 		.forward(sender.input_sender(), |message| match message {
//     //         // 			TaskListSidebarOutput::SelectList(list, service) => {
//     //         // 				WindowInput::ListSelected(list, service)
//     //         // 			},
//     //         // 			TaskListSidebarOutput::CleanContent => WindowInput::CleanContent,
//     //         // 		}),
//     //         // 	content_controller: ContentModel::builder().launch(None).detach(),
//     //         about_dialog,
//     //     };

//     //     // match setup::init_services() {
//     //         // 	Ok(_) => (),
//     //         // 	Err(_) => model.startup_failed = true,
//     //         // };

//     //     widgets.header_revealer.set_child(Some(model.app_header_controller.widget()));
//     //     //     model.app_header_controller.widget()

//     //     let mut actions = RelmActionGroup::<WindowActionGroup>::new();

//     //     // let shortcuts_action = {
//     //     // 	let shortcuts = widgets.shortcuts.clone();
//     //     // 	RelmAction::<ShortcutsAction>::new_stateless(move |_| {
//     //     // 		shortcuts.present();
//     //     // 	})
//     //     // };

//     //     // init the first terminal
//     //     model.session_factory.guard().push_back(());

//     //     let about_action = {
//     //         let sender = model.about_dialog.sender().clone();
//     //         RelmAction::<AboutAction>::new_stateless(move |_| {
//     //             sender.send(()).unwrap_or_default();
//     //         })
//     //     };

//     //     let quit_action = {
//     //         let sender = sender.clone();
//     //         RelmAction::<QuitAction>::new_stateless(move |_| {
//     //             sender
//     //                 .input_sender()
//     //                 .send(Self::Input::Quit)
//     //                 .unwrap_or_default();
//     //         })
//     //     };

//     //     actions.add_action(about_action);
//     //     actions.add_action(quit_action);

//     //     root.insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

//     //     AsyncComponentParts { model, widgets }
//     // }

//     // async fn update(
//     //     &mut self,
//     //     message: Self::Input,
//     //     _sender: AsyncComponentSender<Self>,
//     //     _root: &Self::Root,
//     // ) {
//     //     match message {
//     //         WindowInput::NewSession => (),
//     //         WindowInput::Quit => main_adw_application().quit(),
//     //     }
//     // }
// }
