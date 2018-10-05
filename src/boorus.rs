pub mod boorus {

    use curl::easy::Easy;
    use std::str;
    use rand::prelude::*;

    pub fn get_booru_link<'a>(booru: &'a str, content: String) -> String {

        match booru {
            "danbooru" => danbooru_link(content),
            "gelbooru" => gelbooru_link(content),
            "safebooru" => safebooru_link(content),
            _ => String::from("Error, incorrect input"),
        }
    }

    fn danbooru_link(tags: String) -> String {
        let url = "https://danbooru.donmai.us/posts";
        let api_str = format!("{}.json?tags={}&random=true&limit=1",
                                       url, tags);
        let res = transfer(api_str);

        let result : Vec<&str> = res.split(|c| c == ',' || c == ':').collect();             
        format!("{}/{}", url,&result[1])
    }

    fn safebooru_link(tags: String) -> String {
        let url = "https://safebooru.org/";
        let pid: u8 = random();
        let api_str = format!("{}index.php?page=dapi&s=post&q=index&tags={}&limit=1&pid={}", url, tags, pid);
        let res = transfer(api_str);
        

        res
    }

    fn gelbooru_link(tags: String) -> String {
        tags
    }

    fn transfer(url: String) -> String { 
        let mut handle = Easy::new();
        handle.url(url.as_str()).unwrap();

        let mut buf = Vec::new();
        
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                buf.extend_from_slice(&data);
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }
        
        String::from_utf8(buf).unwrap()
    }

}
