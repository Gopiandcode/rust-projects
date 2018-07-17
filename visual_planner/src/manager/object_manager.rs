use undo::{Modifiable, Modification};

use std::collections::hash_map::HashMap;
use std::ops::AddAssign;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::hash::Hash;



#[derive(Debug)]
pub struct ObjectManager<K,V> 
    where K : Eq  + Clone + Hash + Default + AddAssign<usize>,
          V : Clone + Modifiable {
    id_gen:  K,
    /// stores the true value of the models
    base: HashMap<K,V>,
    /// Stores the temporary value of a model
    temp: HashMap<K,V>,
}

impl<K,V> ObjectManager<K, V> 
    where K : Eq + Clone + Hash + Default + AddAssign<usize>, 
          V : Clone + Modifiable {
        pub fn new() -> Self {
            ObjectManager {
                id_gen: K::default(),
                base: HashMap::new(),
                temp: HashMap::new(),
            }
        }

        pub fn delete_model(&mut self, id: &K) -> Modification {
            self.base.remove(id);
            self.temp.remove(id);

            // todo, store deleted state in memory
           Modification::Deleted 
        }

        // Note: modify this to implement undos
        pub fn commit_changes(&mut self, id: K) -> Modification {
            let object = self.temp.get(&id).expect("Error: Attempted to commit changes to an unknown object"); 
            let mut true_object = self.base.entry(id).or_insert(object.clone());
            true_object.update_state(object)
        }

        pub fn register_model(&mut self, object: V) -> (K, Modification) {
            let old_id = self.id_gen.clone();
            self.id_gen += 1;

            // insert into model and temp
            self.base.insert(old_id, object.clone());
            self.temp.insert(old_id, object);

            (old_id, Modification::New)
        }

        pub fn lookup(&self, id: &K) -> Option<&V> {
            self.temp.get(id)
        }

        pub fn lookup_mut(&mut self, id: &K) -> Option<&mut V> {
            self.temp.get_mut(id)
        }

}
