import { Scene } from "phaser";

export default class Card {
    private scene: Scene;

    constructor(scene: Scene) {
        this.scene = scene;
    }

    public render(x: number, y: number, sprite: Phaser.Textures.Texture): Phaser.GameObjects.Image {
        let card = this.scene.add.image(x, y, sprite).setInteractive();
        this.scene.input.setDraggable(card);
        return card;
    }
}