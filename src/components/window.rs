use gtk::prelude::*;
use adw::subclass::prelude::*;
use panel::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Terms/gtk/window.ui")]
    pub struct Window {
        //Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
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

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
    // impl WorkspaceImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow, //, panel::Workspace,
        @implements gio::ActionGroup, gio::ActionMap; //, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}

// use std::{cell::RefCell, sync::{Arc, Mutex}, path::PathBuf};

// use adw::glib::Propagation;
// use callback_future::CallbackFuture;
// use clap::Parser;
// use relm4::{
//     prelude::*,
//     gtk::{prelude::*, glib::{self, clone}},
//     adw::prelude::*,
//     main_adw_application,
//     new_action_group,
//     new_stateless_action,
//     ComponentBuilder,
//     actions::{RelmAction, RelmActionGroup, ActionGroupName}, factory::{AsyncFactoryVecDeque, AsyncFactoryComponent, widgets, FactoryView}, main_application, tokio::sync::oneshot, AsyncFactorySender
// };
// use relm4_icons::icon_name;

// use crate::{
//     app::{components::about_dialog::AboutDialog, config::info::{PROFILE, APP_NAME}, TermsArgs},
//     fl,
// };

// use super::{app_header::AppHeaderModel, session::SessionModel};

// const HEADER_BAR_REVEALER_DURATION_MS: u32 = 250;

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

// // impl Window {
// //     pub async fn parse_cli_args() -> anyhow::Result<gtk::gio::ApplicationCommandLine>{
// //         let (tx, receiver) = oneshot::channel();
// //         let sender = RefCell::new(tx);
// //         let app = main_application();

// //         app.connect_command_line(move |gapp, cmd: &gtk::gio::ApplicationCommandLine| {

// //             let args: glib::VariantDict = cmd.options_dict();
// //             println!("{}", args);


// //             0
// //         });

// //         match receiver.await {
// //             Ok(cmd) => Ok(cmd),
// //             Err(err) => Err(anyhow::Error::new(err))
// //         }
// //     }
// // }

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
