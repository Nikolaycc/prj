use std::env;
use std::fs::{copy, create_dir, read_to_string, write, File};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;
use toml::*;

const PATH: &str = ".prj";
const PCK: &str = ".prj/pck";

fn s(s: &str) -> String {
    s.to_owned()
}

fn cp(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

#[derive(PartialEq)]
enum OP {
    Elixir,
    Rust,
    Python,
    C,
    CPP,
    Erlang,
    Lua,
    Shell,
    JS,
    Setup,
    Done,
    Err,
    Null,
}

struct Prj {
    lang: String,
    prj: String,
    prjname: String,
    ver: [i32; 3],
    l: OP,
    config: toml::Value,
    tampl: Option<Value>,
    dirtree: Vec<toml::Value>,
    home: Option<String>,
    crdir: String,
}

impl Prj {
    fn new(name: String, lang: String, prj: String, config: toml::Value, home: String) -> Self {
        Self {
            lang: lang,
            prj: prj,
            prjname: name,
            ver: [0, 1, 0],
            l: OP::Null,
            config: config,
            tampl: None,
            dirtree: Vec::new(),
            home: Some(home),
            crdir: env::current_dir().unwrap().display().to_string(),
        }
    }

    fn setup(&self) -> () {
        process::exit(1);
    }

    fn dc(&self) -> io::Result<()> {
        let tmp = self.tampl.clone().unwrap();
        let dirtree = tmp["Config"]["dirtree"].as_array().unwrap();
        create_dir(format!("{}/{}", self.crdir, self.prjname).as_str()).unwrap();
        for d in dirtree {
            for k in d.as_array() {
                let filen = &k[1];
                let dir = &k[0];
                let filec = &k[2];

                let mut ffc = filec.to_string().replace("#{prjname}", &self.prjname);
                ffc = ffc.replace("#{prjname.fcuc}", &cp(&self.prjname));
                println!(
                    "File Name: {}\nDir: {}\nFile Content: {}\n",
                    filen, dir, ffc
                );

                create_dir(format!("{}/{}/{}", self.crdir, self.prjname, dir).as_str()).unwrap_or(
                    {
                        let mut file = File::create(format!(
                            "{}/{}/{}/{}",
                            self.crdir,
                            self.prjname,
                            dir,
                            filen.as_str().unwrap()
                        ))?;
                        file.write_all(ffc.as_bytes())?;
                    },
                );

                let mut file = File::create(format!(
                    "{}/{}/{}/{}",
                    self.crdir,
                    self.prjname,
                    dir,
                    filen.as_str().unwrap()
                ))?;
                file.write_all(ffc.as_bytes())?;
            }
        }
        Ok(())
    }

    fn c(&mut self) -> io::Result<()> {
        match self.lang.as_str() {
            "rust" | "rs" => self.l = OP::Rust,
            "elixir" | "ex" | "exs" => self.l = OP::Elixir,
            "setup" | "st" | "Setup" => self.setup(),
            _ => self.l = OP::Null,
        }

        if OP::Null != self.l
            && self.prj != s("no [type] given")
            && self.prjname != s("no [project name] given")
        {
            println!(
                "{}\n{}\n{}\n{:?}\n{}\nEND CONFIG",
                self.lang, self.prj, self.prjname, self.ver, self.config
            );

            match self.l {
                OP::Elixir => self.lang = "elixir".to_owned(),
                OP::Rust => self.lang = "rust".to_owned(),
                OP::Setup => self.setup(),
                _ => {}
            }

            let ty = &self.lang;
            let np = self.prj.as_str();
            let b = self.config["Project"][cp(ty)]["prj"].as_array().unwrap();
            for a in b {
                print!("{}\n", a);
                if a.as_str().unwrap() == np {
                    let ps = format!(
                        "{}/{}/{}/{}/Config.toml",
                        self.home.as_ref().unwrap(),
                        PCK,
                        self.lang,
                        self.prj
                    );
                    let p = Path::new(&ps);

                    self.tampl = Some(read_to_string(p).unwrap().parse::<Value>().unwrap());
                    self.dc();
                }
            }
        }

        Ok(())
    }

    fn version(&self) -> [i32; 3] {
        self.ver
    }
}

fn main() {
    let mut home: String = String::new();
    match env::home_dir() {
        Some(path) => home = path.display().to_string(),
        None => {
            println!("Impossible to get your home dir!");
            process::exit(0)
        }
    }
    let lang = env::args().nth(1).unwrap_or("no [lang] given".to_owned());
    let prj = env::args().nth(2).unwrap_or("no [type] given".to_owned());
    let name = env::args()
        .nth(3)
        .unwrap_or("no [project name] given".to_owned());
    let config = read_to_string(Path::new(format!("{}/{}/Config.toml", home, PATH).as_str()))
        .unwrap()
        .parse::<Value>()
        .unwrap();

    let mut app: Prj = Prj::new(name, lang, prj, config, home);
    app.c();
}
