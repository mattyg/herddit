import { createApp } from 'vue'
import {createRouter, createWebHashHistory, createWebHistory} from 'vue-router';
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

// Material custom elements
import "@webcomponents/scoped-custom-element-registry";
import '@material/mwc-button';
import '@material/mwc-icon';
import '@material/mwc-icon-button';
import '@material/mwc-checkbox';
import '@material/mwc-formfield';
import '@material/mwc-textarea';
import '@material/mwc-textfield';
import '@material/mwc-circular-progress';
import '@material/mwc-switch';

// Profiles custom elements
import { CreateProfile, AgentAvatar, MyProfile, ProfilesContext, ProfilePrompt, ProfileDetail } from "@holochain-open-dev/profiles";
import { DisplayError } from "@holochain-open-dev/elements";
customElements.define("profiles-context", ProfilesContext);
customElements.define('display-error', DisplayError);
customElements.define("create-profile", CreateProfile);
customElements.define("agent-avatar", AgentAvatar);
customElements.define("my-profile", MyProfile);
customElements.define("profile-prompt", ProfilePrompt);
customElements.define("profile-detail", ProfileDetail);

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