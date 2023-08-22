use godot::engine::{TextureButton,TextureButtonVirtual, AcceptDialog};
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct Daka {
    is_already_clockin:bool,
    #[base]
    base: Base<TextureButton>,
}

#[godot_api]
impl TextureButtonVirtual for Daka{
    fn init(base: Base<TextureButton>) -> Self {
        

        Self {
            is_already_clockin:false,
            base
        }
    }

    fn pressed(&mut self){
        let mut daka_dialog:Gd<AcceptDialog>=self.base.get_node_as("daka");
        if self.is_already_clockin==false {
            daka_dialog.set_text("打卡成功".into());
            self.is_already_clockin=true;
        }else{
            daka_dialog.set_text("已打卡".into());
        }
        daka_dialog.popup_centered(); 
    }


}

// #[godot_api]
// impl Daka{
//     #[func]
//     fn on_Daka_button_pressed(&mut self){
//         godot_print!("daka_detected");
//     }
// }