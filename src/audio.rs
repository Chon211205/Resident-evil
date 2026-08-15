use raylib::audio::{
    RaylibAudio,
    Sound,
};

pub struct AudioManager<'a> {
    shoot_sound: Sound<'a>,
    reload_sound: Sound<'a>,
    door_sound: Sound<'a>,
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

        let door_sound =
            audio
                .new_sound(
                    "assets/sounds/door.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/door.mp3",
                );

        Self {
            shoot_sound,
            reload_sound,
            door_sound,
        }
    }

    pub fn disparo(&self) {
        self.shoot_sound.play();
    }

    pub fn recarga(&self) {
        self.reload_sound.play();
    }

    pub fn puerta(&self) {
        self.door_sound.play();
    }
}