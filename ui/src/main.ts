import { createApp } from 'vue'
import {createRouter, createWebHashHistory} from 'vue-router';
import './style.css'
import App from './App.vue';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';
import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import routes from './routes';

// Profiles custom elements
import "@holochain-open-dev/profiles/elements/profiles-context.js";
import "@holochain-open-dev/profiles/elements/create-profile.js";
import "@holochain-open-dev/profiles/elements/agent-avatar.js";
import "@holochain-open-dev/profiles/elements/my-profile.js";
import "@holochain-open-dev/profiles/elements/profile-prompt.js";
import "@holochain-open-dev/profiles/elements/profile-detail.js";

// Shoelace
import '@shoelace-style/shoelace/dist/themes/light.css';
import '@shoelace-style/shoelace/dist/themes/dark.css';
import { setBasePath } from '@shoelace-style/shoelace/dist/utilities/base-path';
setBasePath('shoelace');

// Initialize Vue App
const router = createRouter({
    history: createWebHashHistory(),
    routes,
});
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
const app = createApp(App);

app.use(pinia);
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