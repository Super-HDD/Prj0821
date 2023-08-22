use godot::prelude::*;
use godot::engine::{TextureButton,TextureButtonVirtual,Label, Control};

use crate::game_manager::{GameManager, CURRENT_MENU_IDX, WindoseMenuIdx};

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct PetButton {
    //app_label:Label,
    
    #[base]
    base: Base<TextureButton>,
}

#[godot_api]
impl TextureButtonVirtual for PetButton{
    
    fn init(base:Base<TextureButton>)->Self{
        godot_print!("pet loaded");
        Self { 
            base
        }
    }

    fn pressed(&mut self){

        unsafe{
            CURRENT_MENU_IDX=WindoseMenuIdx::PetMenu;
        }

        let mut windose_main_menu:Gd<Control>=self.base.get_node_as("../");
        //let mut windose_main_menu:Gd<Control>=self.base.get_parent().unwrap().get_parent().unwrap().get_parent().unwrap().get_node_as("WindoseMainMenu");
        windose_main_menu.set_visible(false);
        
    }
}
