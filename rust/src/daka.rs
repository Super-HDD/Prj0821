use godot::engine::{TextureButton,Node2D,Node2DVirtual,Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DakaWindow{

    label:Gd<Label>,
    close_btn:Gd<TextureButton>,
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for DakaWindow{
    fn init(base: Base<Node2D>) -> Self {
        Self {
            label:Label::new_alloc(),
            close_btn:TextureButton::new_alloc(),

            base
        }
    }

    fn ready(&mut self){
        self.label=self.base.get_node_as("Content/Label");
        self.close_btn=self.base.get_node_as("CloseButton");

        self.close_btn.connect("pressed".into(),self.base.callable("on_close_btn_pressed"));
        
    }
}

#[godot_api]
impl DakaWindow{
    #[func]
    fn on_close_btn_pressed(&mut self){
        self.base.emit_signal("window_closed".into(),&[]);
        self.base.queue_free();
    }

    #[signal]
    fn window_closed();
}


