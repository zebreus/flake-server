use flo_stream::{MessagePublisher, Publisher};

use futures_util::{FutureExt, StreamExt};
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;

use warp::{ws::Message, Filter};

async fn run_server() -> () {
    let publisher: Arc<Publisher<Message>> = Arc::new(Publisher::new(10));
    let mut republisher = publisher.republish_weak();

    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let lines = reader.lines();
    tokio::spawn(async move {
        tokio::pin!(lines);
        while let Ok(item) = lines.next_line().await {
            let res: Message = match item {
                Some(item) => Message::text(item),
                // None => warp::Error::new(Box::new(MyError::Other)),
                None => todo!(),
            };
            republisher.publish(res).await;
        }
    });

    let routes = warp::path("socket")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let mut republisher = publisher.republish_weak();
            let subscriber = republisher.subscribe();
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Echo from stdin to websockets

                let (tx, _rx) = websocket.split();

                subscriber
                    .map(|message| dbg!(Ok(message)))
                    .forward(tx)
                    .map(|result| match result {
                        Ok(_) => println!("websocket closed"),
                        Err(e) => eprintln!("websocket error: {:?}", e),
                    })
            })
        });

    // let mut reader = tokio::io::BufReader::new(tokio::io::stdin());

    // let mut lines = reader.lines();

    // let stream = async_stream::stream! {
    //     while let Ok(item) = lines.next_line().await {
    //         let res: Message = match item {
    //             Some(item) => Message::text(item),
    //             // None => warp::Error::new(Box::new(MyError::Other)),
    //             None => todo!(),
    //         };
    //         yield res.clone();
    //     }
    // };

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[tokio::main]
async fn main() {
    run_server().await
}
