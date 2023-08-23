use godot::prelude::*;
use godot::engine::{Node,NodeVirtual, Engine};



#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager{

    office_scene:Gd<PackedScene>,
    desk_scene:Gd<PackedScene>,
    scene_tree:Gd<SceneTree>,
    desk:Option<Gd<Node2D>>,
    office:Option<Gd<Node2D>>,
    #[base]
    base:Base<Node>
}

#[godot_api]
impl NodeVirtual for GameManager{

    fn init(base:Base<Node>)->Self{

        

        Self {
            office_scene:PackedScene::new(),
            desk_scene:PackedScene::new(),
            desk:None,
            office:None,
            scene_tree:SceneTree::new_alloc(), 
            base 
        }
    }

    fn ready(&mut self){
        //Engine::singleton().register_singleton("GameManager".into(), self.base.share().upcast());

        self.office_scene=load("res://Scene/OfficeScene/Office.tscn");
        self.desk_scene=load("res://Scene/DeskScene/Desk.tscn");
        
        godot_print!("has_singleton:{}",Engine::singleton().has_singleton("GameManager".into()));
        self.scene_tree=self.base.get_tree().unwrap();
        // self.scene_tree.change_scene_to_packed(self.desk_scene.share());
        self.scene_tree.change_scene_to_packed(self.desk_scene.share());
    }

    fn process(&mut self,_delta:f64){
        let input=Input::singleton();
        if input.is_action_just_pressed(StringName::from("left_desk")){
            //self.scene_tree.change_scene_to_packed(self.office_scene.share());
            self.scene_tree.change_scene_to_packed(self.office_scene.share());
        }
    }
    
}

#[godot_api]
impl GameManager{
    #[signal]
    fn time_changed();
}

