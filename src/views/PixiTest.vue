<template>
    <div id="pixi-test"></div>
</template>

<script setup lang="ts">
import * as PIXI from 'pixi.js';
import { onMounted } from 'vue';

const initPixi = async () => {
    const app = new PIXI.Application();
    await app.init({
        width: 800,
        height: 600,
        backgroundColor: 0xEF6644
    });

    document.getElementById("pixi-test")!.appendChild(app.canvas);

    const container = new PIXI.Container();
    container.x = app.screen.width / 2;
    container.y = app.screen.height / 2;
    app.stage.addChild(container);

    // 绘制网格
    const gridSize = 50; // 网格大小
    const gridColor = 0xFFFFFF; // 网格颜色
    const lineWidth = 1; // 线宽

    const drawGrid = (container: PIXI.Container) => {
        const gridGraphics = new PIXI.Graphics();
        // @ts-ignore
        gridGraphics.setStrokeStyle(lineWidth, gridColor, 0.5); // 设置边框样式

        // 绘制水平线
        for (let y = -container.height / 2; y < container.height / 2; y += gridSize) {
            gridGraphics.moveTo(-container.width / 2, y);
            gridGraphics.lineTo(container.width / 2, y);
        }

        // 绘制垂直线
        for (let x = -container.width / 2; x < container.width / 2; x += gridSize) {
            gridGraphics.moveTo(x, -container.height / 2);
            gridGraphics.lineTo(x, container.height / 2);
        }

        container.addChild(gridGraphics);
    };

    drawGrid(container); // 调用绘制网格的函数

    // 绘制其他图形
    for (let i = 0; i < 7; i++) {
        const g = new PIXI.Graphics();
        // @ts-ignore
        g.setStrokeStyle(2, 0xDE3249, 0.5); // 设置边框颜色
        g.fill(0xDE3249); // 设置填充颜色
        g.rect((i % 5) * 50 - 25, Math.floor(i / 5) * 50 - 25, 50, 50); // 绘制矩形
        container.addChild(g);
    }

    container.pivot.x = container.width / 2; // 设置容器的中心点
    container.pivot.y = container.height / 2;

    // 监听动画更新
    app.ticker.add(function (delta) {
        // @ts-ignore
        container.rotation += 0.01 * delta; // 使容器旋转
    });
}

onMounted(() => {
    initPixi().then(() => {
        console.log("Pixi initialized");
    }).catch((error) => {
        console.warn(error);
    });
});
</script>

<style scoped>
#pixi-test {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    /* 使画布垂直居中 */
}
</style>
