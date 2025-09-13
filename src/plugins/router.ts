import { createRouter, createWebHistory } from "vue-router";

import CreateProfile from "../views/CreateProfile.vue";
import DebugView from "../views/DebugView.vue";
import Home from "../views/Home.vue";
import ManageRegistry from "../views/ManageRegistry.vue";
import ProfileEditor from "../views/ProfileEditor.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "Home",
      component: Home,
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
      path: "/registries",
      name: "Registries",
      component: ManageRegistry,
    },
    {
      path: "/debug",
      name: "Debug",
      component: DebugView,
    },

    {
      path: "/:pathMatch(.*)*",
      redirect: { name: "Home" },
    },
  ],
});
