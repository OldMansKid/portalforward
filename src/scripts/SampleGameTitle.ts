import Phaser from 'phaser'
import { BaseScene, createRexButton } from './SampleGameTools';

export class SampleGameTitle extends BaseScene {
  constructor() {
    super('SampleGameTitle')
  }
  preload() {
    this.load.setBaseURL('/')
    this.load.image('space', 'space3.png')
    this.load.image('logo', 'phaser3-logo.png')
    this.load.image('red', 'red.png')
  }

  create() {
    this.add.image(400, 300, 'space')

    const particles = this.add.particles(0, 0, 'red', {
      speed: 100,
      scale: { start: 1, end: 0 },
      blendMode: 'ADD',
    })

    const logo = this.physics.add.image(400, 100, 'logo')

    logo.setVelocity(100, 200)
    logo.setBounce(1, 1)
    logo.setCollideWorldBounds(true)

    particles.startFollow(logo)
    const buttonConfig = this.rexUI!.add.buttons({
      x: 400,
      y: 500,
      width: 300,

      orientation: 'x',

      buttons: [
        createRexButton(this, 'Start Game'),
      ]
    });
    const buttons = buttonConfig.layout().drawBounds(this.add.graphics(), 0xff0000);
    buttons.on("button.click", (button: Phaser.GameObjects.GameObject) => {
      if (button.name === 'Start Game') {
        this.scene.start('SampleGameLevel1')
      }
    });
  }
}
