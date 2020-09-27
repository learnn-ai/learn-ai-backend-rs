use rocket::data::{Data, ToByteUnit};
use reqwest;

// Image file size limit set at ~4 MB,
// just below Azure Face API limit
const IMAGE_SIZE_LIMIT: usize = 4000000;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Faces(pub Vec<Face>);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    pub face_id: String,
    pub face_rectangle: FaceRectangle,
    pub face_landmarks: FaceLandmarks,
    pub face_attributes: FaceAttributes,
    pub recognition_model: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceRectangle {
    pub top: usize,
    pub left: usize,
    pub width: usize,
    pub height: usize
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceLandmarks {
    pub pupil_left: Coordinate,
    pub pupil_right: Coordinate,
    pub nose_tip: Coordinate,
    pub mouth_left: Coordinate,
    pub mouth_right: Coordinate,
    pub eyebrow_left_outer: Coordinate,
    pub eyebrow_left_inner: Coordinate,
    pub eye_left_outer: Coordinate,
    pub eye_left_top: Coordinate,
    pub eye_left_bottom: Coordinate,
    pub eye_left_inner: Coordinate,
    pub eyebrow_right_inner: Coordinate,
    pub eyebrow_right_outer: Coordinate,
    pub eye_right_inner: Coordinate,
    pub eye_right_top: Coordinate,
    pub eye_right_bottom: Coordinate,
    pub eye_right_outer: Coordinate,
    pub nose_root_left: Coordinate,
    pub nose_root_right: Coordinate,
    pub nose_left_alar_top: Coordinate,
    pub nose_right_alar_top: Coordinate,
    pub nose_left_alar_out_tip: Coordinate,
    pub nose_right_alar_out_tip: Coordinate,
    pub upper_lip_top: Coordinate,
    pub upper_lip_bottom: Coordinate,
    pub under_lip_top: Coordinate,
    pub under_lip_bottom: Coordinate
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceAttributes {
    pub smile: f32,
    pub head_pose: HeadPose,
    pub gender: String,
    pub age: f32,
    pub facial_hair: FacialHair,
    pub glasses: String,
    pub emotion: Emotion,
    pub blur: Blur,
    pub exposure: Exposure,
    pub noise: Noise,
    pub makeup: Makeup,
    pub accessories: Vec<Accessory>,
    pub occlusion: Occlusion,
    pub hair: Hair
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadPose {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacialHair {
    pub moustache: f32,
    pub beard: f32,
    pub sideburns: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emotion {
    pub anger: f32,
    pub contempt: f32,
    pub disgust: f32,
    pub fear: f32,
    pub happiness: f32,
    pub neutral: f32,
    pub sadness: f32,
    pub surprise: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blur {
    pub blur_level: String,
    pub value: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exposure {
    pub exposure_level: String,
    pub value: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Noise {
    pub noise_level: String,
    pub value: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Makeup {
    pub eye_makeup: bool,
    pub lip_makeup: bool
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessory {
    #[serde(rename = "type")]
    pub kind: String,
    pub confidence: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Occlusion {
    pub forehead_occluded: bool,
    pub eye_occluded: bool,
    pub mouth_occluded: bool
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hair {
    pub bald: f32,
    pub invisible: bool,
    pub hair_color: Vec<HairColor>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HairColor {
    pub color: String,
    pub confidence: f32
}

impl Faces {
    // Request and parse Azure Face API face detections
    pub async fn new(key: &str, endpoint: &str, image: Data) -> Faces {
        let endpoint = String::from(endpoint) + "face/v1.0/detect";
        let params = [
            ("overload", "stream"),
            ("detectionModel", "detection_01"),
            ("recognitionModel", "recognition_01"),
            ("returnFaceAttributes", "age,gender,headPose,smile,facialHair,glasses,emotion,hair,makeup,occlusion,accessories,blur,exposure,noise"),
            ("returnFaceId", "true"),
            ("returnFaceLandmarks", "true"),
            ("returnRecognitionModel", "true")
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
        
        //println!("{}", res.text().await.unwrap());
        //Faces(vec![])
        let faces: Vec<Face> = res.json().await.unwrap();
        Faces(faces)
    }
}

impl Face {
    // Determine engagement by various face attributes
    pub fn engagement_score(&self) -> usize {
        // smile: higher the value, the more likely you're engaged
        // headpose: if you are not looking at the screen, you are distracted
        // emotion: if neutral < 0.2, you are probably engaged

        // roll is clockwise rotation around z 
        // yaw is counter clockwise rotation around y -15 pitch look down, -10 look up at risk
        // pitch is counter clockwise rotation around x from right view -6.6 yaw left, 11 yaw right are at risk

        let head_yaw: (f32, f32, f32) = (self.face_attributes.head_pose.yaw, -13.3, 11.0);
        let head_pitch: (f32, f32, f32) = (self.face_attributes.head_pose.pitch, -15.0, 0.3);
        let yaw_divisor = head_yaw.1.abs()+head_yaw.2;
        let pitch_divisor = head_pitch.1.abs()+head_pitch.2;
        let yaw_zero_offset = (head_yaw.0-(head_yaw.2+head_yaw.1)/2.0).abs();
        let pitch_zero_offset = (head_pitch.0-(head_pitch.2+head_pitch.1)/2.0).abs();
        let yaw_norm = 1.0-(yaw_zero_offset/yaw_divisor).min(1.0);
        let pitch_norm =  1.0-(pitch_zero_offset/pitch_divisor).min(1.0);
        let smile = self.face_attributes.smile;
        let emotion = 1.0-((self.face_attributes.emotion.neutral*140.0+1.0).log(1.05)/100.0).min(1.0); // weight for < 0.2
        
        //println!("{} {} {} {} : {} {}", yaw_norm, pitch_norm, smile, emotion, head_yaw.0, head_pitch.0);
        (yaw_norm*123.0 + pitch_norm*232.0 + smile*273.0 + emotion*52.0) as usize
    }
}

impl FaceRectangle {
    // Calculate area of bounding box
    pub fn get_area(&self) -> u32 {
        (self.height * self.width) as u32
    }
}