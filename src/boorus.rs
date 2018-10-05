pub mod boorus {

    use curl::easy::Easy;
    use std::collections::HashMap;
    use std::str;


    pub fn get_booru_link<'a>(booru: &'a str, content: String) -> String {
        let booru_names : HashMap<&str, &str> = 
            [("danbooru", "https://danbooru.donmai.us/posts"),
             ("gelbooru", "gelbooru.com"),
             ("safebooru", "safebooru.org")]
                 .iter().cloned().collect();
        
        if !booru_names.contains_key(booru) {
            let error_str = "Error";
            &error_str;
        }
        
        let name = booru_names[booru];
        let mut handle = Easy::new();
        let string_to_send = format!("{}.json?tags={}&random=true&limit=1",
                                       name, content);
        let string_to_send = string_to_send.as_str();

        handle.url(string_to_send).unwrap();

        let mut buf = Vec::new();
        
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                buf.extend_from_slice(&data);
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }
        
        let res = String::from_utf8(buf).unwrap();
        let result : Vec<&str> = res.split(|c| c == ',' || c == ':').collect();             
        
        let result = format!("{}/{}", name,&result[1]);

        let content = result;
        content
    }

}
