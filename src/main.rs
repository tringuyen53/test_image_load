use digest_auth::AuthContext;
use futures_util::StreamExt;
use http::Uri;
use image;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat, RgbImage};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // let resp = reqwest::get("http://vietnam:L3xRay123!@10.50.29.56/jpgimage/1/image.jpg")
    //     .await?;
    // println!("{:#?}", resp);

    // let img_bytes = reqwest::get("http://vietnam:L3xRay123!@10.50.29.56/jpgimage/1/image.jpg").await?
    // let img_bytes = reqwest::get("http://192.168.1.11:88/cgi-bin/CGIStream.cgi?cmd=GetMJStream&usr=test&pwd=test123").await?
    // .bytes().await?;

    // let mut file = File::create("foo.txt").unwrap();
    // let mut img_bytes = reqwest::get("http://climatecam.gi.alaska.edu/mjpg/video.mjpg").await?;
    // while let Some(chunk) = img_bytes.chunk().await? {
    //     println!("Chunk: {:?}", chunk);
    //     file.write_all(&chunk).unwrap();
    //     // let image = image::load_from_memory(&chunk)?;
    //     // image.save("output.jpeg")?;
    // }

    // let mut file = File::create("foo.txt").unwrap();
    // let mut file_frame = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .open("frame.txt")
    //     .unwrap();

    let client = reqwest::Client::new();

    const SOI: u8 = 0xD8;
    ///     End of image
    const EOI: u8 = 0xD9;

    const NONE: u8 = 0xFF;

    let mut store_buffer: Vec<u8> = Vec::new();
    let mut is_begin_frame = false;
    let mut frame_buffer: Vec<u8> = Vec::new();
    // let cam_url = "http://10.50.12.180/mjpgstreamreq/1/image.jpg?resolution=640x480";
    let cam_url = "http://10.50.29.64/axis-cgi/mjpg/video.cgi";
    // let cam_url = "http://10.50.29.64/mjpg/video.mjpg?user=vietnam&pw=L3xRay123!";
    // let cam_url = "http://climatecam.gi.alaska.edu/mjpg/video.mjpg";
    // let cam_url = "http://10.50.31.178/mjpg/1/video.mjpg?resolution=640x480";
    //unauth
    // let cam_url = "http://10.50.13.23/mjpgstreamreq/1/image.jpg";
    let mut count = 1;

    let res = client.get(cam_url).send().await.unwrap();
    let headers = res.headers();
    let mut answer = "".to_string();
    if headers.contains_key("www-authenticate".to_string()) {

        let wwwauth = headers["www-authenticate"].to_str().unwrap_or("");
        let uri: Uri = cam_url.parse().unwrap();
        println!("Uri: {:?}", uri.path());
        let context = AuthContext::new("vietnam", "L3xRay123!", uri.path());
        let mut prompt = digest_auth::parse(wwwauth).unwrap();
        answer = prompt.respond(&context).unwrap().to_header_string();
    }

    let response = client
        .get(cam_url)
        // .basic_auth("vietnam", Some("L3xRay123!"))
        .header(reqwest::header::AUTHORIZATION, answer)
        .send()
        .await
        .unwrap();

    println!("[CAMERA] CAMERA STATUS {:?}", response.status());

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk_fake = chunk.unwrap().clone();
        store_buffer.extend_from_slice(&chunk_fake);

        let mut i = 0;
        let length = store_buffer.len();
        println!("chunk length: {}", chunk_fake.len());
        println!(
            "Before traverse through store buffer: {}",
            store_buffer.len()
        );
        println!("START LOOP");
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

                let img_result =
                    image::load_from_memory_with_format(&frame_buffer, ImageFormat::Jpeg);
                let img = match img_result {
                    Ok(image) => image,
                    Err(_) => return Err(()),
                };
                img.save(format!("img-{}.jpg", count)).unwrap();
                let img16 = img.into_rgb8();
                let data = img16.into_raw() as Vec<u8>;
                println!("Image length: {}", data.len());
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
        println!("END LOOP");
    }
    println!("CAMERA IZ DAED");
    return Ok(());
}
