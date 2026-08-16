use raylib::audio::{
    RaylibAudio,
    Sound,
};

pub struct AudioManager<'a> {
    shoot_sound: Sound<'a>,
    reload_sound: Sound<'a>,
    door_sound: Sound<'a>,
    noammo_sound: Sound<'a>,
    zombie_sound: Sound<'a>,
    damage_sound: Sound<'a>,
    key_sound: Sound<'a>,
    zombiedie_sound: Sound<'a>,
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

        let noammo_sound =
            audio
                .new_sound(
                    "assets/sounds/noammo.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/noammo.mp3",
                );

        let zombie_sound =
            audio
                .new_sound(
                    "assets/sounds/zombie.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/zombie.mp3",
                );

        let damage_sound =
            audio
                .new_sound(
                    "assets/sounds/damage.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/damage.mp3",
                );

        let key_sound =
            audio
                .new_sound(
                    "assets/sounds/key.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/key.mp3",
                );

        let zombiedie_sound =
            audio
                .new_sound(
                    "assets/sounds/zombiedie.mp3",
                )
                .expect(
                    "No se pudo cargar assets/sounds/zombiedie.mp3",
                );

        Self {
            shoot_sound,
            reload_sound,
            door_sound,
            noammo_sound,
            zombie_sound,
            damage_sound,
            key_sound,
            zombiedie_sound,
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

    pub fn sin_municion(&self) {
        self.noammo_sound.play();
    }

    pub fn zombie(&self) {
        self.zombie_sound.play();
    }

    pub fn detener_zombie(&self) {
        self.zombie_sound.stop();
    }

    pub fn dano(&self) {
        self.damage_sound.play();
    }

    pub fn llave(&self) {
        self.key_sound.play();
    }

    pub fn zombie_muere(&self) {
        self.zombiedie_sound.play();
    }
}