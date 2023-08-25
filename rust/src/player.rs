use godot::prelude::*;
use godot::engine::{CharacterBody2D,CharacterBody2DVirtual};

use crate::speech_bubble::SpeechBubble;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct OfficePlayerHuman{

    speech_bubble_scene:Gd<PackedScene>,
    speed:real,
    #[base]
    base:Base<CharacterBody2D>
}

#[godot_api]
impl CharacterBody2DVirtual for OfficePlayerHuman{
    fn init(base:Base<CharacterBody2D>)->Self{
        Self { 
            speech_bubble_scene: PackedScene::new(), 
            speed:200.0,
            base
        }
    }

    fn ready(&mut self){
        self.speech_bubble_scene=load("res://Scene/PopupDialog/SpeechBubble.tscn");
    }

    fn physics_process(&mut self,delta:f64){
        let velocity=self.move_detection();
        self.base.move_and_collide(velocity*delta as f32);
    }

}

#[godot_api]
impl OfficePlayerHuman{
    pub fn speak(&mut self,text:GodotString){
        self.base.add_child(self.speech_bubble_scene.instantiate_as::<SpeechBubble>().upcast());
        let mut bubble=self.base.get_node_as::<SpeechBubble>("SpeechBubble");
        bubble.set_name("BindedBubble".into());
        //bubble.bind_mut().popup(text, position, wait_time)
    }
}

impl OfficePlayerHuman{
    fn move_detection(&mut self)->Vector2{
        //当前速度
        let mut velocity=Vector2::new(0.0,0.0);

        //输入信号
        let input=Input::singleton();

        if input.is_action_pressed("move_up".into()){
            velocity.y=-1.0*self.speed;
        }else if input.is_action_pressed("move_down".into()){
            velocity.y=self.speed;
        }else{
            velocity.y=0.0;
        }

        if input.is_action_pressed("move_left".into()){
            velocity.x=-1.0*self.speed;
        }else if input.is_action_pressed("move_right".into()){
            velocity.x=self.speed;
        }else{
            velocity.x=0.0;
        }

        return velocity;
    }
}