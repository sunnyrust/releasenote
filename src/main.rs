#[macro_use]
extern crate clap;
extern crate libc;
extern crate chrono;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use rusqlite::{params,Connection,NO_PARAMS};
use serde::{Deserialize, Serialize};
//use time::Timespec;  //Also, it should be noted, that time v0.1.* must be used, because Timespec was removed in v0.2.0.
use clap::App;
use std::fmt;
use chrono::prelude::*;

const HEADER: &'static str = "./temple/header.tpl"; 
const TABLE_TR: &'static str = "./temple/tr.tpl"; 
const FOOTER: &'static str = "./temple/footer.tpl"; 
const DB_NAME: &'static str = "releaseNote.db"; 
const HTML: &'static str = "releaseNote"; 
const SOURCE_NAME: &'static str = "rn.txt";
const MARKDOWN_NAME: &'static str = "releaseNote.md"; 

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Releasenote {
    id: Option<i32>,
    env: String,
    owner: String,
    name: String,
    version:  Option<String>,
    address: Option<String>,
    git: Option<String>,
    docker: Option<String>,
    pubtime: Option<String>,
    data: Option<Vec<u8>>,
}
//
fn create_db(conn: &Connection) {
    match conn.execute(
        "CREATE TABLE releasenote (
                    id              INTEGER PRIMARY KEY,
                    env             TEXT NOT NULL,
                    owner           TEXT NOT NULL,
                    name            TEXT NOT NULL,
                    address         TEXT  NULL,
                    version         TEXT  NULL,
                    git             TEXT  NULL,
                    docker          TEXT  NULL,
                    pubtime         TEXT NOT NULL,
                    data            BLOB
                  )",
        NO_PARAMS,
    ){
        Ok(_created) => println!("created is ok"),
        Err(err) => println!("created failed: {}", err),
    }
}


/// to_html
fn to_html( strenv:String) -> std::io::Result<()>{
    let mut file = File::open(HEADER.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut html:String=contents;

    file = File::open(TABLE_TR.to_string())?;
    let mut tr = String::new();
    file.read_to_string(&mut tr)?;
    let conn = Connection::open(DB_NAME).unwrap();
    let mut stmt = conn.prepare("SELECT id,env,owner,name,version,address,git,docker, pubtime,data FROM releasenote  WHERE env  like '%' || ? || '%'").unwrap();
    let releasenote_iter = stmt.query_map(params![&strenv], |row| {
        Ok(Releasenote {
            id: row.get(0).unwrap(),
            env: row.get(1).unwrap(),
            owner: row.get(2).unwrap(),
            name: row.get(3).unwrap(),
            version: row.get(4).unwrap(),
            address: row.get(5).unwrap(),
            git: row.get(6).unwrap(),
            docker: row.get(7).unwrap(),
            pubtime: row.get(8).unwrap(),
            data: row.get(9).unwrap(),
        })
    }).unwrap();
    let mut i=0;
    for rn in releasenote_iter  {
        let mut tr_tmp = String::new();
        tr_tmp=tr.clone();
        if i % 2==0{
            tr_tmp = str::replace(&tr_tmp, "#FFC0CB", "#FAEBD7");
        }
        //#FAEBD7
        
        let r=rn.unwrap();
        let mut name=(r.name).to_string()+"<br />("+&(r.owner).to_string()+")"+"<br />("+&(r.env).to_string()+")";
        tr_tmp = str::replace(&tr_tmp, "#0#", &name);
        tr_tmp = str::replace(&tr_tmp, "#1#", &r.address.unwrap().as_str());
        tr_tmp = str::replace(&tr_tmp, "#2#", &r.version.unwrap().as_str());
        tr_tmp = str::replace(&tr_tmp, "#3#", &r.git.unwrap().as_str());
        tr_tmp = str::replace(&tr_tmp, "#4#", &r.docker.unwrap().as_str());
        html += &tr_tmp;
        i+=1;
    }

    file = File::open(FOOTER.to_string())?;
    let mut cont = String::new();
    file.read_to_string(&mut cont)?;
    html += &cont;
    let mut html_filename=HTML.to_string();
    let now:NaiveDateTime = Local::now().naive_local();
    html_filename += &now.format("%Y%m%d-%H%M%S").to_string();
    html_filename += ".html";
    let mut f = File::create( &html_filename)?;
    f.write_all(html.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

///此处让我深感疑惑，就是错误处理机制。还有？的使用方式。
///read_file
fn read_file(filename: String) {
    let  _file = match std::fs::File::open(filename) {
        Ok(file) => {
            let conn = Connection::open(DB_NAME).unwrap();
            use encodingbufreader::{BufReaderEncoding};
            use encoding::all::{UTF_8};
            for line in BufReaderEncoding::new(file, UTF_8).lines().map(|l| l.unwrap()){
                let json=line.to_string();
                if json.len()>3{
                    let u: Releasenote = serde_json::from_str(&json).unwrap();
                    let pubtime = Some(u.pubtime);
                    conn.execute(
                        "INSERT INTO releasenote (env,owner,name,address,version,git,docker, pubtime,data)
                                  VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                        params![u.env,u.owner,u.name,u.address,u.version,u.git,u.docker,pubtime,u.data],
                    ).unwrap();
                }              
             }
        }
        Err(_why) => {
            println!("文件({ })打开失败.", SOURCE_NAME);
        }
    };
  // Ok(())
}


/// flag用于参数
fn flag() -> () {
    let s = "Version:".to_owned()
        + &crate_version!().to_owned()
        + "  git:"
        + &crate_description!().to_owned();
    let matches = App::new("Sunnycat")
        .version(&*s)
        .author(crate_authors!())
        .about(
            "Release Note 的工具.
        例子:./relelaenote --help
        ",
        )
        .args_from_usage("-c,--createdb '创建数据库'")
        .args_from_usage("-s,--show=[NAME] '显示某个name的信息'")
        .args_from_usage("-i,--insert '插入数据库'")
        .args_from_usage("-H,--html=[ENV] '数据库到HTML'")
        .get_matches();

    if matches.is_present("createdb") {
        let conn = Connection::open(DB_NAME).unwrap();
        create_db(&conn);
        print!(
            " _|￣|○ -----🎉🎉🎉👍💁👌 RUST{}  ⚽🎍😍🎉🎉🎉------○|￣|_  \n",
            "lineonly"
        );
        use std::process;
        process::exit(0x0100);
    }

    if matches.is_present("insert"){
        let _err=read_file(SOURCE_NAME.to_string());
    }

    if matches.is_present("html"){
        let env = matches.value_of("html").unwrap_or("");
        // println!{"env:{:#?}",env}
        let _err= to_html(env.to_string());
    }
}


fn main() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    flag();
    // let now:NaiveDateTime = Local::now().naive_local();
    // println!("time:{}",now.format("%Y%m%d-%H%M%S").to_string());
}
