import Css100LoveView from "../views/Css100Love.vue";
import HomeView from "@/views/Home.vue";
import NbodyView from "@/views/KonvaNBody.vue";
import PixiTestView from "@/views/PixiTest.vue";
import RustNBodyView from "@/views/RustNBody.vue";
import VueWebC from "@/views/VueWebC.vue";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
  { path: "/", component: HomeView },
  { path: "/Css100Love", name: "css100Love", component: Css100LoveView },
  { path: "/NBody", name: "NBody", component: NbodyView },
  { path: "/RustNBody", name: "RustNBody", component: RustNBodyView },
  { path: "/PixiTest", name: "PixiTest", component: PixiTestView },
  { path: "/VueWebC", name: "VueWebC", component: VueWebC },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

// const limport = (router: string) => {
//   const lang = localStorage.getItem("lang") || "en";
//   return function() {
//     return import(`../${lang}/${router}`)
//   }
// }