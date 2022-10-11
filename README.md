## revengine

Современный игровой движок, созданный студентами в образовательных целях. В ранней стадии разработки. В качестве демо предполагается полноценная игра, пока неясно, какая.

## features

- 🚀🚀🚀 Blazingly fast (используем [Rust](https://github.com/rust-lang/rust))
- [WGPU](https://github.com/gfx-rs/wgpu) в качестве графического бэкенда
- [ECS](https://www.gamedev.net/articles/programming/general-and-gameplay-programming/the-entity-component-system-c-game-design-pattern-part-1-r4803/) в качестве основного архитектурного паттерна
- Data Oriented Design, в теории

Далее идут вещи, которые должны быть в движке, но пока их либо нет, либо они в зачаточном состоянии.

### rendering

- PBR pipeline
- Texture mapping
- GPU instancing
- HDR
- Deferred rendering
- Dynamic shadows

### ecs

- Связанные архетипами компоненты хранятся в таблицах
- Системы, компоненты и сущности это структуры

### other

- 3d модели (планируется поддержка только gltf)
- Скелетные анимации
- Графический интерфейс через [egui](https://github.com/emilk/egui)
- Физика и коллизии
- Звук
- Обработка окна осуществляется через [winit](https://github.com/rust-windowing/winit)
- В качестве математики используется [glam](https://github.com/bitshifter/glam-rs)

## credits

Студенты четвёртого курса НГТУ факультетов ФПМИ и АВТФ:

- Бегичев Александр (ПМ-92)
- Кутузов Иван (ПМ-92)
- Жижин Владислав (АА-96)
