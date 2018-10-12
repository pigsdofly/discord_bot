pub mod boorus {

    use curl::easy::Easy;
    use std::str;
    use rand::prelude::*;
    use serenity::framework::standard::Args;

    pub fn get_booru_link(booru: &str, content: String) -> (String, String) {

        match booru {
            "danbooru" => danbooru_link(content),
            "gelbooru" => gelbooru_link(content),
            "safebooru" => safebooru_link(content),
            _ => (String::from("Error, incorrect input"), String::new()),
        }
    }

    pub fn parse_args(mut args: Args, lim: usize) -> Option<String> {
        let mut tags = Vec::new();
        for arg in args.iter::<String>() {
            tags.push(arg.unwrap());
        }
        
        match tags.len() {
            1 => Some(tags[0].clone()),
            x if x < lim => {
                let mut argstr = String::new();
                for tag in tags {
                    if argstr == "" {
                        argstr.push_str(tag.as_str());
                    } else {
                        argstr.push_str(format!("+{}",tag).as_str());
                    }
                }
                Some(argstr)
            },
            _ => None,
        }

    }

    fn danbooru_link(tags: String) -> (String, String) {
        let url = "https://danbooru.donmai.us/posts";
        let api_str = format!("{}.json?tags={}&random=true&limit=1",
                                       url, tags);
        let res = transfer(api_str);

        let result : Vec<&str> = res.split(|c| c == ',' || c == ':').collect();             
        (format!("{}/{}", url,&result[1]), String::new())
    }

    fn safebooru_link(tags: String) -> (String, String) {
        let url = "https://safebooru.org/index.php?page=";
        let pid: u8 = random();
        let api_str = format!("{}dapi&s=post&q=index&tags={}&limit=1&pid={}", url, tags, pid);
        let res = transfer(api_str);
        let res : Vec<&str> = res.split(|c| c == ' ' || c == ',').collect();
        let mut result = String::new();
        let mut image_url = String::new();
        for r in res {
            let temp : Vec<&str> = r.split('=').collect();
            if temp[0] == "file_url" {
                image_url = temp.get(1).expect("No value found").to_string();

            }
            if temp[0] == "id" {
                let id = temp.get(1).expect("No value found").to_string();
                let id_len = id.len();
                
                result = format!("{}post&s=view&id={}",url, &id[1..id_len-1]);
            }

        }
        let image_url = &image_url[1..image_url.len()-1];
        let image_url = format!("https:{}",image_url);
        (result, image_url)
    }

    fn gelbooru_link(tags: String) -> (String, String) {
        let url = "https://gelbooru.com/index.php?page=";
        let api_str = format!("{}dapi&s=post&q=index&tags={}&limit=500", url, tags);
        let res = transfer(api_str);
        let res : Vec<&str> = res.split(|c| c == ' ' || c == ',').collect();
        let mut image_urls = Vec::new();
        let mut ids = Vec::new();
        let mut rng = thread_rng();
        for r in res {
            let temp : Vec<&str> = r.split('=').collect();
            if temp[0] == "file_url" {
                match temp.get(1) {
                    Some(x) => image_urls.push(x.to_string()),
                    None => (),
                };

            }
            if temp[0] == "id" {
                match temp.get(1) {
                    Some(x) => {
                        let id = x.to_string();
                        let id_len = id.len();
                        ids.push(String::from(&id[1..id_len-1]));
                    },
                    None => (),
                };
                
            }

        }
        let id: usize = rng.gen_range(0,image_urls.len()-1);

        let url = format!("{}post&s=view&id={}", url, &ids[id]);
       
        (url, image_urls[id].clone())
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
