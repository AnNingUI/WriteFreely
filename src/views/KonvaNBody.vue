<template>
  <div class="controls" id="NBody">
    <label for="bodyCount">天体数量: {{ bdnum }}</label>
    <input
      type="range"
      id="bodyCount"
      min="10"
      max="50"
      value="10"
      @input="bdnumChange"
    />

    <label for="collisionDistance">相遇距离: {{ copycdnum }} px</label>
    <input
      type="range"
      id="collisionDistance"
      min="5"
      max="200"
      value="1"
      @input="cdnumChange"
    />

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
const bodies = ref<NBody[]>([]);
const renderers = ref<NBodyRenderer[]>([] as NBodyRenderer[]);
const G = 6.6743e-11; // 引力常数

// console.log("dpr" + dpr + "scaling" + scaling);
const canvasWidth = ref(window.innerWidth * 1.5);
const canvasHeight = ref(window.innerHeight * 1.5);
const scale = ref({
  sx: 1,
  sy: 1,
});
let stage: Konva.Stage | null = null; // 提升 stage 的作用域

class NBody {
  vx: number;
  vy: number;
  angle: number;
  orbitRadius: number;

  constructor(
    public x: number,
    public y: number,
    public mass: number,
    public radius: number
  ) {
    this.vx = Math.random() * 2 - 1; // 随机速度
    this.vy = Math.random() * 2 - 1; // 随机速度
    this.angle = 0; // 用于圆周运动
    this.orbitRadius = 0; // 轨道半径
  }

  update(bodies: NBody[], G: number, cdnum: number) {
    for (let other of bodies) {
      if (other !== this) {
        const dx = other.x - this.x;
        const dy = other.y - this.y;
        const distance = dx * dx + dy * dy;

        if (distance < (this.radius + other.radius + cdnum) ** 2) {
          // 处理相遇
          const larger = this.mass >= other.mass ? this : other;
          const smaller = this.mass < other.mass ? this : other;

          this.orbitRadius = this.radius + smaller.radius + cdnum;
          smaller.angle += 0.05; // 控制旋转速度
          smaller.x = larger.x + this.orbitRadius * Math.cos(smaller.angle);
          smaller.y = larger.y + this.orbitRadius * Math.sin(smaller.angle);
          return; // 结束此更新，避免进一步的引力计算
        } else {
          // 计算引力
          const force = (G * this.mass * other.mass) / distance;
          const ax = (force * dx) / distance ** 0.5;
          const ay = (force * dy) / distance ** 0.5;

          this.vx += ax / this.mass;
          this.vy += ay / this.mass;
        }
      }
    }

    this.x += this.vx;
    this.y += this.vy;
    // 边界检测
    if (this.x < this.radius || this.x > canvasWidth.value - this.radius)
      this.vx *= -1;
    if (this.y < this.radius || this.y > canvasHeight.value - this.radius)
      this.vy *= -1;
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

    // 这里确保 body 被正确添加到 layer 中
    this.layer.add(this.body);
  }

  draw() {
    this.body.x(this.nBody.x);
    this.body.y(this.nBody.y);
  }

  drawLine(otherRenderer: NBodyRenderer) {
    const line = new Konva.Line({
      points: [this.body.x(), this.body.y(), otherRenderer.body.x(), otherRenderer.body.y()],
      stroke: "red",
      strokeWidth: 2,
      lineJoin: "round",
      lineCap: "round",
    });
    this.layer.add(line);
    line.remove();
  }

  setColor(color: string) {
    this.body.fill(color);
  }
}


function createBodies(layer: Konva.Layer) {
  bodies.value = []; // 清空当前天体
  renderers.value = []; // 清空渲染器
  stage?.children[0]?.removeChildren();
  for (let i = 0; i < bdnum.value; i++) {
    const radius = Math.random() * 10 + 5;
    const body = new NBody(
      Math.random() * (canvasWidth.value - radius * 2) + radius,
      Math.random() * (canvasHeight.value - radius * 2) + radius,
      Math.random() * 10 + 5,
      radius
    );
    bodies.value.push(body);
    renderers.value.push(new NBodyRenderer(layer, body)); // Store renderer
  }
}

function animate() {
  for (let body of bodies.value) {
    if (stage) {
      let ww = Math.abs(stage.width() - canvasWidth.value);
      let wh = Math.abs(stage.height() - canvasHeight.value);
      if (ww > 0.5 || wh > 0.5) {
        stage.size({ width: canvasWidth.value, height: canvasHeight.value });
      } else {
        body.update(bodies.value, G, cdnum.value);
      }
    }
  }

  // Draw lines and colors
  for (let i = 0; i < bodies.value.length; i++) {
    for (let j = 0; j < bodies.value.length; j++) {
      if (i !== j) {
        const body: NBody = bodies.value[i];
        const other: NBody = bodies.value[j];
        const bodyRenderer: NBodyRenderer = renderers.value[i] as NBodyRenderer;
        const otherRenderer: NBodyRenderer = renderers.value[j] as NBodyRenderer;

        const dx = other.x - body.x;
        const dy = other.y - body.y;
        const distance = dx * dx + dy * dy;
        if (distance < body.radius + other.radius + cdnum.value) {
          if (body.mass > other.mass) {
            bodyRenderer.setColor("red");
            otherRenderer.setColor("white");
            bodyRenderer.drawLine(otherRenderer);
          } else {
            bodyRenderer.setColor("white");
            otherRenderer.setColor("red");
            otherRenderer.drawLine(bodyRenderer);
          }
        } else {
          bodyRenderer.setColor("white"); // Reset color
          otherRenderer.setColor("white");
        }
        bodyRenderer.draw(); // Draw the current body
      }
    }
  }

  requestAnimationFrame(animate);
}

// 处理画布的宽高
const updateCanvasSize = () => {
  let cw = window.innerWidth / 1.5;
  let ch = window.innerHeight / 1.5;
  let sx = cw /  canvasWidth.value;
  let sy = ch / canvasHeight.value;
  scale.value = {
    sx: sx,
    sy: sy,
  }
  canvasWidth.value = cw;
  canvasHeight.value = ch;
  if (stage) {
    stage.size({ width: canvasWidth.value, height: canvasHeight.value });
    // 所有NBodyRenderer的位置也要更新
    for (let body of bodies.value) {
      body.x *= scale.value.sx;
      body.y *= scale.value.sy;
    }
  }
};

// 初始化监听事件
onMounted(() => {
  window.addEventListener("resize", updateCanvasSize);
  init(); // 初始化画布
});

// 回收监听事件
onBeforeUnmount(() => {
  window.removeEventListener("resize", updateCanvasSize);
});

// 处理 bdnum 的变化
function bdnumChange(event: Event) {
  bdnum.value = parseInt((event.target as HTMLInputElement).value);
  scale.value = {
    sx: 1,
    sy: 1,
  }
  createBodies(stage?.children[0] as Konva.Layer);
}

// 处理 cdnum 的变化
function cdnumChange(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  cdnum.value = parseInt(value);
  copycdnum.value = parseInt(value);
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
  if (stage) updateCanvasSize();
  const layer = new Konva.Layer();
  scale.value = {
    sx: 1,
    sy: 1,
  }
  createBodies(layer);
  animate();
  stage?.add(layer);
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
