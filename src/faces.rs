use rocket::data::{Data, ToByteUnit};
use reqwest;

const IMAGE_SIZE_LIMIT: usize = 2 << 20; // 2 megabytes

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Faces(pub Vec<Face>);

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    pub face_id: String,
    pub face_rectangle: FaceRectangle,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceRectangle {
    pub top: usize,
    pub left: usize,
    pub width: usize,
    pub height: usize
}

impl Faces {
    pub async fn new(key: &str, endpoint: &str, image: Data) -> Faces {
        let endpoint = String::from(endpoint) + "face/v1.0/detect";
        let params = [
            ("overload", "stream"),
            ("returnFaceAttributes", "emotion")
        ];
        
        let image_stream = image.open(IMAGE_SIZE_LIMIT.bytes()).stream_to_vec().await.unwrap();
        let client = reqwest::Client::new();
        let res = client.post(&endpoint)
            .header("Content-Type", "application/octet-stream")
            .header("Ocp-Apim-Subscription-Key", key)
            .query(&params)
            .body(image_stream)
            .send()
            .await.unwrap();
        
        let out: Vec<Face> = res.json().await.unwrap();
        //println!("{}", res.text().await.unwrap());
        //Faces(vec![])
        Faces(out)
    }
}

impl Face {
    pub fn engagement_score(&self) -> usize {
        1
    }
}

impl FaceRectangle {
    pub fn get_area(&self) -> u32 {
        (self.height * self.width) as u32
    }
}