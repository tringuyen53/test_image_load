use digest_auth::AuthContext;
use reqwest::Url;

const SOI: u8 = 0xD8;
///     End of image
const EOI: u8 = 0xD9;

const NONE: u8 = 0xFF;

pub async fn test() -> Result<(), ()> {
    let client = reqwest::Client::new();

    let mut store_buffer: Vec<u8> = Vec::new();
    let mut is_begin_frame = false;
    let mut frame_buffer: Vec<u8> = Vec::new();

    // let basic_url = "http://10.50.13.89/mjpgstreamreq/1/image.jpg"; //basic
    // let digest_url = "http://vietnam:L3xRay123!@10.50.29.64/mjpg/1/video.mjpg"; //digest
    let url = "http://10.50.13.89/mjpgstreamreq/1/image.jpg";

    let mut count: i32 = 1;

    let mut answer = String::new();

    let basic_url = Url::parse(url).unwrap();
    let username = basic_url.username();
    let password = basic_url.password().unwrap_or_default();

    let mut digest_url = basic_url.clone();
    digest_url.set_username("");
    digest_url.set_password(None);

    let resp = client.head(url).send().await.unwrap();
    let header = resp.headers();

    if let Some(value) = header.get("www-authenticate") {
        if let Ok(value) = value.to_str() {
            match value.split_once(' ').unwrap_or_default().0 {
                "Digest" => {
                    let mut prompt = digest_auth::parse(value).unwrap();
                    let context = AuthContext::new(username, password, digest_url.as_str());
                    answer = prompt.respond(&context).unwrap().to_header_string();
                }
                "Basic" => {}
                _ => {}
            }
        }
    }

    Ok(())
}
