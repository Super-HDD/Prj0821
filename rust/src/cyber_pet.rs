use godot::engine::global::MouseButton;
use godot::prelude::*;
use godot::engine::{CharacterBody2D, CharacterBody2DVirtual, InputEvent, InputEventMouseButton,Timer, TimerVirtual,Sprite2D, PopupPanel};

use crate::speech_bubble::{self, SpeechBubble};


#[derive(GodotClass)]
#[class(base=Timer)]
pub struct PetTouchTimer{

    #[base]
    base:Base<Timer>
}

#[godot_api]
impl TimerVirtual for PetTouchTimer{
    fn init(base:Base<Timer>)->Self{
        Self { 
            base
        }
    }

    fn ready(&mut self){
        let pet:Gd<CyberPetCharacterBody>=self.base.get_node_as("../");
        self.base.connect("timeout".into(), Callable::from_object_method(pet, "on_touch_enough"));
    }

}



// #[godot_api]
// impl Sprite2DVirtual for CyberPet{
//     fn init(base:Base<Sprite2D>)->Self{
//         Self { 
//             speed:100.0,
//             character_body:,
//             is_touched:false,
//             sprite:base
//          }
//     }

//     fn ready(&mut self){
//         self.character_body=self.sprite.get_node_as("CharacterBody");
//     }
// }



#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct CyberPetCharacterBody{

    speed:real,
    is_touched:bool,
    is_moving_right:bool,
    #[base]
    base:Base<CharacterBody2D>
}

#[godot_api]
impl CharacterBody2DVirtual for CyberPetCharacterBody{

    fn init(base:Base<CharacterBody2D>)->Self{

        Self { 
            speed: 50.0, 
            is_touched: false, 
            is_moving_right:true,
            base:base 
        }
    }


    fn physics_process(&mut self,_delta:f64){
        self.rand_move();
        if self.base.is_on_wall(){
            godot_print!("collide");
            self.is_moving_right=!self.is_moving_right;
        }
    }

    fn input(&mut self,event:Gd<InputEvent>){

        if event.is_class("InputEventMouseButton".into()){
            let mouse_event:Gd<InputEventMouseButton>=event.cast();
             if mouse_event.is_pressed() && mouse_event.get_button_index()==MouseButton::MOUSE_BUTTON_LEFT{
                let sprite:Gd<Sprite2D>=self.base.get_node_as("Sprite2D");
                if sprite.get_rect().has_point(sprite.to_local(mouse_event.get_position())){

                    let bubble_scene:Gd<PackedScene>=load("res://custom_res/popup_dialog/SpeechBubble.tscn");
                    //let mut bubble=bubble_scene.instantiate_as::<SpeechBubble>();
                    self.base.add_child(bubble_scene.instantiate_as::<SpeechBubble>().upcast());
                    let mut bubble:Gd<SpeechBubble>=self.base.get_node_as("SpeechBubble");
                    bubble.set_name("TouchedBubble".into());
                    bubble.bind_mut().popup("摸摸".into(),sprite.get_position()+Vector2::UP*0.5*sprite.get_rect().size.y*sprite.get_scale().y,2.0);
                    godot_print!("sprite position:{}\nsprite size:{}",sprite.get_position(),sprite.get_rect().size.y);
                    let mut touch_timer:Gd<PetTouchTimer>=self.base.get_node_as("Timer");
                    self.is_touched=true;
                    touch_timer.start();
                }
             }
        }
        
        
    } 



}

#[godot_api]
impl CyberPetCharacterBody{
    #[func]
    fn on_touch_enough(&mut self){
        self.is_touched=false;
    }
}

impl CyberPetCharacterBody{
    fn rand_move(&mut self){

        if self.is_touched{
            self.base.set_velocity(Vector2::new(0.0,0.0));
        }else if self.is_moving_right{
            self.base.set_velocity(Vector2::RIGHT*self.speed);
        }else{
            self.base.set_velocity(Vector2::LEFT*self.speed);
        }
        
        self.base.move_and_slide();
    }



}