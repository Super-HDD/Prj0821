use godot::prelude::*;
use godot::engine::{Node2D,Node2DVirtual};

use crate::desk_collegue::DeskCollegue;

const COLLEGUES_NUM:usize=11;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct OfficeScene{
    collegue_list:Array<Gd<DeskCollegue>>,
    #[base]
    base:Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for OfficeScene{
    fn init(base:Base<Node2D>)->Self{
        Self{
            collegue_list:Array::new(),
            base
        }
    }

    fn ready(&mut self){
        //加载同事节点
        {
            let mut i=1;
            while i<=COLLEGUES_NUM{
                let node_path=format!("DeskCollegue{}",i);
                let mut collegue=self.base.get_node_as::<DeskCollegue>(node_path);
                //collegue.bind_mut().initialize(rand::random(),);
                self.collegue_list.extend_array(array![]);
                i+=1;
            }
        }


    }

}