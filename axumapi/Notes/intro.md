# Instantiate 
 ```sh
cargo new axum-api
 ```

 - Add feature 

 ```sh
cargo add axum 
# cargo add axum -F headers
cargo add tokio 
cargo add tokio -F full
 ```

 ```toml
[package]
name = "axum-api"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.4"
 ```



 ## derive_more

 - very important package .. check it out 

 


 ## Path .. 

```rs

// in the handler parameter
 Path(uuid) : Path<Uuid> 

// with {}
 .route("/user/update/{uuid}", put(update_user_post));

```

 `http://localhost:3000/user/update/cb4116fd-8cf6-4faa-b52a-fe87ff85b372`





## Query Params 


```rs

// in the handler parameter
Query(params): Query<QueryParams>,

// without {}
.route("/user/update-user", post(update_user_post))

```

 `POST http://localhost:3000/user/update-user?uuid=cb4116fd-8cf6-4faa-b52a-fe87ff85b372`



### Into and try_into 

```rs

.into() // .. convert one type to another .. fallible .. returns new type 

.try_into() // .. convert one type to another .. infallible .. returns Result<_ , _>

```
