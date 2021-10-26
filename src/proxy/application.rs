#[cfg(test)]
mod tests {
    use crate::proxy::third_party_youtube::ThirdPartyYoutubeClass;
    use crate::proxy::cached_youtube::CachedYoutubeClass;
    use crate::proxy::youtube_manager::YoutubeManager;
    #[test]
    fn application(){
        let service = ThirdPartyYoutubeClass::new();
        let proxy = CachedYoutubeClass::new(Box::new(service));
        let mut manager = YoutubeManager::new(Box::new(proxy));
        manager.react_on_user_input();
    }
}