import Phaser from 'phaser'

export class SampleGame extends Phaser.Scene {
  preload() {
    this.load.setBaseURL('/')
    this.load.image('sky', 'space3.png')
    this.load.image('logo', 'phaser3-logo.png')
    this.load.image('red', 'red.png')
  }

  create() {
    this.add.image(400, 300, 'sky')

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
  }
}
