pub mod boorus {

    use curl::easy::Easy;
    use std::str;
    use rand::prelude::*;
    use serenity::framework::standard::Args;

    pub fn get_booru_link(booru: &str, content: String) -> (String, String) {

        match booru {
            "danbooru" => danbooru_link(content),
            "gelbooru.com" => gelbooru_link(content, booru),
            "safebooru.org" => gelbooru_link(content, booru),
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
            x if x <= lim => {
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

    fn gelbooru_link(tags: String, booru: &str) -> (String, String) {
        let url = format!("https://{}/index.php?page=", booru);
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

        let url = format!("{}post&s=view&id={}", &url, &ids[id]);
        let return_image = image_urls[id].clone();
        let return_image = return_image[1..return_image.len()-1].to_string();
       
        (url, return_image)
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
