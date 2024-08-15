//mod mechfinder_api;
use crate::mechfinder_api::parse_kinetic_data;
use crate::parsetask::decipher_vector_of_shortcuts;
use crate::kinetics_lib_api::KineticData;
use crate::reaction_analyzer::ReactionAnalyzer;

use std::collections::HashMap;
use serde_json::{Value, Map, Number};
/// 0.1.1
/// processing of user-chosen reactions.
///  So you define what reactions you need by using the constructor of mechanism from the mechfinder_api module or
/// manually with help of kinetics_lib_api module. Now you can 


//structure to store user task and reaction data
pub struct UserReactions {
    pub shortcut_reactions: Vec<String>, // vector of reaction shortcut names
    pub  map_of_reactions: HashMap<String, String>, // full "address" of reaction {'library':"id of reaction"}
    pub grouped_map_of_reactions: HashMap<String, Vec<String>>, // now we group reactions by library names {'library':[reaction_ids in that library]}
    pub map_of_reaction_data: HashMap<String, Value>, // data of all reactions
    pub vec_of_equations: Vec<String>, // vector of equations of reactions
    pub substances: Vec<String>, // vector of substance names
    pub stecheodata: ReactionAnalyzer, // matrix of stoichiometric coefficients and other matrices

}

impl UserReactions {
    pub fn new(shortcut_reactions: Vec<String>, map_of_reactions: HashMap<String, String>,grouped_map_of_reactions: HashMap<String, Vec<String>>,
         map_of_reaction_data: HashMap<String, Value>, vec_of_equations: Vec<String>, substances: Vec<String>, stecheodata: ReactionAnalyzer) -> Self {
        Self {
            shortcut_reactions,
            map_of_reactions,
            grouped_map_of_reactions,
            map_of_reaction_data,
            vec_of_equations,
            substances, 
            stecheodata,
        }
    }
    // decifer vector of shortcuts to full reaction names and store them in map {'library':"id of reaction"}
    pub fn create_map_of_reactions(mut self) -> () {
        let vec:Vec<&str> = self.shortcut_reactions.iter().map(|s| s.as_str()).collect();
        self.map_of_reactions = decipher_vector_of_shortcuts(vec);

    }

    pub fn grouped_map_of_reactions(mut self) -> () {
       let mut map_of_reactions: HashMap<String, Vec<String>> = HashMap::new();
       for (k, v) in self.map_of_reactions.iter() {
           if map_of_reactions.contains_key(k) {
               map_of_reactions.get_mut(k).unwrap().push(v.to_string());
           } else {
               map_of_reactions.insert(k.to_string(), vec![v.to_string()]);
           }
       }
       self.grouped_map_of_reactions = map_of_reactions;
      //  self.grouped_map_of_reactions

    }

    pub fn create_map_of_reaction_data(&mut self) -> () {
        let mut vec_of_equations = Vec::new();
        let mut ReactionDataHash = HashMap::new();
        // instance of KineticData with opened library json files
        let mut kin_instance = KineticData::new();
        for (lib, reaction_id_vector) in self.grouped_map_of_reactions.iter() {
            // collecting reaction data for each library name
            kin_instance.open_json_files(lib);
            let mut vec_of_reactions_value: Vec<Value> = Vec::new();
            let mut vec_of_reactions: Vec<String> = Vec::new();
            for reaction_id in reaction_id_vector.iter() {
                let reaction_data_Value = kin_instance.search_reactdata_by_reaction_id( &reaction_id);
                vec_of_reactions_value.push(reaction_data_Value);
                vec_of_reactions.push(reaction_id.to_string());
            }// for
            // now we have a vector of json objects with reaction data and a vector of reaction ids
            // lets parse it to map of structures
            let (mut ReactionDataHash_for_lib, vec_of_equations_for_lib) = parse_kinetic_data( lib, &vec_of_reactions, vec_of_reactions_value);  
            vec_of_equations.extend(vec_of_equations_for_lib);
            ReactionDataHash.extend(ReactionDataHash_for_lib);

        }

        self.map_of_reaction_data = ReactionDataHash;
        self.vec_of_equations = vec_of_equations;

    }
    pub fn analyze_reactions(&mut self) -> () {
        // iniciate instance of ReactionAnalyzer 
        let mut ReactionAnalyzer_instance =ReactionAnalyzer::new();
        // copy vector of reactions to ReactionAnalyzer_instance
        ReactionAnalyzer_instance.reactions = self.vec_of_equations.clone();
        // parse to find substance names
        ReactionAnalyzer_instance.search_substances();
        self .substances = ReactionAnalyzer_instance.substances.clone();
        //find stoichiometric matrix and other matrices
        ReactionAnalyzer_instance.analyse_reactions();

        self.stecheodata = ReactionAnalyzer_instance;

        
    }
    
}
