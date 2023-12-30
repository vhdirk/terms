use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use panel::subclass::prelude::*;

use super::terminal::TerminalInitArgs;

mod imp {
    use std::cell::RefCell;

    use glib::{clone, closure_local, RustClosure};

    use crate::components::{
        header_bar::HeaderBar,
        terminal::{Terminal, TerminalInitArgs},
        terminal_panel::TerminalPanel,
    };

    use super::*;

    // var builder = new Gtk.Builder.from_resource ("/com/raggesilver/BlackBox/gtk/tab-menu.ui");
    // this.tab_view.menu_model = builder.get_object ("tab-menu") as GLib.Menu;

    // this.layout_box.append (this.header_bar_revealer);
    // this.layout_box.append (this.tab_view);

    // this.overlay = new Gtk.Overlay ();
    // this.overlay.child = this.layout_box;

    // this.content = this.overlay;

    // this.set_name ("blackbox-main-Workspace");

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Terms/gtk/workspace.ui")]
    pub struct Workspace {
        pub init_args: RefCell<TerminalInitArgs>,

        #[template_child]
        pub header_bar: TemplateChild<HeaderBar>,

        #[template_child]
        pub tab_view: TemplateChild<adw::TabView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Workspace {
        const NAME: &'static str = "TermsWorkspace";
        type Type = super::Workspace;
        type ParentType = panel::Workspace;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Workspace {
        fn constructed(&self) {
            self.parent_constructed();

            self.setup_widgets();
        }
    }

    impl WidgetImpl for Workspace {}
    impl WindowImpl for Workspace {}
    impl ApplicationWindowImpl for Workspace {}
    impl AdwApplicationWindowImpl for Workspace {}
    impl WorkspaceImpl for Workspace {}

    impl Workspace {
        fn setup_widgets(&self) {
            let panel = TerminalPanel::new(self.init_args.borrow().clone());
            self.tab_view.append(&panel);

            panel.connect_exit(clone!(@weak self as this => move |panel: &TerminalPanel| {
                                    this.tab_view.close_page(&this.tab_view.page(panel));

                                    if this.tab_view.n_pages() == 0 {
                                                    this.obj().close();
                                    }
            }));

            // panel.connect_closure(
            // 	"exit",
            // 	false,
            // 	RustClosure::new_local(clone!(@weak self as this, move |_terminal: TerminalPanel| {

            // 	})),
            // );

            self.connect_signals();
        }

        fn connect_signals(&self) {}

        pub fn set_init_args(&self, init_args: TerminalInitArgs) {
            let mut args = self.init_args.borrow_mut();
            *args = init_args;
        }
    }
}

glib::wrapper! {
        pub struct Workspace(ObjectSubclass<imp::Workspace>)
                @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow, panel::Workspace,
                @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Workspace {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P, init_args: TerminalInitArgs) -> Self {
        let this: Self = glib::Object::builder().property("application", application).build();
        this.imp().set_init_args(init_args);

        this
    }
}

// use super::{app_header::AppHeaderModel, session::SessionModel};

// pub struct Workspace {
//     app_header_controller: AsyncController<AppHeaderModel>,
//     session_factory: AsyncFactoryVecDeque<SessionModel>,
//     // services_sidebar_controller: AsyncController<ServicesSidebarModel>,
//     // task_list_sidebar_controller: AsyncController<TaskListSidebarModel>,
//     // content_controller: AsyncController<ContentModel>,
//     // about_dialog: Controller<AboutDialog>,
// }

// new_action_group!(pub(crate) WorkspaceActionGroup, "win");
// new_stateless_action!(pub(crate) AboutAction, WorkspaceActionGroup, "about");
// new_stateless_action!(pub(crate) QuitAction, WorkspaceActionGroup, "quit");

// #[derive(Debug)]
// pub enum WorkspaceInput {
//     NewSession,
//     Quit,
// }

// #[relm4::factory(pub async)]
// impl AsyncFactoryComponent for Workspace {
//     type Init = TermsArgs;
//     type Input = WorkspaceInput;
//     type Output = ();
//     type CommandOutput = ();
//     type ParentWidget = adw::Application;

//     view! {
//         #[root]
//         adw::ApplicationWorkspace {
//             set_widget_name: "Workspace-main-Workspace",
//             set_size_request: (200, 300),
//             // connect_close_request[sender] => move |_| {
//             //     sender.input(WorkspaceInput::Quit);
//             //     Propagation::Stop
//             // },

//             // #[wrap(Some)]
//             // set_help_overlay: shortcuts = &gtk::Builder::from_resource(
//             // 		"/com/github/vhdirk/Workspace/ui/gtk/help-overlay.ui"
//             // ).object::<gtk::ShortcutsWorkspace>("help_overlay").unwrap() -> gtk::ShortcutsWorkspace {
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
//         //     .launch(adw::ApplicationWorkspace::default())
//         //     .detach();

//         Self {
//             app_header_controller: AppHeaderModel::builder()
//                 .launch(())
//                 .forward(sender.input_sender(), |message| match message {
//                     // ServicesSidebarOutput::ServiceSelected(service) => {
//                     // 	WorkspaceInput::ServiceSelected(service)
//                     // },
//                     // ServicesSidebarOutput::ServiceDisabled(service) => {
//                     // 	WorkspaceInput::ServiceDisabled(service)
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
//             // 				WorkspaceInput::ListSelected(list, service)
//             // 			},
//             // 			TaskListSidebarOutput::CleanContent => WorkspaceInput::CleanContent,
//             // 		}),
//             // 	content_controller: ContentModel::builder().launch(None).detach(),
//             // about_dialog,
//         }
//     }

//     async fn update(&mut self, message: Self::Input, _sender: AsyncFactorySender<Self>) {
//         match message {
//             WorkspaceInput::NewSession => (),
//             WorkspaceInput::Quit => {
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
//     //         .launch(root.upcast_ref::<gtk::Workspace>().clone())
//     //         .detach();

//     //     let widgets = view_output!();

//     //     let mut model = Workspace {
//     //         app_header_controller: AppHeaderModel::builder()
//     //             .launch(())
//     //             .forward(sender.input_sender(), |message| match message {
//     //                 // ServicesSidebarOutput::ServiceSelected(service) => {
//     //                 // 	WorkspaceInput::ServiceSelected(service)
//     //                 // },
//     //                 // ServicesSidebarOutput::ServiceDisabled(service) => {
//     //                 // 	WorkspaceInput::ServiceDisabled(service)
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
//     //         // 				WorkspaceInput::ListSelected(list, service)
//     //         // 			},
//     //         // 			TaskListSidebarOutput::CleanContent => WorkspaceInput::CleanContent,
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

//     //     let mut actions = RelmActionGroup::<WorkspaceActionGroup>::new();

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

//     //     root.insert_action_group(WorkspaceActionGroup::NAME, Some(&actions.into_action_group()));

//     //     AsyncComponentParts { model, widgets }
//     // }

//     // async fn update(
//     //     &mut self,
//     //     message: Self::Input,
//     //     _sender: AsyncComponentSender<Self>,
//     //     _root: &Self::Root,
//     // ) {
//     //     match message {
//     //         WorkspaceInput::NewSession => (),
//     //         WorkspaceInput::Quit => main_adw_application().quit(),
//     //     }
//     // }
// }
