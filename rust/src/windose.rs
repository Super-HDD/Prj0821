use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, TextureButton, Label,Engine};
use time::Time;
use time::macros::time;

use crate::game_manager::GameManager;
use crate::work_window::WorkWindow;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Windose{
    is_working:bool,
    has_worked:bool,
    work_finish_time:Time,
    work_start_time:Time,
    work_icon:Option<Gd<TextureButton>>,
    _cyber_pet_icon:Option<Gd<TextureButton>>,
    _daka_icon:Option<Gd<TextureButton>>,
    clock_label:Option<Gd<Label>>,
    //game_manager:Option<Gd<GameManager>>,
    //game_manager_ref:Option<GdMut<'a,GameManager>>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for Windose{
    fn init(base:Base<Node2D>)->Self{
        Self { 
            is_working: false, 
            has_worked:false,
            work_finish_time:time!(9:00),
            work_start_time:time!(0:00),
            work_icon: None,
            _cyber_pet_icon:None, 
            _daka_icon:None,
            //game_manager:None,
            clock_label: None, 
            //game_manager_ref:None,
            base 
        }
    }

    fn ready(&mut self){
        self.work_icon=Some(self.base.get_node_as("WorkIcon"));
        self.clock_label=Some(self.base.get_node_as("ClockLabel"));
        let mut game_manager=self.base.get_tree().unwrap().get_root().unwrap().get_node_as::<GameManager>("MyGameManager");
        game_manager.connect("time_changed".into(), self.base.callable("on_time_changed"));
        self.work_icon.as_mut().unwrap().connect("pressed".into(),self.base.callable("on_work_icon_pressed"));
        //self.game_manager_ref=Some(self.game_manager.as_mut().unwrap().bind_mut());

        self.clock_label.as_mut().unwrap().set_text(game_manager.bind_mut().get_time_str().try_to::<GodotString>().unwrap());
    }

    // fn process(&mut self,_delta:f64){
    //     let input=Input::singleton();
    //     let mut scene_tree=self.base.get_tree().unwrap();
    //     if self.has_worked && input.is_action_just_pressed(StringName::from("left_desk")){
    //         godot_print!("has_singleton:{}",Engine::singleton().has_singleton("GameManager".into()));
    //         self.base.emit_signal("left_desk".into(), &[]);
    //     }
    // }
}

#[godot_api]
impl Windose{
    #[func]
    pub fn set_work_finish_time(&mut self,hour:u8,minute:u8){
        self.work_finish_time=Time::from_hms(hour, minute, 0).unwrap();
        godot_print!("Windose:work_finished time changed to{}",self.work_finish_time);
    }

    #[func]
    fn on_work_icon_pressed(&mut self){
        if self.has_worked==false{
            let work_window_scene:Gd<PackedScene>=load("res://Scene/DeskScene/SubScene/WorkWindow.tscn");
            let game_manager=self.base.get_tree().unwrap().get_root().unwrap().get_node_as::<GameManager>("MyGameManager");
            self.base.add_child(work_window_scene.instantiate_as::<WorkWindow>().upcast());
            let mut work_window=self.base.get_node_as::<WorkWindow>("WorkWindow");
            work_window.set_position(Vector2::new(7.895,-4.495));
            work_window.set_z_index(3);
            work_window.connect("work_finished".into(),self.base.callable("on_work_finished"));
            work_window.connect("work_finished".into(),game_manager.callable("on_desk_work_finished"));
            self.is_working=true;
        }
    }

    #[func]
    fn on_work_finished(&mut self){
        godot_print!("work finished");
        self.is_working=false;
        self.has_worked=true;
    }

    #[func]
    pub fn on_time_changed(&mut self,hour:u8,minute:u8){
        godot_print!("on_time_changed{}:{}",hour,minute);
        self.clock_label.as_mut().unwrap().set_text(GodotString::from(format!("{}:{:02}",hour,minute)));
    }

    #[signal]
    fn left_desk();

}

