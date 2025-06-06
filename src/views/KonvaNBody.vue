<template>
  <div class="controls" id="NBody">
    <label for="bodyCount">天体数量: {{ bdnum }}</label>
    <input type="range" id="bodyCount" min="10" max="100" value="10" @input="bdnumChange" />

    <label for="collisionDistance">相遇距离: {{ copycdnum }} px</label>
    <input type="range" id="collisionDistance" min="5" max="200" value="5" @input="cdnumChange" />
    <div>平均FPS: {{ FPS }}</div>
    <div id="space"></div>
  </div>
</template>

<script setup lang="ts">
import Konva from "konva";
import { onBeforeUnmount, onMounted, ref } from "vue";

// 定义响应式变量
const bdnum = ref(10);
const cdnum = ref(5);
const copycdnum = ref(5);
const FPS = ref(0);
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
  id!: number;
  private static instances: Map<number, NBody> = new Map();

  constructor(public x: number, public y: number, public mass: number, public radius: number, id: number) {
    this.id = id
    NBody.instances.set(id, this)
  }

  static get(id: number) {
    return NBody.instances.get(id)
  }

  inAABB(nBody: NBody, cd: number) {
    const distSq = Math.sqrt((nBody.x - this.x) ** 2 + (nBody.y - this.y) ** 2);
    return distSq > 0 && distSq - this.radius - nBody.radius <= cd;
  }

  update(bodies: NBody[], G: number, cd: number) {
    for (let other of bodies.filter((n) => n.inAABB(this, cd) && n.id !== this.id)) {
      const dx = other.x - this.x;
      const dy = other.y - this.y;
      const distSq = dx * dx + dy * dy;
      if (distSq < (this.radius + other.radius + cd) ** 2) {
        const [larger, smaller] = this.mass >= other.mass ? [this, other] : [other, this];
        this.orbitRadius = larger.radius + smaller.radius + cd;
        smaller.angle += 0.05;
        smaller.x = larger.x + this.orbitRadius * Math.cos(smaller.angle);
        smaller.y = larger.y + this.orbitRadius * Math.sin(smaller.angle);
      } else {
        const force = (G * this.mass * other.mass) / distSq;
        const ax = (force * dx) / Math.sqrt(distSq);
        const ay = (force * dy) / Math.sqrt(distSq);
        this.vx += ax / this.mass;
        this.vy += ay / this.mass;
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
  id!: number;
  private static instances: Map<number, NBodyRenderer> = new Map();
  constructor(public layer: Konva.Layer, public nBody: NBody) {
    this.body = new Konva.Circle({
      x: nBody.x,
      y: nBody.y,
      radius: nBody.radius,
      fill: "white",
      stroke: "blue",
      strokeWidth: 2,
    });
    this.id = nBody.id
    this.layer.add(this.body);
    NBodyRenderer.instances.set(nBody.id, this);
  }

  static get(id: number) {
    return NBodyRenderer.instances.get(id)
  }

  static by(nbody: NBody) {
    return NBodyRenderer.instances.get(nbody.id) as NBodyRenderer
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
      radius * 10 + 5,
      radius,
      i
    );
    bodies.value.push(body);
    renderers.value.push(new NBodyRenderer(layer, body));
  }
}
const frameTimes = new Float32Array(600);
let frameIndex = 0;
let lastFrameTime = performance.now();
let benchmarkStart = performance.now();

function benchmarkFrameRate() {
  const now = performance.now();
  const delta = now - lastFrameTime;
  lastFrameTime = now;

  if (frameIndex < frameTimes.length) {
    frameTimes[frameIndex++] = delta;
  }

  if (now - benchmarkStart >= 1000) {
    let sum = 0;
    for (let i = 0; i < frameIndex; i++) {
      sum += frameTimes[i];
    }
    const avgDelta = sum / frameIndex;
    const avgFps = 1000 / avgDelta;
    FPS.value = Math.round(avgFps);
    frameIndex = 0;
    benchmarkStart = now;
  }
}

function animate() {
  // 清除旧线
  lineGroup.destroyChildren();
  // 物理更新
  for (let body of bodies.value) {
    body.update(bodies.value, G, cdnum.value);
  }
  for (let i = 0; i < bodies.value.length; i++) {
    const self = bodies.value[i];
    const selfRenderer = NBodyRenderer.by(self)
    const others = bodies.value.filter((n) => n.inAABB(self, cdnum.value) && n.id !== self.id);
    if (others.length === 0) {
      selfRenderer.setColor("white");
    }
    for (let other of others) {
      const otherRenderer = NBodyRenderer.by(other);
      const dx = other.x - self.x, dy = other.y - self.y;
      const distSq = dx * dx + dy * dy;

      if (distSq < (self.radius + other.radius + cdnum.value) ** 2) {
        if (self.mass > other.mass) {
          selfRenderer.setColor("red");
          otherRenderer.setColor("white");
          selfRenderer.drawLineTo(otherRenderer);
        } else {
          selfRenderer.setColor("white");
          otherRenderer.setColor("red");
          otherRenderer.drawLineTo(selfRenderer);
        }
      } else {
        selfRenderer.setColor("white");
        otherRenderer.setColor("white");
      }
    }
  }



  // 最后统一 draw 所有 body
  for (let r of renderers.value) r.draw();

  stage?.batchDraw();
  benchmarkFrameRate()
  requestAnimationFrame(animate);
}


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
