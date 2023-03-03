Warp use the Reply trait to response of the request 
- This on fn request is require to impl if we want to derive the from request

```rust
fn into_response(self) -> warp::reply::Response {
    Response::new(format!("message: {}", self.msg).into())
    // Here we can see it's creating the response instance
    // when may be we want send the response
    // means ever type which impl the Reply trait can convortable 
    // into the Response.
}
```

- after the route path we can add the function `.recover(function)` which help 
use to handle errors (ouccar in above routes).
- There args function in `.recover` take `Rejection` type and we can match our 
custom error through Rejection::find<T>(self) -> Option<&T>

----------

<!-- **CORS (Cross-Origin Resource Sharing)** -->
#### CORS (Cross-Origin Resource Sharing)
- Some time we need some restrictions to allow the web site to some area or not
for other area.
- So here instead of sending http Put like request we send the prefight request
(`HTTP OPTION` request)
    
    - option request asks the server if it's OK to send the request, and the server
    replies with the allowed methods header then then allowed to send PUT request 
    and get the body data.

- we wrap he route with the `cors` and then it's send the option request which has
deifined the `allowed headers/methods/orign`

- In `CORS` we can check the `Headers`, `Origin`, `Methods` and also specified to 
allow. it's throw and error in `Forbidden` return type. 

---
#### REST (Representational State Transfer)
- A RESTful API is usually a way to GET, UPDATE, CREATE, and DELETE data through 
HTTP endpoints, which are grouped by resources.

```rust 
    let store_filter = warp::any().map(move || store.clone());

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter) 
```
    
- from this snippits we can create the store/database which can assess over 
our every filters.
- we can appand it between filter and filter replyer callback.
- and also note that the after in the filter replyer callback function which 
take one more field for store varible.

- Same for the `query` store variable it just need to add the `warp::query()`

- __Custom Error__
    - we can create our custom error which juts require to impl interfaces(traits)
    - we can create the Enums for our custom error which require some traits to impl.
        + std::fmt::Display
        + warp::reject::Reject

--- 
#### LOGING:
- There are multiple lever of logging
    - info
    - warn
    - error
    - dubug
- for loging we use some external crate like `log` and `env_logger`.
- `env_logger` helps to define the log level which only log, which is define by the 
environment varible `RUST_LOG`.
- we can also apply filter on warp like the `CORS` and sending data to each filters,
by creating varible from `warp::log::custom(log< |Info|->() >)` and add to rutes by 
`.with` function to the routes.

- There is one more library called 'log4rs' which is also use for loging, but log4rs
contains some extra functionality like to configure by the config file (`_.yaml`)
and can also update config without restarting the server.
- we can save the logs to external file and also manage output format to json too.



--- 
#### ETC
- Different betwenn Mutext and RwLock
    + ___Mutext___ -> for either a writer or reader
    + ___RwLock___ -> allows many readers simultaneously but just one writer at a time

- This both type is a port of std::sync module which means we `can't use it on async
environment`.
- But the `RwLock` dose have the implementations in async as will.

- There are two format to send data from Client side to the server
    + __JSON__
    + __DATA-URLENCODE__ => like the key and values assign by `=` and multiple value 
    saperated by the `&` sign.

- Rust library project contains `lib.rs` which provide a public interface to an underlying 
functionality.

--- 
### Requests:
- GET_QUESTIONS

```console 
curl --location --request GET 'localhost:3030/questions'
```

- ADD_QUSTION

```console
curl --location --request POST 'localhost:3030/questions' \
    --header 'Content-Type: application/json' \
    --data-raw '{
        "id":"1",
        "title": "new title",
        "content": "update content"
}'
```

- UPDATE_QUESTION

```console
curl --location --request PUT 'localhost:3030/questions/2' \
    --header 'Content-Type: application/json' \
    --data-raw '{
      "id": "2",
      "title": "update title",
      "content": "update content"
}'
```


- DELETE_QUSTION

```console
curl --location --request DELETE 'localhost:3030/questions/1'
```

- Add answer

```console
curl --location --request POST 'localhost:3030/questions/1/answers' \
    --header 'Content-type: application/x-www-form-urlencoded' \
    --data-urlencode 'id=1' \
    --data-urlencode 'content="first question content answer' \
```

- Get answers

```console
curl --location --request GET 'localhost:3030/questions/2/answers'
```
