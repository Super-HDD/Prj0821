use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual, ProgressBar, Tween, TextureProgressBar};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct WorkWindow{
    progress_bar:Option<Gd<TextureProgressBar>>,
    tween:Option<Gd<Tween>>,
    #[base]
    base:Base<Node2D>
}

// #[derive(GodotClass)]
// #[class(base=TextureButton)]
// pub struct WorkIcon{
//     #[base]
//     base:Base<TextureButton>
// }


#[godot_api]
impl Node2DVirtual for WorkWindow{
    fn init(base:Base<Node2D>)->Self{

        Self{
            progress_bar:None,
            tween:None,
            base
        }
    }

    fn ready(&mut self){
        //self.progress_bar=Some(self.base.get_node_as::<ProgressBar>("ProgressBar"));
        self.progress_bar=Some(self.base.get_node_as::<TextureProgressBar>("TextureProgressBar"));
        self.tween=self.progress_bar.as_mut().unwrap().create_tween();
        self.base.set_visible(true);
        self.tween.as_mut().unwrap().tween_property(self.base.get_node_as::<TextureProgressBar>("TextureProgressBar").upcast(), "value".into(), 100.0.to_variant(), 5.0);
        self.tween.as_mut().unwrap().connect("finished".into(), self.base.callable("on_tween_finish"));
        self.tween.as_mut().unwrap().play();
    }

}

#[godot_api]
impl WorkWindow{
    #[func]
    fn on_tween_finish(&mut self){
        self.base.emit_signal("work_finished".into(), &[]);
        self.base.queue_free();
    }

    #[signal]
    fn work_finished();
}

// #[godot_api]
// impl TextureButtonVirtual for WorkIcon{
//     fn init(base:Base<TextureButton>)->Self{
        
//         Self{
//             base
//         }
//     }

//     fn pressed(&mut self){
//         let
//     }
// }