import * as Phaser from 'phaser';

export default class Game extends Phaser.Scene {
    private testText: Phaser.GameObjects.Text;

    constructor() {
        super({
            key: "Game"
        })
    }

    preload() {
        this.load.image("tui", "src/assets/tui.png");
        this.load.image("zhuang", "src/assets/zhuang.png");
    }

    create() {
        this.testText = this.add.text(75, 350, ["TEXT HELLO"]);

    }

    update() {

    }
}