use std::io::{Error, ErrorKind};


//return either a json formatted structure, or an error message from reqwest library
async fn my_async_call(url: &str) -> Result<serde_json::Value,  Error>{

   let response: reqwest::Response = reqwest::get(url)
   .await
   .map_err(|_| Error::new(ErrorKind::Other, "could not retrieve response"))?; //map one type of error to another using | | function

   let json_response = response.json::<serde_json::Value>()
                                    .await
                                    .map_err(|_| Error::new(ErrorKind::Other, "could not decode to JSON"))?;
    // solution for when using reqwest::Error library
    // //await for waiting the async tasks to finish
    // let response: serde_json::Value = reqwest::get(url)
    // .await?
    // .json::<serde_json::Value>()
    // .await?;

    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_fn(){
        let api_url="https://cat-fact.herokuapp.com/facts/"; //public api endpoint\
        let my_res = my_async_call(api_url).await;
        match my_res{
            Ok(r) => {
                dbg!(r);
            },
            Err(_) => {
                panic!("Failed to make request!");
            }
        };
    }
}