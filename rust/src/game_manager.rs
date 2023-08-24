use godot::prelude::*;
use godot::engine::{Node,NodeVirtual, Engine};
use time::macros::time;
use time::{Time,Duration};

use crate::desk_scene::DeskScene;

pub enum TimeNode{
    NewDayBegin,
    MorningFirstWorkFinished,
    MidDayRest,
    MidDayWakeUpByAlarm,
    MidDayWakeUp,
    AfternoonFirstWorkFinished,
    AfternoonSecondWorkFinished,
    TimeToMeetBoss
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager{

    office_scene:Gd<PackedScene>,
    desk_scene:Gd<PackedScene>,
    scene_tree:Gd<SceneTree>,
    time:Time,
    next_time:Time,
    current_time_node:TimeNode,
    #[base]
    base:Base<Node>
}

#[godot_api]
impl NodeVirtual for GameManager{

    fn init(base:Base<Node>)->Self{

        Self {
            office_scene:PackedScene::new(),
            desk_scene:PackedScene::new(),
            current_time_node:TimeNode::NewDayBegin,
            scene_tree:SceneTree::new_alloc(),
            time:time!(7:59), 
            next_time:time!(9:00),
            base 
        }
    }

    fn ready(&mut self){
        self.office_scene=load("res://Scene/OfficeScene/Office.tscn");
        self.desk_scene=load("res://Scene/DeskScene/Desk.tscn");
        
        self.scene_tree=self.base.get_tree().unwrap();
        self.scene_tree.change_scene_to_packed(self.desk_scene.share());
        godot_print!("current scene{:?}",self.scene_tree.get_current_scene());
        //self.scene_tree.get_root().unwrap().get_node_as::<DeskScene>("DeskScene").bind_mut().set_desk_work_finish_time(9, 0);
        self.base.emit_signal("change_desk_work_finish_time".into(), &[{9}.to_variant(),{0}.to_variant()]);
        godot_print!("change work finish time signal emitted");
        //self.next_time=self.time+Duration::minutes(181);
        
    }

    // fn process(&mut self,_delta:f64){
    //     let input=Input::singleton();
    //     if input.is_action_just_pressed(StringName::from("left_desk")){
    //         self.scene_tree.change_scene_to_packed(self.office_scene.share());
    //     }
    // }
    
}

#[godot_api]
impl GameManager{

    #[signal]
    fn time_changed(hour:u8,minute:u8);

    #[func]
    pub fn get_time_str(&self)->Variant{
        return GodotString::from(format!("{}:{}",self.time.hour(),self.time.minute())).to_variant();
    }

    #[func]
    fn leave_desk(&mut self){
        match self.current_time_node{
            TimeNode::NewDayBegin=>{
                self.current_time_node=TimeNode::MorningFirstWorkFinished;
                self.scene_tree.change_scene_to_packed(self.office_scene.share());
                self.next_time=time!(9:30);
            }
            
            _=>todo!()
        }
    }

    #[func]
    pub fn on_desk_work_finished(&mut self){
        self.time=self.next_time;
        godot_print!("current time:{}",self.time);
        self.base.emit_signal("time_changed".into(), &[self.time.hour().to_variant(),self.time.minute().to_variant()]);
    }
}

impl GameManager{
    pub fn test(){
        godot_print!("ok");
    }
}

