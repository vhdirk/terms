pub mod components;
pub mod config;
mod util;

use std::{cell::RefCell, sync::{Arc, Mutex}, path::PathBuf};

use adw::glib::Propagation;
use callback_future::CallbackFuture;
use clap::Parser;
use relm4::{
    prelude::*,
    gtk::{prelude::*, glib::{self, clone}},
    adw::prelude::*,
    main_adw_application,
    new_action_group,
    new_stateless_action,
    ComponentBuilder,
    actions::{RelmAction, RelmActionGroup, ActionGroupName}, factory::AsyncFactoryVecDeque, main_application, tokio::sync::oneshot
};
use relm4_icons::icon_name;

use crate::{
    app::{components::about_dialog::AboutDialog, config::info::{PROFILE, APP_NAME}},
    fl,
};

use self::components::{app_header::AppHeaderModel, session::SessionModel};

const HEADER_BAR_REVEALER_DURATION_MS: u32 = 250;

#[derive(Parser, Clone, Debug, Default, PartialEq)]
#[command(author, version, about)]
pub struct TermsArgs {
	#[clap(short='w', long = "working-directory", env = "CWD", value_name = "CWD", help=fl!("cli-working-directory"))]
	pub working_directory: Option<PathBuf>,

	#[clap(short='c', long = "command", env = "CMD", value_name = "CMD", help=fl!("cli-command"))]
	pub command: Option<String>,
}


pub struct Terms {
    app_header_controller: AsyncController<AppHeaderModel>,
    session_factory: AsyncFactoryVecDeque<SessionModel>,
    // services_sidebar_controller: AsyncController<ServicesSidebarModel>,
    // task_list_sidebar_controller: AsyncController<TaskListSidebarModel>,
    // content_controller: AsyncController<ContentModel>,
    about_dialog: Controller<AboutDialog>,
}


new_action_group!(pub(super) WindowActionGroup, "win");
new_stateless_action!(AboutAction, WindowActionGroup, "about");
new_stateless_action!(QuitAction, WindowActionGroup, "quit");


#[derive(Debug)]
pub enum TermsInput {
    NewWindow,
    NewSession,
    Quit,
}

// impl Terms {
//     pub async fn parse_cli_args() -> anyhow::Result<gtk::gio::ApplicationCommandLine>{
//         let (tx, receiver) = oneshot::channel();
//         let sender = RefCell::new(tx);
//         let app = main_application();

//         app.connect_command_line(move |gapp, cmd: &gtk::gio::ApplicationCommandLine| {
            
//             let args: glib::VariantDict = cmd.options_dict();
//             println!("{}", args);


//             0
//         });

//         match receiver.await {
//             Ok(cmd) => Ok(cmd),
//             Err(err) => Err(anyhow::Error::new(err))
//         }
//     }
// }

#[relm4::component(pub async)]
impl AsyncComponent for Terms {
    type CommandOutput = ();
    type Input = TermsInput;
    type Output = ();
    type Init = TermsArgs;

    view! {

    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {


        let about_dialog = ComponentBuilder::default()
            .launch(root.upcast_ref::<gtk::Window>().clone())
            .detach();

        let widgets = view_output!();


        let mut model = Terms {
            app_header_controller: AppHeaderModel::builder()
                .launch(())
                .forward(sender.input_sender(), |message| match message {
                    // ServicesSidebarOutput::ServiceSelected(service) => {
                    // 	TermsInput::ServiceSelected(service)
                    // },
                    // ServicesSidebarOutput::ServiceDisabled(service) => {
                    // 	TermsInput::ServiceDisabled(service)
                    // },
                }),
            session_factory: AsyncFactoryVecDeque::builder()
                .launch(widgets.tab_view.clone())
                .forward(sender.input_sender(), |message| match message {
     
                }),
            // 	task_list_sidebar_controller: TaskListSidebarModel::builder()
            // 		.launch(Service::Computer)
            // 		.forward(sender.input_sender(), |message| match message {
            // 			TaskListSidebarOutput::SelectList(list, service) => {
            // 				TermsInput::ListSelected(list, service)
            // 			},
            // 			TaskListSidebarOutput::CleanContent => TermsInput::CleanContent,
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
        model.session_factory.guard().push_back(());



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
            TermsInput::NewSession => (),
            TermsInput::Quit => main_adw_application().quit(),
        }
    }
}


impl Terms {
    pub fn new_tab(&mut self) {

        

        // var tab = new Session (this, this.tab_view.n_pages + 1, command, cwd);
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
