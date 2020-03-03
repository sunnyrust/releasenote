#[macro_use]
extern crate clap;
extern crate libc;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use rusqlite::{params,Connection,NO_PARAMS};
use serde::{Deserialize, Serialize};
//use time::Timespec;  //Also, it should be noted, that time v0.1.* must be used, because Timespec was removed in v0.2.0.
use clap::App;
use std::fmt;

const HEADER: &'static str = "header.tpl"; 
const TABLE_TR: &'static str = "tr.tpl"; 
const FOOTER: &'static str = "footer.tpl"; 
const DB_NAME: &'static str = "releaseNote.db"; 
const HTML: &'static str = "releaseNote.html"; 
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
#[derive(Serialize, Deserialize)]
struct Releasenote {
    id: Option<i32>,
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
fn to_html() -> std::io::Result<()>{
    let mut file = File::open(HEADER.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut html:String=contents;

    file = File::open(TABLE_TR.to_string())?;
    let mut tr = String::new();
    file.read_to_string(&mut tr)?;
    let conn = Connection::open(DB_NAME).unwrap();
    let mut stmt = conn.prepare("SELECT id,name,version,address,git,docker, pubtime,data FROM releasenote").unwrap();
    let releasenote_iter = stmt.query_map(params![], |row| {
        Ok(Releasenote {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            version: row.get(2).unwrap(),
            address: row.get(3).unwrap(),
            git: row.get(4).unwrap(),
            docker: row.get(5).unwrap(),
            pubtime: row.get(6).unwrap(),
            data: row.get(7).unwrap(),
        })
    }).unwrap();

    // for rn in releasenote_iter  {
    //     println!("Found person {:#?}", Some(rn));
    // }

    tr = str::replace(&tr, "#0#", "CMS");
    tr = str::replace(&tr, "#1#", "www.baidu.com");
    html += &tr;
    file = File::open(FOOTER.to_string())?;
    let mut cont = String::new();
    file.read_to_string(&mut cont)?;
    html += &cont;
    // println!("{:#?}",html);
    
    let mut f = File::create( HTML.to_string())?;
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
                // let j=r#"{"name":"SmartOMP","version":"V1.0.1.3-20200301","address":"10.11.35.104:9099","git":"a1d7c36ccc449178254458e14319c8985fe76253","docker":"dockerhub.cloudminds.com/SmartOMP-101","pubtime":"2020-03-01 19:58:25"}"#;
                if json.len()>3{
                    let u: Releasenote = serde_json::from_str(&json).unwrap();
                    // println!("data = {:#?}", u.data);
                    // let mut data = "" ;
                    // if u.data!="None"{
                    //     data=Some(u.data);
                    // }
                    let pubtime = Some(u.pubtime);
                    conn.execute(
                        "INSERT INTO releasenote (name,address,version,git,docker, pubtime,data)
                                  VALUES (?1, ?2,?3,?4,?5,?6,?7)",
                        params![u.name,u.address,u.version,u.git,u.docker,pubtime,u.data],
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
    // println!("{:?}",s);
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
        .args_from_usage("-H,--html '数据库到HTML'")
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
        let _err= to_html();
    }
}


fn main() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    flag();
    // println!("Hello, world!");
}
