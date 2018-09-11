#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
use std::fs;
use std::io::{BufReader,BufWriter,Write,Read};
use std::path::Path;
fn main()->std::io::Result<()> {
    let args:Vec<_> = std::env::args().collect();
    let mut wtr = BufWriter::new(fs::File::create("./data/tweets.csv")?);
    for Video{title,url,scenes} in parse_directly(&Path::new(&args[1])).unwrap(){
        for Scene{script,start_time} in scenes{
            writeln!(wtr,"{}、from {} {}?t={}",script,title,url,start_time)?;
        }
    }
    Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
struct Video{
    title:String,
    url:String,
    scenes:Vec<Scene>,
}

#[derive(Debug,Serialize,Deserialize)]
struct Scene{
    script:String,
    start_time:String
}

fn parse_directly(path:&Path)->std::io::Result<Vec<Video>>{
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
