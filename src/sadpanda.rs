pub mod sadpanda {
/* Module for returning tags and cover art from a certain website. */
    use curl::easy::Easy;
    // Returns image url and tags
    pub fn retrieve_tags(url: String) -> (String, Vec<String>) {
        let (g_id, g_tok) = split_url(url);
        let mut v = Vec::new();
        
        v.push(String::from("a"));
        (String::from(""),v)
    }
    
    fn split_url(url: String) -> (i32, String) {
        let s_url : Vec<&str> = url.split('/').collect();
        let gid = s_url[4].to_string();
        let gid = gid.parse::<i32>().unwrap();
        let gtok = s_url[5].clone();
        (gid, String::from(gtok))
    }
    
    
    #[cfg(test)]
    mod test {
        use super::*;
        
        #[test]
        fn split() {
            assert_eq!(split_url(String::from("https://e-hentai.org/g/618395/0439fa3666/")), 
                      (618395, String::from("0439fa3666")));
        }
    }
}


