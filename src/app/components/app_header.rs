use relm4::{
	adw,
	adw::prelude::*,
	component::{AsyncComponent, AsyncComponentParts},
	gtk,
	prelude::*,
	AsyncComponentSender, SimpleComponent,
};
use relm4_icons::icon_name;

use crate::{app::config::info::APP_NAME, fl};


#[derive(Debug)]
pub struct AppHeaderModel{
}


#[derive(Debug)]
pub struct AppHeaderInput {}


#[derive(Debug)]
pub enum AppHeaderOutput {}


#[relm4::component(pub async)]
impl SimpleAsyncComponent for AppHeaderModel {
	type Input = AppHeaderInput;
	type Output = AppHeaderOutput;
	type Init = ();

	view! {
        #[root]
        &adw::Bin {
            set_valign: gtk::Align::Start,
            set_vexpand: false,
            set_visible: true,

            set_css_classes: &["custom-headerbar", "flat"],

            gtk::WindowHandle {
                set_hexpand: true,
                gtk::Box {
                    #[name(hb_stack)]
                    gtk::Stack {
                        set_hexpand: true,
                        set_hhomogeneous: true,
                        set_vhomogeneous: true,
                        set_transition_type: gtk::StackTransitionType::None,

                        #[name(single_tab_content)]
                        gtk::CenterBox {
                            set_visible: true,
                            #[wrap(Some)]
                            set_start_widget = &gtk::WindowControls{
                                set_side: gtk::PackType::Start,
                            },

                            #[wrap(Some)]
                            set_center_widget = &adw::WindowTitle{
                                set_title: "Terms",
                            },

                            #[wrap(Some)]
                            set_end_widget = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 0,
                                set_valign: gtk::Align::Center,

                                //set_visible: -> single-tab-mode

                                #[name(new_tab_button)]
                                gtk::Button {
                                    set_tooltip: fl!("new-tab"),
                                    set_icon_name: icon_name::PLUS, //LIST_ADD_SYMBOLIC
                                    set_visible: true,
                                },

                                #[name(exit_fullscreen_button)]
                                gtk::Button {
                                    set_tooltip: fl!("exit-fullscreen"),
                                    // set_icon_name: icon_name::M, //VIEW_RESTORE_SYMBOLIC
                                    set_can_focus: false,

                                    // TODO
                                    set_visible: false,
                                },

                                #[name(menu_button)]
                                gtk::MenuButton {
                                    set_tooltip: fl!("menu"),
                                    set_icon_name: icon_name::MENU, //OPEN_MENU_SYMBOLIC
                                    set_can_focus: false,

                                    // TODO: show-menu-button
                                    set_visible: true,

                                    // #[wrap(Some)]
                                    // set_popover = gtk::PopoverMenu {

                                    // }
                                },
           //             <child>
                //               <object class="GtkMenuButton">
                //                 <property name="tooltip-text" translatable="yes">Menu</property>
                //                 <property name="icon-name">open-menu-symbolic</property>

                //                 <property name="popover">
                //                   <object class="GtkPopoverMenu">
                //                     <property name="menu-model">main-menu</property>
                //                     <child type="style-switcher">
                //                       <object class="TerminalStyleSwitcher" />
                //                     </child>
                //                   </object>
                //                 </property>



                                gtk::WindowControls{
                                    set_side: gtk::PackType::End,

                                    // TODO:
                                    set_visible: true
                                },

                                       //               <object class="GtkWindowControls">
                //                 <property name="side">end</property>

                //                 <binding name="visible">
                //                   <closure type="gboolean" function="show_window_controls">
                //                     <lookup name="fullscreened">
                //                       <lookup name="window">TerminalHeaderBar</lookup>
                //                     </lookup>
                //                     <lookup name="floating-mode">TerminalHeaderBar</lookup>
                //                     <lookup name="single-tab-mode">TerminalHeaderBar</lookup>
                //                     <constant type="gboolean">false</constant>
                //                   </closure>
                //                 </binding>
                //               </object>


                            },
                        },
                    }
                }
            }
			// css_name: "headerbar",
			// tab_bar: &adw::TabBar

        },

	}

	async fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = AppHeaderModel {

		};

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}
}
