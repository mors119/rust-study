use std::time::{Duration, Instant};
use tokio::time;

/*
! 요약
비동기 프로그래밍은 I/O를 기다리는 동안 현재 작업을 잠시 멈추고, 그 사이 다른 작업을 진행할 수 있게 해주는 방식이다.

Rust의 async fn은 즉시 최종 값을 반환하는 것이 아니라 Future<Output = T> 형태의 "미래 작업"을 만든다.

이 Future는 tokio 같은 runtime에 의해 poll되며, 바로 진행할 수 없으면 Pending 상태가 되고, 나중에 다시 깨워졌을 때 이어서 실행된다.

핵심 장점은 여러 개의 느린 I/O 작업을 동시에 진행해 전체 대기 시간을 줄일 수 있다는 점이다.

주의:
- async는 CPU 연산을 빠르게 만드는 기술이 아니다.
- CPU-bound 작업에는 일반 스레드나 병렬 처리 방식이 더 적합할 수 있다.
*/

/*
비동기 프로그래밍은 '동시성(Concurrent)' 작업을 할 수 있게 하는 프로그래밍 (기다리는 시간 동안 다른 일을 하게 하려고 쓴다)

OS가 아닌, 일반 프로그램에서 쓰레드를 만들 수 있고, 이런 쓰레드를 'Green 쓰레드'라고 한다.
(Sun micro systems의 green team이 Java VM(Virtual Machine) 상에서 자신들이 만든 스케줄러에 의해서 제어되는 쓰레드를 만들었고 이 쓰레드의 이름이 'Green 쓰레드'였다.)
OS 쓰레드는 대부분 '선점형(Preemptive)' 스케줄링 방식을 따른다. OS는 쓰레드 하나를 생성하고, 작업을 다른 쓰레드에게 전환하고 하는 것이, CPU 입장에서는 꽤 많은 리소스가 소모된다.
반면에 Green 쓰레드는 '협력형' 스케줄링 방식을 따른다. 따라서, 쓰레드 자신이 I/O 작업을 기다릴 수도 있고, 자신의 하고 있는 작업을 스스로 멈추고 다른 쓰레드에게 실행권을 넘길 수도 있다.

일반적인 상황에서는 OS 쓰레드를 쓰는것이 편하고 성능도 낫다.
Green 쓰레드가 유리한 상황은, CPU에 비해 아주 느린 I/O 장치를 사용할 때, 예를들어 하드디스크 읽기/쓰기, 네트웍 통신 등을 사용할 때이다.
OS 쓰레드 하나에 네트워크 소켓 하나를 담당하게 하면 너무 큰 낭비다.

Green 쓰레드가 유리한 분야: 네트워크 request/response 처리, 파일 읽기/쓰기 처리, 데이터베이스 쿼리

비동기 프로그래밍을 하기 위해서는 두 가지가 구비되어야 한다.
프로그래밍 언어에서 비동기 프로그래밍 기능을 제공해야한다. : async, .await
Green 쓰레드의 실행과 스케줄링을 하는 라이브러리가 필요하다. : tokio (cargo add tokio --features full) 또는 async-std

CPU를 독점하지 않으면서 어떤 작업을 반복적으로 하는 방식
'인터럽트 방식'은, 메시지가 도착했다는 인터럽트 신호를 받으면, 메시지 큐로 가서 메시지를 읽는 방식이다.
(인터럽트 방식은 자신의 리소스를 소모하지 않고 OS, 하드웨어등에 의해 폴링을 수행한다. 이를 제공하지 않으면 폴링 방식을 쓸 수 밖에 없다.)
'폴링 방식'은, 1ms 간격 등으로 계속 메시지 큐를 읽어서, 메시지가 있는지 확인해가면 읽은 방식이다.

async 키워드는 함수(fn), 클로저 혹은 { }로 감싸진 코드 블록 앞에 사용될 수 있고, async에 의해서 해당 코드는 Future 트레잇을 구현한 'Future 객체'가 된다.
Future 객체들이, 자신이 수행할 기회가 된 건지 계속해서 폴링한다.
async에 의해 Future 객체가 되었다고 해서 바로 실행되는 것은 아니고, 이 객체가 .await의 값이 되었을 때만 실행될 수 있는 후보군에 들어가게 된다.

즉,
async fn foo()  → Future 생성 (일 안 함)
.await          → 실행 요청
tokio runtime   → 실제 실행 담당
*/

#[tokio::main] // async 코드를 실행할 수 있는 runtime을 자동으로 만들어주는 매크로
pub async fn run() {
    let start = Instant::now(); // 지금 시점의 고정된 시간 기준점을 가져오는 것, 얼마나 걸렸나 측정하기 위함.

    let task1 = fetch_data("user", 1000);
    let task2 = fetch_data("order", 1000);

    let (user, order) = tokio::join!(task1, task2); // join!이 두 Future를 함께 진행시킨다.

    println!("user = {}", user);
    println!("order = {}", order);
    println!("elapsed = {:?}", start.elapsed()); // 시작 후 경과 시간
}

async fn fetch_data(name: &str, delay_ms: u64) -> String {
    println!("{name} request started");

    // thread::sleep = 현재 OS thread를 실제로 멈춤
    // tokio::time::sleep(...).await = 현재 task를 보류, 그 동안 runtime은 다른 task를 실행 가능
    time::sleep(Duration::from_millis(delay_ms)).await;

    println!("{name} request finished");
    format!("{name} data")
}
