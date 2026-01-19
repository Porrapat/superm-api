# SuperM API 🛒

A lightweight REST API built with Rust and Axum for managing supermarket product data. This API provides endpoints to retrieve product information, including nutritional data, pricing, and product images.

👉 Live demo (Leptos version):
https://superm-leptos.porrapat.com/products

👉 Companion Repository
https://www.github.com/Porrapat/superm-leptos

## 🛠️ Tech Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Axum](https://github.com/tokio-rs/axum)** - Web framework built on Tokio
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime
- **[Tower HTTP](https://github.com/tower-rs/tower-http)** - HTTP middleware (CORS, file serving, tracing)
- **[Serde](https://serde.rs/)** - Serialization/deserialization framework

## 🚀 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Porrapat/superm-api.git
   cd superm-api
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Configure environment variables**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` file as needed:
   ```env
   PORT=3001
   BASE_URL=http://localhost:3001
   ```

## ▶️ Running the API

### Development Mode
```bash
cargo run
```

### Production Build
```bash
cargo build --release
./target/release/superm-api
```

The API will be available at `http://localhost:3001` (or your configured PORT).

## 📡 API Endpoints

### Root Endpoint
```
GET /
```
Returns a welcome message.

**Response:**
```
Hello, SuperM API!
```

---

### Get All Products
```
GET /products-list
```
Returns a list of all products with basic information.

**Response Example:**
```json
[
  {
    "id": 1,
    "name": "Tomato",
    "thumbnail": "http://localhost:3001/images/tomato.jpg",
    "final_price": 100,
    "original_price": 120
  },
  {
    "id": 2,
    "name": "Pineapple",
    "thumbnail": "http://localhost:3001/images/pineapple.jpg",
    "final_price": 200,
    "original_price": 200
  }
  // ... more products
]
```

---

### Get Product by ID
```
GET /products/id/{id}
```
Returns detailed information for a specific product.

**Parameters:**
- `id` (path parameter) - Product ID (1-10)

**Response Example:**
```json
[
  {
    "id": 1,
    "name": "Tomato",
    "description": "Juicy and ripe, our tomatoes are perfect for fresh salads, sauces, and cooking...",
    "thumbnail": "http://localhost:3001/images/tomato.jpg",
    "nutrition": {
      "fat": 2,
      "carbs": 10,
      "protein": 4.2
    },
    "final_price": 100,
    "original_price": 120
  }
]
```

**Status Codes:**
- `200 OK` - Product found
- `404 Not Found` - Product not found

---

### Get Product Images
```
GET /images/{filename}
```
Serves static product images.

**Available Images:**
- almonds.jpg
- apple.jpg
- banana.jpg
- bread.jpg
- cheese.jpg
- cookie.jpg
- honey.jpg
- pineapple.jpg
- sugar.jpg
- tomato.jpg

## AI Tools Used

- **Claude (via OpenRouter, VSCode + Cline)**  
  Used as the primary coding assistant for converting all the resources into Axum code.

All outputs from AI tools were reviewed, tested, and manually integrated by me.

## 📝 License

This project is open source and available for educational and commercial use.

## 👤 Author

**Porrapat**
- GitHub: [@Porrapat](https://github.com/Porrapat)

---

Built with ❤️ using Rust and Axum
