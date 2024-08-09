use kira::{manager::{AudioManager,AudioManagerSettings,backend::DefaultBackend},
sound::static_sound::{StaticSoundData,StaticSoundSettings}};

pub struct Sound {
    manager: AudioManager,
    sound_data: StaticSoundData,
}

impl Sound {
    pub fn new() -> Self {
        Sound{
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
            sound_data: StaticSoundData::from_file("camera.ogg").unwrap(),
        }
    }

    pub fn play(&mut self) {
        self.manager.play(self.sound_data.clone()).unwrap();
    }
}