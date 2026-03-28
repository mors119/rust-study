use scraper::html::Html;
use scraper::{ElementRef, Node};

const HTML: &str = r#"
<b>앵무새 투이의 모험</b>
<br>
"하, 이번에도라니.
무례한 녀석."
<br>
나는 여전히 앞을 본 채로 
걸음을 멈추고 대답했다.
<div> 맞아 이번에는 꼭 
농장에 들어갈거야 </div>
"..."
<br>
"내가 대답하지 않자.
멜루스는 날개를 들어
길을 열어주고는 상냥하게 말했다."
<br>
"네가 걱정되서 그래
불쌍한 투이.
이번엔 꼭 통과해야 할텐데."
"#;

pub fn run() {
    let fragment = Html::parse_fragment(HTML);
    let root = fragment.root_element();

    let mut output = String::new();

    for node in root.children() {
        match node.value() {
            Node::Element(element) => match element.name() {
                // <br>은 줄바꿈으로 취급
                "br" => {
                    output.push('\n');
                }

                // <b>는 태그는 버리고 내부 텍스트만 사용
                "b" => {
                    if let Some(ele) = ElementRef::wrap(node) {
                        output.push_str(&ele.text().collect::<String>());
                    }
                }

                // <div>도 태그는 버리고 내부 텍스트만 살림
                "div" => {
                    if let Some(ele) = ElementRef::wrap(node) {
                        output.push_str(&ele.text().collect::<String>());
                    }
                }

                // 그 외 element는 내부 텍스트만 수집
                _ => {
                    if let Some(ele) = ElementRef::wrap(node) {
                        output.push_str(&ele.text().collect::<String>());
                    }
                }
            },

            // 일반 텍스트 노드
            Node::Text(text) => {
                output.push_str(&text);
            }

            _ => {}
        }
    }

    // 전체 결과를 마지막에 한 번 더 정리
    let cleaned = normalize_text(&output);

    println!("{}", cleaned);
}

/// 노드 하나를 보고 출력 문자열에 누적한다.
///
/// 매개변수:
/// - node:
///   현재 순회 중인 DOM 노드
/// - out:
///   최종 결과를 누적할 문자열 버퍼

/// 최종 텍스트를 사람이 읽기 좋게 정리한다.
///
/// 여기서는:
/// - 양쪽 공백 정리
/// - 연속된 빈 줄 축소
/// - 줄 단위 trim
fn normalize_text(input: &str) -> String {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    lines.join("\n")
}
