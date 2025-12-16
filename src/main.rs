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

// products/id/:number
/*
[{"id":1,"name":"Tomato","description":"Juicy and ripe, our tomatoes are perfect for fresh salads, sauces, and cooking. They offer a balanced flavor that's both sweet and tangy, making them a versatile ingredient in any dish. Packed with vitamins and antioxidants, these tomatoes are a healthy choice for your meals.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640667/react-tutorial/superm-v2/tomato.jpg","nutrition":{"fat": 2, "carbs": 10, "protein": 4.2},"final_price":100,"original_price":120}]
[{"id":2,"name":"Pineapple","description":"Bursting with tropical sweetness, our pineapples are picked at peak ripeness for a juicy and refreshing taste. Ideal for snacking, fruit salads, or adding a touch of zest to savory dishes and desserts, they offer a vibrant and flavorful experience.<br>\n<br>\nPacked with vitamin C and digestive enzymes, these pineapples not only taste great but also support your health. Enjoy them fresh, grilled, or blended into smoothies for a nutritious and versatile addition to any meal1.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/pineapple.jpg","nutrition":{"fat": 2, "carbs": 10, "protein": 1},"final_price":200,"original_price":200}]
[{"id":3,"name":"Banana","description":"Sweet and creamy, our bananas are perfect for snacking, adding to breakfast dishes, or blending into smoothies. With their smooth texture and natural sweetness, they provide a delicious and nutritious option. Packed with potassium and fiber, these bananas support heart health and digestion, making them a great addition to any meal.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/banana.jpg","nutrition":{"fat": 2, "carbs": 20, "protein": 3},"final_price":75,"original_price":50}]
[{"id":4,"name":"Apple","description":"Crisp and juicy, our apples offer a refreshing burst of flavor and are perfect for snacking, baking, or adding to salads. With a variety of sweet and tart options available, they provide a delicious and versatile fruit choice. Packed with fiber and vitamins, apples contribute to overall health, supporting digestion and providing essential nutrients for a balanced diet.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640668/react-tutorial/superm-v2/apple.jpg","nutrition":{"fat": 1, "carbs": 15, "protein": 1},"final_price":100,"original_price":130}]
[{"id":5,"name":"Cheese","description":"Rich and flavorful, our cheese selection offers a variety of textures and tastes to suit every palate.<br>\n<br>\nFrom creamy and tangy to sharp and crumbly, each cheese is crafted with care to enhance your dishes or serve as a delectable snack. Whether you're enjoying it on its own, paired with wine, or melted into recipes, our cheese adds a gourmet touch to any meal. Packed with protein and calcium, it also provides essential nutrients for a balanced diet.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640667/react-tutorial/superm-v2/cheese.jpg","nutrition":{"fat": 24, "carbs": 5, "protein": 19},"final_price":500,"original_price":500}]
[{"id":6,"name":"Honey","description":"Golden and naturally sweet, our honey is perfect for adding a touch of sweetness to teas, baked goods, or yogurt. Sourced from local beekeepers, it captures the essence of the flowers in every jar.<br>\n<br>\nThis pure honey is rich in antioxidants and has been enjoyed for centuries for its natural health benefits. Its smooth texture and distinct flavor make it a versatile ingredient in both sweet and savory recipes.<br>\n<br>\nIdeal for drizzling over breakfast or using as a natural sweetener in cooking, our honey brings a wholesome, delightful taste to your kitchen.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640696/react-tutorial/superm-v2/honey.jpg","nutrition":{"fat": 20, "carbs": 10, "protein": 8},"final_price":400,"original_price":500}]
[{"id":7,"name":"Sugar","description":"Sweet and versatile, our sugar is perfect for baking, cooking, and sweetening beverages. Its fine granules dissolve easily, making it ideal for a wide range of recipes and desserts.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/sugar.jpg","nutrition":{"fat": 9, "carbs": 10, "protein": 0},"final_price":100,"original_price":100}]
[{"id":8,"name":"Almonds","description":"Crunchy and nutty, our almonds are a great snack on their own or a perfect addition to salads, baked goods, and trail mixes. They provide a satisfying texture and rich flavor that enhances any dish.\n\nPacked with healthy fats, protein, and vitamins, almonds offer a nutritious boost to your diet. Their versatility makes them a staple for both sweet and savory recipes.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/almonds.jpg","nutrition":{"fat": 20, "carbs": 9, "protein": 15},"final_price":300,"original_price":350}]
[{"id":9,"name":"Bread","description":"Our bread loaf features a perfectly crusty exterior and a soft, airy interior, making it ideal for a variety of uses from sandwiches to toast. Baked fresh daily with premium ingredients, it offers a rich flavor and satisfying texture.<br>\n<br>\nWhether used for hearty meals or light snacks, this versatile loaf provides essential carbohydrates and complements any topping or spread. Enjoy it with everything from simple butter to gourmet cheeses for a delightful eating experience.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640695/react-tutorial/superm-v2/bread.jpg","nutrition":{"fat": 4, "carbs": 33, "protein": 9},"final_price":200,"original_price":250}]
[{"id":10,"name":"Chocolate chip cookie","description":"Our chocolate chip cookie is a classic treat with a crisp edge and a chewy center, studded with rich, melty chocolate chips. Perfectly balanced in sweetness and texture, it offers a comforting, indulgent experience with every bite.","thumbnail":"https://res.cloudinary.com/dbfn5lnvx/image/upload/v1726640683/react-tutorial/superm-v2/cookie.jpg","nutrition":{"fat": 9, "carbs": 19, "protein": 13},"final_price":200,"original_price":250}]
*/
