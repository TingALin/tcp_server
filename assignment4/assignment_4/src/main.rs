use structopt::StructOpt;
use std::fmt::Display;

// #[derive(Debug, StructOpt)]
// #[structopt(name = "Ting", version = "0.2.0", about = "An example of StructOpt usage.")]
// struct Cli {   
//     // #[structopt(short = "v", parse(from_occurrences))]
//     // verbose: u32, 
//     // #[structopt(short = "s", long = "speed", default_value = "42")]
//     // speed: f64,    
//     // #[structopt(parse(from_str))]
//     // input: String,
    
//     pattern: String,
//     #[structopt(parse(from_os_str))]
//     path: std::path::PathBuf, // PathBuf就像一个String，但用作跨平台工作的文件系统路径。
// }
// // impl fmt::Display for Cli {
// //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// //         write!(f, "({}, {})", self.pattern, self.path)
// //     }
// // }

// fn main() {
//     // let opt = Opt::from_args();//assignment_4 --help
//     // println!("{:?}", opt);//cargo run vv --speed 55 -vv
//     let pattern = std::env::args().nth(1).expect("no pattern is given");
//     let path = std::env::args().nth(2).expect("no path is given");
//     let args = Cli{
//         pattern: pattern,
//         path: std::path::PathBuf::from(path),
//     };

//     let args = Cli::from_args(); //cargo run args --help
//     //这不是最好的实现：会先把整个文件读取到内存中——不管文件有多大。找到一种方法来优化它！
//     //（一个想法可能是使用BufReader，而不是read_to_string()）
//     let content = std::fs::read_to_string(&args.path).expect("could not read file");
//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

// cargo run add 1 2 3
use structopt::StructOpt;
use std::fmt::{self, Display, Formatter};

#[derive(StructOpt)]
#[structopt(name ="app")]
pub struct AppArgs{
    #[structopt(subcommand)]
    pub command:Command
}
#[derive(StructOpt)]
pub enum Command{
    #[structopt(name="add")]
    Add(Elements),

    #[structopt(name="times")]
    Times(Elements),
}
#[derive(StructOpt)]
pub struct Elements{
    pub elements: Vec<u32>,
}

impl Display for Elements {
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.elements)
    }
}

fn main(){
    let opt = AppArgs::from_args();

    match opt.command{
        Command::Add(e) =>{
            let result = e.iter().fold(0,|acc, &x| acc+x);
            println!("Operant:{}, Result:{}", e, result);
        
        Command::Times(e) =>{
            let result = e.iter().fold(0,|acc, &x| acc*x);
            println!("Operant:{}, Result:{}", e, result);
        }
    }
}