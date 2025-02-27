use cc_api_quote::{QuoteIntent, QuoteIntentRes};
use oxhttp::{
    model::{Method, Request, StatusCode},
    Client,
};
use revm::primitives::Address;

fn main() {
    let url = std::env::var("URL").expect("ENV VAR `URL` NEEDS TO BE SET");
    let client = Client::new();

    let weth: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse()
        .unwrap();
    let usdt: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7"
        .parse()
        .unwrap();

    for i in 0..1000 {
        println!("i {}", i);
        let req = Request::builder()
            .method(Method::POST)
            .uri(&url)
            .body(
                serde_json::to_string(&QuoteIntent {
                    from: usdt,
                    to: weth,
                    input: 1000e6 as u128,
                })
                .expect("work"),
            )
            .expect("work");
        let res = client.request(req).unwrap();
        if res.status() == StatusCode::OK {
            let res = res.into_body().to_string().unwrap();
            let res: QuoteIntentRes = serde_json::from_str(&res).unwrap();
            println!("{:?}", res);
        } else {
            println!("error {:?}", res.status());
        }
    }
}
