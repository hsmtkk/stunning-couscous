#[allow(dead_code)]

use super::device::Device;
use super::percent::Percent;

pub struct RemoteControl
{
    device: Box<dyn Device>,
}

impl RemoteControl {
    pub fn new(device: Box<dyn Device>) -> RemoteControl {
        RemoteControl{device}
    }

    pub fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
        } else {
            self.device.enable();
        }
    }

    pub fn volue_down(&mut self){
        let new_vol: Percent = self.device.get_volume() - Percent::new(10).unwrap();
        self.device.set_volume(new_vol);
    }

    pub fn volume_up(&mut self){
        let new_vol: Percent = self.device.get_volume() + Percent::new(10).unwrap();
        self.device.set_volume(new_vol);
    }
}

pub struct AdvancedRemoteControl{
    remote_control: RemoteControl,
}

impl AdvancedRemoteControl {
    pub fn new(remote_control: RemoteControl) -> AdvancedRemoteControl {
        AdvancedRemoteControl{remote_control}
    }
    pub fn mute(&mut self){
        for _ in 0..10 {
            self.remote_control.volue_down();
        }
    }
}