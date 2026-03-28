# Rust study

### Beginner

기초적인 Rust 사용 방법을 학습.

##### 실행방법

```bash
cd beginner/
cargo run [폴더명] [카테고리번호]
```

##### 예시

```bash
cargo run chapter 1
```

```text
beginner/src
├── chapters
│ ├── 00.call.md
│ ├── 01.main.md
│ ├── chapter01_mainfn.rs
│ ├── chapter02_types.md
│ ├── chapter02_var_types.rs
│ ├── chapter03_scalar_types.rs
│ ├── chapter04_sequence_types.rs
│ ├── chapter05_types_default.rs
│ ├── chapter06_function.rs
│ ├── chapter07_method.rs
│ ├── chapter08_macro.rs
│ ├── chapter09_closure.rs
│ ├── chapter10_if.rs
│ ├── chapter11_match.rs
│ ├── chapter12_for.rs
│ ├── chapter13_loop.rs
│ ├── chapter14_while.rs
│ ├── chapter15_vec.rs
│ ├── chapter16_hashmap.rs
│ ├── chapter17_hash_set.rs
│ ├── chapter18_str.rs
│ ├── chapter19_string.rs
│ ├── chapter20_str_and_string.rs
│ ├── chapter21_iter.rs
│ ├── chapter22_map.rs
│ ├── chapter23_filter.rs
│ ├── chapter24_filter_map.rs
│ ├── chapter25_for_each.rs
│ ├── chapter26_take_while.rs
│ ├── chapter27_example.rs
│ ├── chapter28_owner_ship.rs
│ ├── chapter29_copy.rs
│ ├── chapter30_struct.rs
│ ├── chapter31_enum.rs
│ ├── chapter32_option.rs
│ ├── chapter33_result.rs
│ ├── chapter34_trait_1.rs
│ ├── chapter35_trait_2.rs
│ ├── chapter36_polymorphism.rs
│ ├── chapter37_generic_1.rs
│ ├── chapter38_generic_2.rs
│ ├── chapter39_lite_time.rs
│ ├── chapter40_smart_pointer_1.rs
│ ├── chapter41_smart_pointer_2.rs
│ ├── chapter42_error_handling.rs
│ ├── chapter43_module.rs
│ ├── chapter44_etc_1.rs
│ ├── chapter45_etc_2.rs
│ ├── dchapter.rs
│ └── mod.rs
├── main.rs
└── match_chapter.rs
```

### Advanced

라이브러리 사용과 파일 시스템, 네트워크, 스레드 등 고급 기능 기초 학습

##### 실행방법

```bash
cd advanced/
cargo run [폴더명] [카테고리번호]
```

##### 예시

```bash
cargo run asynchronous 1
```

```bash
# axum은 테스트를 많이 해서 번호 없이 실행
cargo run axum
```

```text
advanced/src
├── asynchronous
│   ├── async01_async_await.rs
│   ├── async02_tokio_file.rs
│   ├── async03_tokio_byte.rs
│   ├── async04_tokio_buf.rs
│   ├── async05_tokio_tcp_server.rs
│   ├── async06_tokio_tcp_client.rs
│   ├── async07_tokio_web_socket.rs
│   ├── async08_tokio_sample.rs
│   ├── async09_tokio_web_socket_test.rs
│   └── mod.rs
├── command
│   ├── cmd01_command_line.rs
│   ├── cmd02_console_input.rs
│   ├── cmd03_console_output.rs
│   ├── cmd04_log.rs
│   └── mod.rs
├── crawling
│   ├── crawling01_reqwest.md
│   ├── crawling01_reqwest.rs
│   ├── crawling02_get.rs
│   ├── crawling03_post.rs
│   ├── crawling04_scraper.rs
│   ├── crawling05_select.rs
│   ├── crawling06_node.rs
│   ├── crawling06-2_thirty_four.md
│   └── mod.rs
├── encryption
│   ├── encryption01_random.rs
│   ├── encryption02_hash.rs
│   ├── encryption03_symmetric_key.rs
│   ├── encryption04_asymmetric_key.rs
│   └── mod.rs
├── files
│   ├── file01_create.rs
│   ├── file02_write.rs
│   ├── file03_open.rs
│   ├── file04_read.rs
│   ├── file05_del_copy_move.rs
│   ├── file06_directory.rs
│   ├── file07_file_backup.rs
│   └── mod.rs
├── main.rs
├── matcher.rs
├── network
│   ├── mod.rs
│   ├── net01_tcp.rs
│   ├── net02_udp.rs
│   └── net03_udp_like_tcp.rs
├── thread
│   ├── mod.rs
│   ├── thread01_basic.rs
│   ├── thread02_move.rs
│   ├── thread03_mpsc.rs
│   ├── thread04_mpmc.rs
│   ├── thread05_rc.rs
│   ├── thread06_arc.rs
│   ├── thread07_mutex.rs
│   └── thread08_arc_mutex.rs
└── web_server
    ├── db.rs
    ├── folder.md
    ├── mod.rs
    ├── user
    │   ├── handler.rs
    │   ├── mod.rs
    │   ├── model.rs
    │   └── repo.rs
    ├── web_server01_main.rs
    └── web_server02_actix_web.rs
```
