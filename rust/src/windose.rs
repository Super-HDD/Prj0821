use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, TextureButton, Label};

use crate::work_window::WorkWindow;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Windose{
    is_working:bool,
    work_finish_time:String,
    work_icon:Option<Gd<TextureButton>>,
    clock_label:Option<Gd<Label>>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for Windose{
    fn init(base:Base<Node2D>)->Self{
        Self { 
            is_working: false, 
            work_finish_time:String::from("12:00"),
            work_icon: None, 
            clock_label: None, 
            base 
        }
    }

    fn ready(&mut self){
        self.work_icon=Some(self.base.get_node_as("WorkIcon"));
        self.clock_label=Some(self.base.get_node_as("ClockLabel"));

        self.work_icon.as_mut().unwrap().connect("pressed".into(),self.base.callable("on_work_icon_pressed"));
    }
}

#[godot_api]
impl Windose{
    #[func]
    fn on_work_icon_pressed(&mut self){
        let work_window_scene:Gd<PackedScene>=load("res://Scene/WorkingDesk/SubScene/WorkWindow.tscn");
        self.base.add_child(work_window_scene.instantiate_as::<WorkWindow>().upcast());
        let mut work_window=self.base.get_node_as::<WorkWindow>("WorkWindow");
        work_window.set_position(Vector2::new(7.895,-4.495));
        work_window.connect("work_finished".into(),self.base.callable("on_work_finished"));
        self.is_working=true;
    }

    #[func]
    fn on_work_finished(&mut self){
        self.clock_label.as_mut().unwrap().set_text(self.work_finish_time.as_str().into());
        self.is_working=false;
    }

}