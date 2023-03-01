mod transfer_market_mod;

use transfer_market_mod::transfer_market_player::*;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut player_list: HashMap<String, Option<SoccerPlayer>> = HashMap::new();
    let mut money: i64 = 5_000_000;

    let server_addr = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(server_addr).expect("서버 접속 불가");
    socket.set_nonblocking(true).expect("에러...");
    println!("서버에 접속했습니다: {}", server_addr);

    let mut reader = BufReader::new(socket.try_clone().unwrap());

    println!(
        "원하는 작업의 번호를 입력:\n
    1. Show Players list
    2. Purchase Player
    3. Sales Player"
    );

    loop {
        println!("선택: ");

        let mut buf: String = String::new();
        std::io::stdin().read_line(&mut buf).expect("입력 에러");

        let k = buf.as_bytes();
        socket.write_all(&k);

        buf.pop();
        buf.pop();
        let num = buf.as_str();

        match num {
            "1" => {
                let mut buf = Vec::new();
                loop {
                    if let Ok(n) = reader.read_until(b'@', &mut buf) {
                        let mut string_read = String::from_utf8(buf).expect("fasfadf");
                        string_read.pop();
                        println!("{}", string_read);
                        break;
                    }
                }
            }
            "2" => {
                println!("구매하실 선수를 입력해주세요: ");

                let mut pname = String::new();
                std::io::stdin()
                    .read_line(&mut pname)
                    .expect("에러에러에러");
                let msg = pname.as_bytes();
                socket.write_all(&msg);

                let mut player_price = String::new();
                loop {
                    if let Ok(n) = reader.read_line(&mut player_price) {
                        break;
                    }
                }

                match player_price.trim().parse() {
                    Ok(n) => {
                        println!("가격 : {}\n구입하시겠습니까? (y or n) : ", n);
                        let mut yorn = String::new();
                        std::io::stdin().read_line(&mut yorn).expect("ssssssss");

                        yorn.pop();
                        yorn.pop();
                        let yy = yorn.as_str();

                        let nn = String::from("nn\n");
                        let nnn = nn.as_bytes();

                        match yy {
                            "y" => {
                                if money < n {
                                    println! {"자금이 부족합니다."};
                                    socket.write_all(&nnn);
                                    continue;
                                }
                                if let Some(n) = player_list.get(&pname) {
                                    match n {
                                        Some(m) => {
                                            println! {"이미 존재하는 선수입니다."};
                                            socket.write_all(&nnn);
                                            continue;
                                        }
                                        None => println!("구입 가능!"),
                                    }
                                }

                                let y = String::from("y\n");
                                let yyy = y.as_bytes();
                                socket.write_all(&yyy);

                                money -= n;
                                player_list
                                    .insert(pname.clone(), Some(SoccerPlayer::new(pname, n)));
                                println!("구입 완료");
                            }
                            _ => println!("구입 안함~"),
                        }
                    }
                    Err(e) => println!("{}: {}", e, player_price),
                }
            }
            "3" => {
                println!("판매할 선수의 이름: ");
                let mut pname = String::new();
                std::io::stdin().read_line(&mut pname).expect("gggggg");
                let msg = pname.as_bytes();
                socket.write_all(&msg);

                match player_list.get(&pname) {
                    Some(p) => match p {
                        Some(pp) => {
                            let chk = String::from("yy\n");
                            let chkk = chk.as_bytes();
                            socket.write_all(&chkk);
                            player_list.insert(pname, None);
                            println!("판매 완료");
                        }
                        None => {
                            let chk = String::from("nn\n");
                            let chkk = chk.as_bytes();
                            socket.write_all(&chkk);
                            println!("선수가 이미 팔리거나 없습니다.");
                        }
                    },
                    None => {
                        println!("선수가 존재하지 않습니다.");
                        let chk = String::from("nn\n");
                        let chkk = chk.as_bytes();
                        socket.write_all(&chkk);
                    }
                }
            }
            _ => println!("1, 2, 3중 선택해 주세요"),
        }
    }
}
