pub mod components;
pub mod config;

use adw::glib::Propagation;
use relm4::{
    prelude::*,
    gtk::prelude::*,
    adw::prelude::*,
    main_adw_application,
    new_action_group,
    new_stateless_action,
    ComponentBuilder,
    actions::{RelmAction, RelmActionGroup, ActionGroupName}, factory::AsyncFactoryVecDeque
};
use relm4_icons::icon_name;


use crate::{
    app::{components::about_dialog::AboutDialog, config::info::{PROFILE, APP_NAME}},
    fl,
};

use self::components::{app_header::AppHeaderModel, terminal_tab::TerminalTabModel};

const HEADER_BAR_REVEALER_DURATION_MS: u32 = 250;


pub struct Terms {
    app_header_controller: AsyncController<AppHeaderModel>,
    tab_factory: AsyncFactoryVecDeque<TerminalTabModel>,
    // services_sidebar_controller: AsyncController<ServicesSidebarModel>,
    // task_list_sidebar_controller: AsyncController<TaskListSidebarModel>,
    // content_controller: AsyncController<ContentModel>,
    about_dialog: Controller<AboutDialog>,
}


new_action_group!(pub(super) WindowActionGroup, "win");
new_stateless_action!(AboutAction, WindowActionGroup, "about");
new_stateless_action!(QuitAction, WindowActionGroup, "quit");


#[derive(Debug)]
pub enum AppInput {
    NewSession,
    Quit,
}

#[relm4::component(pub async)]
impl AsyncComponent for Terms {
    type CommandOutput = ();
    type Input = AppInput;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_widget_name: "terms-main-window",
            set_size_request: (200, 300),
            connect_close_request[sender] => move |_| {
                sender.input(AppInput::Quit);
                Propagation::Stop
            },

            // #[wrap(Some)]
            // set_help_overlay: shortcuts = &gtk::Builder::from_resource(
            // 		"/com/github/vhdirk/Terms/ui/gtk/help-overlay.ui"
            // ).object::<gtk::ShortcutsWindow>("help_overlay").unwrap() -> gtk::ShortcutsWindow {
            // 	set_transient_for: Some(&root),
            // 	set_application: Some(&main_adw_application()),
            // },

            add_css_class?: if PROFILE == "Devel" {
                Some("devel")
            } else {
                None
            },

            gtk::Overlay {
                #[name = "layout_box"]
                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,

                    #[name = "header_revealer"]
                    gtk::Revealer {
                        set_transition_duration: HEADER_BAR_REVEALER_DURATION_MS,
                        set_valign: gtk::Align::Start,

                        // TODO
                        set_reveal_child: true,

                    },

                    #[name = "tab_view"]
                    adw::TabView {
                        set_shortcuts: adw::TabViewShortcuts::NONE,
                        
                    }


                }
            },
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let app = main_adw_application();
        let captured_sender = sender.clone();
        // app.connect_open(move |_, files, _| {
        // 	let bytes = files[0].uri();
        // 	let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
        // 	let captured_sender = captured_sender.clone();
        // 	relm4::tokio::spawn(async move {
        // 		let response = Service::Microsoft
        // 			.get_service()
        // 			.handle_uri_params(uri)
        // 			.await;
        // 		match response {
        // 			Ok(_) => {
        // 				captured_sender.input(AppInput::ReloadSidebar(Service::Microsoft));
        // 				tracing::info!("Token stored");
        // 			},
        // 			Err(err) => tracing::error!("An error ocurred: {}", err),
        // 		}
        // 	});
        // });

        let about_dialog = ComponentBuilder::default()
            .launch(root.upcast_ref::<gtk::Window>().clone())
            .detach();

        let widgets = view_output!();


        let mut model = Terms {
            app_header_controller: AppHeaderModel::builder()
                .launch(())
                .forward(sender.input_sender(), |message| match message {
                    // ServicesSidebarOutput::ServiceSelected(service) => {
                    // 	AppInput::ServiceSelected(service)
                    // },
                    // ServicesSidebarOutput::ServiceDisabled(service) => {
                    // 	AppInput::ServiceDisabled(service)
                    // },
                }),
            tab_factory: AsyncFactoryVecDeque::builder()
                .launch(widgets.tab_view.clone())
                .forward(sender.input_sender(), |message| match message {
     
                }),
            // 	task_list_sidebar_controller: TaskListSidebarModel::builder()
            // 		.launch(Service::Computer)
            // 		.forward(sender.input_sender(), |message| match message {
            // 			TaskListSidebarOutput::SelectList(list, service) => {
            // 				AppInput::ListSelected(list, service)
            // 			},
            // 			TaskListSidebarOutput::CleanContent => AppInput::CleanContent,
            // 		}),
            // 	content_controller: ContentModel::builder().launch(None).detach(),
            about_dialog,
        };

        // match setup::init_services() {
            // 	Ok(_) => (),
            // 	Err(_) => model.startup_failed = true,
            // };

        widgets.header_revealer.set_child(Some(model.app_header_controller.widget()));
        //     model.app_header_controller.widget()

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        // let shortcuts_action = {
        // 	let shortcuts = widgets.shortcuts.clone();
        // 	RelmAction::<ShortcutsAction>::new_stateless(move |_| {
        // 		shortcuts.present();
        // 	})
        // };



        // init the first terminal
        model.tab_factory.guard().push_back(());



        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap_or_default();
            })
        };

        let quit_action = {
            let sender = sender.clone();
            RelmAction::<QuitAction>::new_stateless(move |_| {
                sender
                    .input_sender()
                    .send(Self::Input::Quit)
                    .unwrap_or_default();
            })
        };

        actions.add_action(about_action);
        actions.add_action(quit_action);

        root.insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInput::NewSession => (),
            AppInput::Quit => main_adw_application().quit(),
        }
    }
}


impl Terms {
    pub fn new_tab(&mut self) {

        

        // var tab = new TerminalTab (this, this.tab_view.n_pages + 1, command, cwd);
        // var page = this.tab_view.add_page (tab, null);

        // tab.bind_property ("title",
        //                 page,
        //                 "title",
        //                 GLib.BindingFlags.SYNC_CREATE,
        //                 null,
        //                 null);

        // tab.close_request.connect ((_tab) => {
        // var _page = this.tab_view.get_page (_tab);
        // if (_page != null) {
        //     this.tab_view.close_page (_page);
        // }
        // });

        // this.tab_view.set_selected_page (page);
    }

}
