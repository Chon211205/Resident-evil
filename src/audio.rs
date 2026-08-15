use raylib::audio::{
    RaylibAudio,
    Sound,
};

pub struct AudioManager<'a> {
    shoot_sound: Sound<'a>,
    reload_sound: Sound<'a>,
}

impl<'a> AudioManager<'a> {
    pub fn new(
        audio: &'a RaylibAudio,
    ) -> Self {
        let shoot_sound =
            audio
                .new_sound(
                    "assets/sounds/shoot.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/shoot.mp3",
                );

        let reload_sound =
            audio
                .new_sound(
                    "assets/sounds/reload.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/reload.mp3",
                );

        Self {
            shoot_sound,
            reload_sound,
        }
    }

    pub fn disparo(&self) {
        self.shoot_sound.play();
    }

    pub fn recarga(&self) {
        self.reload_sound.play();
    }
}