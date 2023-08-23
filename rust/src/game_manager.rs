use godot::prelude::*;
use godot::engine::{Node,NodeVirtual};



#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager{

    office_scene:Gd<PackedScene>,
    desk_scene:Gd<PackedScene>,
    scene_tree:Gd<SceneTree>,
    #[base]
    base:Base<Node>
}

#[godot_api]
impl NodeVirtual for GameManager{

    fn init(base:Base<Node>)->Self{
        Self {
            office_scene:PackedScene::new(),
            desk_scene:PackedScene::new(),
            scene_tree:SceneTree::new_alloc(), 
            base 
        }
    }

    fn ready(&mut self){
        self.office_scene=load("res://Scene/OfficeScene/Office.tscn");
        self.desk_scene=load("res://Scene/DeskScene/Desk.tscn");
        self.scene_tree=self.base.get_tree().unwrap();
        //self.scene_tree.get_root().as_mut().unwrap().add_child(self.desk_scene.instantiate_as::<Node2D>().upcast());
        //self.scene_tree.set_current_scene(scene_tree_temp.get_root().as_mut().unwrap().get_node_as("Node2D"));
        self.scene_tree.change_scene_to_packed(self.desk_scene.share());
    }

    fn process(&mut self,_delta:f64){
        let input=Input::singleton();
        if input.is_action_just_pressed(StringName::from("left_desk")){
            self.scene_tree.change_scene_to_packed(self.office_scene.share());
        }
    }
    

}

