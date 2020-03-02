#[macro_use]
extern crate clap;
extern crate libc;
use std::error::Error;
use rusqlite::{params,Connection,NO_PARAMS};
use serde::{Deserialize, Serialize};
//use time::Timespec;  //Also, it should be noted, that time v0.1.* must be used, because Timespec was removed in v0.2.0.
use clap::App;
use std::fmt;

const DB_NAME: &'static str = "releaseNote.db"; 

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

///此处让我深感疑惑，就是错误处理机制。还有？的使用方式。
///read_file
fn read_file(filename: String) {
    let  _file = match std::fs::File::open(filename) {
        Ok(file) => {
            // println!("文件({ })打开成功.", "rn.txt");
            let conn = Connection::open(DB_NAME).unwrap();
            use encodingbufreader::{BufReaderEncoding};
            use encoding::all::{UTF_8};
            for line in BufReaderEncoding::new(file, UTF_8).lines().map(|l| l.unwrap()){
                let json=line.to_string();
                // let j=r#"{"name":"SmartOMP","version":"V1.0.1.3-20200301","address":"10.11.35.104:9099","git":"a1d7c36ccc449178254458e14319c8985fe76253","docker":"dockerhub.cloudminds.com/SmartOMP-101","pubtime":"2020-03-01 19:58:25"}"#;
                if json.len()>3{
                    let u: Releasenote = serde_json::from_str(&json).unwrap();
                    // println!("name = {}", u.name);
                    // println!("address = {:#?}", u.address);
                    // println!("git = {:#?}", u.git);
                    // println!("docker = {:#?}", u.docker);
                    // println!("pubtime = {:#?}", u.pubtime);
                    // println!("data = {:#?}", u.data);
                    // let mut data = "" ;
                    // if u.data!="None"{
                    //     data=Some(u.data);
                    // }
                    let pubtime = Some(u.pubtime);
                    conn.execute(
                        "INSERT INTO releasenote (name,address,git,docker, pubtime,data)
                                  VALUES (?1, ?2,?3,?4,?5,?6)",
                        params![u.name,u.address,u.git,u.docker,pubtime,u.data],
                    ).unwrap();
                }
                
             }
        }
        Err(_why) => {
            println!("文件({ })打开失败.", "rn.txt");
        
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
        let _err=read_file("rn.txt".to_string());
        
    }
}


fn main() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    flag();
    // println!("Hello, world!");
}
