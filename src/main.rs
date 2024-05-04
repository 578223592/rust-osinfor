use structopt::StructOpt;
use gethostname::gethostname;
use local_ip_address::local_ip;
use sys_info::os_release;
use std::fs;
use std::process::Command;

#[derive(StructOpt, Debug)]
struct Opt {
    /// show architecture
    #[structopt(short = "a", long = "arch")]
    arch: bool,

    /// show hostname
    #[structopt(short = "h", long = "hostname")]
    host_name: bool,

    /// show IP address
    #[structopt(short = "p", long = "ipaddress")]
    ip_address: bool,

    /// show OS version
    #[structopt(short = "v", long = "OSVersion")]
    os_version: bool,

    /// show num of pkg
    #[structopt(short = "n", long = "numpkg")]
    num_pkg: bool,
}

fn show_num_pkgs() {
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        if os_release.contains("ubuntu") ||os_release.contains("debian")||os_release.contains("mint"){
            let output = Command::new("sh")
                .arg("-c")
                .arg("dpkg -l | grep ^ii | wc -l")
                .output()
                .expect("failed to execute process");
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                print!("{}", result); //no new line
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed: {}", error);
            }
        } else if os_release.contains("rhel") || os_release.contains("centos")||os_release.contains("fedora") {
            //TODO R系列系统的输出
            println!("Red Hat系列");
        } else {
            println!("未知 Linux 发行版");
        }
    } else {
        println!("无法读取系统信息文件");
    }
}

fn main() {
    let opt = Opt::from_args();
    if opt.arch {
        println!("System arch: {}", std::env::consts::ARCH);
    } else if opt.host_name {
        println!("Hostname: {:?}", gethostname());
    }else if opt.ip_address{
        match local_ip() {
            Ok(ip) => println!("IP address: {:?}", ip),
            Err(e) => println!("get IP address Error: {}", e),
        }
    }else if opt.os_version{
        match os_release() {
            Ok(release) => println!("OS Version: {}", release),
            Err(e) => println!("OS version error: {}", e),
        }
    }else if opt.num_pkg{
        show_num_pkgs();
    }

    // println!("{:?}", opt);
    // println!("Hello, world!");
}
