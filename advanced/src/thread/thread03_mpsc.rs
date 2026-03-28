use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/*
누가 보내고 (producer), 누가 받느냐 (consumer)

스레드 관점 = 동기: 호출한 쪽이 끝날 때까지 기다림 / 비동기: 호출하고 바로 리턴
채널/큐 관점 = 동기 채널: 버퍼 제한 → 꽉 차면 send가 대기 / 비동기 채널: 버퍼 무제한 → send가 바로 리턴
여기서는 채널/큐 관점의 동기/비동기로 나눈다.

* mpsc = Multi Producer Single Consumer = 여러 개의 sender → 하나의 receiver
mpsc::channel() → 비동기 채널
버퍼: 사실상 무제한
send(): → 절대 block 안 함
producer는 계속 밀어넣을 수 있음
큐에 던지면 끝난다.

mpsc::sync_channel(size) → 동기 채널
버퍼: size로 제한
send(): → 버퍼 꽉 차면 block
상대가 소비해야 다음걸 보낼 수 있다.

recv()는 항상 blocking 상태다. (메시지를 계속 기다리다가 도착하면 다음 행동을 함.)

* mpmc = Multi Producer Multi Consumer = 여러 개의 sender → 여러 개의 receiver
*/

pub fn run() {
    single_sender();
    multi_sender();
    multi_sender_thread_consumer();
}

// 싱글 스레드 전송
fn single_sender() {
    let v: Vec<u32> = (1..=1000).filter(|x| x % 2 == 0).collect();

    let (tx, rx) = mpsc::channel(); // 채널을 이용하는 (Sender, Receiver)가 튜플 형태로 리턴
    let _ = thread::spawn(move || {
        let sum: u32 = v.into_iter().sum(); // v 소유권 이동
        tx.send(sum).unwrap();
        // send(sum) 을 호출하면 sum 값의 소유권이 sender 측에서 채널 내부로 이동한다.
        // 이후 receiver가 recv() 하면 그 소유권이 receiver 쪽으로 이동한다.
    });

    let sum = rx.recv().unwrap(); // recv로 채널로 전송된 데이터를 수신
    println!("sum={}", sum);
}

// 멀티 스레드 전송
fn multi_sender() {
    let (tx1, rx) = mpsc::channel(); // Receiver는 한 개 이기에 인덱스 없이 그냥 rx로 처리
    let tx2 = tx1.clone(); // 여러 producer가 같은 receiver로 보내기 위해 Sender를 복제한다.
    // clone된 Sender들도 모두 같은 채널에 연결된다.

    // 여기서는 join()을 쓰지 않아도 된다.
    // `for r in rx` 가 모든 Sender가 drop되어 채널이 닫힐 때까지 메시지를 계속 받으며 대기하기 때문이다.
    let _ = thread::spawn(move || {
        // thread::spawn 으로 생성한 스레드는 현재 함수보다 더 오래 살아 있을 수 있으므로,
        // 클로저가 바깥 변수의 참조를 붙잡는 대신 move로 소유권을 넘겨 안전하게 사용한다.
        let vals = vec!["A1", "A2", "A3", "A4"];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let _ = thread::spawn(move || {
        let vals = vec!["B1", "B2", "B3", "B4"];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // tx에 의해 여러 개의 메시지가 보내지면 이것은 채널에 사용된 FIFO(First In First Out) 버퍼에 메시지가 쌓인다.
    for r in rx {
        println!("{}", r);
        // 출력 결과는 스레드끼리 경합하는 상황과 운영체제의 스케줄링에 따라 다를 수 있다.
    }
}

// 스레드에서 메시지 받기
fn multi_sender_thread_consumer() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    let _ = thread::spawn(move || {
        let vals = vec!["A1", "A2", "A3", "A4"];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let _ = thread::spawn(move || {
        let vals = vec!["B1", "B2", "B3", "B4"];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle = thread::spawn(move || {
        for r in rx {
            println!("{}", r);
        }
    });

    // main 스레드가 먼저 끝나지 않도록 consumer 스레드를 기다린다.
    handle.join().unwrap();
}
