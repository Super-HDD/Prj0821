use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, TextureButton, Label,Engine};
use time::Time;
use time::macros::time;

use crate::game_manager::GameManager;
use crate::player_speech_bubble::PlayerSpeechBubble;
use crate::work_window::WorkWindow;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Windose{
    is_working:bool,
    has_worked:bool,
    work_finish_time:Time,
    work_start_time:Time,
    work_icon:Option<Gd<TextureButton>>,
    cyber_pet_icon:Gd<TextureButton>,
    daka_icon:Gd<TextureButton>,
    clock_label:Option<Gd<Label>>,
    dialog_bubble_scene:Gd<PackedScene>,
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
            cyber_pet_icon:TextureButton::new_alloc(), 
            daka_icon:TextureButton::new_alloc(),
            //game_manager:None,
            clock_label: None, 
            dialog_bubble_scene:PackedScene::new(),
            //game_manager_ref:None,
            base 
        }
    }

    fn ready(&mut self){
        self.dialog_bubble_scene=load("res://Scene/Player/PlayerDialogBubble.tscn");
        self.work_icon=Some(self.base.get_node_as("WorkIcon"));
        self.clock_label=Some(self.base.get_node_as("ClockLabel"));
        let mut game_manager=self.base.get_tree().unwrap().get_root().unwrap().get_node_as::<GameManager>("MyGameManager");
        game_manager.connect("time_changed".into(), self.base.callable("on_time_changed"));
        self.work_icon.as_mut().unwrap().connect("pressed".into(),self.base.callable("on_work_icon_pressed"));

        self.clock_label.as_mut().unwrap().set_text(game_manager.bind_mut().get_time_str().try_to::<GodotString>().unwrap());
    }

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
            work_window.set_position(Vector2::new(11.842,-6.742));
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
        self.base.add_child(self.dialog_bubble_scene.instantiate_as::<PlayerSpeechBubble>().upcast());
        let mut player_dialog=self.base.get_node_as::<PlayerSpeechBubble>("PlayerSpeechBubble");
        player_dialog.bind_mut().set_text_list(array!["工作结束了".into(),"点击离开工位".into()]);
        player_dialog.bind_mut().popup(Vector2::new(-63.158,576.43));
    }

    #[func]
    pub fn on_time_changed(&mut self,hour:u8,minute:u8){
        godot_print!("on_time_changed{}:{}",hour,minute);
        self.clock_label.as_mut().unwrap().set_text(GodotString::from(format!("{}:{:02}",hour,minute)));
    }

    #[signal]
    fn left_desk();

}

