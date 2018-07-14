pub mod components;
pub mod id;

use self::components::*;
use self::id::*;

use style_scheme::StyleScheme;
use render_window::RenderWindow;
use types::*;

use std::collections::hash_map::HashMap;

use cairo::Context;





   
#[derive(Debug)]
pub enum Model {
    BoxModel(BoxModel),
    BoxEdge(BoxEdge)
}


#[derive(Debug)]
pub struct ModelManager {
    /// stores the true value of the models
    base_models: HashMap<usize, Model>,
    
    /// Stores the temporary value of a model
    temp_models: HashMap<usize, Model>,

    boxe_models: Vec<(usize, BoxModel)>,
}


impl ModelManager {

    // pub fn lookup_id(&self, id: ModelID) -> &BoxModel {
    //    &self.models[id.0]
    // }

    // pub fn lookup_id_mut(&mut self, id: ModelID) -> &mut BoxModel {
    //    &mut self.models[id.0]
    // }


    // pub fn reverse_lookup(&self, model: &BoxModel) -> ModelID {
    //     for (index, value) in self.models.iter().enumerate() {
    //         if value == model {
    //             return ModelID(index);
    //         }
    //     }

    //     panic!("reverse lookup called on un-registered model");
    // }
}
