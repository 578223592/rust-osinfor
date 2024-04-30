use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// show architecture
    #[structopt(short = "a", long = "arch")]
    arch: bool,
}


fn main() {
    let opt = Opt::from_args();
    if opt.arch {
        println!("System arch: {}", std::env::consts::ARCH);
    }
    // println!("{:?}", opt);
    // println!("Hello, world!");
}
