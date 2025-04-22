<template>
    <div class="controls">
        <div style="display: flex; justify-content: space-between;">
            <button @click="() => {
                showBall = !showBall
            }">{{ showBall ? '隐藏' : '显示' }}[__球体__]</button>

            <button @click="() => {
                showOrbit = !showOrbit
            }">{{ showOrbit ? '隐藏' : '显示' }}[碰撞轨道]</button>
        </div>

        <div style="display: flex; justify-content: space-between;">
            <button @click="() => {
                showId = !showId
            }">{{ showId ? '隐藏' : '显示' }}[___id___]</button>

            <button @click="() => {
                showLine = !showLine
            }">{{ showLine ? '隐藏' : '显示' }}[__连线__]</button>
        </div>

        <label for="bodyCount">天体数量: <span>{{ bodyCount }}</span></label>
        <input type="range" id="bodyCount" min="1" max="100" v-model="bodyCount" @change="initializeSimulation" />

        <label for="collisionDistance">相遇距离: <span>{{ collisionDistance }}</span> px</label>
        <input type="range" id="collisionDistance" min="1" max="200" v-model="collisionDistance"
            @change="updateSimulation" />
        <div>平均FPS: {{ FPS }}</div>
        <canvas ref="canvas" :width="canvasWidth" :height="canvasHeight"></canvas>
    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onBeforeUnmount, onMounted, onUnmounted, ref } from "vue";

interface NBody {
    id: number;
    x: number;
    y: number;
    mass: number;
    radius: number;
    vx: number;
    vy: number;
    angle: number;
    orbit_radius: number;
    is_center: boolean;
}

const inAABB = (self: NBody, other: NBody, cd: number) => {
    const distSq = Math.sqrt((other.x - self.x) ** 2 + (other.y - self.y) ** 2);
    return distSq > 0 && distSq - self.radius - other.radius <= cd;
}

// const dpr = window.devicePixelRatio || 1; // 获取设备像素比
const showOrbit = ref(false)
const showBall = ref(false)
const showId = ref(false)
const showLine = ref(false)
const bodyCount = ref(10);
const collisionDistance = ref(5);
const canvasWidth = ref(window.innerWidth / 1.5);
const canvasHeight = ref(window.innerHeight / 1.5);
const canvas = ref<HTMLCanvasElement | null>(null);
const ctx = ref<CanvasRenderingContext2D | null>(null);
const animationFrameId = ref<number | null>(null);
const bufferCanvas = document.createElement("canvas");
const bufferCtx = bufferCanvas.getContext("2d");
const FPS = ref(0);
const bodies = ref<Map<number, NBody>>(new Map());

const initializeSimulation = async () => {
    try {
        if (canvas.value && ctx.value) {
            // 更新后端的画布尺寸
            await invoke("update_canvas_size", {
                width: canvasWidth.value,
                height: canvasHeight.value,
            });

            // 创建天体
            const newBodies: NBody[] = await invoke("create_bodies", {
                count: Number(bodyCount.value),
                canvasWidth: canvasWidth.value,
                canvasHeight: canvasHeight.value,
            });
            newBodies.forEach((body) => {
                bodies.value.set(body.id, body);
            });

            // 设置缓冲画布大小
            bufferCanvas.width = canvasWidth.value;
            bufferCanvas.height = canvasHeight.value;

            // 开始动画
            await animate();
        }
    } catch (error) {
        console.error("Error initializing simulation:", error);
    }
};

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
        FPS.value = Number(avgFps.toFixed(2));
        frameIndex = 0;
        benchmarkStart = now;
    }
}

const animate = async () => {
    if (ctx.value && canvas.value && bufferCtx) {
        // 清除缓冲画布
        bufferCtx.clearRect(0, 0, bufferCanvas.width, bufferCanvas.height);

        // 更新模拟
        await updateSimulation();

        // 渲染天体到缓冲画布
        bodies.value.forEach((body, _, self) => {
            bufferCtx.save();
            if (showBall.value) {
                showBallFun(bufferCtx, body);
            }

            for (let [otherId, otherBody] of self) {
                if (inAABB(body, otherBody, collisionDistance.value) && body.id !== otherId) {
                    let dx = body.x - otherBody.x;
                    let dy = body.y - otherBody.y;
                    let d2 = dx * dx + dy * dy;
                    if (showLine.value) {
                        showLineFun(bufferCtx, body, otherBody, otherId)
                    }
                    if (showOrbit.value) {
                        if (Math.sqrt(d2) <= body.radius + otherBody.radius + collisionDistance.value) {
                            showOrbitFun(bufferCtx, body, otherBody)
                            showBall.value && showBallFun(bufferCtx, body);
                        }
                    }
                }
            }



            if (showId.value) {
                bufferCtx.strokeStyle = "red";
                bufferCtx.strokeText(`${body.id}`, body.x, body.y);
            }
            bufferCtx.restore();
        });

        // 将缓冲画布的内容绘制到主画布上
        ctx.value.clearRect(0, 0, canvasWidth.value, canvasHeight.value);
        ctx.value.drawImage(bufferCanvas, 0, 0);

        // 循环下一帧
        benchmarkFrameRate()
        animationFrameId.value = requestAnimationFrame(animate);
    }
};

function showBallFun(ctx: CanvasRenderingContext2D, body: NBody) {
    ctx.fillStyle = body.is_center && showOrbit.value ? "red" : "white";
    ctx.beginPath();
    ctx.arc(body.x, body.y, body.radius, 0, 2 * Math.PI);
    ctx.fill();
}

function showLineFun(ctx: CanvasRenderingContext2D, body: NBody, otherBody: NBody, i: number) {
    ctx.strokeStyle = `rgba(${Math.min(255, Math.abs(127 - i))}, 127, 127, ${1 / (i + 1)})`; // 例如红色
    ctx.beginPath();
    ctx.moveTo(body.x, body.y);
    ctx.lineTo(otherBody.x, otherBody.y);
    ctx.stroke();
}

function showOrbitFun(ctx: CanvasRenderingContext2D, body: NBody, otherBody: NBody) {
    let obj = { body, otherBody };
    let maxBody = Object.values(obj).reduce((max, current) =>
        current.radius > max.radius ? current : max
    );
    maxBody.is_center = true;
    ctx.setLineDash([5, 5]); // 设置虚线样式
    ctx.strokeStyle = 'white';
    ctx.beginPath();
    ctx.arc(maxBody.x, maxBody.y, maxBody.orbit_radius, 0, Math.PI * 2);
    ctx.stroke();
    ctx.setLineDash([]); // 重置为实线
}

const updateSimulation = async () => {
    bodies.value.clear();
    try {
        if (ctx.value && canvas.value) {
            const updatedBodies: NBody[] = await invoke("update_simulation", {
                dd: Number(collisionDistance.value),
            });
            updatedBodies.forEach((body) => {
                bodies.value.set(body.id, body);
            });
        }
    } catch (error) {
        console.error("Error updating simulation:", error);
    }
};

// 防抖函数
const debounce = (fn: Function, delay: number) => {
    let timeoutId: number | null = null;
    return (...args: any[]) => {
        if (timeoutId) clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            fn(...args);
        }, delay);
    };
};

// 监听画布尺寸变化，更新前端和后端
const handleResize = debounce(async () => {
    if (canvas.value) {
        canvasWidth.value = window.innerWidth / 1.5;
        canvasHeight.value = window.innerHeight / 1.5;
        canvas.value.width = canvasWidth.value;
        canvas.value.height = canvasHeight.value;

        // canvas.value.style.width = `${canvasWidth.value / dpr}px`;
        // canvas.value.style.height = `${canvasHeight.value / dpr}px`;

        // 更新后端的画布尺寸
        await invoke("update_canvas_size", {
            width: canvasWidth.value,
            height: canvasHeight.value,
        });

        // 重新初始化模拟
        await initializeSimulation();
    }
}, 300);

onMounted(async () => {
    document.body.style.backgroundColor = 'var(--background-color)';

    canvas.value = document.querySelector("canvas");
    if (canvas.value) {
        ctx.value = canvas.value.getContext("2d");
    }

    // 初始化模拟
    await initializeSimulation();

    window.addEventListener("resize", handleResize);
});

onBeforeUnmount(() => {
    document.body.style.backgroundColor = ''; // 清除样式
});

onUnmounted(() => {
    if (animationFrameId.value !== null) {
        cancelAnimationFrame(animationFrameId.value);
    }
    bodies.value.clear();
    window.removeEventListener("resize", handleResize);
});
</script>

<style scoped>
body {
    margin: 0;
    padding: 0;
}

.controls {
    background-color: var(--background-color);
    margin: 20px;
    color: #000;
    display: flex;
    flex-direction: column;
    align-items: center;
}

label {
    margin: 10px 0;
}

canvas {
    border: 5px solid #e272a5;
    background-color: #250202;
}
</style>
