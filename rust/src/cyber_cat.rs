use godot::engine::global::MouseButton;
use godot::prelude::*;
use godot::engine::{CharacterBody2D,CharacterBody2DVirtual, AnimatedSprite2D, Timer,InputEventMouseButton,InputEvent};
use rand::Rng;
use std::num;

const LEFT_BORDER_X:real=0.0;
const RIGHT_BORDER_X:real=900.0;

const MIN_SIT_TIME:f64=1.0;
const MAX_SIT_TIME:f64=3.0;
const LOVE_TIME:f64=2.0;

const SPEED:real=50.0;

const DEST_MARGIN:real=1.0;

pub enum CyberCatState{
    IDLE,
    SHIT,
    EAT,
    LOVE
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct CyberCat{
    
    is_in_event:bool,
    is_cursor_inside_touch_area:bool,
    event_buffer:Array<u8>,
    dest_x:real,
    state:CyberCatState,
    is_walking:bool,
    is_facing_right:bool,
    goodwill:i32,

    default_mouse_icon:Gd<Resource>,
    touch_mouse_icon:Gd<Resource>,
    anim:Gd<AnimatedSprite2D>,
    timer:Gd<Timer>,
    #[base]
    base:Base<CharacterBody2D>
}


#[godot_api]
impl CharacterBody2DVirtual for CyberCat{
    fn init(base:Base<CharacterBody2D>)->Self{
        Self {
            is_in_event:false,
            is_cursor_inside_touch_area:false,
            event_buffer:Array::new(),
            dest_x:-1.0,
            state:CyberCatState::IDLE,
            is_walking:false,
            is_facing_right:true,
            goodwill:0,

            default_mouse_icon:Resource::new(),
            touch_mouse_icon:Resource::new(),
            anim:AnimatedSprite2D::new_alloc(),
            timer:Timer::new_alloc(),
            base
        }
    }

    fn ready(&mut self){
        //加载子节点
        self.anim=self.base.get_node_as("AnimatedSprite2D");
        self.timer=self.base.get_node_as("Timer");

        //加载资源
        self.touch_mouse_icon=load("res://assets/MouseIcon/touch_mouse_icon.png");
        self.default_mouse_icon=load("res://assets/MouseIcon/WindoseCursor.png");

        //TODO:获取好感度
        //todo!("获取好感度");

        //配置子节点
        //配置Timer
        self.timer.connect("timeout".into(),self.base.callable("on_time_out"));
        let on_mouse_entered_action=self.base.callable("on_mouse_entered");
        self.base.connect("mouse_entered".into(),on_mouse_entered_action);
        let on_mouse_exited_action=self.base.callable("on_mouse_exited");
        self.base.connect("mouse_exited".into(),on_mouse_exited_action);
        //坐下
        self.sit();
    }

    fn process(&mut self,_delta:f64){

    }

    fn physics_process(&mut self,delta:f64){
        //更新位置
        let motion=self.base.get_velocity()*delta as f32;
        self.base.move_and_collide(motion);

        //如果正在移动，判定是否到达指定位置
        if self.is_walking{
            godot_print!("base_x:{},anim_x{},distance:{}",self.base.get_position().x,self.anim.get_position().x,(self.base.get_position().x-self.dest_x).abs());
            if (self.base.get_position().x-self.dest_x).abs() <= DEST_MARGIN{
                self.on_arrive_dest();
            }
        }
    }

    fn input(&mut self,event:Gd<InputEvent>){

        if self.is_cursor_inside_touch_area&&event.is_class("InputEventMouseButton".into()){
            let mouse_event:Gd<InputEventMouseButton>=event.cast();
             if mouse_event.is_pressed() && mouse_event.get_button_index()==MouseButton::MOUSE_BUTTON_LEFT{
                self.touch();
             }
        } 
    } 

}

#[godot_api]
impl CyberCat {
    #[func]
    fn sit(&mut self){

        //停止运动
        self.base.set_velocity(Vector2::new(0.0,0.0));

        //播放sit动画
        self.anim.set_animation("sit".into());
        self.anim.play();

        //随机等待时间
        self.timer.set_wait_time(rand::thread_rng().gen_range(MIN_SIT_TIME..MAX_SIT_TIME));
        self.timer.start();
        self.is_walking=false;
    }

    #[func]
    fn walk_to(&mut self,dest_x:real){
        self.dest_x=dest_x;
        if self.dest_x<LEFT_BORDER_X || self.dest_x >RIGHT_BORDER_X {
            //dest_x=rand::thread_rng().gen_range(LEFT_BORDER_X..RIGHT_BORDER_X);
            panic!("dest_x Out of border");
        }

        self.is_walking=true;

        //设置速度
        if self.base.get_position().x<self.dest_x{
            if self.is_facing_right==false{
                self.is_facing_right=true;
                self.anim.set_flip_h(false);
            }
            self.base.set_velocity(Vector2::RIGHT*SPEED);
        }else{
            if self.is_facing_right==true{
                self.is_facing_right=false;
                self.anim.set_flip_h(true);
            }
            self.base.set_velocity(Vector2::LEFT*SPEED);
        }

        self.anim.set_animation("walk".into());
        self.anim.play();
    }

    #[func]
    fn touch(&mut self){
        self.base.set_velocity(Vector2::new(0.0,0.0));
        self.is_walking=false;
        self.is_in_event=true;
        self.state=CyberCatState::LOVE;

        self.anim.set_animation("love".into());
        self.anim.play();

        self.timer.stop();
        self.timer.set_wait_time(LOVE_TIME);
        self.timer.start();
        self.on_mouse_exited();
    }

    #[func]
    fn on_time_out(&mut self){
        
        match self.state{
            CyberCatState::IDLE=>{
                self.walk_to(rand::thread_rng().gen_range(LEFT_BORDER_X..RIGHT_BORDER_X));
            }
            CyberCatState::LOVE=>{
                self.state=CyberCatState::IDLE;
                self.sit();
            }
            _=>todo!()
        }
    }

    #[func]
    fn on_mouse_entered(&mut self){
        match self.state{
            CyberCatState::IDLE=>{
                let mut input=Input::singleton();
                input.set_custom_mouse_cursor(self.touch_mouse_icon.share());
                self.is_cursor_inside_touch_area=true;
            }
            _=>{}
        }   
    }

    #[func]
    fn on_mouse_exited(&mut self){
        let mut input=Input::singleton();
        input.set_custom_mouse_cursor(self.default_mouse_icon.share());
        self.is_cursor_inside_touch_area=false;
    }

    #[func]
    fn on_arrive_dest(&mut self){
        self.is_walking=false;
        match self.state{
            CyberCatState::IDLE=>{
                self.sit();
            }
            CyberCatState::EAT=>{
                todo!("eat action");
            }
            CyberCatState::SHIT=>{
                todo!("shit action");
            }
            CyberCatState::LOVE=>{
                
            }
        }
    }

    #[signal]
    fn arrive_dest();
}