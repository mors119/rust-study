use crossbeam::channel as cbc;
use std::{sync::mpsc, thread, time::Duration};

pub fn run() {
    create_bounded();
    create_unbounded();
    async_unbounded_log_pipeline();
    sync_bounded_job_queue();
}

// crossbeam bounded (버퍼 크기 제한 있음)
// ------------------------------
fn create_bounded() {
    // capacity = 5
    // → 최대 5개까지 큐에 쌓이고, 그 이후 send는 BLOCK됨
    let (s, r) = cbc::bounded::<String>(5);

    let _ = thread::spawn(move || {
        for i in 1..=10 {
            // ❗ 버퍼가 꽉 차면 여기서 자동으로 block됨 (backpressure)
            s.send(format!("{i}")).unwrap();
            println!("[Sent] : {i}");
        }
    });

    let h2 = thread::spawn(move || {
        for msg in r {
            // ❗ r은 Iterator처럼 동작
            // → 메시지가 오면 하나씩 꺼냄
            // → 모든 Sender가 drop되면 반복 종료
            println!("{}", msg);

            // ❗ 이 sleep은 실무용이 아니라 "느리게 소비해서 blocking 상황을 보기 위한 것"
            thread::sleep(Duration::from_millis(500));
        }
    });

    h2.join().unwrap();
}

// crossbeam unbounded (버퍼 무제한)
// ------------------------------
fn create_unbounded() {
    let (s, r) = cbc::unbounded();

    // Producer handles 저장
    let mut producer_handles = vec![];

    // Producer 생성
    for i in 1..=6 {
        // ❗ Sender clone
        // → 모든 producer가 "같은 채널"에 연결됨
        let handle = run_producer(s.clone(), i);
        producer_handles.push(handle);
    }

    // ❗ 매우 중요
    // 메인 스레드가 들고 있는 Sender 제거 → 이제 Sender는 producer thread들만 들고 있음
    drop(s);

    // Consumer handles 저장
    let mut consumer_handles = vec![];

    // Consumer 생성 (MPMC)
    for i in 1..=3 {
        // ❗ Receiver도 clone 가능 (mpmc 특징) → 여러 consumer가 같은 큐에서 경쟁적으로 가져감
        let handle = run_consumer(r.clone(), i);
        consumer_handles.push(handle);
    }

    // Producer 먼저 join
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Consumer join
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("main function is finished!");
}

// Producer
fn run_producer(s: cbc::Sender<u32>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Producer thread {} - pushing!", num);

        for _ in 1..=100 {
            // ❗ unbounded이므로 send는 절대 block되지 않음
            // → 메모리에 계속 쌓임 (주의 필요)
            s.send(num).expect("Sending failed");
        }

        // ❗ 여기서 thread 종료 → s(drop됨)
        // 모든 producer가 끝나면 channel이 닫히게 됨
    })
}

// Consumer
fn run_consumer(r: cbc::Receiver<u32>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut i = 0;

        println!("Consumer thread {} - popping!", num);

        loop {
            // ❗ 핵심: recv는 blocking → 데이터 없으면 기다림 → 채널이 닫히면 Err 반환
            if let Err(_) = r.recv() {
                // ❗ 모든 Sender가 drop되면 여기로 들어옴
                println!("Stopping consumer thread {}, messages received: {}", num, i);
                break;
            }

            i += 1;
        }
    })
}

// async_unbounded
// ------------------------------
fn async_unbounded_log_pipeline() {
    println!("\n[1] async + unbounded example started");

    // channel()은 사실상 크기 제한이 없는 큐라고 생각하면 된다.
    // sender는 메시지를 보낼 때 버퍼가 꽉 차서 막히지 않는다.
    let (tx, rx) = mpsc::channel::<String>();

    // 여러 생산자가 같은 로그 수집 채널로 메시지를 보낸다고 가정한다.
    // 예를 들어:
    // - auth service
    // - payment service
    // - notification service
    let mut producer_handles = Vec::new();

    for service_name in ["auth", "payment", "notification"] {
        // Sender는 clone 가능하다.
        // clone된 Sender들은 모두 같은 채널에 연결된다.
        let tx = tx.clone();
        let service_name = service_name.to_string();

        let handle = thread::spawn(move || {
            for i in 1..=5 {
                let message = format!("[{service_name}] log event #{i}");

                // unbounded 채널이므로 send는 보통 즉시 반환된다.
                // 즉, receiver가 느리더라도 sender는 큐에 계속 적재할 수 있다.
                tx.send(message).expect("failed to send log");

                // 실무에서 이런 sleep은 보통 비즈니스 동작이나 외부 I/O를 흉내낼 때만 쓴다.
                // 여기서는 로그가 한 번에 몰리지 않도록 보기 좋게 넣은 것이다.
                thread::sleep(Duration::from_millis(80));
            }

            println!("producer [{service_name}] finished");
        });

        producer_handles.push(handle);
    }

    // 메인 스레드가 갖고 있던 원본 Sender를 명시적으로 제거한다.
    // 이걸 하지 않으면 "아직 sender가 하나 살아 있다"고 판단되어
    // receiver 쪽 반복문이 끝나지 않을 수 있다.
    drop(tx);

    // 단일 consumer가 로그를 처리한다.
    // std::mpsc는 receiver가 하나뿐이라는 점이 핵심이다.
    let consumer_handle = thread::spawn(move || {
        // for msg in rx 는 내부적으로 recv()를 반복 호출하는 것과 비슷하다.
        // 메시지가 오면 하나씩 받고,
        // 모든 Sender가 drop되면 반복이 끝난다.
        for msg in rx {
            println!("log collector received: {msg}");

            // 로그 저장, 파일 기록, 원격 전송 같은 느린 작업을 흉내낸다.
            // receiver가 느려도 sender는 unbounded 큐에 계속 밀어 넣을 수 있다.
            thread::sleep(Duration::from_millis(150));
        }

        println!("log collector finished");
    });

    // 생산자들이 모두 끝날 때까지 기다린다.
    for handle in producer_handles {
        handle.join().expect("producer thread panicked");
    }

    // 그 다음 consumer가 채널 종료를 감지하고 자연스럽게 끝날 때까지 기다린다.
    consumer_handle.join().expect("consumer thread panicked");

    println!("[1] async + unbounded example finished");
}

// sync_bounded
// ------------------------------
fn sync_bounded_job_queue() {
    println!("\n[2] sync + bounded example started");

    // sync_channel(3)은 버퍼 크기가 3인 bounded queue다.
    // 즉, 큐에 최대 3개까지만 쌓을 수 있다.
    // 그 이후 send는 receiver가 소비해서 빈 공간을 만들 때까지 block된다.
    let (tx, rx) = mpsc::sync_channel::<Job>(3);

    let mut producer_handles = Vec::new();

    // 여러 producer가 작업을 생성한다고 가정한다.
    // 예를 들면 웹 요청 처리 중 생성된 썸네일 작업, 이메일 작업, 정산 작업 같은 것들이다.
    for producer_id in 1..=2 {
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            for seq in 1..=5 {
                let job = Job {
                    producer_id,
                    job_id: seq,
                    payload: format!("file-{producer_id}-{seq}.png"),
                };

                println!(
                    "producer {producer_id} preparing job {} ({})",
                    job.job_id, job.payload
                );

                // sync_channel에서는 버퍼가 꽉 차면 여기서 block된다.
                // 이게 바로 backpressure다.
                // 생산 속도가 소비 속도보다 너무 빠를 때 자연스럽게 속도를 제어한다.
                tx.send(job).expect("failed to send job");

                println!("producer {producer_id} successfully queued job {seq}");

                // 실제 실무에서는 DB 조회, 파일 생성, 네트워크 요청 등으로 시간이 걸릴 수 있다.
                thread::sleep(Duration::from_millis(100));
            }

            println!("producer {producer_id} finished");
        });

        producer_handles.push(handle);
    }

    drop(tx);

    let worker_handle = thread::spawn(move || {
        // std::mpsc는 single consumer이므로 worker는 하나다.
        // 여러 worker가 하나의 큐에서 경쟁적으로 가져가는 패턴이 필요하면
        // std::mpsc만으로는 불편하고 다른 구조나 라이브러리를 고려해야 한다.
        for job in rx {
            println!(
                "worker started job {} from producer {} ({})",
                job.job_id, job.producer_id, job.payload
            );

            // 느린 작업 처리라고 가정한다.
            // 예를 들어 이미지 리사이징, 압축, 업로드, 메일 발송 등
            thread::sleep(Duration::from_millis(500));

            println!(
                "worker finished job {} from producer {}",
                job.job_id, job.producer_id
            );
        }

        println!("worker finished");
    });

    for handle in producer_handles {
        handle.join().expect("producer thread panicked");
    }

    worker_handle.join().expect("worker thread panicked");

    println!("[2] sync + bounded example finished");
}

#[derive(Debug)]
struct Job {
    producer_id: u32,
    job_id: u32,
    payload: String,
}
