//! # 느낀점
//! 피파 이적시장을 간단히 만들어 보았다.
//! 러스트 기본 문법과 네트워크, 쓰레드 다루는 법을 공부하였다.
//! match문을 남발하지 말자.. 때로는 if로 에러처리하는 것이 좋다.
//! 에러 메세지가 터미널에 도배되기 때문에... match문으로 해도 상관은 없겠지만
//! 코드가 길어질 뿐이니 if문으로 에러 처리를 하자...
//! ```
//!  loop {
//! if let Ok((client, addr)) = server.accept() {
//!     println!("client accepted: {}", addr);
//!     start_thread(client, tx.clone(), players_hash.clone());
//! }
//! ```
//! read_line() 메서드는 개행 문자가 있을 때까지 읽는다.
//! 만약 개행 문자가 없다면 버퍼를 읽지 않는다.
//! 네트워크 할때 매우 조심해야한다. 다른 언어같은경우 클라이언트에서 데이터를 보낼때까지 기다려주는데
//! 러스트는 기다리지 않고 바로 실행해버린다.
//! 클라이언트 쪽에서 데이터를 보내지도 않았는데 read_line()이 버퍼를 보고는 데이터가 없다고 한다.
//! 반드시 서버측에서 loop문을 이용해 클라이언트가 데이터를 보낼때까지 기다렸다가 다음 라인으로 넘어가야한다.
//! stdin()으로 read_line()을 하면 내가 뭔가 입력했을 때 개행문자까지 들어가서 버퍼를 읽을수 있다.
//! 하지만 String을 따로 만들어서 보낼 때는 read_line()이 버퍼를 읽을수 있도록 반드시 마지막에 개행문자를 붙이자.
//! 혹시 개행문자가 여러개라서 read_line()이 곤란할 때는 read_until(b'\n', &mut variable)을 이용하자
//! ```
//! loop {
//!             if let Ok(n) = reader.read_line(&mut tmp) {
//!                println!("get data: {}", tmp);
//!                 break;
//!             }
//!         }
//! ```

mod transfer_market_mod;

use transfer_market_mod::transfer_market_player::*;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut players_hash: HashMap<String, Arc<Mutex<PlayerBucket>>> = HashMap::new(); // players list
    players_hash.insert(
        String::from("Messi"),
        Arc::new(Mutex::new(PlayerBucket::new(
            SoccerPlayer::new(String::from("Messi"), 1850000),
            2,
        ))),
    ); // create Messi

    players_hash.insert(
        String::from("Ronaldo"),
        Arc::new(Mutex::new(PlayerBucket::new(
            SoccerPlayer::new(String::from("Ronaldo"), 2250000),
            3,
        ))),
    ); // create Ronaldo

    players_hash.insert(
        String::from("Son"),
        Arc::new(Mutex::new(PlayerBucket::new(
            SoccerPlayer::new(String::from("Son"), 2500000),
            4,
        ))),
    ); // create Son

    //print_hash(players_hash);

    // Server Settings
    let server_addr = "127.0.0.1:8888";
    let server = TcpListener::bind(server_addr).expect("Server execute failed");
    server.set_nonblocking(true).expect("Error...");
    println!("Server on!");

    // thread channels
    let (tx, rx) = mpsc::channel::<(String, TcpStream)>();

    // get client
    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("client accepted: {}", addr);
            start_thread(client, tx.clone(), players_hash.clone());
        }

        // do task requested
        if let Ok((result, client)) = rx.try_recv() {
            println!("Result: {}, client: {:?}", result, client);
            thread::sleep(Duration::from_millis(1000));
            let mut cli = client;
            let bytes = result.as_bytes();
            match cli.write_all(&bytes) {
                Ok(n) => println!("Write Successfully"),
                Err(e) => println!("Write Failed"),
            }
        }
    }
}

// fn print_hash(players: HashMap<String, Arc<Mutex<PlayerBucket>>>) {
//     for (player, bucket) in players.iter() {
//         match players.get(player) {
//             Some(n) => n.lock().expect("printerror").print_bucket(),
//             None => println!("No player"),
//         }
//     }
// }

/// thread start
fn start_thread(
    client: TcpStream,
    tx: mpsc::Sender<(String, TcpStream)>,
    players: HashMap<String, Arc<Mutex<PlayerBucket>>>,
) {
    let mut reader = BufReader::new(client.try_clone().unwrap());
    
    thread::spawn(move || loop {

        let mut tmp: String = String::new();

        loop {
            if let Ok(n) = reader.read_line(&mut tmp) {
                println!("get data: {}", tmp);
                break;
            }
        }

        let mut select_num = match tmp.trim().parse() {
            Ok(n) => n,
            Err(e) => 0,
        };

        // receive menu and show
        match select_num {
            1 => show_players(players.clone(), client.try_clone().unwrap(), tx.clone()),
            2 => purchase_players(players.clone(), client.try_clone().unwrap(), tx.clone()),
            3 => sale_players(players.clone(), client.try_clone().unwrap(), tx.clone()),
            _ => {}
        }
    });
}
/// show soccer players and count
fn show_players(
    players: HashMap<String, Arc<Mutex<PlayerBucket>>>,
    client: TcpStream,
    tx: mpsc::Sender<(String, TcpStream)>,
) {
    println!("showplayers");
    let mut listvec: Vec<String> = Vec::new();

    // get all info about players by string
    for (player, _bucket) in players.iter() {
        match players.get(player) {
            Some(n) => {
                listvec.push(n.lock().expect("printerror").get_player_to_string());
            }
            None => println!("No player"),
        }
    }

    // unite all string and send
    let s = listvec.join("\n");
    let s = format!("{}{}", s, "@");
    tx.send((s, client)).unwrap();
}

/// fuction that clients purchase soccerplayers
fn purchase_players(
    players: HashMap<String, Arc<Mutex<PlayerBucket>>>,
    client: TcpStream,
    tx: mpsc::Sender<(String, TcpStream)>,
) {
    let mut reader = BufReader::new(client.try_clone().unwrap());
    let mut pname: String = String::new();

    loop {
        if let Ok(n) = reader.read_line(&mut pname) {
            break;
        }
    }

    println!("선수 이름 : {}", pname);
    //맨 뒤에 \n 이거 때문인가? 두번 pop을 해야 작동이 된다.
    //String::from으로 \n 넣었을 땐 pop 한번만 해야하고 stdin()으로 받았을 땐 두번 해야하는듯?
    pname.pop();
    pname.pop();
    match players.get(&pname) {
        Some(p) => {
            if p.lock().expect("Asdsdf").get_count() > 0 {
                println!("선수 존재");
                let price = p
                    .lock()
                    .expect("lockerror")
                    .get_player()
                    .get_price()
                    .to_string();
                let pprice = format!("{}{}", price, "\n");
                tx.send((pprice, client.try_clone().unwrap())).unwrap();


                println!("구입 의사 대기..");
                let mut yesorno = String::new();
                loop {
                    if let Ok(n) = reader.read_line(&mut yesorno) {
                        break;
                    }
                }
                
                yesorno.pop();
                let yn = yesorno.as_str();
                // 구매 처리
                if yn == "y" {
                    println!("구입하기");
                    p.lock().expect("lockerror").count_change(-1);
                } else {
                    println!("구입 안함");
                }
            } else {
                println!("매물이 없음");
                tx.send(("No Count\n".to_string(), client.try_clone().unwrap()))
                    .unwrap();
            }
        }
        None => {
            println!("No players");
            tx.send(("No player\n".to_string(), client.try_clone().unwrap()))
                .unwrap();
        }
    }
}

fn sale_players(
    players: HashMap<String, Arc<Mutex<PlayerBucket>>>,
    client: TcpStream,
    tx: mpsc::Sender<(String, TcpStream)>,
) {
    let mut reader = BufReader::new(client.try_clone().unwrap());
    let mut pname: String = String::new();

    loop {
        if let Ok(n) = reader.read_line(&mut pname) {break;}
    }

    //개행 문자 제거
    pname.pop();
    pname.pop();

    let mut chk = String::new();
    loop {
        if let Ok(n) = reader.read_line(&mut chk) {break;}
    }

    chk.pop();
    let chkk = chk.as_str();

    match chkk {
        "yy" => println!("판매 가능"),
        _ => {println!("판매 불가"); return;},
    }

    //여기서는 선수 세 명만 있다고 가정하겠음, 클라이언트에서 예외처리함
    match players.get(&pname) {
        Some(p) => {
            println!("선수 존재, 수량 1 증가");
            p.lock().expect("sale failed").count_change(1);
        }
        None => {
            println!("No players");
        }
    }
}