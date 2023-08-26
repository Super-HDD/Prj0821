use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, TextureButton, Label};
use time::Time;
use time::macros::time;

use crate::cat_window::CatWindow;
use crate::daka::DakaWindow;
use crate::game_manager::GameManager;
use crate::player_speech_bubble::PlayerSpeechBubble;
use crate::work_window::WorkWindow;

const CAT_WINDOW_POS:Vector2=Vector2{x:65.0,y:44.0};
const DAKA_WINDOW_POS:Vector2=Vector2{x:185.0,y:39.0};
const WORK_WINDOW_POS:Vector2=Vector2{x:8.0,y:0.0};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Windose{
    is_daka_window_open:bool,
    is_cat_window_open:bool,
    is_working:bool,
    has_worked:bool,
    work_finish_time:Time,

    work_icon:Option<Gd<TextureButton>>,
    cat_icon:Gd<TextureButton>,
    daka_icon:Gd<TextureButton>,
    clock_label:Option<Gd<Label>>,
    dialog_bubble_scene:Gd<PackedScene>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for Windose{
    fn init(base:Base<Node2D>)->Self{
        Self {
            is_daka_window_open:false,
            is_cat_window_open:false, 
            is_working: false, 
            has_worked:false,
            work_finish_time:time!(9:00),

            work_icon: None,
            cat_icon:TextureButton::new_alloc(), 
            daka_icon:TextureButton::new_alloc(),
            clock_label: None, 
            dialog_bubble_scene:PackedScene::new(),

            base 
        }
    }

    fn ready(&mut self){
        //加载子节点
        //三个图标
        self.work_icon=Some(self.base.get_node_as("WorkIcon"));
        self.cat_icon=self.base.get_node_as("CatIcon");
        self.daka_icon=self.base.get_node_as("DakaIcon");

        //右下角时间
        self.clock_label=Some(self.base.get_node_as("ClockLabel"));

        //加载资源
        self.dialog_bubble_scene=load("res://Scene/Player/PlayerDialogBubble.tscn");

        //连接GameManager时间改变信号
        let mut game_manager=self.base.get_tree().unwrap().get_root().unwrap().get_node_as::<GameManager>("MyGameManager");
        game_manager.connect("time_changed".into(), self.base.callable("on_time_changed"));

        //连接图标点击信号
        self.work_icon.as_mut().unwrap().connect("pressed".into(),self.base.callable("on_work_icon_pressed"));
        self.cat_icon.connect("pressed".into(),self.base.callable("on_cat_icon_pressed"));
        self.daka_icon.connect("pressed".into(),self.base.callable("on_daka_icon_pressed"));

        //设置时间
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
            work_window.set_position(WORK_WINDOW_POS);
            //work_window.set_z_index(3);
            work_window.connect("work_finished".into(),self.base.callable("on_work_finished"));
            work_window.connect("work_finished".into(),game_manager.callable("on_desk_work_finished"));
            self.is_working=true;
        }
    }

    #[func]
    fn on_cat_icon_pressed(&mut self){
        if self.is_cat_window_open{
            return;
        }

        self.is_cat_window_open=true;
        let cat_window_scene=load::<PackedScene>("res://Scene/DeskScene/SubScene/CatWindow.tscn");
        self.base.add_child(cat_window_scene.instantiate_as::<CatWindow>().upcast());
        let mut cat_window=self.base.get_node_as::<CatWindow>("CatWindow");
        cat_window.set_position(CAT_WINDOW_POS);
        cat_window.connect("window_closed".into(),self.base.callable("on_cat_window_closed"));
    }
    
    #[func]
    fn on_daka_icon_pressed(&mut self){
        if self.is_daka_window_open{
            return;
        }

        self.is_daka_window_open=true;
        let daka_window_scene=load::<PackedScene>("res://Scene/DeskScene/SubScene/DakaWindow.tscn");
        self.base.add_child(daka_window_scene.instantiate_as::<DakaWindow>().upcast());
        let mut cat_window=self.base.get_node_as::<DakaWindow>("DakaWindow");
        cat_window.set_position(DAKA_WINDOW_POS);
        cat_window.connect("window_closed".into(),self.base.callable("on_daka_window_closed"));
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
    fn on_cat_window_closed(&mut self){
        self.is_cat_window_open=false;
    }

    #[func]
    fn on_daka_window_closed(&mut self){
        self.is_daka_window_open=false;
    }

    #[func]
    pub fn on_time_changed(&mut self,hour:u8,minute:u8){
        godot_print!("on_time_changed{}:{}",hour,minute);
        self.clock_label.as_mut().unwrap().set_text(GodotString::from(format!("{}:{:02}",hour,minute)));
    }

    #[signal]
    fn left_desk();

}

