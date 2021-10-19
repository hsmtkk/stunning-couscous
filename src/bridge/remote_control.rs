use super::device::Device;
use super::percent::Percent;

pub struct RemoteControl
{
    device: Box<dyn Device>,
}

impl RemoteControl {
    #[allow(dead_code)]
    pub fn new(device: Box<dyn Device>) -> RemoteControl {
        RemoteControl{device}
    }

    #[allow(dead_code)]
    pub fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
        } else {
            self.device.enable();
        }
    }

    #[allow(dead_code)]
    pub fn volue_down(&mut self){
        let new_vol: Percent = self.device.get_volume() - Percent::new(10).unwrap();
        self.device.set_volume(new_vol);
    }

    #[allow(dead_code)]
    pub fn volume_up(&mut self){
        let new_vol: Percent = self.device.get_volume() + Percent::new(10).unwrap();
        self.device.set_volume(new_vol);
    }
}

#[allow(dead_code)]
pub struct AdvancedRemoteControl{
    remote_control: RemoteControl,
}

impl AdvancedRemoteControl {
    #[allow(dead_code)]
    pub fn new(remote_control: RemoteControl) -> AdvancedRemoteControl {
        AdvancedRemoteControl{remote_control}
    }

    #[allow(dead_code)]
    pub fn mute(&mut self){
        for _ in 0..10 {
            self.remote_control.volue_down();
        }
    }
}