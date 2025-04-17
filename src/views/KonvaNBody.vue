<template>
  <div class="controls" id="NBody">
    <label for="bodyCount">天体数量: {{ bdnum }}</label>
    <input type="range" id="bodyCount" min="10" max="50" value="10" @input="bdnumChange" />

    <label for="collisionDistance">相遇距离: {{ copycdnum }} px</label>
    <input type="range" id="collisionDistance" min="5" max="200" value="5" @input="cdnumChange" />

    <div id="space"></div>
  </div>
</template>

<script setup lang="ts">
import { is, matchSync } from "kaia-fp";
import Konva from "konva";
import { onBeforeUnmount, onMounted, ref } from "vue";

// 定义响应式变量
const bdnum = ref(10);
const cdnum = ref(5);
const copycdnum = ref(5);
const bodies = ref<NBody[]>([]);
const renderers = ref<NBodyRenderer[]>([]);
const G = 6.6743e-11;

const canvasWidth = ref(window.innerWidth * 1.5);
const canvasHeight = ref(window.innerHeight * 1.5);
const scale = ref({ sx: 1, sy: 1 });

let stage: Konva.Stage | null = null;
let lineLayer: Konva.Layer;
let lineGroup: Konva.Group;

class NBody {
  vx = Math.random() * 2 - 1;
  vy = Math.random() * 2 - 1;
  angle = 0;
  orbitRadius = 0;

  constructor(public x: number, public y: number, public mass: number, public radius: number) { }

  update(bodies: NBody[], G: number, cd: number) {
    for (let other of bodies) {
      if (other !== this) {
        const dx = other.x - this.x;
        const dy = other.y - this.y;
        const distSq = dx * dx + dy * dy;

        matchSync()
          .with(is.number().lt((this.radius + other.radius + cd) ** 2).match, () => {
            const [larger, smaller] = this.mass >= other.mass ? [this, other] : [other, this];
            this.orbitRadius = larger.radius + smaller.radius + cd;
            smaller.angle += 0.05;
            smaller.x = larger.x + this.orbitRadius * Math.cos(smaller.angle);
            smaller.y = larger.y + this.orbitRadius * Math.sin(smaller.angle);
          })
          .otherwise(() => {
            const force = (G * this.mass * other.mass) / distSq;
            const ax = (force * dx) / Math.sqrt(distSq);
            const ay = (force * dy) / Math.sqrt(distSq);
            this.vx += ax / this.mass;
            this.vy += ay / this.mass;
          })
          .run(distSq);
      }
    }

    this.x += this.vx;
    this.y += this.vy;

    // 边界反弹 + clamp 限制位置
    if (this.x < this.radius || this.x > canvasWidth.value - this.radius) {
      this.vx *= -1;
      this.x = Math.min(Math.max(this.radius, this.x), canvasWidth.value - this.radius);
    }
    if (this.y < this.radius || this.y > canvasHeight.value - this.radius) {
      this.vy *= -1;
      this.y = Math.min(Math.max(this.radius, this.y), canvasHeight.value - this.radius);
    }
  }
}

class NBodyRenderer {
  body: Konva.Circle;

  constructor(public layer: Konva.Layer, public nBody: NBody) {
    this.body = new Konva.Circle({
      x: nBody.x,
      y: nBody.y,
      radius: nBody.radius,
      fill: "white",
      stroke: "blue",
      strokeWidth: 2,
    });
    this.layer.add(this.body);
  }

  draw() {
    this.body.x(this.nBody.x);
    this.body.y(this.nBody.y);
  }

  setColor(color: string) {
    this.body.fill(color);
  }

  drawLineTo(other: NBodyRenderer) {
    const line = new Konva.Line({
      points: [this.body.x(), this.body.y(), other.body.x(), other.body.y()],
      stroke: "red",
      strokeWidth: 2,
    });
    lineGroup.add(line);
  }
}

function createBodies(layer: Konva.Layer) {
  bodies.value = [];
  renderers.value = [];
  layer.destroyChildren(); // 清除旧内容
  for (let i = 0; i < bdnum.value; i++) {
    const radius = Math.random() * 10 + 5;
    const body = new NBody(
      Math.random() * (canvasWidth.value - radius * 2) + radius,
      Math.random() * (canvasHeight.value - radius * 2) + radius,
      Math.random() * 10 + 5,
      radius
    );
    bodies.value.push(body);
    renderers.value.push(new NBodyRenderer(layer, body));
  }
}

function animate() {
  // 物理更新
  for (let body of bodies.value) {
    body.update(bodies.value, G, cdnum.value);
  }

  // 清除旧线
  lineGroup.destroyChildren();

  // 处理颜色和连线
  for (let i = 0; i < bodies.value.length; i++) {
    for (let j = i + 1; j < bodies.value.length; j++) {
      const a = bodies.value[i], b = bodies.value[j];
      const ar = renderers.value[i] as NBodyRenderer, br = renderers.value[j] as NBodyRenderer;
      const dx = b.x - a.x, dy = b.y - a.y;
      const distSq = dx * dx + dy * dy;

      if (distSq < (a.radius + b.radius + cdnum.value) ** 2) {
        const t = handleColor(ar, br).run([a, b]);
        if (t?.isRight()) {
          const [colorA, colorB, ra, rb] = t.value;
          ra.setColor(colorA);
          rb.setColor(colorB);
          ra.drawLineTo(rb);
        }

      } else {
        ar.setColor("white");
        br.setColor("white");
      }
    }
  }

  // 最后统一 draw 所有 body
  for (let r of renderers.value) r.draw();

  stage?.batchDraw();
  requestAnimationFrame(animate);
}

const handleColor = (br: NBodyRenderer, or: NBodyRenderer) => {
  type Res = ["red" | "white", "white" | "red", NBodyRenderer, NBodyRenderer];
  return matchSync<[NBody, NBody], Res>()
    .with2(([a, b]) => a.mass > b.mass, () => ["red", "white", br, or])
    .with2(([a, b]) => a.mass < b.mass, () => ["red", "white", or, br])
    .otherwise(() => ["white", "white", br, or]);
};

function updateCanvasSize() {
  const cw = window.innerWidth / 1.5;
  const ch = window.innerHeight / 1.5;
  scale.value = { sx: cw / canvasWidth.value, sy: ch / canvasHeight.value };
  canvasWidth.value = cw;
  canvasHeight.value = ch;

  if (stage) {
    stage.size({ width: cw, height: ch });
    for (let b of bodies.value) {
      b.x *= scale.value.sx;
      b.y *= scale.value.sy;
    }
  }
}

function spawnCanvas() {
  stage = new Konva.Stage({
    container: "space",
    width: canvasWidth.value,
    height: canvasHeight.value,
  });
}

async function init() {
  spawnCanvas();
  updateCanvasSize();

  const layer = new Konva.Layer();
  lineLayer = new Konva.Layer();
  lineGroup = new Konva.Group();
  lineLayer.add(lineGroup);

  stage?.add(layer);
  stage?.add(lineLayer);

  createBodies(layer);
  animate();
}

onMounted(() => {
  window.addEventListener("resize", updateCanvasSize);
  init();
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", updateCanvasSize);
});

function bdnumChange(e: Event) {
  bdnum.value = parseInt((e.target as HTMLInputElement).value);
  scale.value = { sx: 1, sy: 1 };
  createBodies(stage?.children[0] as Konva.Layer);
}

function cdnumChange(e: Event) {
  const value = parseInt((e.target as HTMLInputElement).value);
  cdnum.value = value;
  copycdnum.value = value;
}
</script>

<style scoped>
.controls {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 5px;
}

label {
  font-size: 16px;
  margin-bottom: 5px;
}

input[type="range"] {
  width: 200px;
  margin-bottom: 10px;
}

#space {
  border: 5px solid #f78f8f;
  background-color: #000000;
}
</style>
