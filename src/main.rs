#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate rand;
use std::fs;
use std::io::{BufReader,BufWriter,Write,Read};
use std::path::Path;
use rand::Rng;
fn main()->std::io::Result<()> {
    let args:Vec<_> = std::env::args().collect();
    let mut wtr = BufWriter::new(fs::File::create("./data/tweets.csv")?);
    let mut res = vec![];
    for Video{title,url,scenes} in parse_directly(&Path::new(&args[1])).unwrap(){
        for Scene{script,start_time} in scenes{
            let tweet = format!("{} from {} {}?t={}",script,title,url,start_time);
            writeln!(wtr,"{}",tweet)?;
            res.push(tweet)
        }
    }
    let mut rng = rand::thread_rng();
    println!("{}",rng.choose(&res).unwrap());
    Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Video{
    pub title:String,
    pub url:String,
    pub scenes:Vec<Scene>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Scene{
    pub script:String,
    pub start_time:String
}

pub fn parse_directly(path:&Path)->std::io::Result<Vec<Video>>{
    if path.is_dir(){
        Ok(fs::read_dir(path)?.filter_map(|e|e.ok())
           .map(|e|e.path())
           .filter(|e|match e.extension(){
               Some(ext) => ext == "toml",
               None => false,
           })
           .filter_map(|e|parse_file(&e).ok())
           .collect())
    }else{
        Err(std::io::Error::from(std::io::ErrorKind::Other))
    }
}

fn parse_file(path:&Path)->std::io::Result<Video>{
    let mut input = String::new();
    BufReader::new(&fs::File::open(path)?).read_to_string(&mut input)?;
    toml::from_str(&input)
        .map_err(|e|{eprintln!("{:?}",e);
                     std::io::Error::from(std::io::ErrorKind::Other)})
}
