use godot::prelude::*;
use godot::engine::{TextureButtonVirtual, TextureButton, Control, CodeEdit};


#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct PetEditorUI {
    pet_editor_ui:Gd<Control>,
    pet_code_edit:Gd<CodeEdit>,
    pet_code:String,
    #[base]
    close_button: Base<TextureButton>,
}

#[godot_api]
impl TextureButtonVirtual for PetEditorUI {
    fn init(base:Base<TextureButton>)->Self{

        Self { 
            pet_editor_ui:Control::new_alloc(),
            pet_code_edit:CodeEdit::new_alloc(),
            pet_code:String::from("PetName:LuLu\r\n"),
            close_button: base,
        }
    }

    fn ready(&mut self){
        self.pet_editor_ui=self.close_button.get_node_as("../");
        self.pet_code_edit=self.close_button.get_node_as("CodeEdit");
        self.pet_code_edit.set_text(GodotString::from(&self.pet_code));
    }



    fn pressed(&mut self){
        
        self.pet_code=String::from(self.pet_code_edit.get_text());
        self.pet_editor_ui.set_visible(false);
    }

}