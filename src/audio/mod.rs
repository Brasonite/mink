use std::{collections::HashMap, time::Duration};

use kira::{
    track::{TrackBuilder, TrackHandle},
    AudioManager, AudioManagerSettings, DefaultBackend, Easing, StartTime, Tween,
};
use pyo3::prelude::*;

use crate::{
    assets::{music::Music, sound::Sound},
    math::audio::linear_to_db,
};

pub mod api;

#[derive(FromPyObject)]
enum PlayableAudio<'a> {
    #[pyo3(transparent)]
    Music(Bound<'a, Music>),
    #[pyo3(transparent)]
    Sound(Bound<'a, Sound>),
}

#[pyclass]
pub struct Audio {
    pub volume: f32,
    pub manager: AudioManager<DefaultBackend>,
    pub tracks: HashMap<String, TrackHandle>,
}

impl Audio {
    pub fn new() -> Self {
        let mut manager = AudioManager::new(AudioManagerSettings::default())
            .expect("Failed to create audio manager");
        let tracks = HashMap::from_iter([(
            "master".to_string(),
            manager
                .add_sub_track(TrackBuilder::new())
                .expect("Failed to create master audio track"),
        )]);

        Self {
            volume: 1.0,
            manager,
            tracks,
        }
    }
}

#[pymethods]
impl Audio {
    #[setter]
    pub fn set_volume(&mut self, value: f32) {
        self.volume = value;
        self.tracks.get_mut("master").unwrap().set_volume(
            linear_to_db(value),
            Tween {
                start_time: StartTime::Immediate,
                duration: Duration::from_secs(0),
                easing: Easing::Linear,
            },
        );
    }

    fn play(&mut self, audio: PlayableAudio) {
        match audio {
            PlayableAudio::Music(music) => {
                let handle = self
                    .manager
                    .play(music.borrow().data.clone())
                    .expect("Failed to play music");

                music.borrow_mut().handle = Some(handle);
            }
            PlayableAudio::Sound(sound) => {
                self.manager
                    .play(sound.borrow().data.clone())
                    .expect("Failed to play sound");
            }
        }
    }
}
