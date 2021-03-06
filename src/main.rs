use core::time;
use digest_auth::AuthContext;
use futures_util::StreamExt;
use http::Uri;
use image;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat, RgbImage};
use reqwest::Url;

use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::{
    convert::TryInto,
    time::{Duration, SystemTime},
};
use surf::Client;
use async_std::future;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // tokio::spawn(async {
    //     let client = reqwest::Client::new();
    //     let cam_url = "http://10.50.29.96/mjpgstreamreq/1/image.jpg";
    //     let _resp = client.get(cam_url).send().await.unwrap();
    //     println!("Async task 1 started.");
    // });
    // tokio::spawn(async {
    //     let client = reqwest::Client::new();
    //     let cam_url = "http://10.50.29.96/mjpgstreamreq/1/image.jpg";
    //     let _resp = client.get(cam_url).send().await.unwrap();
    //     println!("Async task 2 started.");
    // });
    // tokio::spawn(async {
    //     let client = reqwest::Client::new();
    //     let cam_url = "http://10.50.29.96/mjpgstreamreq/1/image.jpg";
    //     let _resp = client.get(cam_url).send().await.unwrap();
    //     println!("Async task 3 started.");
    // });

   

    // let client = reqwest::Client::new();

    const SOI: u8 = 0xD8;
    ///     End of image
    const EOI: u8 = 0xD9;

    const NONE: u8 = 0xFF;

    let mut store_buffer: Vec<u8> = Vec::new();
    let mut is_begin_frame = false;
    let mut frame_buffer: Vec<u8> = Vec::new();
    // let cam_url = "http://10.50.12.180:80/mjpgstreamreq/1/image.jpg?resolution=640x480";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.31.178/mjpg/1/video.mjpg";
    // let cam_url = "http://10.50.29.64/axis-cgi/mjpg/video.cgi";
    // let cam_url = "http://10.50.29.64/mjpg/1/video.mjpg"; //digest
    // let cam_url = "http://climatecam.gi.alaska.edu/mjpg/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.31.178/mjpg/1/video.mjpg?resolution=640x480"; //digest
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.117/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.56/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.39/mjpg/1/video.mjpg?resolution=640x480";
    // let cam_url = "http://10.50.29.36/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.13.89/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.29.36:80/mjpgstreamreq/1/image.jpg?resolution=640x480";
    // let cam_url = "http://10.50.31.241/mjpg/1/video.mjpg";
    let cam_url = "http://vietnam:L3xRay123!@10.50.30.212/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.13.89/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay12@10.50.31.179/mjpg/1/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay12@10.50.31.179:80/mjpg/1/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay123@10.50.12.187:80/video/mjpeg/stream2";
    // let cam_url = "http://10.50.13.23/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.30.108/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.36/jpgimage/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.22/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.30.211/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay12@10.50.13.226/mjpeg.cgi";
    // let cam_url = "http://10.50.31.190/video/mjpeg/stream1";
    // unauth
    // let cam_url = "http://10.50.30.100/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.13.23/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.29.32/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.29.96/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.30.197/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.29.77/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.30.118/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.236/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.241/mjpg/1/video.mjpg";
    // let cam_url = "http://10.50.29.162/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.171/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.53/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.172/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.78/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.74/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.169/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.13.231/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.63/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.29.117/";
    let mut count: i32 = 1;

    // let client: Client = surf::Config::new()
    // .set_http_keep_alive(false)
    //     // .set_timeout(Some(std::time::Duration::from_secs(30)))
    //     .try_into()
    //     .unwrap();

        
        let client = reqwest::Client::builder()
        // .timeout(time::Duration::from_secs(30))
        .build()
        .unwrap();
        // let resp = client.get(cam_url).send().await;
        // println!("Response: {:?}", resp);

    // // let mut answer = String::new();
    // // let cam_url = surf::Url::parse(cam_url).unwrap();
    // let basic_url = Url::parse(cam_url).unwrap();
    // let username = basic_url.username();
    // let password = basic_url.password().unwrap_or_default();

    // // let mut digest_url = basic_url.clone();
    // // digest_url.set_username("");
    // // digest_url.set_password(None);
    // // digest_url.set_port(Some(80));

    // // println!("CAM URL: {}", basic_url);
    // // println!("CAM DIGEST URL: {}", digest_url);
    // // let resp = surf::get(digest_url.as_str()).await;
    // // let resp = client.get(digest_url.as_str()).send().await;
    // // if resp.is_err() {
    // //     println!("Requeset header error {:?}",resp.err());
    // //     return Ok(());
    // // }
    // // let resp = resp.unwrap();
    // // let header = resp.headers();

    // // println!("Port: {:?}", basic_url.port());
    // println!("usr: {} - pwd: {}", username, password);
    // // println!("[CAMERA] CAMERA STATUS {:?}", resp.status());
    
    // // match client.get(digest_url.as_str()).send().await {
    // //     Ok(mut resp) => {
    // //         resp.take_body();
    // //         if let Some(value) = resp.header("www-authenticate") {
    // //             match value.to_string().split_once(' ').unwrap_or_default().0 {
    // //                 "Digest" => {
    // //                     println!("Digest Camera.");
    // //                     let mut prompt = digest_auth::parse(&value.to_string()).unwrap();
    // //                     let context = AuthContext::new(username, password, digest_url.path());
    // //                     answer = prompt.respond(&context).unwrap().to_header_string();
    // //                     println!("Answer: {:?}", answer);
    // //                 }
    // //                 _ => {}
    // //             }
    // //         }
    // //         let response = match answer.as_str() {
    // //             "" => client.get(basic_url.as_str()).send().await,
    // //             _ => {
    // //                 client
    // //                     .get(digest_url.as_str())
    // //                     .header("authorization", answer.clone())
    // //                     .send()
    // //                     .await
    // //             }
    // //         };
    // //         match response {
    // //             Ok(mut response) => {
    // //                 let status_code = response.status().to_string();
    // //                 println!("[CAMERA] CAMERA STATUS {:?}", status_code);
    // //                 response.take_body();
    // //             }
    // //             Err(e) => {
    // //                 println!("Response error: {:?}", e);
    // //             }
    // //         }
    // //     },
    // //     Err(e) => {
    // //         println!("Response first error: {:?}", e);
    // //     }
    // // }
    // // // loop {}
    
    // // if let Some(value) = header.get("www-authenticate") {
    // //     if let Ok(value) = value.to_str() {
    // //         match value.split_once(' ').unwrap_or_default().0 {
    // //             "Digest" => {
    // //                 let mut prompt = digest_auth::parse(value).unwrap();
    // //                 let context = AuthContext::new(username, password, digest_url.path());
    // //                 answer = prompt.respond(&context).unwrap().to_header_string();
    // //             }
    // //             _ => {}
    // //         }
    // //     }
    // // }
    
    // let is_digest = false;
    // let cam_url = surf::Url::parse(cam_url).unwrap();
    // let response = match is_digest {
    //     false => {
    //         // client
    //         let f = surf::get(cam_url)
    //         .send();
    //         match future::timeout(std::time::Duration::from_secs(15), f).await {
    //             Ok(v) => v,
    //             Err(e) => {
    //                 println!("timeout error: {:?}", e);
    //                 Err(surf::Error::from_str(408, "request timeout"))
    //             }
    //         }
    //     }
    //     _ => {
    //         let mut digest_url = cam_url.clone();
    //         digest_url.set_username("").expect("couldn't set username");
    //         digest_url.set_password(None).expect("couldn't set password");
    //         let resp = client.get(digest_url.as_str()).send().await;

    //         let answer = match resp {
    //             Ok(resp) => match resp.header("www-authenticate") {
    //                 Some(value) => {
    //                     match value.as_str().split_once(' ').unwrap_or_default().0 {
    //                                 "Digest" => {
    //                                     let mut prompt = digest_auth::parse(value.as_str()).unwrap();
    //                                     let context =
    //                                         AuthContext::new(cam_url.username(), cam_url.password().unwrap_or_default(), digest_url.as_str());
    //                                     prompt.respond(&context).unwrap().to_header_string()
    //                                 }
    //                                 _ => "".to_owned(),
    //                             }
    //                 }
    //                 None => "".to_owned(),
    //             },
    //             Err(e) => {
    //                 "".to_owned()
    //             }
    //         };
    //         println!("Answer: {:?}", answer);

    //         // client
    //             surf::get(digest_url.as_str())
    //             // .basic_auth("vietnam", Some("L3xRay123!"))
    //             .header(surf::http::headers::AUTHORIZATION, answer)
    //             .send()
    //             .await
    //             }
    // };

    // match response {
    //     Ok(mut response) => {
    //         println!("Resp status: {:?}", response.status());
    //         // let status_code = response.status().to_string();
    //         response.take_body();
    //         // response.body_bytes();
    //     },
    //     Err(e) => {
    //         println!("Response error: {:?}", e.status());
    //     },
    // };
    // tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    // loop {}
    // return Ok(());

    // //END
    let basic_url = reqwest::Url::parse(cam_url).unwrap();
    println!("{:?}", basic_url);
    let username = basic_url.username();
    let password = basic_url.password().unwrap_or_default();
    let mut digest_url = basic_url.clone();
    digest_url.set_username("").expect("couldn't set username");
    digest_url.set_password(None).expect("couldn't set password");
    let resp = client.get(digest_url.as_str()).send().await;

    let answer = match resp {
        Ok(resp) => match resp.headers().get("www-authenticate") {
            Some(value) => {
                println!("Header: {:?}", resp.headers());
                match value.to_str() {
                    Ok(value) => {
                        match value.split_once(' ').unwrap_or_default().0 {
                            "Digest" => {
                                let mut prompt = digest_auth::parse(value).unwrap();
                                let context =
                                    AuthContext::new("vietnam", "L3xRay123!", digest_url.as_str());
                                prompt.respond(&context).unwrap().to_header_string()
                            }
                            _ => "".to_owned(),
                        }
                    },
                    Err(_) => "".to_owned()
                }
            }
            None => "".to_owned(),
        },
        Err(_) => {
            "".to_owned()
        }
    };
    let mut is_sent_error_code = false;

    let mut retry_count = 0;
    'main: while retry_count < 6 {
        // smol::Timer::after(std::time::Duration::from_secs(2u64.pow(retry_count)));
        let answer = answer.clone();
        let response = match answer.as_str() {
            "" => {
                println!("Basic");
                client
                    .get(cam_url)
                    // .timeout(time::Duration::from_secs(60))
                    // .basic_auth(username, Some(password))
                    // .header(reqwest::header::AUTHORIZATION, answer.clone())
                    .send()
                    .await
            }
            _ => {
                client
                    .get(digest_url.as_str())
                    // .basic_auth("vietnam", Some("L3xRay123!"))
                    .header(reqwest::header::AUTHORIZATION, answer)
                    .send()
                    .await
            }
        };
        if response.is_err() {
            smol::Timer::after(std::time::Duration::from_secs(2u64.pow(retry_count))).await;
            println!("Response error after {:?} retry: {:?}", retry_count, response.err());
            
            retry_count += 1;
            continue;
            // break;
        } 
        retry_count = 0;
        
        let response = response.unwrap();
        let status_code = response.status().as_u16();
        println!("[CAMERA] CAMERA STATUS {:?}", status_code);
        match (status_code, is_sent_error_code) {
            (200, false) => {println!("200, false");}
            (_, false) | (200, true) => {
                println!("Error code first time {:?}", status_code);
                is_sent_error_code = !is_sent_error_code;
            }
            (_, _) => {}
        };
        if status_code == 401 {
            // *is_frame_getting.lock().unwrap() = false;
            // retry_count += 1;
            break 'main;
        }

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            if chunk.is_err() {
                continue;
            }
            let chunk_fake = chunk.unwrap().clone();
            store_buffer.extend_from_slice(&chunk_fake);

            let mut i = 0;
            let length = store_buffer.len();
            // println!("chunk length: {}", chunk_fake.len());
            // println!(
            //     "Before traverse through store buffer: {}",
            //     store_buffer.len()
            // );
            // println!("START LOOP");
            while i < length {
                if i == length - 1 {
                    frame_buffer.push(store_buffer[i]);
                    store_buffer.clear();
                    break;
                }

                if store_buffer[i] == NONE && store_buffer[i + 1] == EOI && is_begin_frame == true {
                    frame_buffer.push(store_buffer[i]);
                    frame_buffer.push(EOI);
                    println!(
                        "[FRAME] END OF FRAME (from {}): {}",
                        cam_url,
                        frame_buffer.len()
                    );

                    //SAVING IMAGE
                    // let img_result =
                    //     image::load_from_memory_with_format(&frame_buffer, ImageFormat::Jpeg);
                    // let img = match img_result {
                    //     Ok(image) => image,
                    //     Err(_) => return Err(()),
                    // };
                    // img.save(format!("img-{}.jpg", count)).unwrap();
                    // let img16 = img.into_rgb8();
                    // let data = img16.into_raw() as Vec<u8>;
                    // println!("Image length: {}", data.len());
                    count += 1;

                    let _drain: Vec<_> = store_buffer.drain(..=i + 1).collect();
                    is_begin_frame = false;
                    frame_buffer.clear();

                    println!("store buffer: {}", store_buffer.len());
                    println!("frame buffer: {}", frame_buffer.len());
                    break;
                }

                if is_begin_frame == true {
                    frame_buffer.push(store_buffer[i]);
                }

                if store_buffer[i] == NONE && store_buffer[i + 1] == SOI {
                    println!("[FRAME] START OF FRAME (from {})", cam_url);

                    is_begin_frame = true;
                    frame_buffer.push(store_buffer[i]);
                    frame_buffer.push(SOI);
                    i += 1;
                }
                i += 1;
            }
            // println!("END LOOP");
            if store_buffer.len() == 18 {
                println!("Fuk im out!");
                store_buffer.clear();
                break;
            }
        }
        println!("End of request");
    }
    Ok(())
    // tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    // println!("End of sleep");
    // return Ok(());
}
