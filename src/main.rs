use clap::Parser;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{mpsc, LazyLock};
use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short, long)]
    config: String,
    #[arg(long)]
    pwd: Option<String>
}

#[derive(Deserialize)]
struct Config {
    user: String,
    password: String,
    isp: String,
    interface: Option<Vec<String>>,
    err_log: String,
    log: String,
    force_start: bool,
}

#[derive(Deserialize)]
struct LoginResult {
    result: i8
}

static ARGS: LazyLock<Args> = LazyLock::new(|| {
    Args::parse()
});

static CONFIG:LazyLock<Config> = LazyLock::new(|| {
    let mut tome_str_from_file = String::new();
    let _ = File::open(ARGS.config.clone()).unwrap().read_to_string(&mut tome_str_from_file).unwrap();
    let config: Config = toml::from_str(&tome_str_from_file).unwrap();
    config
});

fn main() {
    let std_out = File::create(&CONFIG.log).unwrap();
    let std_err = File::create(&CONFIG.err_log).unwrap();
    let mut projd = daemonize::Daemonize::new()
            .stderr(std_err)
            .stdout(std_out)
            .privileged_action(move || {
                let first_login = login();
                if let Err(e) = first_login {
                    if CONFIG.force_start == false {
                        panic!("First time login fail, stop startup: {:?}", e)
                    }
                }
                sleep(Duration::from_secs(3));
                service();
            });
    if let Some(path_str) = &ARGS.pwd {
        projd = projd.working_directory(Path::new(path_str))
    } else {
        projd = projd.working_directory(".")
    };
    match projd.start() {
        Err(_) => {}
        Ok(_) => {}
    }
}

fn service() {
    let (falt_tx, falt_rx) = mpsc::channel::<()>();
    let _: JoinHandle<()> = thread::spawn(move || {
        // 定期检测联通
        loop {
            let ping_result = network_test();
            if let Err(_) = ping_result {
                println!("network error reconnecting");
                falt_tx.send(()).unwrap();
            } else {
                println!("device is connected to njtech-home")
            };
            thread::sleep(Duration::from_secs(30));
        };
    });
    // let reconn_task_handle: JoinHandle<()> = thread::spawn(move || {
    loop {
        falt_rx.recv().unwrap();
        if let Err(_) = login() {
            println!("reconnect falt")
        };
    };
    // });
}

fn login() -> Result<(), ()> {
    let login_url = format!("http://10.50.255.11:801/eportal/portal/login?\
            callback=dr1003&login_method=1\
            &user_account=,0,{}@{}\
            &user_password={}\
            &wlan_user_ip=10.38.64.137", CONFIG.user, CONFIG.isp, CONFIG.password);
    
    let resp = ureq::get(&login_url)
      .set("Accept", "*/*")
      .set("Accept-Encoding", "gzip, deflate")
      .set("Accept-Language", "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")
      .set("Connection", "keep-alive")
      .set("Host", "10.50.255.11:801")
      .set("Referer", "http://10.50.255.11/")
      .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0")
      .call().unwrap().into_string().unwrap();
    let part_right = resp.split("(").collect::<Vec<&str>>().get(1).unwrap().to_string();
    let part_final = part_right.split(")").collect::<Vec<&str>>().get(0).unwrap().to_string();
    let result = serde_json::from_str::<LoginResult>(&part_final).unwrap();
    if result.result == 1 {
        println!("Login success!");
    } else {
        println!("{}", part_final);
        println!("Login Error!");
        return Err(())
    }
    Ok(())
}

// 尝试curl百度
fn network_test() -> Result<(), ()>{
    ureq::get("https://baidu.com").call()
    .map(|_| {()})
    .map_err(|_| {()})
}