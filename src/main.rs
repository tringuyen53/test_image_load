use digest_auth::AuthContext;
use futures_util::StreamExt;
use http::Uri;
use image;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat, RgbImage};
use reqwest::Url;
use core::time;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
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


    
    // let client = reqwest::Client::builder()
    // .timeout(time::Duration::from_secs(60))
    // .build().unwrap();

    let client = reqwest::Client::new();

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
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.64/mjpg/1/video.mjpg"; //digest
    // let cam_url = "http://climatecam.gi.alaska.edu/mjpg/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.31.178/mjpg/1/video.mjpg?resolution=640x480"; //digest
    let cam_url = "http://vietnam:L3xRay123!@10.50.29.117/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.56/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.31.39/mjpg/1/video.mjpg?resolution=640x480";
    // let cam_url = "http://10.50.29.36/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.13.89/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://10.50.29.36:80/mjpgstreamreq/1/image.jpg?resolution=640x480";
    // let cam_url = "http://10.50.31.241/mjpg/1/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay123@10.50.30.212/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.13.89/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay12@10.50.31.179/mjpg/1/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay12@10.50.31.179:80/mjpg/1/video.mjpg";
    // let cam_url = "http://vietnam:L3xRay123@10.50.12.187:80/video/mjpeg/stream2";
    // let cam_url = "http://10.50.13.23/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.30.108/mjpgstreamreq/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.36/jpgimage/1/image.jpg";
    // let cam_url = "http://vietnam:L3xRay123!@10.50.29.22/mjpgstreamreq/1/image.jpg";
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
    let mut count: i32 = 1;

    let mut answer = String::new();
    let basic_url = Url::parse(cam_url).unwrap();
    let username = basic_url.username();
    let password = basic_url.password().unwrap_or_default();

    let mut digest_url = basic_url.clone();
    digest_url.set_username("");
    digest_url.set_password(None);
    // digest_url.set_port(Some(80));

    println!("CAM URL: {}", basic_url);
    println!("CAM DIGEST URL: {}", digest_url);
    let resp = surf::get(digest_url.as_str()).await;
    if resp.is_err() {
        println!("Requeset header error {:?}",resp.err());
        return Ok(());
    }
    let resp = resp.unwrap();
    // println!("Port: {:?}", basic_url.port());
    println!("usr: {} - pwd: {}", username, password);
    // println!("[CAMERA] CAMERA STATUS {:?}", resp.status());
    if let Some(value) = resp.header("www-authenticate") {
        match value.to_string().split_once(' ').unwrap_or_default().0 {
            "Digest" => {
                println!("Digest Camera.");
                let mut prompt = digest_auth::parse(&value.to_string()).unwrap();
                let context = AuthContext::new(username, password, digest_url.path());
                answer = prompt.respond(&context).unwrap().to_header_string();
                println!("Answer: {:?}", answer);
            }
            _ => {}
        }
    }

    let mut retry_count = 0;
    'main: while retry_count < 6 {
        smol::Timer::after(std::time::Duration::from_secs(2u64.pow(retry_count)));
        let response = match answer.as_str() {
            "" => {
                println!("Basic");
                client
                    .get(cam_url)
                    // .basic_auth(username, Some(password))
                    // .header(reqwest::header::AUTHORIZATION, answer.clone())
                    .send()
                    .await
            }
            _ => {
                println!("Digest");
                client
                    .get(digest_url.as_str())
                    // .basic_auth("vietnam", Some("L3xRay123!"))
                    .header(reqwest::header::AUTHORIZATION, answer.clone())
                    .send()
                    .await
            }
        };
        if response.is_err() {
            println!("Response error");

            retry_count += 1;
            // continue;
            break;
        } else {
            retry_count = 0;
        }
        let response = response.unwrap();
        let status_code = response.status().as_u16();
        println!("[CAMERA] CAMERA STATUS {:?}", status_code);
        if status_code == 401 {
            println!("Unauthorized");
            // *is_frame_getting.lock().unwrap() = false;
            // retry_count += 1;
            break 'main;
        }

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
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
