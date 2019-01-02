

use std::ffi::OsString;
use regex::Regex;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap,LinkedList};

struct FileIndexer{
    file_map:HashMap<String,Vec<OsString>>,
    is_anime:Regex,
    title:Regex
}

impl FileIndexer{
    pub fn new() -> Self{
        let title = match Regex::new(r"(?m)\](.*?)-") {
            Ok(r) => r,
            Err(e) => {
                panic!("Could not compile regex: {}", e)
            }
        };
        let is_anime = match Regex::new(r"^\[([a-zA-Z].*?)\]") {
            Ok(r) => r,
            Err(e) => {
                panic!("Could not compile regex: {}", e);
            }
        };
        let mut file_map:HashMap<String,Vec<OsString>> = HashMap::new();
        return FileIndexer{file_map,is_anime,title};
    }
    
    pub fn index(mut self,search_path:&Path){
        //takes ownership of self
        for entry in search_path.read_dir().expect("Could not read directory at search_path"){
            let file = entry.unwrap();
            //covnert the filename from osstring to String which we can work with
            let file_name = file.file_name().into_string().unwrap();
            let ospath = file.path().into_os_string();

            if self.is_anime.is_match(file_name.as_str()){
                match self.title.find(file_name.as_str()){
                    Some(m) =>{
                        //clean  up our match output a bit
                        //put the title into a map of lists ... might be a better data structure/ actual struct
                        let mut key = &file_name[m.start()+1..m.end()-1];
                        key = key.trim();
                        if self.file_map.contains_key(key){
                            let v = self.file_map.get_mut(key).unwrap();
                            v.push(ospath);
                        }else{
                            let vec = vec!(ospath);
                            self.file_map.insert(key.to_string(), vec);
                        }
                    },
                    None => {
                        //could not get the show name from the file name, but we still believe this to be an anime 
                        //so we should put it into an unknown category
                        //TODO
                        //println!("unknown file could not map {:?}",ospath );
                        warn!("unknown file could not map {:?}",ospath );
                    }
                }
            }
        }
    }

    fn get_show_name(&self, file_name:String) -> Option<String>{
        
                 match self.title.find(file_name.as_str()){
                    Some(m) =>{
                        //clean  up our match output a bit
                        //put the title into a map of lists ... might be a better data structure/ actual struct
                        let mut key = &file_name[m.start()+1..m.end()-1];
                        key = key.trim();
                        return Some(key.to_string());
                        /* if self.file_map.contains_key(key){
                            let v = self.file_map.get_mut(key).unwrap();
                            v.push(ospath);
                        }else{
                            let vec = vec!(ospath);
                            self.file_map.insert(key.to_string(), vec);
                        } */
                    },
                    None => {
                        //could not get the show name from the file name, but we still believe this to be an anime 
                        //so we should put it into an unknown category
                        //TODO
                        //println!("unknown file could not map {:?}",ospath );
                        warn!("unknown file could not map {:?}",file_name );
                        return None;
                    }
                }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_file_indexer(){
        let fileindexer = FileIndexer::new();
    }
    
    #[test]
    fn test_get_show_name(){
        let fileindexer = FileIndexer::new();
        let input = "[Doki] Shinmai Maou no Testament - OVA (1920x1080 HEVC BD FLAC) [B8D7528D].mkv";
        let expected = "Shinmai Maou no Testament";
        let actual = fileindexer.get_show_name(String::from(input));
        match actual{
            Some(x) => assert_eq!(x,expected ),
            None => assert!(false)
        }
    }
    #[test]
    fn test_get_show_name_when_uses_underscores(){
        let fileindexer = FileIndexer::new();
        let input = "[HorribleSubs]_Shingeki_no_Kyojin_S2_-_26_[720p].mkv";
        let expected = "Shingeki no Kyojin S2";
        let actual = fileindexer.get_show_name(String::from(input));
        match actual{
            Some(x) => assert_eq!(x,expected ),
            None => assert!(false)
        }
    }
}