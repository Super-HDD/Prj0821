use godot::prelude::*;
use godot::engine::{TextureButtonVirtual, TextureButton, Control};
#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct PetEditor {
    //app_label:Label,
    #[base]
    base: Base<TextureButton>,
}

#[godot_api]
impl TextureButtonVirtual for PetEditor{
    fn init(base: Base<TextureButton>) -> Self {
        godot_print!("PetEditor loaded");
        Self {     
            base
        }
    }

    fn pressed(&mut self){
        let mut pet_editor:Gd<Control>=self.base.get_node_as("../PetEditor");
        pet_editor.set_visible(true);
    }

}