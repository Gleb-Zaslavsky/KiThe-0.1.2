#![allow(warnings)]
//v 0.1.1
 /// ru 
 /// Модуль снабжен библиотекой кинетических параметров химических реакций, полученной в результате парсинга общедоступныхбаз данных 
 /// Модуль берет на вход название библиотеки и вектор веществ а затем выдает следующие данные:
 /// 1) все реакции исходных веществ между собой, и всех их возможных продуктов между собой. 
 /// 2) HashMap с кинетическими данными всех найденных реакций
/// ----------------------------------------------------------------
 /// eng
 /// The module is equipped with a library of kinetic parameters of chemical reactions obtained as a result of parsing publicly available databases 
 /// The module takes as input the name of the library and the vector of substances and then produces the following data:
 /// 1) all reactions of starting substances with each other, and all their possible products with each other. 
 /// 2) HashMap with kinetic data of all found reactions

mod mechfinder;
pub mod kinetics;
use kinetics::{ ElementaryStruct, FalloffStruct, PressureStruct, ThreeBodyStruct};

use serde_json::{Value, Map, Number};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::f64;



#[warn(unused_imports)]
enum ReactionTypes {
    elem,
    falloff,
    pressure,
    threebody
}

/* Нам необходимо расширить существующую струкутуру, полученную парсингом json чтобы добавить */

pub fn parse_kinetic_data( big_mech:&str,  vec_of_reactions:&Vec<String>,  vec_of_reaction_value:Vec<Value>)->(Map<String, Value>, Vec<String> ) {
    let mut ReactionDataHash:   Map<String, Value>  = Map::new();
    let mut vec_of_equations:Vec<String> = Vec::new();
    //let mut parsed_reactions: Vec<_> = Vec::new();
    for reaction_record in &vec_of_reaction_value {
        if let Some(j) = &vec_of_reaction_value.iter().position(|x| x == reaction_record) {
            
        let react_id:String = vec_of_reactions[*j].to_owned();
        let react_code = format!("{}_{}", big_mech, &react_id);
        println!("{:?}", &reaction_record);
        let react_type_value:&Value = &reaction_record["type"];
        let equation_of_reaction:String = reaction_record["eq"].clone().as_str().unwrap().to_owned();
        vec_of_equations.push(equation_of_reaction);
       // println!("{:?}", &react_type_value);
        let react_type_value:String =  serde_json::from_value(react_type_value.to_owned()).unwrap();
       // print!("тип реакции {:?}", &react_type_value);
        match react_type_value.as_str() {

            "elem" =>{
                 // парсим json запись в структуру, конструкция if let Ok нужна для обработки ошибки при записи в структуру, Т.К. 
                 // в структуре есть поля которых нет в json
                 if let Ok(mut elemstruct) = serde_json::from_value::<ElementaryStruct>(reaction_record.clone()) {
                    elemstruct.react = Some(HashMap::new());
                    elemstruct.reactnumber = Some(0);
                     println!("{:?}", &elemstruct);   
                     let v = serde_json::to_value(&elemstruct).unwrap();
                     ReactionDataHash.insert(react_code.clone(), v );
                     /* 
                     get value syntax
                     let v:ElementaryStruct = serde_json::from_value::<ElementaryStruct>(ReactionDataHash[&react_id].clone()).unwrap();
                     print!("value {:?}", v);
                     println!("K_const {:?}", v.K_const(298.15) );
                    */
                } 
                     }
            "falloff" =>{
                if let Ok(mut faloffstruct) = serde_json::from_value::<FalloffStruct>(reaction_record.clone()) {
                  //  elemstruct.react = Some(HashMap::new());
                   // elemstruct.reactnumber = Some(0);
                     println!("{:?}", &faloffstruct);   
                     let v = serde_json::to_value(&faloffstruct).unwrap();
                     ReactionDataHash.insert(react_code, v );
                   // parsed_reactions.push(elemstruct);
                    
                } 

             }
            "pressure" =>{

             }
            "threebody" | "three-body" =>{
                if let Ok(mut threebodystruct) = serde_json::from_value::<ThreeBodyStruct>(reaction_record.clone()) {
                    //  elemstruct.react = Some(HashMap::new());
                     // elemstruct.reactnumber = Some(0);
                       println!("{:?}", &threebodystruct);   
                       let v = serde_json::to_value(&threebodystruct).unwrap();
                       ReactionDataHash.insert(react_code, v );
                     // parsed_reactions.push(elemstruct);
                      
                  } 
             }
            _ => {  print!("")}
    }; // match

    } // if let Some
   
  //  ReactionDataHash.insert(react_id, elemstruct);
} // for

    return (ReactionDataHash, vec_of_equations);
}
#[derive(Debug)]
 pub struct Mechanism_search {
    pub task_substances:Vec<String>, 
    pub task_library:String, 
    pub mechanism: Vec<String>, 
    pub reactants: Vec<String>,
    pub vec_of_reactions: Vec<String>, 
   

}
 // 
impl Mechanism_search {
    pub fn new(task_substances:Vec<String>, task_library:String, mechanism:Vec<String>, reactants:Vec<String>, vec_of_reactions:Vec<String>)-> Self { 
        Self{
            task_substances,
            task_library,
            mechanism,
            reactants,
            vec_of_reactions,
        } 
        }
    
    pub fn default() -> Self {    
        Self{
            task_substances:Vec::new(), 
            task_library: String::new(),
            mechanism: Vec::new(),
            reactants: Vec::new(),
            vec_of_reactions: Vec::new(),
        }
    }
    pub fn mechfinder_api(&mut self)-> ( Vec<String>, Vec<String>, Vec<String>,) {
        /* 
        let tuple = [ "O", "NH3", "NO", "O2", "N2", "N2O", "CO", "C"];
        O,NH3,NO,O2,N2,N2O,CO,C
        let big_mech = "NUIG".to_string();
            let vec: Vec<&str> =  tuple.into_iter().collect();
        */
    

        let vec: Vec<&str> = self.task_substances.iter().map(|s| s.as_str()).collect();
        let big_mech = self.task_library.clone();
        println!("задание {:?}, библиотека {:?}", &big_mech, &vec);
        let (mechanism, reactants, vec_of_reactions,  vec_of_reaction_value) 
        = mechfinder::mechfinder(&big_mech, vec);
       // println!("mechanism {:?}", &mechanism);
       let   (mut ReactionDataHash ,vec_of_equations)  =  parse_kinetic_data(&big_mech,  &vec_of_reactions, vec_of_reaction_value.clone());  // парсим данные о реакциях
        self.mechanism = mechanism;
        self.reactants = reactants;
        self.vec_of_reactions = vec_of_reactions;
        return (self.mechanism.to_owned(), self.reactants.to_owned(), self.vec_of_reactions.to_owned()) 
        
    }
}

//tests
const ELEM_TESTING_JSON: &str =  r#"{"type": "elem",
                 "eq": "NAPH+C2H3<=>NAPHV+C2H4",
                  "Arrenius": [0.408, 4.02, 36822.949]}"#;
const FALOFF_TESTING_JSON: &str = r#" {"type": "falloff",
                 "eq": "C4H71-3+CH3(+M)<=>C5H10-2(+M)",
                 "low_rate": [3.91e+60, -12.81, 26143.75],
                "high_rate": [100000000000000.0, -0.32, -1097.2009],
                 "eff": {"H2": 2.0, "H2O": 6.0, "CH4": 2.0, "CO": 1.5, "CO2": 2.0, "C2H6": 3.0, "AR": 0.7},
                 "troe": [0.104, 1606.0, 60000.0, 6118.0]} "#;
const PRES_TESTING_JSON: &str= " '1736': {'type': 'pres', 'eq': 'SC4H9<=>C3H6+CH3',
               'Arrenius': {'0.001': [2.89e+40, -9.76, 140552.983],
                              '0.01': [1.8e+44, -10.5, 154800.281],
                             '0.1': [2.51e+46, -10.73, 168311.37099999998],
                             '1.0': [4.74e+44, -9.85, 175020.903], 
                             '10.0': [3.79e+37, -7.44, 169846.532],
                             '100.0': [4.79e+26, -4.01, 154344.334]}}";
const THREE_BODY_TESTING_JSON: &str =      r#"{"type": "threebody",
              "eq": "H2+M<=>H+H+M",
              "Arrenius": [4.577e+19, -1.4, 436705.19999999995],
             "eff": {"H2": 2.5, "H2O": 12.0, "CO": 1.9, "CO2": 3.8, "HE": 0.83, "CH4": 2.0, "C2H6": 3.0} }"#;
 #[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_mechfinder_api() {
        let mut mech_search = Mechanism_search::new(
            vec!["O".to_string(), "NH3".to_string(), "NO".to_string()],
            "NUIG".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        let (mechanism, reactants, vec_of_reactions) = mech_search.mechfinder_api();

        assert!(!mechanism.is_empty());
        assert!(!reactants.is_empty());
        assert!(!vec_of_reactions.is_empty());

    }

   #[test]
    fn test_default_values() {
        let mech_search = Mechanism_search::default();

        assert!(mech_search.task_substances.is_empty());
        assert!(mech_search.task_library.is_empty());
        assert!(mech_search.mechanism.is_empty());
        assert!(mech_search.reactants.is_empty());
        assert!(mech_search.vec_of_reactions.is_empty());
    }
    
   #[test]

    fn test_ELEM_parse_kinetic_data(){
        let big_mech: &str = "NUIG"; 
        /* 
        let test_data = [ELEM_TESTING_JSON, FALOFF_TESTING_JSON, PRES_TESTING_JSON, THREE_BODY_TESTING_JSON];
        let   test_reactions_numbers = vec!("1", "2532", "1736", "5");
        let vec_of_reactions: Vec<String> =  test_reactions_numbers.iter().map(|&s| s.trim().to_string()).collect();
        let vec_of_reaction_value: Vec<Value> = test_data.iter().map(|&s| serde_json::from_str(&s).unwrap()).collect();
        */
        let   vec_of_reactions = vec!("1".to_string() );
        let reaction = ELEM_TESTING_JSON; 
        let vec_of_reaction_value: Vec<Value>  =  vec!(serde_json::from_str( reaction).unwrap());
        let (ReactionDataHash, _) = parse_kinetic_data(big_mech,
              &vec_of_reactions,  
              vec_of_reaction_value);

        assert!(!ReactionDataHash.is_empty());
     //   let elem_saved_to_hash = ReactionDataHash[test_reactions_numbers[0]];
        let key = format!("{}_{}", big_mech, &vec_of_reactions[0]);
        let elem_react_testing_instance:ElementaryStruct = serde_json::from_value::<ElementaryStruct>(  ReactionDataHash[&key].clone()).unwrap();
        println!("K_const {:?}", elem_react_testing_instance.K_const(298.15) );
        assert!(elem_react_testing_instance.K_const(298.15) > 0.0);
    }
    #[test]

    fn test_THREEBODY_parse_kinetic_data(){
        let big_mech: &str = "NUIG"; 
        let   vec_of_reactions = vec!("2".to_string() );
        let reaction =THREE_BODY_TESTING_JSON; 
        let vec_of_reaction_value: Vec<Value>  =  vec!(serde_json::from_str( reaction).unwrap());
        let (ReactionDataHash, _) = parse_kinetic_data(big_mech,
              &vec_of_reactions,  
              vec_of_reaction_value);

        assert!(!ReactionDataHash.is_empty());
        let key = format!("{}_{}", big_mech, &vec_of_reactions[0]);
        let threebody_react_testing_instance:ThreeBodyStruct = serde_json::from_value::<ThreeBodyStruct>(  ReactionDataHash[&key].clone()).unwrap();
        let mut Concentrations: HashMap<String, f64 > = HashMap::new();
        Concentrations.insert("H".to_string(), 0.5);
        Concentrations.insert("O".to_string(), 0.5);    
        assert!(threebody_react_testing_instance.K_const(298.15, Concentrations) > 0.0);
       // assert!(elem_react_testing_instance.K_const(298.15) > 0.0);
    }
    #[test]

    fn test_FALOFF_parse_kinetic_data(){
        let big_mech: &str = "NUIG"; 
        let   vec_of_reactions = vec!("3".to_string() );
        let reaction = FALOFF_TESTING_JSON; 
        let vec_of_reaction_value: Vec<Value>  =  vec!(serde_json::from_str( reaction).unwrap());
        let (ReactionDataHash, _) = parse_kinetic_data(big_mech,
              &vec_of_reactions,  
              vec_of_reaction_value);

        assert!(!ReactionDataHash.is_empty());
        let key = format!("{}_{}", big_mech, &vec_of_reactions[0]);
        let falloff_react_testing_instance:FalloffStruct = serde_json::from_value::<FalloffStruct >(  ReactionDataHash[&key].clone()).unwrap();
        let mut Concentrations: HashMap<String, f64 > = HashMap::new();
        Concentrations.insert("H".to_string(), 0.5);
        Concentrations.insert("O".to_string(), 0.5);    
        assert!(falloff_react_testing_instance.K_const(298.15, Concentrations) > 0.0);
       // assert!(elem_react_testing_instance.K_const(298.15) > 0.0);
    }
}            


