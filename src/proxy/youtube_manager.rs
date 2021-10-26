use super::third_party_youtube::ThirdPartyYoutubeLib;

pub struct YoutubeManager {
    service:Box<ThirdPartyYoutubeLib>,
}

impl YoutubeManager  {
    pub fn new(service: Box<ThirdPartyYoutubeLib>) -> YoutubeManager {
        YoutubeManager{service}
    }

    fn render_video_page(&mut self, id:u32) {
        self.service.get_video_info(id);
    }

    fn render_list_panel(&mut self) {
        self.service.list_videos();
    }

    pub fn react_on_user_input(&mut self) {
        self.render_video_page(0);
        self.render_list_panel();
    }
}