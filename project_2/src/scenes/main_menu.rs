//! Main menu scene.
use std::{cell::RefCell, rc::Rc};

use builder::{config::Config, BuildFromCfg};
use error_stack::ResultExt;
use gui::{
    manager::{widget::Widget, Manager as GuiManager},
    widget::{Base, Builder as GuiBuilder, Button, Textbox},
};
use renderer::Drawable;
use scene::Scene;

/// Main menu scene.
pub struct MainMenu {
    /// Main menu GUI.
    gui: GuiManager,
    /// Request exit flag.
    request_exit_flag: Rc<RefCell<bool>>
}

impl Scene for MainMenu {
    fn handle_event(
        &mut self,
        e: scene::event::Event,
        state: &mut dyn scene::State,
    ) -> error_stack::Result<(), scene::Error> {
        self.gui.handle_event(e.clone()).change_context(scene::Error::msg("Gui failed"))?;
        if *self.request_exit_flag.borrow() {
            state.exit();
        }
        Ok(())
    }
}

impl Drawable for MainMenu {
    fn draw(&self, renderer: &mut dyn renderer::Renderer) {
        self.gui.draw(renderer);
    }
}

impl BuildFromCfg<Box<dyn Scene>> for MainMenu {
    fn build(
        mut cfg: Config,
        res: &mut dyn resources::Manager,
    ) -> error_stack::Result<Box<dyn Scene>, builder::Error> {
        let gui_cfg = cfg
            .take::<Config>("gui")
            .change_context(builder::Error::msg("Failed to build scene GUI"))?;
        let gui = GuiManager::new(&GuiBuilder::default(), res, gui_cfg)
            .change_context(builder::Error::msg("Failed to init GUI manager"))?;

        let main_menu = gui
            .get_by_id_cast::<Base>("main_menu")
            .change_context(builder::Error::msg("Widget \"main_menu\" not found"))?;

        let create_server = gui
            .get_by_id_cast::<Button>("create_server")
            .change_context(builder::Error::msg("Widget \"create_server\" not found"))?;
        let connect_to_server = gui
            .get_by_id_cast::<Button>("connect_to_server")
            .change_context(builder::Error::msg("Widget \"connect_to_server\" not found"))?;
        let settings = gui
            .get_by_id_cast::<Button>("settings")
            .change_context(builder::Error::msg("Widget \"settings\" not found"))?;
        let exit = gui
            .get_by_id_cast::<Button>("exit")
            .change_context(builder::Error::msg("Widget \"exit\" not found"))?;

        let request_exit_flag = Rc::new(RefCell::new(false));
        let request_exit_flag_c = request_exit_flag.clone();
        exit.borrow_mut().click_cb(move |_| {
            *request_exit_flag_c.borrow_mut() = true;
        });

        let create_server_menu = gui
            .get_by_id_cast::<Base>("create_server_menu")
            .change_context(builder::Error::msg("Widget \"create_server_menu\" not found"))?;
        let server_address = gui
            .get_by_id_cast::<Textbox>("server_address")
            .change_context(builder::Error::msg("Widget \"server_address\" not found"))?;
        let create = gui
            .get_by_id_cast::<Button>("create")
            .change_context(builder::Error::msg("Widget \"create\" not found"))?;
        let create_server_menu_back = gui
            .get_by_id_cast::<Button>("create_server_menu_back")
            .change_context(builder::Error::msg("Widget \"create\" not found"))?;

        create.borrow_mut().click_cb(move |_| {
            println!("Server created with addres: {}", server_address.borrow().get_text());
        });

        let connect_server_menu = gui
            .get_by_id_cast::<Base>("connect_server_menu")
            .change_context(builder::Error::msg("Widget \"connect_server_menu\" not found"))?;
        let connect_address = gui
            .get_by_id_cast::<Textbox>("connect_address")
            .change_context(builder::Error::msg("Widget \"connect_address\" not found"))?;
        let connect = gui
            .get_by_id_cast::<Button>("connect")
            .change_context(builder::Error::msg("Widget \"connect\" not found"))?;
        let connect_server_menu_back = gui
            .get_by_id_cast::<Button>("connect_server_menu_back")
            .change_context(builder::Error::msg("Widget \"connect_server_menu\" not found"))?;

        connect.borrow_mut().click_cb(move |_| {
            println!("Client connected to addres: {}", connect_address.borrow().get_text());
        });

        let settings_menu = gui
            .get_by_id_cast::<Base>("settings_menu")
            .change_context(builder::Error::msg("Widget \"settings_menu\" not found"))?;
        let settings_menu_back = gui
            .get_by_id_cast::<Button>("settings_menu_back")
            .change_context(builder::Error::msg("Widget \"settings_menu\" not found"))?;

        let main_menuc = main_menu.clone();
        let create_server_menuc = create_server_menu.clone();
        create_server.borrow_mut().click_cb(move |_| {
            main_menuc.borrow_mut().set_visible_flag(false);
            create_server_menuc.borrow_mut().set_visible_flag(true);
        });
        let main_menuc = main_menu.clone();
        create_server_menu_back.borrow_mut().click_cb(move |_| {
            main_menuc.borrow_mut().set_visible_flag(true);
            create_server_menu.borrow_mut().set_visible_flag(false);
        });

        let main_menuc = main_menu.clone();
        let connect_server_menuc = connect_server_menu.clone();
        connect_to_server.borrow_mut().click_cb(move |_| {
            main_menuc.borrow_mut().set_visible_flag(false);
            connect_server_menuc.borrow_mut().set_visible_flag(true);
        });
        let main_menuc = main_menu.clone();
        connect_server_menu_back.borrow_mut().click_cb(move |_| {
            main_menuc.borrow_mut().set_visible_flag(true);
            connect_server_menu.borrow_mut().set_visible_flag(false);
        });

        let main_menuc = main_menu.clone();
        let settings_menuc = settings_menu.clone();
        settings.borrow_mut().click_cb(move |_| {
            main_menuc.borrow_mut().set_visible_flag(false);
            settings_menuc.borrow_mut().set_visible_flag(true);
        });
        let main_menuc = main_menu.clone();
        settings_menu_back.borrow_mut().click_cb(move |_| {
            main_menuc.borrow_mut().set_visible_flag(true);
            settings_menu.borrow_mut().set_visible_flag(false);
        });

        Ok(Box::new(Self { gui, request_exit_flag }))
    }
}
