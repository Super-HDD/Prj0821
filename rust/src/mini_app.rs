

use godot::prelude::*;
use godot::engine::{TextureButton,TextureButtonVirtual};

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct MiniApp {
    //app_label:Label,
    #[base]
    base: Base<TextureButton>,
}

#[godot_api]
impl TextureButtonVirtual for MiniApp{
    fn init(base: Base<TextureButton>) -> Self {
        godot_print!("MiniApp loaded");
        Self {     
            base
        }
    }

}

