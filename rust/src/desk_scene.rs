use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual};

use crate::game_manager::GameManager;
use crate::windose::Windose;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DeskScene{

    game_manager:Option<Gd<GameManager>>,
    windose:Option<Gd<Windose>>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for DeskScene{
    fn init(base:Base<Node2D>)->Self{
        Self{
            game_manager:None,
            windose:None,
            base
        }
    }
}
