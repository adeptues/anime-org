#[macro_use] 
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;
extern crate regex;

use std::ffi::OsString;
use regex::Regex;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap,LinkedList};
fn main() {
    let path = "/home/tom/tom/anime-org/anime-titles.xml";
    let search_path = Path::new("/home/tom/tom/anime-org/files");
    let p = Path::new(path);
    /* let db = match anidb::AnimeTitles::load(p) {
        Ok(s) => s,
        Err(_) => panic!("could not load the database")
    }; */
    //Compile our regexes
    let title_regex = match Regex::new(r"(?m)\](.*?)-") {
        Ok(r) => r,
        Err(e) => {
            println!("Could not compile regex: {}", e);
            return;
        }
    };
    let is_anime_regex = match Regex::new(r"^\[([a-zA-Z].*?)\]") {
        Ok(r) => r,
        Err(e) => {
            println!("Could not compile regex: {}", e);
            return;
        }
    };
    let mut file_map:HashMap<String,Vec<OsString>> = HashMap::new();
    //find all the mkv files and apply our is anime regex
    for entry in search_path.read_dir().expect("Could not read directory at search_path"){
        let file = entry.unwrap();
        //covnert the filename from osstring to String which we can work with
        let file_name = file.file_name().into_string().unwrap();
        let ospath = file.path().into_os_string();

        if is_anime_regex.is_match(file_name.as_str()){
            match title_regex.find(file_name.as_str()){
                Some(m) =>{
                    //clean  up our match output a bit
                    //put the title into a map of lists ... might be a better data structure/ actual struct
                    let mut key = &file_name[m.start()+1..m.end()-1];
                    key = key.trim();
                    if file_map.contains_key(key){
                        let v = file_map.get_mut(key).unwrap();
                        v.push(ospath);
                    }else{
                        let vec = vec!(ospath);
                        file_map.insert(key.to_string(), vec);
                    }
                },
                None => {
                    //could not get the show name from the file name, but we still believe this to be an anime 
                    //so we should put it into an unknown category
                    //TODO
                    println!("unknown file could not map {:?}",ospath );
                }
            }
        }
    }
    //we've now built our filemap, we need to search anidb for the proper filename this can be a future improvement
    
    //for now turn the filemap into a list of tuple where each tuple describes the move operation
    let tuples = to_tuple_list(file_map);
    for pair in tuples{
        let from = Path::new(&pair.0);
        let to = Path::new(&pair.1);
        let mut to_path_buf = to.to_path_buf();
        to_path_buf.pop();
        let to_location = Path::new(&to_path_buf);
        println!("moving file {:?} to {:?}",from,to );
        if to_location.exists() && to_location.is_dir(){
            //go ahead and copy
            std::fs::rename(from, to).expect("Could not copy file");
            println!("\t Success!");
        }else{
            println!("Creating directory {:?}",to_location );
            //create the directory and copy
            std::fs::create_dir(to_location).expect("Could not create directory");
            std::fs::rename(from, to).expect("Could not copy file");
            println!("\t Success!");
        }

    }
}

fn to_tuple_list(file_map:HashMap<String,Vec<OsString>>) -> Vec<(OsString,OsString)>{
    let outputDir = "/home/tom/tom/anime-org/files/output";
    let mut output:Vec<(OsString,OsString)> = Vec::new();
    /* for (k,v) in file_map.iter(){
        //get the filename create a new path
        let output_paths:Vec<OsString> = v.iter().map( |f| Path::new(f).to_path_buf()).map(|mut x| {
            let file_name = x.file_name().unwrap();
            let outpath = Path::new(outputDir);
            let temp = outpath.join(k).join(file_name);
            return temp.to_path_buf().into_os_string();
            }).collect();
            
        let partial:Vec<(OsString,OsString)> = output_paths.iter().zip(v.iter()).collect();
    } */

    for(k,v) in file_map.iter(){
        for i in v{
            let orginal_path = Path::new(i);
            let new_path = Path::new(outputDir);
            let new_path = new_path.join(k).join(orginal_path.file_name().unwrap());
            output.push((orginal_path.to_path_buf().into_os_string(),new_path.to_path_buf().into_os_string()));
        }
    }
    return output;
}

/// Error handling in rust seems to be in limbo at the moment as far as best practices go, so for now until a real solution appears it seems best
/// to create your own application error type and wrap them 
mod error{
    use std;
    //TODO not sure if we actually need to implement the error trait sow we can have debug information

    /// Application error wrapper to umbrella over multiple kinds of errors until theres a proper solution in rust std like the
    /// failure crate
    pub enum Error{
        IO(std::io::Error),
        FileNotFound
    }

    /// Convert std io errors when using the ? operator automatically
    impl From<std::io::Error> for Error{
        fn from(input:std::io::Error) -> Self{
            return Error::IO(input);
        }
    }
}

mod anidb{
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
}
