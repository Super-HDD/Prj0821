

// use godot::prelude::*;
// use godot::engine::{TextureButton,TextureButtonVirtual,Label};

// #[derive(GodotClass)]
// #[class(base=TextureButton)]
// pub struct MiniApp {
//     //app_label:Label,
//     app_behavior:Box<dyn MiniAppVirtual>,
//     #[base]
//     base: Base<TextureButton>,
// }

// #[godot_api]
// impl TextureButtonVirtual for MiniApp{
//     fn init(base: Base<TextureButton>) -> Self {
//         godot_print!("MiniApp loaded");
//         Self {     
//             base
//         }
//     }

//     fn pressed(&mut self){
//         let mut base=self;
//        // MiniAppVirtual::on_button_pressed(mut base);
//     }
// }

// pub trait MiniAppVirtual{
//     fn on_button_pressed<T>(base:&mut Base<T>); 
// }