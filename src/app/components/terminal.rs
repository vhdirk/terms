use relm4::{component::AsyncComponentParts, prelude::*, AsyncComponentSender};

#[derive(Debug)]
pub struct TerminalModel {}

#[derive(Debug)]
pub struct TerminalInput {}

#[derive(Debug)]
pub enum TerminalOutput {}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for TerminalModel {
	type Input = TerminalInput;
	type Output = TerminalOutput;
	type Init = ();

	view! {
        vte::Terminal {

        },

	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = TerminalModel {};

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}
}
