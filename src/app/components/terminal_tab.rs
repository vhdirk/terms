use relm4::{component::AsyncComponentParts, prelude::*, gtk::prelude::*, AsyncComponentSender, factory::AsyncFactoryComponent, AsyncFactorySender};

use super::terminal::TerminalModel;

#[derive(Debug)]
pub struct TerminalTabInput {}

#[derive(Debug)]
pub enum TerminalTabOutput {}

#[derive(Debug)]
pub struct TerminalTabModel {
    // terminal_controller: AsyncController<TerminalModel>,

}

#[relm4::factory(pub async)]
impl AsyncFactoryComponent for TerminalTabModel {
    type Input = TerminalTabInput;
    type Output = TerminalTabOutput;
    type Init = ();
    type CommandOutput = ();
    type ParentWidget = adw::TabView;
	// type Widgets = ListWidgets;

	view! {
        #[root]
        &gtk::Box {
            set_visible: true,
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            #[name(banner)]
            adw::Banner{             
                set_visible: true,
            },

            #[name(scrolled)]
            gtk::ScrolledWindow{
                set_visible: true,

                vte::Terminal {}
            }

        },

// //  <property name="orientation">vertical</property>
// //     <property name="spacing">0</property>

// //     <child>
// //       <object class="AdwBanner" id="banner">
// //         <property name="revealed">false</property>
// //       </object>
// //     </child>

// //     <child>
// //       <object class="GtkScrolledWindow" id="scrolled">
// //         <!-- <property name="child">
// //           <lookup name="terminal">TerminalTerminalTab</lookup>
// //         </property> -->
// //       </object>
// //     </child>

// //     <child>
// //       <object class="TerminalSearchToolbar" id="search_toolbar">
// //         <!-- <binding name="terminal">
// //           <lookup name="terminal">TerminalTerminalTab</lookup>
// //         </binding> -->

// //         <property name="terminal" bind-source="TerminalTerminalTab" bind-property="terminal" bind-flags="sync-create" />
// //       </object>
// //     </child>
// //   </template>


//         },
// 	}

        #[local_ref]
        returned_widget -> adw::TabPage {
            set_title: "some tab",
        }
    }

    async fn init_model(value: Self::Init, _index: &DynamicIndex, sender: AsyncFactorySender<Self>) -> Self {
        // let terminal_controller = TerminalModel::builder()
		// 	.launch(())
		// 	.forward(sender.input_sender(), |message| match message {
		// 	});
        Self {  }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncFactorySender<Self>) {
        // match msg {
        //     // CounterMsg::Increment => {
        //     //     self.value = self.value.wrapping_add(1);
        //     // }
        //     // CounterMsg::Decrement => {
        //     //     self.value = self.value.wrapping_sub(1);
        //     // }
        // }
    }
}


// #[derive(Debug)]
// pub struct TerminalTabModel {
//     terminal_controller: AsyncController<TerminalModel>,

// }

// #[derive(Debug)]
// pub struct TerminalTabInput {}

// #[derive(Debug)]
// pub enum TerminalTabOutput {}

// #[relm4::component(pub async)]
// impl SimpleAsyncComponent for TerminalTabModel {
// 	type Input = TerminalTabInput;
// 	type Output = TerminalTabOutput;
// 	type Init = ();


// 	async fn init(
// 		_init: Self::Init,
// 		root: Self::Root,
// 		sender: AsyncComponentSender<Self>,
// 	) -> AsyncComponentParts<Self> {
// 		let model = TerminalTabModel {
//             terminal_controller: TerminalModel::builder()
//                 .launch(())
//                 .forward(sender.input_sender(), |message| match message {
//                     // ServicesSidebarOutput::ServiceSelected(service) => {
//                     // 	AppInput::ServiceSelected(service)
//                     // },
//                     // ServicesSidebarOutput::ServiceDisabled(service) => {
//                     // 	AppInput::ServiceDisabled(service)
//                     // },
//                 }),
//         };

// 		let widgets = view_output!();

//         widgets.scrolled.set_child(Some(model.terminal_controller.widget()));

// 		AsyncComponentParts { model, widgets }
// 	}
// }
