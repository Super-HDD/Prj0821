use godot::prelude::*;
use godot::engine::{TextureButton,TextureButtonVirtual, Control};


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


        let mut cyber_pet_window:Gd<Control>=self.base.get_node_as("../CyberPetWindow");
        //let mut windose_main_menu:Gd<Control>=self.base.get_parent().unwrap().get_parent().unwrap().get_parent().unwrap().get_node_as("WindoseMainMenu");
        cyber_pet_window.set_visible(true);
        
    }
}
