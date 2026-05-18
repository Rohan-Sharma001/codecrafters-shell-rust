use std::env;

pub fn separator(command: &str) -> Vec<String> { //TOKENIZE
    let mut vector_of_args: Vec<String> = Vec::<String>::new();
    let home_dir = match env::var("HOME") {
        Ok(home) => home,
        Err(_) => "".to_string()
    };

    let mut active_single_quotes = false; 
    let mut active_double_quotes = false;
    let mut i = 0;
    while i < command.len() {
        let mut substring_to_be_added: String = "".to_string();
        let mut j: usize = i;
        while j < command.len() {
            let character_j = command.as_bytes()[j] as char;

            if character_j == '\'' {
                if !active_double_quotes {active_single_quotes = !active_single_quotes; j+=1; continue;}
            }
            if character_j == '\"' {
                if !active_single_quotes {active_double_quotes = !active_double_quotes; j+=1; continue;}
            }

            //CHARACTER IF_ELSE
            //Double quotes not implemented yet
            if active_single_quotes {
                substring_to_be_added.push(character_j);
            }
            else if active_double_quotes {
                // substring_to_be_added.push(character_j);
                if character_j == '\\' && j+1 < command.len() {
                    let character_j1 = command.as_bytes()[j+1] as char;
                    if character_j1 == '\"' || character_j1 == '\\' || character_j1 == '$' || character_j1 == '`' || character_j1 == '\n'  {
                        substring_to_be_added.push(character_j1);
                        j+=1;
                    }
                    else {substring_to_be_added.push(character_j)};
                } else {substring_to_be_added.push(character_j)};
            }
            else {
                if character_j == ' ' {break;}
                else if character_j == '~' {substring_to_be_added.push_str(&home_dir);}
                else if character_j == '\\' {
                    if j+1 < command.len() {
                        j+=1;
                        substring_to_be_added.push(command.as_bytes()[j] as char);
                    }
                }
                else {substring_to_be_added.push(character_j);}
            }
            j+=1;
        }
        if substring_to_be_added.len() > 0 {vector_of_args.push(substring_to_be_added);}
        i = j + 1;
    }
    return vector_of_args;
}

//SHELL BUILTINS
