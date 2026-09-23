# NOHC: Biohazard Guatemala

Videojuego de acción y supervivencia en primera persona desarrollado en **Rust** con **raylib**. El jugador controla al agente **Nohc**, enviado a investigar una sede recién descubierta de Umbrella Corps en una mansión abandonada de Guatemala.

Lo que comienza como una investigación se convierte en una lucha contra armas biológicas, zombis, distintas variantes de Licker y dos perseguidores invencibles: Tyrant y Nemesis. Nohc deberá sobrevivir, encontrar un antivirus y escapar para entregarlo a un laboratorio del Gobierno de los Estados Unidos.

## Historia y niveles

### Nivel 1: Mansión

Nohc llega a una mansión abandonada llena de armas biológicas. Debe explorar sus dos pisos, conseguir llaves, abrir puertas, recoger recursos y combatir las distintas clases de enemigos.

Objetivo:

- Derrotar 20 zombis normales.
- Derrotar 15 zombis medios.
- Derrotar 5 zombis fuertes.
- Sobrevivir a Tyrant y Nemesis, quienes no pueden morir y persiguen al jugador.

### Nivel 2: Laboratorio

Durante la exploración de la mansión se descubre un laboratorio subterráneo. En sus instalaciones se encuentra un antivirus que podría servir para desarrollar una cura contra el brote de una enfermedad similar al virus T en Petén.

Objetivo:

- Explorar el laboratorio.
- Conseguir llaves para abrir las puertas bloqueadas.
- Sobrevivir a los enemigos y administrar los recursos.
- Encontrar el antivirus al final del laboratorio.

### Nivel final: Helipuerto

Nohc llega a un helipuerto rodeado por Tyrant, Nemesis, zombis y Lickers. Para escapar debe reparar una radio y solicitar una evacuación.

Objetivo:

- Derrotar al menos 12 enemigos.
- Derrotar por lo menos un Licker normal, uno medio y uno fuerte.
- Recoger las 3 piezas de radio.
- Llegar al punto de comunicación e interactuar para llamar al helicóptero.
- Sobrevivir a Tyrant y Nemesis.

Al completar la misión se muestra el epílogo y la pantalla final de éxito.

## Gameplay

El juego utiliza una cámara en primera persona con movimiento, rotación horizontal y apuntado vertical. La munición, la vida y el combustible son limitados, por lo que es importante explorar el mapa y recoger los objetos que dejan los enemigos.

Armas disponibles:

- **Pistola:** arma de distancia con cargador y munición de reserva. Permite realizar headshots contra zombis y Lickers.
- **Hacha:** arma cuerpo a cuerpo que también permite bloquear ataques.
- **Lanzallamas:** causa daño continuo a varios enemigos cercanos, pero consume combustible rápidamente.

Los enemigos pueden soltar:

- Munición de pistola.
- Combustible para el lanzallamas.
- Curación.
- Llaves, excepto en el nivel final.

El minimapa de la esquina muestra al jugador, su dirección y los objetos importantes del escenario. Cada nivel cuenta con una leyenda adaptada a sus elementos.

## Controles de teclado y mouse

| Acción | Control |
|---|---|
| Moverse | `W`, `A`, `S`, `D` |
| Girar y apuntar arriba/abajo | Mover el mouse |
| Apuntar con pistola/lanzallamas | Clic derecho |
| Bloquear con el hacha | Clic derecho |
| Disparar o atacar | Clic izquierdo |
| Equipar pistola | `1` |
| Equipar hacha | `2` |
| Equipar lanzallamas | `3` |
| Interactuar, abrir o recoger | `E` |
| Recargar la pistola | `R` |
| Liberar o capturar el cursor | `TAB` |
| Reiniciar la partida | `F5` |
| Activar pantalla completa | `F11` |
| Volver al menú | `BACKSPACE` |
| Aceptar o continuar pantallas | `ENTER` |

## Controles de gamepad

El juego detecta controles compatibles con raylib, incluyendo controles de Xbox y PlayStation conectados por USB o Bluetooth.

| Acción | Control |
|---|---|
| Moverse | Stick izquierdo |
| Girar y apuntar arriba/abajo | Stick derecho |
| Apuntar o bloquear | Gatillo izquierdo (`LT`/`L2`) |
| Disparar o atacar | Gatillo derecho (`RT`/`R2`) |
| Interactuar o aceptar | `A` en Xbox / `X` en PlayStation |
| Volver | `B` en Xbox / círculo en PlayStation |
| Recargar | `X` en Xbox / cuadrado en PlayStation |
| Equipar pistola | Cruceta arriba |
| Equipar hacha | Cruceta izquierda |
| Equipar lanzallamas | Cruceta derecha |

## Enemigos

- **Zombi normal:** enemigo básico.
- **Zombi medio:** posee más resistencia.
- **Zombi fuerte:** variante de mayor vida y peligro.
- **Licker normal, medio y fuerte:** criaturas rápidas con animaciones propias y sistema de headshots.
- **Tyrant:** perseguidor invencible que aparece durante la campaña.
- **Nemesis:** perseguidor invencible capaz de disparar misiles.

En el nivel final los Lickers permanecen en el suelo y no pueden trepar.

## Sistemas implementados

El proyecto incluye los siguientes elementos:

- Renderizado de escenario en primera persona mediante raycasting.
- Cámara con movimiento hacia delante y atrás.
- Rotación horizontal con teclado, mouse y stick derecho.
- Apuntado vertical con mouse y gamepad.
- Soporte completo para control.
- Minimapa ubicado en una esquina, con jugador, dirección, objetos y leyenda por nivel.
- Objetivo y estado de misión integrados en el HUD.
- Contador de FPS visible y objetivo de ejecución de 60 FPS.
- Música de fondo propia para mansión y laboratorio.
- Opción de música normal o **Bad Blood de Taylor Swift** durante la partida.
- Efectos de sonido para armas, enemigos, puertas, objetos, Nemesis y lanzallamas.
- Sprites animadas para enemigos, armas y perseguidores.
- Tres armas con comportamientos diferentes.
- Sistema de daño, curación, munición, recarga y combustible.
- Headshots para zombis y Lickers.
- Enemigos de diferentes niveles de resistencia.
- Sistema de drops con probabilidades configuradas por nivel.
- Tyrant y Nemesis invencibles con persecución continua.
- Misiles de Nemesis.
- Generación de nuevas hordas y refuerzos.
- Pantalla de bienvenida.
- Menú de selección entre múltiples niveles.
- Menú de opciones y controles.
- Pantallas de historia antes de cada nivel.
- Pantalla de éxito al completar una misión.
- Epílogo al finalizar el juego.
- Continuación entre niveles mediante `ENTER`.
- Modo de hordas después de completar una misión y continuar jugando.

## Criterios del proyecto cubiertos

| Criterio | Implementación |
|---|---|
| Soporte para control | Gamepad con movimiento, cámara, combate, interacción y menús |
| Estética del nivel | Mansión, laboratorio y helipuerto con texturas, HUD y sprites temáticos |
| FPS desplegados | Contador visible con límite configurado a 60 FPS |
| Cámara con movimiento y rotación | Movimiento libre, giro horizontal y apuntado vertical |
| Rotación con mouse | Cámara controlada horizontalmente y verticalmente con el mouse |
| Minimapa | Posición y dirección del jugador en una esquina |
| Música de fondo | Música diferente según el escenario |
| Música de Taylor Swift | Modo seleccionable con Bad Blood |
| Efectos de sonido | Disparos, golpes, enemigos, puertas, objetos y ambiente |
| Animación de sprites | Animaciones de movimiento, persecución, ataque y armas |
| Pantalla de bienvenida | Menú principal con presentación del juego |
| Selección de niveles | Mansión, laboratorio y nivel final |
| Pantalla de éxito | Pantalla `GREAT` después de completar cada objetivo |

## Ejecución

### Prueba de diorama 3D con ray tracing

El juego principal también tiene una prueba integrada de trazado de rayos en el primer piso de la mansión (`mapa_nivel1.txt`). Se activa automáticamente al jugar ese escenario; utiliza el mapa vivo para que las puertas abiertas sigan abiertas y conserva movimiento, combate, enemigos, objetos y HUD. `F3` alterna entre el render 3D nuevo y el clásico; `F` enciende o apaga la linterna. El segundo piso, laboratorio y escenario final siguen con el render original. El entorno se dibuja a 400×300 y se amplía a 800×600; las ventanas son huecos transparentes y el metal refleja. Los enemigos y objetos que aún son sprites 2D se superponen al entorno y pueden necesitar ajustes de oclusión.

En ese primer piso, los sprites de objetos y enemigos proyectan una sombra de contacto dibujada bajo su posición en el suelo. No son sombras trazadas por rayos y pueden superponerse ligeramente al borde de una pared cercana.


La prueba independiente construye una habitación 3D con cuatro paredes de madera, suelo y techo abierto. Una pared tiene un hueco con marco metálico y vidrio; también hay una puerta y cajas en el interior. Traza rayos por CPU: el vidrio refracta, el metal refleja y los rayos que no impactan muestrean el panorama nocturno.

```bash
cargo run --release --bin diorama
```

Arrastrar con el botón izquierdo rota la cámara. La rueda del mouse acerca y aleja; las flechas también rotan la vista. `C` alterna entre la cámara exterior y el interior con techo cerrado. Dentro, `F` enciende o apaga la linterna. `R` alterna el brillo de la madera para comparar el efecto de su rugosidad con un acabado mate. Para generar una imagen estática sin abrir ventana:

```bash
cargo run --release --bin diorama -- --snapshot
```

La imagen se guarda como `diorama-preview.png`. Es una prueba de renderizado a 320×180, no una conversión del juego completo: todavía no incorpora enemigos, combate ni interacción en la escena 3D.

Para generar una vista frontal de la ventana:

```bash
cargo run --release --bin diorama -- --snapshot-window
```

La imagen se guarda en `diorama-window-preview.png`.

El interior combina oscuridad ambiental, una lámpara cálida, luz fría cerca de la ventana y una linterna. Para comparar la iluminación desde la misma cámara:

```bash
cargo run --release --bin diorama -- --snapshot-interior
cargo run --release --bin diorama -- --snapshot-interior-dark
```

Estas vistas se guardan en `diorama-interior-preview.png` y `diorama-interior-dark-preview.png`.

Las paredes y la puerta de prueba usan `Wood035_2K-PNG`: color, normales OpenGL y rugosidad. Tienen distintos parámetros de brillo y albedo, pero comparten los mismos mapas; para contar como dos materiales distintos bajo una rúbrica que exige textura propia, necesitarán texturas diferentes. El suelo usa `Wood051_2K-PNG` con sus propios mapas; la textura se repite cada dos unidades del mundo para que las vetas se aprecien sin verse demasiado pequeñas.

Para generar una vista del suelo desde arriba:

```bash
cargo run --release --bin diorama -- --snapshot-floor
```

La imagen se guarda en `diorama-floor-preview.png`.

La misma vista con la madera mate se genera con `cargo run --release --bin diorama -- --snapshot-floor-matte` y se guarda en `diorama-floor-matte-preview.png`.

Para generar una vista cercana de la puerta:

```bash
cargo run --release --bin diorama -- --snapshot-door
```

Esta vista se guarda en `diorama-wood-preview.png`. El mapa de desplazamiento del paquete aún no se usa, porque requeriría alterar la geometría de los bloques.

El renderizador filtra las texturas con niveles de detalle y suaviza la imagen al ampliarla. Esto reduce el granulado y el parpadeo de las texturas 2K cuando el diorama se dibuja a 320×180.

Requisitos:

- Rust y Cargo instalados.
- Un sistema compatible con raylib.
- La carpeta `assets` completa, incluyendo imágenes y sonidos.

Desde la carpeta del proyecto, ejecutar:

- Para que vaya a 20 FPS

```bash
cargo run
```
- Para que vaya a 60 FPS

```bash
cargo run --release
```

Para verificar que el proyecto compila:

```bash
cargo test
```

## video en funcionamiento con control:

[video](https://youtube.com/shorts/4BkeAJ5_iaE)
