mod device;
mod percent;
mod remote_control;

#[cfg(test)]
mod tests {
    use crate::bridge::remote_control::{RemoteControl, AdvancedRemoteControl};
    use crate::bridge::device::{TV, Radio};

    #[test]
    fn test(){
        let tv = TV::new();
        let mut remote = RemoteControl::new(Box::new(tv));
        remote.toggle_power();

        let radio = Radio::new();
        let mut remote = AdvancedRemoteControl::new(RemoteControl::new(Box::new(radio)));
        remote.mute();
    }
}