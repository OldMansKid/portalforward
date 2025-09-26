import { BaseScene } from "./SampleGameTools";

export class SampleGameLevel1 extends BaseScene {
  platforms: Phaser.Physics.Arcade.StaticGroup | null = null;
  sky: string = "sky";
  ground: string = "ground";
  star: string = "star";
  bomb: string = "bomb";
  dude: string = "dude";
  constructor() {
    super("SampleGameLevel1");
  }
  preload() {
    this.load.image(this.sky, "sky.png");
    this.load.image(this.ground, "platform.png");
    this.load.image(this.star, "star.png");
    this.load.image(this.bomb, "bomb.png");
    this.load.spritesheet(this.dude, "dude.png", {frameWidth: 32, frameHeight: 48});
  }

  create() {
    const centerX = this.cameras.main.width / 2;
    const centerY = this.cameras.main.height / 2;
    this.add.image(centerX, centerY, this.sky);
    this.platforms = this.physics.add.staticGroup();
    const ground = this.textures.get(this.ground);
    const groundHeight = ground.getSourceImage().height;
    const groundWidth = ground.getSourceImage().width;
    this.platforms.create(centerX, this.cameras.main.height - groundHeight, this.ground).setScale(2).refreshBody();
    this.platforms.create(centerX + groundWidth / 2, this.cameras.main.height - groundHeight - 150, this.ground);
    this.platforms.create(50, this.cameras.main.height - groundHeight - 300, this.ground);
    this.platforms.create(750, this.cameras.main.height - groundHeight - 380, this.ground);
  }
}
