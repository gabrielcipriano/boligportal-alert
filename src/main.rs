mod payload;
mod bolig_response;

use payload::{BoligportalQueryParams, BoligportalSearchPayload};
// use std::collections::HashMap;

use reqwest::Client;
use tokio::sync::Mutex;
use std::{collections::VecDeque, sync::Arc};


use tokio_cron_scheduler::{Job, JobScheduler};


// use payload::BoligportalSearchPayload;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::bolig_response::Listing;



#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Subscribes to alerts. Please provide a valid token.")]
    Subscribe(String),
    #[command(description = "lists alerts.")]
    List,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(reqwest::Client::new());
    let listing_queue: Arc<Mutex<Vec<Listing>>> = Arc::new(Mutex::new(Vec::new()));
    let last_delivered_ids: Arc<Mutex<VecDeque<i32>>> = Arc::new(Mutex::new(VecDeque::new()));

    let chat_ids: Arc<Mutex<Vec<ChatId>>> = Arc::new(Mutex::new(Vec::new()));

    async fn fetch_new_listings(client: Arc<Client>, queue: Arc<Mutex<Vec<Listing>>>, seen_ids: Arc<Mutex<VecDeque<i32>>>) -> Result<(), Box<dyn std::error::Error>> {
        let query_params = BoligportalQueryParams::new(1);
        let url = reqwest::Url::parse_with_params("https://www.boligportal.dk/en/api/search/list", query_params.to_params_tuples());
    
        let body = BoligportalSearchPayload::default();

        let res = client
            .post(url.unwrap())
            .body(body.to_string())
            .send()
            .await?;

        let json: bolig_response::JsonResponse = res.json().await?;

        let mut queue = queue.lock().await;
        let mut seen_ids = seen_ids.lock().await;

        for listing in json.results.iter() {
            if !seen_ids.contains(&listing.id) {
                seen_ids.push_front(listing.id);
                queue.push(listing.clone());
            } else {
                println!("Already seen: {}", listing.id);
                break;
            }
        }

        if seen_ids.len() > 18 {
            seen_ids.drain(18..);
        }

        Ok(())
    }


    let mut sched = JobScheduler::new().await?;


    // Add async job
    let client_fetch_job = client.clone();
    let queue_fetch_job = listing_queue.clone();
    let seen_ids_fetch_job = last_delivered_ids.clone();

    sched.add(
        Job::new_async("23 */13 * * * *", move |_uuid, _l| {
            let client_fetch_job = Arc::clone(&client_fetch_job);
            let queue_fetch_job = Arc::clone(&queue_fetch_job);
            let seen_ids_fetch_job = Arc::clone(&seen_ids_fetch_job);

            Box::pin(async move {
                println!("I run async every 9 minutes");
                println!("currnt time {:?}", chrono::Local::now().time());
                println!("Queue length: {}", queue_fetch_job.lock().await.len());

                fetch_new_listings(
                    Arc::clone(&client_fetch_job),
                    Arc::clone(&queue_fetch_job),
                    Arc::clone(&seen_ids_fetch_job),
                ).await.unwrap();
            })
        })?
    ).await?;

    let bot = Bot::from_env();


    // Add async job
    // let queue_print_job = listing_queue.clone();
    // let chats_print_job = chat_ids.clone();
    // let bot_print_job = Arc::clone(&bot);
    // sched.add(
    //     Job::new_async("1/5 * * * * *", move |_uuid, _l| {
    //         let queue = queue_print_job.clone();
    //         let chats = chats_print_job.clone();
    //         let bot = bot_print_job.clone();
    //         Box::pin(async move {
    //             println!("I run async every 5 seconds");

    //             if chats.lock().await.len() == 0 || queue.lock().await.len() == 0{
    //                 return;
    //             }

    //             for chat_id in chats.lock().await.iter() {
    //                 for listing in queue.lock().await.iter() {
    //                     bot.send_message(*chat_id, listing.human_friendly()).await.unwrap();
    //                 }
    //             }

    //             // queue.lock().await.iter().for_each(|listing| {
    //             //     println!("{:?}", listing.human_friendly());
    //             // });
    //             println!("Queue length: {}", queue.lock().await.len());
    //         })
    //     })?
    // ).await?;


    let cmd_handler = move |bot: Bot, msg: Message, cmd: Command| {
        let chats = chat_ids.clone();
        let queue = listing_queue.clone();
        async move {
            match cmd {
                Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
                Command::Subscribe(token) => {
                    if token != "cipricelly" {
                        bot.send_message(msg.chat.id, "Invalid token").await?;
                        return Ok(());
                    }
                    if chats.lock().await.iter().find(|chat_id| **chat_id == msg.chat.id).is_some() {
                        bot.send_message(msg.chat.id, "You are already subscribed").await?;
                        return Ok(());
                    }
                    chats.lock().await.push(msg.chat.id);
                    bot.send_message(msg.chat.id, "valid token 🎉").await?
                }
                Command::List => {
                    if chats.lock().await.iter().find(|chat_id| **chat_id == msg.chat.id).is_none() {
                        bot.send_message(msg.chat.id, "You are not subscribed").await?;
                        return Ok(());
                    } else {
                        let mut listings_human_friendly: Vec<String> = Vec::new();
                        println!("Queue length: {}", queue.lock().await.len());
                        queue.lock().await.iter().for_each(|listing| {
                            listings_human_friendly.push(listing.human_friendly());
                        });
                        if listings_human_friendly.len() == 0 {
                            bot.send_message(msg.chat.id, "No new listings").await?;
                            return Ok(());
                        }
                        for listing_msg in listings_human_friendly.chunks(3) {
                            bot.send_message(msg.chat.id, listing_msg.join("\n-------\n").as_str()).await?;
                        }
                        // bot.send_message(msg.chat.id, listings_human_friendly.join("\n-------\n")).await?;
                        queue.lock().await.clear();
                        return Ok(())
                    }
                    // let mut listings_human_friendly: Vec<String> = Vec::new();
                    // queue.lock().await.iter().for_each(|listing| {
                    //     listings_human_friendly.push(listing.human_friendly());
                    // });
                    // bot.send_message(msg.chat.id, listings_human_friendly.join("\n-------\n")).await?;
                }
            };
            Ok(())
        }
    };

    sched.shutdown_on_ctrl_c();

    // Add code to be run during/after shutdown
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    sched.start().await?;



    Command::repl(bot, cmd_handler).await;

    // tokio::time::sleep(Duration::from_secs(100)).await;
    
    // let html = reqwest::get(url)
    //     .await?
    //     .text()
    //     .await?;
    // println!("{html:#?}");
    // let document = Html::parse_document(&html);

    // let selector = Selector::parse("#store").unwrap();

    // let json_str = document.select(&selector).next().unwrap().inner_html();
    
    // let j: JsonStore = serde_json::from_str(&json_str).unwrap();

    
    // j.props.page_props.results.iter().for_each(|listing| {
    //     if listing.available_from.is_none() {
    //         println!("{:?}", listing);
    //     }
    // });

    // let shared_displayed_ids: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));


    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {

    //     let listings_human_friendly: Vec<String> = Vec::new();

    //     let res = client
    //         .post(url.unwrap())
    //         .body(body.to_string())
    //         .send()
    //         .await?;

    //     let json: JsonResponse = res.json().await?;

    //     json.results.iter().for_each(|listing| {
    //         if !displayed_ids.contains(&listing.id) {
    //             displayed_ids.push(listing.id);
    //             let human_friendly = listing.human_friendly();
    //             listings_human_friendly.push(human_friendly);
    //         }
    //     });


    //     bot.send_message(msg.chat.id, listings_human_friendly.join("\n")).await?;
    //     Ok(())
    // })
    // .await;

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     let mut listings_human_friendly: Vec<String> = Vec::new();

    //     let query_params = BoligportalQueryParams::new(1);
    //     let url = reqwest::Url::parse_with_params("https://www.boligportal.dk/en/api/search/list", query_params.to_params_tuples());
    
    //     let body = BoligportalSearchPayload::default();
    
    //     let client = reqwest::Client::new();

    //     let res = client
    //         .post(url.unwrap())
    //         .body(body.to_string())
    //         .send()
    //         .await?;

    //     let json: JsonResponse = res.json().await?;

    //     // let mut displayed_ids = shared_displayed_ids.lock().await;

    //     json.results.iter().for_each(|listing| {
    //         // if !displayed_ids.contains(&listing.id) {
    //         // displayed_ids.push(listing.id);
    //         let human_friendly = listing.human_friendly();
    //         listings_human_friendly.push(human_friendly);
    //         // }
    //     });


    //     bot.send_message(msg.chat.id, listings_human_friendly.join("\n")).await?;
    //     Ok(())
    // })
    // .await;


    // let handler = Update::filter_message().endpoint(
    //     |bot: Bot, messages_total: Arc<AtomicU64>, msg: Message| async move {
    //         let previous = messages_total.fetch_add(1, Ordering::Relaxed);
    //         bot.send_message(msg.chat.id, format!("I received {previous} messages in total."))
    //             .await?;
    //         respond(())
    //     },
    // );

    // Dispatcher::builder(bot, handler)
    //     // Pass the shared state to the handler as a dependency.
    //     .dependencies(dptree::deps![messages_total])
    //     .enable_ctrlc_handler()
    //     .build()
    //     .dispatch()
    //     .await;

    Ok(())
}