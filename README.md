# stela-nubo
Stela nubo is a game created with Bevy and Rust

## Goals
- [x] Spawn and despawn objects around the player deterministicly
- [x] Move around the space in a spaceship 
- [x] collide with the objects
- [x] Input System
- [x] Be able to shoot 
- [x] destroy objects
- [x] destruction effect
- [x] player damage
- [x] player death
- [x] Menu
- [x] App Quit
- [x] Game States: Start, Playing, End
- [x] Display Health
- [x] Display Points
- [x] Player model
- [x] Flying animations
- [x] Floating
- [x] Accelerate Forward
- [x] Turn Left
- [x] Turn Right
- [ ] Player sounds
- [ ] Pages
- [ ] Wind system
- [ ] Wind sounds
- [ ] Page sounds
- [ ] Wind Shaders
- [ ] Skybox
- [ ] Ground
- [ ] Trees
- [ ] Fauna spawning
- [ ] Fauna models
- [ ] Fauna animation
- [ ] Fauna sounds
- [ ] UI Sounds
- [ ] Game entrypoint design
- [ ] Game start animation sequence
- [ ] Game entrypoint sounds


## Game Design
A young witch was practicing spells, but a minor mistake happened and now she's chasing down pages of her journal.

Fly around on a broom stick to track down journal pages, and avoid obstacles. 

The game will start with pages in a book fluttering up, the camera will move to meet the player as she mounts her brooom
and launches into the air. 
Player will be launched from a high hilltop probably about 20 meters above the ground.
Pages will start spawned out as a stream ahead of the player, with the wind at their back.
The wind will change directions, and strength. The force applied will vary locally based on local noise.

Obstacles will include hitting the ground, birds, and trees. 
Trees and the ground will apply an uplift & repelling force on the pages to prevent them from falling or getting too low.

Terrain and trees should be randomly generated through noise.


### Asset requirements
- [x] Witch with a broom model
- [ ] Page model
- [ ] Journal model
- [ ] Landscape heightmap
- [ ] Tree map
- [ ] Fauna (Owls, Ravens, Bats, Bugs)
- [ ] Player sounds
- [ ] UI Sounds
- [ ] Bird noises
- [ ] Fluttering pages
- [ ] Wind noises