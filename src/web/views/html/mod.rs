use horrorshow::template;

template! {
    Html(title: &str) {
        h1 : title;
        p : "Hello, world!";
    }
}

pub fn render() -> String {
    Html::new("blarg")
        .into_string()
        .unwrap_or_else(|e| e.to_string())
}
