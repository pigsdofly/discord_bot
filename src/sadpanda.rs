use serde_json;
pub mod sadpanda {
/* Module for returning tags and cover art from a certain website. */
    use curl::easy::Easy;
    //use serde_json::{Result, Value};
    
    
    // Returns image url and tags
    pub fn retrieve_tags(url: String) -> (String, Vec<String>) {
        let (g_id, g_tok) = split_url(url);
        let js_string = make_json(g_id,g_tok);
        let mut v = Vec::new();
        
        v.push(String::from("a"));
        (String::from(""),v)
    }
    
    //Get the id and token from the URL
    fn split_url(url: String) -> (i32, String) {
        let s_url : Vec<&str> = url.split('/').collect();
        let gid = s_url[4].to_string();
        let gid = gid.parse::<i32>().unwrap();
        let gtok = s_url[5].clone();
        (gid, String::from(gtok))
    }
    
    //Create json string out of id and token
    fn make_json(g_id: i32, g_tok: String) -> String{
       
        let request = json!({
            "method": "gdata",
            "gidlist": [
                    [g_id,g_tok] 
            ],
            "namespace": 1
        });
        println!("{}", request.to_string());
        format!("{}", request.to_string())
    }
    
    #[cfg(test)]
    mod test {
        use super::*;
        
        #[test]
        fn split() {
            assert_eq!(split_url(String::from("https://e-hentai.org/g/618395/0439fa3666/")), 
                      (618395, String::from("0439fa3666")));
        }
        
        #[test]
        fn json() {
            assert_eq!(make_json(618395,String::from("0439fa3666")),String::from(r#"{"gidlist":[[618395,"0439fa3666"]],"method":"gdata","namespace":1}"#));
        }
    }
}


