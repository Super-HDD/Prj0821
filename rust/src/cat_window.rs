use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, TextureButton};
use crate::cyber_cat::CyberCat;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct CatWindow{

    close_btn:Gd<TextureButton>,
    cat:Option<Gd<CyberCat>>,

    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for CatWindow{
    fn init(base:Base<Node2D>)->Self{

        Self { 
            
            close_btn:TextureButton::new_alloc(),
            cat:None,
            base
         }
    }

    fn ready(&mut self){
        //加载子节点
        self.cat=Some(self.base.get_node_as("Content/Cat"));
        self.close_btn=self.base.get_node_as("CloseButton");

        //配置子节点
        self.close_btn.connect("pressed".into(),self.base.callable("on_close_btn_pressed"));
    }
}

#[godot_api]
impl CatWindow{
    #[func]
    fn on_close_btn_pressed(&mut self){
        self.base.emit_signal("window_closed".into(),&[]);
        self.base.queue_free();
    }

    #[signal]
    fn window_closed();
}