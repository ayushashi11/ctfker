use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::thread;

pub struct Deployed{
    port: i32,
    th: thread::JoinHandle<()>
}

pub fn getpaths(path: PathBuf) -> std::io::Result<(Vec<String>, Vec<i32>)>{
    let fpath = path.join("projs.toml");
    let toml_file = tsu::toml_from_path(fpath.to_str().ok_or(Error::new(ErrorKind::Other, "invalid path"))?);
    let projs = toml_file.get("projects").unwrap();
    if !projs.is_array(){
        if projs.is_str(){ return Ok((vec![], vec![])); }
    }
    println!("{:#?}", projs);
    let mut res:Vec<String> = vec![];
    let mut ports: Vec<i32> = vec![];
    let mut c = 8081;
    for p in  projs.as_array().unwrap(){
        let ppath = path.join(p.to_string()).join("project.toml");
        let project_toml_file = tsu::toml_from_path(ppath.to_str().ok_or(Error::new(ErrorKind::Other, "invalid path 0x2"))?);
        if let Some(val) = project_toml_file.get("common"){
            if val.as_bool().unwrap_or(false){
                //
            }
            else{
                res.push(p.to_string());
            }
        }
        else {
            res.push(p.to_string());
        }
    }
    Ok((res, ports))
}