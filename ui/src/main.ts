import { createApp } from 'vue'
import {createRouter, createWebHashHistory} from 'vue-router';
import './style.css'
import App from './App.vue';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';
import { createPinia } from 'pinia';
import routes from './routes';

// Profiles custom elements
import "@holochain-open-dev/profiles/profiles-context";
import "@holochain-open-dev/profiles/create-profile";
import "@holochain-open-dev/profiles/agent-avatar";
import "@holochain-open-dev/profiles/my-profile";
import "@holochain-open-dev/profiles/profile-prompt";
import "@holochain-open-dev/profiles/profile-detail";

// Initialize Vue App
const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`,
});
const pinia = createPinia();
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