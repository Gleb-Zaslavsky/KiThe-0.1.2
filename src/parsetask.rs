/// 0.1.1
/// ru
///  в проекте используются шорткаты для названий реакций в стиле
/// "сокращенное название библиотеки реакции"+"номер реакции в библиотеке",
///  например C_10 означает реакция '10' в библиотеке CANTERA. 
/// То же означает C10 и Сantera_10
/// eng
/// The project uses shortcuts for reaction names in the style 
/// of "short name of the reaction library" + "reaction number in the library" 
/// for example C_10 means reaction '10' in the CANTERA library. 
/// The same means C10 and Сantera_10
use regex::Regex;
use std::collections::{HashMap, HashSet};



    pub fn decipher_shortcut_name(short: &str) -> (String, String) {
        let re = Regex::new(r"(\d+(\.\d*)?)\*?").unwrap();
        let captures = re.captures(short);
        println!("captures {:?}", captures);
        let mut shortcut = short.to_string().to_owned();

          let (mut library_encrypted_name, reaction_number) = {
            match captures {
                Some(cap) => {
                    let reaction_number = cap.get(0).unwrap().as_str().to_string();
                    let library_encrypted_name = shortcut
                    .trim()
                    .to_owned()
                    .replace(&reaction_number, "")
                    .replace("_", "").to_lowercase();
                    (library_encrypted_name, reaction_number)
                   }
                   None => ( "Unknown".to_string(), "Unknown".to_string()),
               }
           };
           println!("library_encrypted_name {:?}", &library_encrypted_name);
           let library_name
        =  match library_encrypted_name.as_str() {
            "c"|"cantera" =>  "CANTERA".to_string(),
            "n"|"nuig" =>  "NUIG".to_string(),
            "unknown" =>  "Unknown".to_string(),
            "a"|"aramco"=>  "ARAMCO".to_string(),
            _=> library_encrypted_name.clone()
            
        };
        println!("library_name {:?}", &library_name); 
        println!("reaction_number {:?}", &reaction_number);
        
        (library_name.to_owned(), reaction_number)
    }   
    pub fn decipher_vector_of_shortcuts(vec_of_shortcuts: Vec<&str>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for shortcut in vec_of_shortcuts {
            let (library_name, reaction_number) = decipher_shortcut_name(shortcut);
            map.insert(library_name, reaction_number);
        }
        map
    }

    pub fn cipher_vector_of_shortcuts(map: HashMap<String, String>) -> Vec<String> {
        let mut vec = Vec::new();
        for (library_name, reaction_number) in map {
            vec.push(format!("{}_{}", library_name, reaction_number));
        }
        vec
    }
    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decipher_shortcut_name() {
        assert_eq!(
            decipher_shortcut_name("C_10"),
            ("CANTERA".to_string(), "10".to_string())
        );
        assert_eq!(
            decipher_shortcut_name("C10"),
            ("CANTERA".to_string(), "10".to_string())
        );
        assert_eq!(
            decipher_shortcut_name("Cantera_10"),
            ("CANTERA".to_string(), "10".to_string())
        );
       
        assert_eq!(
            decipher_shortcut_name("A10"),
            ("ARAMCO".to_string(), "10".to_string())
        );

        assert_eq!(
            decipher_shortcut_name("N_10"),
            ("NUIG".to_string(), "10".to_string())
        );
      
      
        assert_eq!(
            decipher_shortcut_name("invalid"),
            ("Unknown".to_string(), "Unknown".to_string())
        );
        
    }

    #[test]
    fn test_decipher_vector_of_shortcuts() {
        let vec_of_shortcuts = vec!["C_10", "A10", "n_10"];
        let expected_map = vec![
            ("CANTERA".to_string(), "10".to_string()),
            ("ARAMCO".to_string(), "10".to_string()),
            ("NUIG".to_string(), "10".to_string()),
        ]
        .into_iter()
        .collect::<HashMap<String, String>>();

        assert_eq!(decipher_vector_of_shortcuts(vec_of_shortcuts), expected_map);
    }
    #[test]
    fn test_cipher_vector_of_shortcuts() {
        let map = vec![
            ("CANTERA".to_string(), "10".to_string()),
            ("ARAMCO".to_string(), "10".to_string()),
            ("NUIG".to_string(), "10".to_string()),
        ]
        .into_iter()
        .collect::<HashMap<String, String>>();

        let expected_vec = vec!["CANTERA_10".to_string(), "ARAMCO_10".to_string(), "NUIG_10".to_string()];
        let res: HashSet<String> = cipher_vector_of_shortcuts(map).into_iter().collect();
        let exp: HashSet<String> = expected_vec.into_iter().collect();
        assert_eq!(res, exp);
    }

}
