 //use std::io::Error;
    use error::Error;
    use serde_xml_rs::deserialize;
    use std::fs::File;
    use std::io::prelude::*;
    //use std::io::Result;
    use std::path::Path;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Title{
        #[serde(rename = "type", default)]
        title_type:String,
        #[serde(rename = "xml:lang", default)]
        lang:String,
        #[serde(rename = "$value")]
        value:String
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Anime{
        aid:String,
        #[serde(rename = "title", default)]
        titles:Vec<Title>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AnimeTitles{
        anime:Vec<Anime>
    }
    impl AnimeTitles{
        pub fn find_by_aid(&self,aid:usize) -> Option<&Anime>{
            for anime in self.anime.iter(){
                let id:usize = anime.aid.parse().unwrap();
                if id == aid{
                    return Some(anime);
                }
            }
            return None;
        }
        pub fn load(file_path:&Path) -> Result<AnimeTitles,Error>{
            //load from the xml file and return an animetitles object
            if file_path.exists(){
                let f:File = File::open(file_path)?;
                let anime_titles:AnimeTitles = deserialize(f).unwrap();
                return Ok(anime_titles);
            }
            return Err(Error::FileNotFound)
        }
    }