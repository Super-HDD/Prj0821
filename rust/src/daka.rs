use godot::engine::{TextureButton,TextureButtonVirtual, Label,Control};
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct Daka {
    is_already_clockin:bool,
    #[base]
    base: Base<TextureButton>,
}

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct DakaCloseButton {
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
        let mut daka_dialog:Gd<Control>=self.base.get_node_as("DakaDialog");
        let mut daka_label:Gd<Label>=self.base.get_node_as("DakaDialog/Label");
        if self.is_already_clockin==false {
            let daka_text=format!("{}\n打卡成功",self.base.get_node_as::<Label>("../ClockLabel").get_text());
            daka_label.set_text(daka_text.as_str().into());
            self.is_already_clockin=true;
        }else{
            daka_label.set_text("已打卡".into());
        }
        daka_dialog.set_visible(true);
    }
}

#[godot_api]
impl TextureButtonVirtual for DakaCloseButton{
    fn init(base: Base<TextureButton>) -> Self {
        Self {
            base
        }
    }

    fn pressed(&mut self) {
        let mut daka_dialog:Gd<Control>=self.base.get_node_as("../");
        daka_dialog.set_visible(false);
    }
}

