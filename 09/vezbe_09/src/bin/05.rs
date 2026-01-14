use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.rust-lang.org/learn".to_string(),
    ];

    trpl::run(async {
        let title_fut_1 = page_title(&args[0]);
        let title_fut_2 = page_title(&args[1]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
