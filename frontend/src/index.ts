import * as Phaser from 'phaser';
import Game from './scenes/Game';

const config = {
    type: Phaser.AUTO,
    backgroundColor: '#173845',
    width: 800,
    height: 600,
    scene: [Game]
};

const game = new Phaser.Game(config);
