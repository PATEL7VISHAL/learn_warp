Warp use the Reply trait to response of the request 
    - This on fn request is require to impl if we want to derive the from request
        ```rust
        fn into_response(self) -> warp::reply::Response {
            Response::new(format!("message: {}", self.msg).into())
            // Here we can see it's creating the response instance
            // when may be we want send the response
            // means ever type which impl the Reply trait can convortable 
            // into the Response.
        }```

    - after the route path we can add the function `.recover(function)` which help 
    use to handle errors (ouccar in above routes).
    - There args function in `.recover` take `Rejection` type and we can match our 
    custom error through Rejection::find<T>(self) -> Option<&T>


**CORS (Cross-Origin Resource Sharing)**
    * Some time we need some restrictions to allow the web site to some area or not
    for other area.
    * So here instead of sending http Put like request we send the prefight request
    (`HTTP OPTION` request)
        
        * option request asks the server if it's OK to send the request, and the server
        replies with the allowed methods header then then allowed to send PUT request 
        and get the body data.

    * we wrap he route with the `cors` and then it's send the option request which has
    deifined the `allowed headers/methods/orign`

    * In `CORS` we can check the `Headers`, `Origin`, `Methods` and also specified to 
    allow. it's throw and error in `Forbidden` return type. 
    
