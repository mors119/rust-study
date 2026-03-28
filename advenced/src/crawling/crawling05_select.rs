use scraper::{Html, Selector};

const HTML: &str = r#"
<!DOCTYPE html>
<meta charset="utf-8">
<title>Hello, World</title>

<form action="submit_url" method="get" class="form-example">
    <div class="form-example">
        <label for="user-name">이름</label>
        <input type="text" id="user-name" name="user-name" required>
    </div>
    <div class="form-example">
        <label for="user-email">이메일</label>
        <input type="email" id="user-email" name="jeff's email" required>
    </div>
    <input type="submit">
</form>
"#;

pub fn run() {
    // HTML 문자열 조각(fragment)을 파싱
    // parse_fragment는 전체 문서가 아니라 일부 HTML 조각을 다룰 때 적합하다.
    let fragment = Html::parse_fragment(HTML);

    // CSS selector 생성
    let form_selector = Selector::parse("form.form-example").expect("form selector 파싱 실패");
    let input_selector = Selector::parse("input").expect("input selector 파싱 실패");

    // 첫 번째 form 요소 선택
    let form = fragment
        .select(&form_selector)
        .next()
        .expect("form 요소를 찾지 못했습니다.");

    println!("[form outer html]");
    // html()은 선택된 요소 자신까지 포함한 전체 HTML을 반환
    println!("{}", form.html());
    println!();

    println!("[form inner html]");
    // inner_html()은 선택된 요소 내부의 자식 HTML만 반환
    println!("{}", form.inner_html());
    println!();

    // form 태그의 속성 추출
    println!("[form attributes]");
    println!("action = {:?}", form.value().attr("action"));
    println!("method = {:?}", form.value().attr("method"));
    println!("class  = {:?}", form.value().attr("class"));
    println!();

    // 내부 input 태그 순회
    println!("[inputs]");
    for input in form.select(&input_selector) {
        let value = input.value();

        println!("type = {:?}", value.attr("type"));
        println!("id   = {:?}", value.attr("id"));
        println!("name = {:?}", value.attr("name"));
        println!("--------------------------------");
    }
}
