import { createRouter, createWebHistory } from "vue-router";

import ProfileChooser from "./views/ProfileChooser.vue";
import ProfileEditor from "./views/ProfileEditor.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/profiles",
    },
    {
      path: "/profiles",
      name: "ProfileChooser",
      component: ProfileChooser,
    },
    {
      path: "/profiles/:id",
      name: "ProfileEditor",
      component: ProfileEditor,
    },
  ],
});
