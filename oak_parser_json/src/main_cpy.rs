#![feature(plugin)]
#![plugin(oak)]

extern crate oak_runtime;
use oak_runtime::*;

grammar! json {
    // #![show_api]
    program = lbrace spacing json_members (coma lbrace json_members rbrace)* rbrace
    // well_formed_json = lbrace json_expr? (("," json_expr)+)? rbrace
    //JSON

    json_expr
    = number > make_json_number
    /   json_string > make_json_string
    /   json_array > make_json_array

    // json_object
    // = lbrace json_members (coma json_members)* rbracket > make_json_object

    json_members
    = json_pair (coma json_pair)* > make_json_members

    json_pair
    = json_string colon json_expr > make_json_pair

    json_array
    = lbracket (json_expr coma)* json_expr spacing rbracket

    //generic types
    dquote = "\"" -> (^)
    spacing = [" \n\r\t"]* -> (^)
    digit = ["0-9"]
    colon = ":" spacing -> (^)
    semicolon = ";" spacing -> (^)
    coma = "," spacing -> (^)
    number = digit+ spacing > to_number
    not_zero_digit = ["1-9"]
    digits = digit+
    // not_zero_number = not_zero_digit digit+ spacing > to_number
    // negative_number = "-" number
    // negative_digit = "-" not_zero_digit

    json_string
    = dquote json_char+ dquote spacing > to_string

    json_char
    = ["a-zA-Z"]

    lparen = "(" spacing -> (^)
    rparen = ")" spacing -> (^)
    lbracket = "[" spacing -> (^)
    rbracket = "]" spacing -> (^)
    lbrace = "{" spacing -> (^)
    rbrace = "}" spacing -> (^)

    use std::str::FromStr;

    pub type PExpr = Box<JSONPair>;
    pub type JObj = Box<JSONObject>;

    //Enums
    #[derive(Debug)]
    pub enum JSONExpr {
        Str(String),
        Number(u32),
        Array(Vec<Box<JSONExpr>>)
    }

    #[derive(Debug)]
    pub enum JSONPair {
        Pair(String, Box<JSONExpr>),
        Json(Vec<Box<JSONPair>>)
    }

    #[derive(Debug)]
    pub enum JSONObject {
        Json(Vec<Box<JSONPair>>)
    }

    //Functions

    fn make_json_number(number:u32)-> Box<JSONExpr> {
        Box::new(JSONExpr::Number(number))
    }

    fn make_json_string(string:String) -> Box<JSONExpr> {
        Box::new(JSONExpr::Str(string))
    }

    fn make_json_pair(string:String, expr:Box<JSONExpr>) -> PExpr {
        Box::new(JSONPair::Pair(string,expr))
    }

    fn make_json_array(array:Vec<Box<JSONExpr>>, front:Box<JSONExpr>) -> Box<JSONExpr> {
        let mut vector = Vec::new();
        for i in array{
            vector.push(i);
        }
        vector.push(front);
        Box::new(JSONExpr::Array(vector))
    }

    fn make_json_members(pair: Box<JSONPair>, rest: Vec<Box<JSONPair>>) -> PExpr {
        let mut vector = vec![pair];
        for i in rest{
            vector.push(i);
        }
        Box::new(JSONPair::Json(vector))
    }

    // fn make_json_object(object: Box<JSONObject>, rest: Vec<Box<JSONObject>>) -> JObj {
    //     let mut vector = vec![object];
    //     for i in rest{
    //         vector.push(i);
    //     }
    //     Box::new(JSONObject::Json(vector))
    // }

    fn to_number(raw_text: Vec<char>) -> u32 {
        u32::from_str(&*to_string(raw_text)).unwrap()
    }


    fn to_string(raw_text: Vec<char>) -> String {
        raw_text.into_iter().collect()
    }

}


fn analyse_state(state: ParseState<StrStream, json::PExpr>)  {
    use oak_runtime::parse_state::ParseResult::*;
    match state.into_result() {
        Success(data) => println!("Full match: {:?}", data),
        Partial(data, expectation) => {
            println!("Partial match: {:?} because {:?}", data, expectation);
        }
        Failure(expectation) => {
            println!("Failure: {:?}", expectation);
        }
    }
}

fn main() {
     analyse_state(json::parse_program("{\"ue\" : \"pstl\" }".into_state())); // Complete
      analyse_state(json::parse_program("{\"ue\" : \"pstl\", ".into_state())); // Partial
      analyse_state(json::parse_program("{\"pstl\"".into_state())); // Error

    let json =
        "{
        \"ue\" : \"pst\",
        \"note\" : [20, 21, 22],
        \"enseignement\" : \"ptal sensei\"
        }
        ";
    let mut sjson = json.into_state();
    analyse_state(json::parse_program(sjson));

    let json_full =

    "{
        \"glossary\": {
            \"title\": \"example glossary\",
    		\"GlossDiv\": {
                \"title\": \"S\",
    			\"GlossList\": {
                    \"GlossEntry\": {
                        \"ID\": \"SGML\",
    					\"SortAs\": \"SGML\",
    					\"GlossTerm\": \"Standard Generalized Markup Language\",
    					\"Acronym\": \"SGML\",
    					\"Abbrev\": \"ISO 8879:1986\",
    					\"GlossDef\": {
                            \"para\": \"A meta-markup language, used to create markup languages such as DocBook.\",
    						\"GlossSeeAlso\": [\"GML\", \"XML\"]
                        },
    					\"GlossSee\": \"markup\"
                    }
                }
            }
        }
    }" ;

    sjson = json_full.into_state();
    analyse_state(json::parse_program(sjson));

}
