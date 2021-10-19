use super::percent::Percent;

pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn get_volume(&self) -> Percent;
    fn set_volume(&mut self, volume:Percent);
}

pub struct TV {
    enabled: bool,
    volume: Percent,
}

impl TV {
    #[allow(dead_code)]
    pub fn new() -> TV {
        TV{enabled:false, volume:Percent::new(0).unwrap()}
    }
}

impl Device for TV {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self){
        self.enabled = true;
    }

    fn disable(&mut self){
        self.enabled = false;
    }

    fn get_volume(&self) -> Percent {
        self.volume.clone()
    }

    fn set_volume(&mut self, volume:Percent){
        self.volume = volume;
    }
}

pub struct Radio {
    enabled: bool,
    volume: Percent,
}

impl Radio {
    #[allow(dead_code)]
    pub fn new() -> Radio {
        Radio{enabled:false, volume:Percent::new(0).unwrap()}
    }
}

impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self){
        self.enabled = true;
    }

    fn disable(&mut self){
        self.enabled = false;
    }

    fn get_volume(&self) -> Percent {
        self.volume.clone()
    }

    fn set_volume(&mut self, volume:Percent){
        self.volume = volume;
    }
}