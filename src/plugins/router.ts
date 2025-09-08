import { createRouter, createWebHistory } from "vue-router";

import ProfileChooser from "./views/ProfileChooser.vue";
import ProfileEditor from "./views/ProfileEditor.vue";
import CreateProfile from "./views/CreateProfile.vue";
import DebugView from "./views/DebugView.vue";

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
      path: "/profiles/new",
      name: "CreateProfile",
      component: CreateProfile,
    },
    {
      path: "/profiles/:id",
      name: "ProfileEditor",
      component: ProfileEditor,
    },
    {
      path: "/debug",
      name: "Debug",
      component: DebugView,
    },
  ],
});
