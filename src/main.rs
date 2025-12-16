use dotenvy::dotenv;
use std::env;
use axum::{
    routing::get,
    Router,
    Json
};

use tower_http::{
    services::ServeDir
};

use serde::Serialize;

#[derive(Serialize)]
struct Product {
    id: u32,
    name: String,
    original_price: u32,
    final_price: u32,
    thumbnail: String,
}

async fn products_list() -> Json<Vec<Product>> {
    dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    Json(vec![
        Product {
            id: 4,
            name: "Apple".into(),
            original_price: 130,
            final_price: 100,
            thumbnail: format!("{}/images/apple.jpg", base_url),
        },
        Product {
            id: 5,
            name: "Cheese".into(),
            original_price: 500,
            final_price: 500,
            thumbnail: format!("{}/images/cheese.jpg", base_url),
        },
        Product {
            id: 6,
            name: "Honey".into(),
            original_price: 500,
            final_price: 400,
            thumbnail: format!("{}/images/honey.jpg", base_url),
        },
        Product {
            id: 7,
            name: "Sugar".into(),
            original_price: 100,
            final_price: 100,
            thumbnail: format!("{}/images/sugar.jpg", base_url),
        },
        Product {
            id: 1,
            name: "Tomato".into(),
            original_price: 120,
            final_price: 100,
            thumbnail: format!("{}/images/tomato.jpg", base_url),
        },
        Product {
            id: 8,
            name: "Almonds".into(),
            original_price: 350,
            final_price: 300,
            thumbnail: format!("{}/images/almonds.jpg", base_url),
        },
        Product {
            id: 9,
            name: "Bread".into(),
            original_price: 250,
            final_price: 200,
            thumbnail: format!("{}/images/bread.jpg", base_url),
        },
        Product {
            id: 10,
            name: "Chocolate chip cookie".into(),
            original_price: 250,
            final_price: 200,
            thumbnail: format!("{}/images/cookie.jpg", base_url),
        },
        Product {
            id: 2,
            name: "Pineapple".into(),
            original_price: 200,
            final_price: 200,
            thumbnail: format!("{}/images/pineapple.jpg", base_url),
        },
        Product {
            id: 3,
            name: "Banana".into(),
            original_price: 50,
            final_price: 75,
            thumbnail: format!("{}/images/banana.jpg", base_url),
        },
    ])
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/products-list", get(products_list))
        .nest_service("/images", ServeDir::new("images"));

    println!("SuperM API running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
// products-list
/*
[{"id":4,"name":"Apple","original_price":130,"final_price":100,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/apple.jpg"}, 
 {"id":5,"name":"Cheese","original_price":500,"final_price":500,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640667/react-tutorial/superm-v2/cheese.jpg"}, 
 {"id":6,"name":"Honey","original_price":500,"final_price":400,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640696/react-tutorial/superm-v2/honey.jpg"}, 
 {"id":7,"name":"Sugar","original_price":100,"final_price":100,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/sugar.jpg"}, 
 {"id":1,"name":"Tomato","original_price":120,"final_price":100,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640667/react-tutorial/superm-v2/tomato.jpg"}, 
 {"id":8,"name":"Almonds","original_price":350,"final_price":300,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/almonds.jpg"}, 
 {"id":9,"name":"Bread","original_price":250,"final_price":200,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/bread.jpg"}, 
 {"id":10,"name":"Chocolate chip cookie","original_price":250,"final_price":200,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640683/react-tutorial/superm-v2/cookie.jpg"}, 
 {"id":2,"name":"Pineapple","original_price":200,"final_price":200,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/pineapple.jpg"}, 
 {"id":3,"name":"Banana","original_price":50,"final_price":75,"thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/banana.jpg"}]

*/
/*
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/apple.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640667/react-tutorial/superm-v2/cheese.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640696/react-tutorial/superm-v2/honey.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/sugar.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640667/react-tutorial/superm-v2/tomato.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/almonds.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/bread.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640683/react-tutorial/superm-v2/cookie.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/pineapple.jpg
https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/banana.jpg
*/

// products/id/:number
/*
[{"id":4,"name":"Apple","description":"Crisp and juicy, our apples offer a refreshing burst of flavor and are perfect for snacking, baking, or adding to salads. With a variety of sweet and tart options available, they provide a delicious and versatile fruit choice. Packed with fiber and vitamins, apples contribute to overall health, supporting digestion and providing essential nutrients for a balanced diet.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/apple.jpg","nutrition":{"fat": 1, "carbs": 15, "protein": 1},"final_price":100,"original_price":130}]
*/
