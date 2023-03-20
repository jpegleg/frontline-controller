use std::fs::File;
use std::io::Read;
use std::{thread, time};
use std::str;

use uuid::Uuid;
use chrono::prelude::*;
use serde::Deserialize;
use redis::Commands;

extern crate redis;
extern crate base64;

mod apis;
mod locksys;
mod reactions;

const SMIG: u64 = 201;

#[derive(Deserialize)]
struct Config {
    net: String,
    cpu: String,
    mem: String,
    dsk: String,
    api1: String,
    api2: String,
    api3: String,
    api4: String,
    url1: String,
    url2: String,
    url3: String,
    url4: String,
    urlp1: String,
    urlp2: String,
    urlp3: String,
    urlp4: String,
    health1: String,
    health2: String,
    health3: String,
    health4: String,
}

fn compare_strings(uid: &Uuid, str1: &str, str2: &str) -> f64 {
    let readu: DateTime<Utc> = Utc::now();
    println!("{} {} {} {}", readu, uid, &str1, &str2);
    if let (Ok(num1), Ok(num2)) = (str1.parse::<f64>(), str2.parse::<f64>()) {
        if num1 == num2 {
            0.0
        } else if num1 > num2 {
            1.0
        } else {
            2.0
        }
    } else {
        255.0
    }
}

fn redisset(insertit: &str, valit: String) -> redis::RedisResult<()> {
    let redis_client = redis::Client::open("redis://localhost:6379/")?;
    let mut rcon = redis_client.get_connection()?;
    let _mop: String = rcon.set(insertit, valit).unwrap();
    Ok(())
}

pub fn apiter(policyitem: String, policyurl: String, healthslot: String ) {
    let redis_client2 = redis::Client::open("redis://localhost:6379/");
    let wcon = redis_client2.expect("Failed to connect to redis.").get_connection();
    let redis_clienta = redis::Client::open("redis://localhost:6379/");
    let rcon1 = redis_clienta.expect("Failed to connect to redis.").get_connection();
    let healthe: i32 = rcon1.expect("Failed to connect to redis").get(&healthslot).unwrap();
    let eostate: String = wcon.expect("Failed to connect to redis").get(policyurl).unwrap();
    if eostate == policyitem {
        if healthe < 100 {
            let newstate = healthe + 1; 
            let _mop1 = redisset(&healthslot, newstate.to_string());
        }   
    } else {
        let newstate = healthe - 4;
        let _mop1 = redisset(&healthslot, newstate.to_string());
    }

}

fn main() {
    let _out = redisset(&"cooloff".to_string(), "0".to_string());

    let syscont = thread::spawn(|| {
        loop {
            let mut file = File::open("policy.toml").expect("Failed to open policy.toml");
            let mut contents = String::new();
            let mut _mop = file.read_to_string(&mut contents);
            let config: Config = toml::from_str(&contents).unwrap();
            let uid = Uuid::new_v4();
            let readu: DateTime<Utc> = Utc::now();

            let redis_client = redis::Client::open("redis://localhost:6379/");
            let rcon = redis_client.expect("Failed to connect to redis.").get_connection();
            let health: i32 = rcon.expect("Failed to connect to redis").get("health00").unwrap();

            println!("{} {} Health status: {}", readu, &uid, &health);
            let dskzz: String = locksys::dskz();
            let policydsk: &str = &config.dsk;
            let mydsk: &str = &dskzz;
            let de = compare_strings(&uid, mydsk, policydsk);
            if de == 1.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if de == 0.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if de == 2.0 {
                if health < 100 {
                    let newstate = health + 1;
                    let _mop = redisset(&"health00".to_string(), newstate.to_string());
                }
            }
            let _hog1 = redisset(&"DSK".to_string(), dskzz);

            let redis_client = redis::Client::open("redis://localhost:6379/");
            let rcon = redis_client.expect("Failed to connect to redis.").get_connection();
            let health: i32 = rcon.expect("Failed to connect to redis").get("health00").unwrap();

            let memzz: String = locksys::memz();
            let policymem: &str = &config.mem;
            let mymem: &str = &memzz;
            let me = compare_strings(&uid, mymem, policymem);
            if me == 1.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if me == 0.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if me == 2.0 {
                if health < 100 {
                    let newstate = health + 1;
                    let _mop = redisset(&"health00".to_string(), newstate.to_string());
                }
            }
            let _hog2 = redisset(&"MEM".to_string(), memzz);

            let redis_client = redis::Client::open("redis://localhost:6379/");
            let rcon = redis_client.expect("Failed to connect to redis.").get_connection();
            let health: i32 = rcon.expect("Failed to connect to redis").get("health00").unwrap();

            let netzz: String = locksys::netz();
            let policynet: &str = &config.net;
            let mynet: &str = &netzz;
            let te = compare_strings(&uid, mynet, policynet);
            if te == 1.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if te == 0.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if te == 2.0 {
                if health < 100 {
                    let newstate = health + 1;
                    let _mop = redisset(&"health00".to_string(), newstate.to_string());
                }
            }
            let _hog3 = redisset(&"NET".to_string(), netzz);

            let redis_client = redis::Client::open("redis://localhost:6379/");
            let rcon = redis_client.expect("Failed to connect to redis.").get_connection();
            let health: i32 = rcon.expect("Failed to connect to redis").get("health00").unwrap();

            let cpuzz: String = locksys::cpuz();
            let policycpu: &str = &config.cpu;
            let mycpu: &str = &cpuzz;
            let ce = compare_strings(&uid, mycpu, policycpu);
            if ce == 1.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if ce == 0.0 {
                let newstate = health - 4;
                let _mop = redisset(&"health00".to_string(), newstate.to_string());
            }
            if ce == 2.0 {
                if health < 100 {
                    let newstate = health + 1;
                    let _mop = redisset(&"health00".to_string(), newstate.to_string());
                }
            }
            let _hog4 = redisset(&"CPU".to_string(), cpuzz);

            thread::sleep(time::Duration::from_millis(SMIG));
        }
    });

    thread::sleep(time::Duration::from_millis(SMIG));
    let webcont = thread::spawn(|| {
        loop {
            let mut file = File::open("policy.toml").expect("Failed to open policy.toml");
            let mut contents = String::new();
            let mut _mop = file.read_to_string(&mut contents);
            let config: Config = toml::from_str(&contents).unwrap();
            let uid = Uuid::new_v4();

            let apiout1: String = apis::apicon(&uid, &config.url1, &config.urlp1, 55);
            let _mop = redisset(&config.url1, apiout1);
            let _hog = apiter(config.api1, config.url1, config.health1); 
            thread::sleep(time::Duration::from_millis(SMIG));

            let apiout2: String = apis::apicon(&uid, &config.url2, &config.urlp2, 500);
            let _mop = redisset(&config.url2, apiout2);
            let _hog = apiter(config.api2, config.url2, config.health2); 
            thread::sleep(time::Duration::from_millis(SMIG));

            let apiout3: String = apis::apicon(&uid, &config.url3, &config.urlp3, 500);
            let _mop = redisset(&config.url3, apiout3);
            let _hog = apiter(config.api3, config.url3, config.health3); 
            thread::sleep(time::Duration::from_millis(SMIG));

            let apiout4: String = apis::apicon(&uid, &config.url4, &config.urlp4, 500);
            let _mop = redisset(&config.url4, apiout4);
            let _hog = apiter(config.api4, config.url4, config.health4);
        }
    });
    let reacont = thread::spawn(|| {
        loop {
            thread::sleep(time::Duration::from_millis(SMIG));
            let uid = Uuid::new_v4();

            let redis_client = redis::Client::open("redis://localhost:6379/");
            let rcon = redis_client.expect("Failed to connect to redis.").get_connection();
            let health: i32 = rcon.expect("Failed to connect to redis").get("health00").unwrap();
            
            let redis_client1 = redis::Client::open("redis://localhost:6379/");
            let rcon1 = redis_client1.expect("Failed to connect to redis.").get_connection();
            let health1: i32 = rcon1.expect("Failed to connect to redis").get("health01").unwrap();

            let redis_client2 = redis::Client::open("redis://localhost:6379/");
            let rcon2 = redis_client2.expect("Failed to connect to redis.").get_connection();
            let health2: i32 = rcon2.expect("Failed to connect to redis").get("health02").unwrap();

            let redis_client3 = redis::Client::open("redis://localhost:6379/");
            let rcon3 = redis_client3.expect("Failed to connect to redis.").get_connection();
            let health3: i32 = rcon3.expect("Failed to connect to redis").get("health03").unwrap();

            let redis_client4 = redis::Client::open("redis://localhost:6379/");
            let rcon4 = redis_client4.expect("Failed to connect to redis.").get_connection();
            let health4: i32 = rcon4.expect("Failed to connect to redis").get("health04").unwrap();

            let redis_client5 = redis::Client::open("redis://localhost:6379/");
            let rcon5 = redis_client5.expect("Failed to connect to redis.").get_connection();
            let cooloff: i32 = rcon5.expect("Failed to connect to redis").get("cooloff").unwrap();

            let readu: DateTime<Utc> = Utc::now();
            
            if health < 99 {
                println!("{} {} Health is at: {} <-+-+-+-<<<", &readu, &uid, health);
                if health < 92 {
                    if cooloff == 0 {
                        println!("{} {} STARTING AUTOMATED RECOVERY local1", &readu, &uid);
                        let _mop = reactions::local1(&uid);
                        let _hog = redisset(&"cooloff".to_string(), "1".to_string());
                        thread::sleep(time::Duration::from_millis(540000));
                        let _out = redisset(&"cooloff".to_string(), "0".to_string());

                    }
                }
            } 

            if health1 < 99 {
                println!("{} {} Health1 is at: {} <-+-+-+-<<<", &readu, &uid, health1);
                if health1 < 75 {
                    if cooloff == 0 {
                        println!("{} {} STARTING AUTOMATED RECOVERY a1", &readu, &uid);
                        let _mop = reactions::a1(&uid);
                        let _hog = redisset(&"cooloff".to_string(), "1".to_string());
                        thread::sleep(time::Duration::from_millis(540000));
                        let _out = redisset(&"cooloff".to_string(), "0".to_string());

                    }
                }

            }

            if health2 < 99 {
                println!("{} {} Health2 is at: {} <-+-+-+-<<<", &readu, &uid, health2);
                if health1 < 75 {
                    if cooloff == 0 {
                        println!("{} {} STARTING AUTOMATED RECOVERY b1", &readu, &uid);
                        let _mop = reactions::b1(&uid);
                        let _hog = redisset(&"cooloff".to_string(), "1".to_string());
                        thread::sleep(time::Duration::from_millis(540000));
                        let _out = redisset(&"cooloff".to_string(), "0".to_string());
                    }
                }

            } 

            if health3 < 99 {
                println!("{} {} Health3 is at: {} <-+-+-+-<<<", &readu, &uid, health3);
                if health1 < 75 {
                    if cooloff == 0 {
                        println!("{} {} STARTING AUTOMATED RECOVERY c1", &readu, &uid);
                        let _mop = reactions::c1(&uid);
                        let _hog = redisset(&"cooloff".to_string(), "1".to_string());
                        thread::sleep(time::Duration::from_millis(540000));
                        let _out = redisset(&"cooloff".to_string(), "0".to_string());
                    }
                }

            } 

            if health4 < 99 {
                println!("{} {} Health4 is at: {} <-+-+-+-<<<", &readu, &uid, health4);
                if health4 < 75 {
                    if cooloff == 0 {
                        println!("{} {} STARTING AUTOMATED RECOVERY d1", &readu, &uid);
                        let _mop = reactions::d1(&uid);
                        let _hog = redisset(&"cooloff".to_string(), "1".to_string());
                        thread::sleep(time::Duration::from_millis(540000));
                        let _out = redisset(&"cooloff".to_string(), "0".to_string());
                    }
                }

            } 

        }
    });

    syscont.join().unwrap();
    webcont.join().unwrap();
    reacont.join().unwrap();

}
