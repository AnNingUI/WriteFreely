import router from "@/router";
import { createApp } from "vue";
import App from "./App.vue";

import { TestImgs } from "@/components/webCom/test-imgs";
customElements.define("test-imgs", TestImgs);

const app = createApp(App);
app.use(router);
app.mount("#app");
