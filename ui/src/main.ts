import { createApp } from 'vue'
import {createRouter, createWebHashHistory} from 'vue-router';
import './style.css'
import App from './App.vue';
import AllPosts from './herd/posts/AllPosts.vue';
import CreatePost from './herd/posts/CreatePost.vue';
import PostDetail from './herd/posts/PostDetail.vue';
import CreateHerd from './herd/herds/CreateHerd.vue';
import WateringHole from './herd/herds/WateringHole.vue';
import HerdDetail from './herd/herds/HerdDetail.vue';
import AgentProfileDetail from './herd/profiles/AgentProfileDetail.vue';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';

// Profiles custom elements
import "@webcomponents/scoped-custom-element-registry";
import "@holochain-open-dev/profiles/profiles-context";
import "@holochain-open-dev/profiles/create-profile";
import "@holochain-open-dev/profiles/agent-avatar";
import "@holochain-open-dev/profiles/my-profile";
import "@holochain-open-dev/profiles/profile-prompt";
import "@holochain-open-dev/profiles/profile-detail";

// Routes
const herd_routes = [
    { path: '', component: AllPosts },
    { path: 'posts/create', component: CreatePost },
    { path: 'posts/:postHashString', component: PostDetail },
];
const routes = [
    { path: '', component: WateringHole },
    { path: '/agents/:agentPubKeyString', component: AgentProfileDetail },
    { path: '/herds/create', component: CreateHerd },
    { path: '/herds/private/:password', component: HerdDetail, children: herd_routes },
    { path: '/herds/:listingHashString', component: HerdDetail, children: herd_routes },
];
const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`,
});

// Initialize Vue App
const app = createApp(App);
app.use(router);
app.use(Vue3Toastify, {
    autoClose: 2000,
    closeOnClick: false,
    position: 'bottom-left',
    containerId: 'toasts-container',
    style: {
      opacity: '1',
      userSelect: 'initial',
      width: 'auto'
    },
  } as ToastContainerOptions);
app.mount('#app');