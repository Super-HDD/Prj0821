use godot::prelude::*;
use godot::engine::{RichTextLabel, TextureRect, Timer};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SpeechBubble{

    margin_offset:Vector2,
    label:Option<Gd<RichTextLabel>>,
    bg:Option<Gd<TextureRect>>,
    anchor:Option<Gd<Node2D>>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for SpeechBubble{
    fn init(base:Base<Node2D>)->Self{
     
        Self { 
            margin_offset:Vector2::new(50.0,10.0),
            label:None, 
            bg:None, 
            anchor:None, 
            base 
        }
    }

    fn ready(&mut self){

        self.anchor=Some(self.base.get_node_as("Anchor"));
        self.label=Some(self.base.get_node_as("Anchor/BubbleBg/Text"));
        self.bg=Some(self.anchor.as_ref().unwrap().get_node_as("BubbleBg"));
        self.base.set_visible(false);
        self.label.as_mut().unwrap().set_text("Hello".into());

    }


}

#[godot_api]
impl SpeechBubble{
    #[signal]
    fn bubble_vanished();

    #[func]
    pub fn popup(&mut self,text:GodotString,position:Vector2,wait_time:f64,is_text_flip:bool){
        self.base.set_position(position);


        //自适应文字大小
        self.label.as_mut().unwrap().set_text(text);
        let mut text_size=Vector2::new(self.label.as_ref().unwrap().get_content_width() as f32,self.label.as_ref().unwrap().get_content_height() as f32);
        text_size+=self.margin_offset;
        self.bg.as_mut().unwrap().set_size(text_size);
        godot_print!("text size:{}",text_size);
        let label_size=self.label.as_mut().unwrap().get_size();
        self.label.as_mut().unwrap().set_pivot_offset(label_size/2.0 as f32);
        if is_text_flip {
            self.label.as_mut().unwrap().set_scale(Vector2::new(-1.0,-1.0));
        }
        //对话气泡可见
        self.base.set_visible(true);


        //定时关闭
        if wait_time>=0.0 {
            let mut timer=self.base.get_node_as::<Timer>("Timer");
            timer.connect("timeout".into(), self.base.callable("on_time_out"));
            timer.set_wait_time(wait_time);
            timer.start();
        }

    }

    #[func]
    fn on_time_out(&mut self){
        godot_print!("time's up");
        self.base.emit_signal("bubble_vanished".into(),&[]);
        self.base.queue_free();
        
    }
}



