import type { Label } from "phaser3-rex-plugins/templates/ui/ui-components";
import UIPlugin from "phaser3-rex-plugins/templates/ui/ui-plugin";

export class BaseScene extends Phaser.Scene {
  rexUI: UIPlugin | null;
  constructor(key: string) {
    super(key);
    this.rexUI = null;
  }
}

export function createGameConfig(scenes: Phaser.Types.Scenes.SceneType | Phaser.Types.Scenes.SceneType[], parent: string): Phaser.Types.Core.GameConfig {
  return {
    type: Phaser.AUTO,
    width: 800,
    height: 600,
    scene: scenes,
    parent: parent,
    physics: {
      default: 'arcade',
      arcade: {
        gravity: { x: 0, y: 200 },
        debug: false,
      },
    },
    plugins: {
      scene: [
        {
          key: 'rexUI',
          plugin: UIPlugin,
          mapping: 'rexUI',
        },
      ],
    }
  }
}

export function createRexButton(scene: BaseScene, text: string): Label {
  return scene.rexUI!.add.label({
    width: 40, height: 30, background: scene.rexUI!.add.roundRectangle(0, 0, 0, 0, 20, 0x5e92f3),
    text: scene.add.text(0, 0, text, { fontSize: '20px' }),
    space: { left: 10, right: 10, top: 10, bottom: 10 },
    align: "center",
    name: text,
  })
}
