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
    zombie2_sound: Sound<'a>,
    zombie3_sound: Sound<'a>,

    damage_sound: Sound<'a>,
    key_sound: Sound<'a>,
    zombiedie_sound: Sound<'a>,
    collectammo_sound: Sound<'a>,

    axe_sound: Sound<'a>,
    axe_block_sound: Sound<'a>,
    heal_sound: Sound<'a>,
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
                    "No se pudo cargar shoot.mp3",
                );

        let reload_sound =
            audio
                .new_sound(
                    "assets/sounds/reload.mp3",
                )
                .expect(
                    "No se pudo cargar reload.mp3",
                );

        let door_sound =
            audio
                .new_sound(
                    "assets/sounds/door.mp3",
                )
                .expect(
                    "No se pudo cargar door.mp3",
                );

        let noammo_sound =
            audio
                .new_sound(
                    "assets/sounds/noammo.mp3",
                )
                .expect(
                    "No se pudo cargar noammo.mp3",
                );

        let zombie_sound =
            audio
                .new_sound(
                    "assets/sounds/zombie.mp3",
                )
                .expect(
                    "No se pudo cargar zombie.mp3",
                );

        let zombie2_sound =
            audio
                .new_sound(
                    "assets/sounds/zombie2.mp3",
                )
                .expect(
                    "No se pudo cargar zombie2.mp3",
                );

        let zombie3_sound =
            audio
                .new_sound(
                    "assets/sounds/zombie3.mp3",
                )
                .expect(
                    "No se pudo cargar zombie3.mp3",
                );

        let damage_sound =
            audio
                .new_sound(
                    "assets/sounds/damage.mp3",
                )
                .expect(
                    "No se pudo cargar damage.mp3",
                );

        let key_sound =
            audio
                .new_sound(
                    "assets/sounds/key.mp3",
                )
                .expect(
                    "No se pudo cargar key.mp3",
                );

        let zombiedie_sound =
            audio
                .new_sound(
                    "assets/sounds/zombiedie.mp3",
                )
                .expect(
                    "No se pudo cargar zombiedie.mp3",
                );

        let collectammo_sound =
            audio
                .new_sound(
                    "assets/sounds/collectammo.mp3",
                )
                .expect(
                    "No se pudo cargar collectammo.mp3",
                );

        let axe_sound =
            audio
                .new_sound(
                    "assets/sounds/axe.mp3",
                )
                .expect(
                    "No se pudo cargar axe.mp3",
                );

        let axe_block_sound =
            audio
                .new_sound(
                    "assets/sounds/axeblock.mp3",
                )
                .expect(
                    "No se pudo cargar axeblock.mp3",
                );

        let heal_sound =
            audio
                .new_sound(
                    "assets/sounds/heal.mp3",
                )
                .expect(
                    "No se pudo cargar heal.mp3",
                );

        Self {
            shoot_sound,
            reload_sound,
            door_sound,
            noammo_sound,

            zombie_sound,
            zombie2_sound,
            zombie3_sound,

            damage_sound,
            key_sound,
            zombiedie_sound,
            collectammo_sound,

            axe_sound,
            axe_block_sound,
            heal_sound,
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

    pub fn zombie_medio(&self) {
        self.zombie2_sound.play();
    }

    pub fn detener_zombie_medio(
        &self,
    ) {
        self.zombie2_sound.stop();
    }

    pub fn zombie_fuerte(&self) {
        self.zombie3_sound.play();
    }

    pub fn detener_zombie_fuerte(
        &self,
    ) {
        self.zombie3_sound.stop();
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

    pub fn recoger_municion(
        &self,
    ) {
        self.collectammo_sound.play();
    }

    pub fn hachazo(&self) {
        self.axe_sound.play();
    }

    pub fn bloqueo_hacha(&self) {
        self.axe_block_sound.play();
    }

    pub fn curacion(&self) {
        self.heal_sound.play();
    }
}