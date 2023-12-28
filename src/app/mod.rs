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
    actions::{RelmAction, RelmActionGroup, ActionGroupName}, factory::{AsyncFactoryVecDeque, FactoryVecDeque}, main_application, tokio::sync::oneshot
};
use relm4_icons::icon_name;

use crate::{
    app::{components::about_dialog::AboutDialog, config::info::{PROFILE, APP_NAME}},
    fl,
};

use self::components::{app_header::AppHeaderModel, session::SessionModel, window::Window};

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
    windows: AsyncFactoryVecDeque<Window>,
}

#[derive(Debug)]
pub enum TermsMsg {
    Activate(TermsArgs),
}

#[relm4::component(pub)]
impl SimpleComponent for Terms {
    type Init = TermsArgs;
    type Input = TermsMsg;
    type Output = ();

    view! {
        #[root]
        main_adw_application() -> adw::Application {
            connect_activate => TermsMsg::Activate(init.clone())
        }
    }
    // Initialize the component.
    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let model = Terms {
            windows: AsyncFactoryVecDeque::builder()
                .launch(root.clone())
                .detach()
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let mut windows_guard = self.windows.guard();

        match msg {
            TermsMsg::Activate(init) => {
                windows_guard.push_back(init);
            }
        }
    }
}

