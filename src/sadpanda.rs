use serde_json;
use reqwest;
pub mod sadpanda {
/* Module for returning tags and cover art from a certain website. */
    use curl::easy::Easy;
    use std::collections::HashMap;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use serde_json::{Value};
    
    
    // Returns image url and tags
    pub fn retrieve_tags(url: String) -> (String, String, String) {
        let (g_id, g_tok) = split_url(url);
        let js_string = make_json(g_id,g_tok);
        let result = clean_post(js_string).unwrap();
        let title = result["gmetadata"][0]["title"].to_string().clone();
        let thumbnail = result["gmetadata"][0]["thumb"].to_string().clone();
        let tags_raw = result["gmetadata"][0]["tags"].as_array().unwrap();
        let mut tags : String = String::new();
        let mut previous : String = String::new();
        let mut first : String = String::new();
        for t in tags_raw {
            //take tag, check first word

            let string_t = t.to_string();
            let split_t = string_t.split(':');
            let mut split_tag = split_t.collect::<Vec<&str>>(); 
            let mut tag = String::new();
            if split_tag.len() == 1 {
                first = String::from("\"misc");
                tag = String::from(&split_tag[0][1..]);
            } else {
                first = String::from(split_tag[0]);
                tag = split_tag[1].to_string();
            }
            let mut first_char : String = String::new();
            if first != previous {
                let temp = format!("\n{}: ", String::from(&first[1..]));
                tags.push_str(&temp);
                first_char = String::from("");
            } else {
                first_char = String::from(", ");
            }

            previous = first.clone();


            
            let tag = format!("{}{}",first_char, String::from(&tag[..tag.len()-1]));
            tags.push_str(&tag);
        }
        
        (title, thumbnail, tags)
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
        format!("{}", request.to_string())
    }
    
    fn post(data: String) -> Result<reqwest::Response, reqwest::Error> {
        let mut head = HeaderMap::new();
        head.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
        let client = reqwest::Client::new();
        let res = client.post("https://api.e-hentai.org/api.php").headers(head).body(data);
        let res = res.send();
        
        Ok(res?)
    }
    
    
    fn clean_post(data: String) -> Result<serde_json::Value, serde_json::Error> {
        let mut res = post(data).unwrap();
        /*let mut r = match(res) {
            Err(e) => return Err(e),
            Ok(r) => r,
        };*/
        let result : Value = serde_json::from_str(&res.text().unwrap())?;
        
        Ok(result)
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
        
        #[test]
        fn posted() {
            
            assert!(clean_post(String::from(r#"{
  "method": "gdata",
  "gidlist": [
      [618395,"0439fa3666"]
  ],
  "namespace": 1
}"#)).is_err(),"error");
        }
        
        #[test]
        fn tags() {
            assert_eq!(retrieve_tags(String::from("https://e-hentai.org/g/618395/0439fa3666/")),(String::from("(Kouroumu 8) [Handful☆Happiness! (Fuyuki Nanahara)] TOUHOU GUNMANIA A2 (Touhou Project)"),
            String::from("https://ehgt.org/14/63/1463dfbc16847c9ebef92c46a90e21ca881b2a12-1729712-4271-6032-jpg_l.jpg"), 
            String::from("")));
        }
    }
}


