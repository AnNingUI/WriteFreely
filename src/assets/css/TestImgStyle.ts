
import { css } from "lit";

/* -------------------------------------------- styles -------------------------------------------- */
export const responsiveStyles = css`
/* responsive styles */
:host {
  display: block;
  width: 92vw; /* 确保容器宽度为100% */
  height: 92vh; /* 确保容器高度为100% */
  overflow: hidden; /* 避免溢出 */
}
.container {
  display: flex;
  justify-content: center; /* 水平居中 */
  align-items: center; /* 垂直居中 */
  width: 100%;
  height: 100%;
}
.open {
  position: relative;
  .test {
    width: fit-content;
    height: fit-content;
    opacity: 1; /* 显示文字 */
    padding: 10px; /* 增加内边距 */
    margin: 10px; /* 增加外边距 */
    color: #fffbfb; /* 文字颜色 */
    /* 位置于右下角 */
    position: absolute;
    left: 5%;
    transition: opacity 0.9s cubic-bezier(0.65, 0.05, 0.36, 1); /* 添加过渡效果 */
  }
}

.close {
  .test {
    opacity: 0; /* 隐藏文字 */
  }
}
@media screen and (min-width: 481px) {
  .open {
    position: relative;
    width: 178%; 
    height: 100%; 
    transition: width 0.7s ease-in-out; /* 添加过渡效果 */
  }
  .close {
    width: 10%; 
    height: 100%; 
    transition: width 0.7s ease-in-out; /* 添加过渡效果 */
  }
  .test {
    top: 90%;
  }
  @media screen and (max-height: 560px) and (min-height: 360px) {
    .test {
      top: 85%;
    }
  }
  @media screen and (max-height: 359px) {
    .test {
      top: 80%;
    }
  }
}
@media screen and (max-width: 480px) {
  .open {
    width: 100%; 
    height: 56.25%; 
    transition: height 0.5s ease-in-out; /* 添加过渡效果 */
  }
  .close {
    width: 100%; 
    height: 20%; 
    transition: height 0.5s ease-in-out; /* 添加过渡效果 */
  }
  .container {
    flex-direction: column; /* 变为垂直排列 */
  }
  .test {
    top: 80%;
    font-size: 0.6rem; /* 文字大小 */
    transition: font-size 0.2s ease-in-out; /* 添加过渡效果 */
  }
}
` 

export const originalStyles = css`
/* original styles */
:host {
  display: block;
  width: 92vw; /* 确保容器宽度为100% */
  height: 92vh; /* 确保容器高度为100% */
  overflow: hidden; /* 避免溢出 */
}
.container {
  display: flex;
  justify-content: center; /* 水平居中 */
  align-items: center; /* 垂直居中 */
  width: 100%;
  height: 100%;
}
.open {
  width: 178%; 
  height: 100%; 
  transition: width 0.7s ease-in-out; /* 添加过渡效果 */
  position: relative;
  .test {
    opacity: 1; /* 显示文字 */
    padding: 10px; /* 增加内边距 */
    margin: 10px; /* 增加外边距 */
    color: #fffbfb; /* 文字颜色 */
    /* 位置于右下角 */
    position: absolute;
    top: 90%;
    left: 5%;
    transition: opacity 0.9s cubic-bezier(0.65, 0.05, 0.36, 1); /* 添加过渡效果 */
  }
}

.close {
  width: 10%; 
  height: 100%; 
  transition: width 0.7s ease-in-out; /* 添加过渡效果 */
  .test {
    opacity: 0; /* 隐藏文字 */
  }
}
@media screen and (max-width: 480px) {
  .open>.test {
    left: 3%;
    font-size: 0.6em; /* 文字大小 */
    margin: 5px; /* 增加外边距 */
    transition: opacity 0.9s cubic-bezier(0.65, 0.05, 0.36, 1); /* 添加过渡效果 */
  }
}
`

/* ----------------------------------------------------------------------------------------------- */