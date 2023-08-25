use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, Sprite2D, Texture2D, Timer};
use std::fs;
use rand::{self, thread_rng, Rng};
//use serde::{Deserialize, Serialize};

use crate::speech_bubble::SpeechBubble;



#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DeskCollegue{
    bumps_list:Vec<Vec<String>>,
    timer:Gd<Timer>,
    id:u32,
    sprite:Gd<Sprite2D>,
    bubble_scene:Gd<PackedScene>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for DeskCollegue{

    fn init(base:Base<Node2D>)->Self{
        Self {
            bumps_list:Vec::new(),
            timer:Timer::new_alloc(),
            id:0,
            bubble_scene:PackedScene::new(),
            sprite:Sprite2D::new_alloc(), 
            base
        }
    }

    fn ready(&mut self){
        self.sprite=self.base.get_node_as("Sprite2D");
        self.timer=self.base.get_node_as("Timer");
        self.bubble_scene=load("res://Scene/PopupDialog/SpeechBubble.tscn");
        self.initialize(rand::random(),0);
    }
}

#[godot_api]
impl DeskCollegue{
    #[func]
    fn initialize(&mut self,is_man:bool,id:u32){
        self.id=id;
        if is_man {
            self.sprite.set_texture(load::<Texture2D>("res://assets/OfficeScene/OfficeDeskCollegue/DeskMan.PNG"));
        }else{
            self.sprite.set_texture(load::<Texture2D>("res://assets/OfficeScene/OfficeDeskCollegue/DeskWoman.PNG"));
        }
        godot_print!("pwd:{}",std::env::current_dir ().unwrap ().display());
        self.load_text_list("D:/workspace/Godot/Prj0821/godot/text/bumps.json");
        self.timer.connect("timeout".into(), self.base.callable("time_to_speak"));
        self.timer.set_wait_time(2.0);
        self.timer.start();     
    }

    #[func]
    fn time_to_speak(&mut self){
        let mut rng=rand::thread_rng();
        if rng.gen::<f64>()<0.2 {
            let bumps=self.bumps_list.get(self.id as usize).unwrap();
            self.speak(bumps.get(rng.gen_range(0..bumps.len())).unwrap().as_str().into());
        }else{
            self.timer.start();
        }   
    }

    #[func]
    fn speak(&mut self,text:GodotString){
        self.base.add_child(self.bubble_scene.instantiate_as::<SpeechBubble>().upcast());
        let mut bubble=self.base.get_node_as::<SpeechBubble>("SpeechBubble");
        bubble.set_name("BindedBubble".into());
        bubble.bind_mut().popup(text,Vector2::new(137.0,248.0),2.0);
        bubble.connect("bubble_vanished".into(), self.base.callable("on_bubble_vanished"));
    }

    #[func]
    fn on_bubble_vanished(&mut self){
        self.timer.set_wait_time(2.0);
        self.timer.start();
    }
}

impl DeskCollegue{
    fn load_text_list(&mut self,filepath:&str){
        self.bumps_list = {
            godot_print!("start reading");
            let data = fs::read_to_string(filepath).expect("LogRocket: error reading file");
            vec![vec![String::from("怎么还没下班"),"想喝咖啡了".to_string(),"待会摸摸鱼吧".to_string()]]
        };
        godot_print!("{:?}",self.bumps_list);
    }
}

