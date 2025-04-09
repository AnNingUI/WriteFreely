<template>
    <div class="frame" id="css100love">
        <div class="center">
            <div class="heart"></div>
        </div>
    </div>
</template>

<script setup lang="ts">

</script>

<style scoped>

.frame {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 400px;
    height: 400px;
    margin-top: -200px;
    margin-left: -200px;
    border-radius: 2px;
    box-shadow: 1px 2px 10px 0px rgba(0, 0, 0, 0.3);
    background: #0a1421;
}

.center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}

@keyframes beat {
    0% {
        transform: scale(1);
    }
    35% {
        transform: scale(1.2);
    }
}

.heart {
    width: 70px;
    height: 70px;
    background: #ff1736;
    position: relative;
    rotate: -45deg;
    box-shadow: -17px 17px 80px #ff1736;
    animation: beat 0.9s ease-in-out infinite;

    &:before,
    &:after {
        content: '';
        position: absolute;
        width: 70px;
        height: 70px;
        background: #ff1736;
        border-radius: 50%;
    }
    &:before {
        top: -50%;
        box-shadow: -12px -20px 80px #ff1736;
    }
    &:after {
        right: -50%;
        box-shadow: 20px 12px 80px #ff1736;
    }
}
</style>
