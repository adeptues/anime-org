
#[macro_use] 
extern crate serde_derive;
#[macro_use]
extern crate structopt;

extern crate serde;
extern crate serde_xml_rs;
extern crate regex;
#[macro_use]
extern crate log;
extern crate env_logger;

mod anidb;
mod indexing;

use structopt::StructOpt;
use std::ffi::OsString;

use std::path::Path;
use std::collections::{HashMap};
use env_logger::{Builder, Target};

#[derive(StructOpt, Debug)]
#[structopt(name = "anime-org")]
struct Opt{
    /// The search path for where to look for files that need moving
    #[structopt(short = "s", long = "search_path")]
    search_path:String,
    /// The output directory, where directories should be created and files moved too
    #[structopt(short = "o", long = "output_path")]
    output_path:String
}

fn main() {
    //let path = "/home/tom/tom/anime-org/anime-titles.xml";
    
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    info!("********** Start Up ************");
    //let search_path = Path::new("/home/tom/tom/anime-org/files");
    //TODO change opt to have a Path type instead of search path
    let opt = Opt::from_args();
    debug!("search_path {:?}",opt.search_path);
    debug!("output_path {:?}",opt.output_path);
    let search_path = Path::new(&opt.search_path);
    
    /* let db = match anidb::AnimeTitles::load(p) {
        Ok(s) => s,
        Err(_) => panic!("could not load the database")
    }; */
    //Compile our regexes
    /* let title_regex = match Regex::new(r"(?m)\](.*?)-") {
        Ok(r) => r,
        Err(e) => {
            error!("Could not compile regex: {}", e);
            return;
        }
    };
    let is_anime_regex = match Regex::new(r"^\[([a-zA-Z].*?)\]") {
        Ok(r) => r,
        Err(e) => {
            error!("Could not compile regex: {}", e);
            return;
        }
    }; */
    //let mut file_map:HashMap<String,Vec<OsString>> = HashMap::new();
    //find all the mkv files and apply our is anime regex
    /* for entry in search_path.read_dir().expect("Could not read directory at search_path"){
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
                    //println!("unknown file could not map {:?}",ospath );
                    warn!("unknown file could not map {:?}",ospath );
                }
            }
        }
    } */
    let indexer = indexing::FileIndexer::new();
    let file_map = indexer.index(search_path);
    //we've now built our filemap, we need to search anidb for the proper filename this can be a future improvement
    
    //for now turn the filemap into a list of tuple where each tuple describes the move operation
    let tuples = to_tuple_list(file_map,opt.output_path);
    for pair in tuples{
        let from = Path::new(&pair.0);
        let to = Path::new(&pair.1);
        let mut to_path_buf = to.to_path_buf();
        to_path_buf.pop();
        let to_location = Path::new(&to_path_buf);
        
        if to_location.exists() && to_location.is_dir(){
            //go ahead and copy
            match std::fs::rename(from, to){
                Ok(_) => info!("moved file {:?} to {:?}",from,to ),
                Err(_) => error!("Could not move {:?} to {:?}",from,to)
            }
            
        }else{
            //create the directory and copy
            match std::fs::create_dir(to_location){
                Ok(_) => info!("Creating directory {:?}",to_location ),
                Err(e) => error!("{} Could not create directory at {:?}",e,to_location)
            }
            //std::fs::rename(from, to).expect("Could not copy file");
            match std::fs::rename(from, to){
                Ok(_) => info!("moved file {:?} to {:?}",from,to),
                Err(e) => error!("{:?} Could not move {:?} to {:?}",e,from,to)
            }
        }
    }
}



fn to_tuple_list(file_map:HashMap<String,Vec<OsString>>,output_dir:String) -> Vec<(OsString,OsString)>{
    //let outputDir = "/home/tom/tom/anime-org/files/output";
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
            let new_path = Path::new(&output_dir);
            let new_path = new_path.join(k).join(orginal_path.file_name().unwrap());
            output.push((orginal_path.to_path_buf().into_os_string(),new_path.to_path_buf().into_os_string()));
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
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

