use super::third_party_youtube::ThirdPartyYoutubeLib;

pub struct CachedYoutubeClass {
    service: Box<dyn ThirdPartyYoutubeLib>,
    list_cache: Vec<String>,
    video_cache: String,
    need_reset: bool,
}

impl CachedYoutubeClass {
    pub fn new(service:Box<dyn ThirdPartyYoutubeLib>) -> CachedYoutubeClass {
        CachedYoutubeClass{service, list_cache:vec![], video_cache:"".to_string(), need_reset:false}
    }
}

impl ThirdPartyYoutubeLib for CachedYoutubeClass {
    fn list_videos(&mut self) -> Vec<String>{
        if self.list_cache.is_empty() || self.need_reset {
            self.list_cache = self.service.list_videos();
        }
        self.list_cache.clone()
    }

    fn get_video_info(&mut self, id:u32) -> String {
        if self.video_cache.is_empty() || self.need_reset {
            self.video_cache = self.service.get_video_info(id);
        }
        self.video_cache.clone()
    }
}