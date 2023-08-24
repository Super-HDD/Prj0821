use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, RichTextLabel};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PlayerSpeechBubble{
    text_list:Array<GodotString>,
    idx:usize,
    rich_text_label:Gd<RichTextLabel>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for PlayerSpeechBubble{
    fn init(base:Base<Node2D>)->Self{
        Self { 
            text_list: Array::new(),
            idx:0, 
            rich_text_label:RichTextLabel::new_alloc(),
            base 
        }
    }

    fn ready(&mut self){
        self.base.set_visible(false);
        self.rich_text_label=self.base.get_node_as("TextureRect/RichTextLabel");
    }

    fn process(&mut self,_delta:f64){
        let input=Input::singleton();
        if input.is_action_just_pressed("next_player_bubble_page".into()) && self.base.is_visible()==true {
            self.idx+=1;
            if self.idx==self.text_list.len() {
                self.base.queue_free();
            }else{
                self.rich_text_label.set_text(self.text_list.get(self.idx));
                godot_print!("{}",self.rich_text_label.get_text());
            }
        }
        
    }
}

#[godot_api]
impl PlayerSpeechBubble{
    #[func]
    pub fn set_text_list(&mut self,text_list:Array<GodotString>){
        self.text_list=text_list;
    }

    #[func]
    pub fn extend_list(&mut self,text_array:Array<GodotString>){
        self.text_list.extend_array(text_array);
    }

    #[func]
    pub fn popup(&mut self,position:Vector2){
        self.idx=0;
        self.rich_text_label.set_text(self.text_list.get(self.idx));
        godot_print!("{}",self.rich_text_label.get_text());
        self.base.set_position(position);
        self.base.set_visible(true);
    }

}