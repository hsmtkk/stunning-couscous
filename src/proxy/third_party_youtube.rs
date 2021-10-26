pub trait ThirdPartyYoutubeLib {
    fn list_videos(&mut self) -> Vec<String>;
    fn get_video_info(&mut self, id:u32) -> String;
}

pub struct ThirdPartyYoutubeClass {}

impl ThirdPartyYoutubeClass {
    pub fn new() -> ThirdPartyYoutubeClass {
        ThirdPartyYoutubeClass{}
    }
}

impl ThirdPartyYoutubeLib for ThirdPartyYoutubeClass {
    fn list_videos(&mut self) -> Vec<String> {
        vec!["alpha".to_string(), "bravo".to_string()]
    }

    fn get_video_info(&mut self, _id:u32) -> String {
        "alpha".to_string()
    }
}
