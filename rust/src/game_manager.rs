use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual};

pub enum WindoseMenuIdx {
    AFK,
    MainMenu,
    PetMenu,
    PetScene,
    PetEditor,
}

pub static mut IS_ALREADY_CLOCKIN:bool=false;
pub static mut CURRENT_MENU_IDX:WindoseMenuIdx=WindoseMenuIdx::MainMenu;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameManager{


    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for GameManager{
    fn init(base:Base<Node2D>)-> GameManager{
        Self{
            base
        }
    }
}

