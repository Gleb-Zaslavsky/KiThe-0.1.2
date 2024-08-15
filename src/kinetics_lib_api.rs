//v 0.1.1
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

use serde_json::Value;
// Basis functionality to search in reaction library

enum Substance {
  reactants,

  products
}

pub struct KineticData {
    pub HashMapOfReactantsAndProducts: HashMap<String, HashMap<String, HashSet<String>>>,
    pub LibKineticData:HashMap<String, Value>,
    pub AllEquations: Vec<String>,
    pub EquationReactionMap: HashMap<String,String>,
    pub UserEquationReactionMap: HashMap<String,String>,
    pub FoundReactionsByProducts: Vec<String>,
    pub FoundReactionsByReagents: Vec<String>,
    pub FoundReactionDatasByIDs: Vec<Value>,
}
impl  KineticData {
    pub fn new() -> Self {
        Self {
            HashMapOfReactantsAndProducts: HashMap::new(),
            LibKineticData: HashMap::new(),
            AllEquations: Vec::new(),
            EquationReactionMap: HashMap::new(),
            UserEquationReactionMap: HashMap::new(),
            FoundReactionsByProducts: Vec::new(),
            FoundReactionsByReagents: Vec::new(),
            FoundReactionDatasByIDs: Vec::new(),
        }
    }
    pub fn open_json_files(&mut self, big_mech: &str) -> (){
        let mut file = File::open("Reactbase.json").unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();
        // библиотека:{ номер реакции :{ данные реакции }}
        let reactlibrary:HashMap<String, HashMap<String, Value>>   = serde_json::from_str::<HashMap<String, HashMap<String, _>>>(&file_contents).unwrap();
        let library_of_kinetic_parameters = reactlibrary.get(big_mech).unwrap();
        self.LibKineticData = library_of_kinetic_parameters.to_owned();
        //
        let mut file = File::open("dict_reaction.json").unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();
        // библиотека:{ номер реакции :{ "reagents"/"products":{}: }}
        let reaction_db:HashMap<String, HashMap<String, HashMap<String, HashSet<String>>>> = serde_json::from_str(&file_contents).unwrap();
        let library_of_reagents_and_products: &HashMap<String, HashMap<String, HashSet<String>>> = reaction_db.get(big_mech).unwrap();
        self.HashMapOfReactantsAndProducts = library_of_reagents_and_products.to_owned();
    
    }
    // returns vector of all reaction equations and HashMap {reaction equation : reaction ID}
   pub fn print_all_reactions(&mut self) -> () {
    
     let all_reactions: Vec<String> = self.LibKineticData.keys().map(|k| k.to_string()).collect();
     let all_equations: Vec<String> = self.LibKineticData.keys()
     .map(|k| self.LibKineticData.get(k).unwrap().get("eq").unwrap().as_str().unwrap().to_string()).collect();
      self.AllEquations = all_equations.clone();
      let EquationReactionIDMap: HashMap<String, String> = all_equations
      .iter()
      .zip(all_reactions.iter())
      .map(|(eq, r)| (eq.to_string(), r.to_string())).collect();
      self.EquationReactionMap = EquationReactionIDMap;
    /* 
    {
     for r in all_reactions {
         let Eq: &str = self.LibKineticData.get(&r).unwrap().get("eq").unwrap().as_str().unwrap();
         println!("{}", &Eq);}
     };//
     */
   
} //end print_all_reactions
  // returns reaction ID and reaction data (parsed from json) for given reaction equation
  pub fn search_reaction_by_equation(&mut self, equation: &str) ->(String, Value){

    let reaction_id = self.EquationReactionMap.get(equation).unwrap();
    let reaction = self.LibKineticData.get(reaction_id).unwrap();
    return (reaction_id.clone(),  reaction.clone());

  }

  // returns reaction ID and reaction data (parsed from json) for given reagents/products
  fn search_reaction_by_substances(&mut self, substances: Vec<String>, SubstanceType: &str) -> Vec<String>{
    let substances=   substances.iter().cloned().collect();;
    let mut found_reactions: Vec<String> = Vec::new();
    let db_object = &self.HashMapOfReactantsAndProducts;
    let db_object_mut = &mut db_object.clone();
    let all_reaction_ID: Vec<&str> = self.LibKineticData.keys().map(|k| k.as_str()).collect();
    for r_id in all_reaction_ID{
        let base_substances: &mut HashSet<String> =  db_object_mut.get_mut(r_id).expect("REASON").get_mut(SubstanceType).expect("REASON");
        if base_substances.is_subset(&substances)  {
            found_reactions.push(r_id.to_string());
        }
    }
    return found_reactions;
  }

 pub fn search_reaction_by_reagents_and_products(&mut self, reagents: Vec<String>) -> (){
    let found_reactions_by_reagents = self.search_reaction_by_substances(reagents.clone(), "reagents"); 
    let FoundReactionsByReagents = found_reactions_by_reagents.clone();
    let found_reactions_by_products = self.search_reaction_by_substances(reagents, "products");
    self.FoundReactionsByProducts = found_reactions_by_products.clone();
 }
 
 pub fn search_reactdata_by_reaction_id(&mut self, reaction_id: &str) -> Value {
    let reaction = self.LibKineticData.get(reaction_id).unwrap();
    return reaction.clone();
 }

 pub fn search_reactdata_for_vector_of_IDs(&mut self, reaction_ids: Vec<String>) -> Vec<Value>{

    let mut vec_of_reactions: Vec<Value> = Vec::new();
    for r_id in reaction_ids {
        let reaction = self.LibKineticData.get(&r_id).unwrap();
        vec_of_reactions.push(reaction.clone());
    }
    self.FoundReactionDatasByIDs = vec_of_reactions.clone();
    return vec_of_reactions;

} 
 
}
