import { createApp } from 'vue'
import {createRouter, createWebHashHistory, createWebHistory} from 'vue-router';
import './style.css'
import App from './App.vue';
import AllPosts from './herd/posts/AllPosts.vue';
import CreatePost from './herd/posts/CreatePost.vue';
import PostDetail from './herd/posts/PostDetail.vue';
import CreateHerd from './herd/herds/CreateHerd.vue';
import Home from './herd/herds/Home.vue';
import HerdDetail from './herd/herds/HerdDetail.vue';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';
import '@material/mwc-icon/mwc-icon.js';

const herd_routes = [
    { path: '', component: AllPosts },
    { path: 'posts/create', component: CreatePost },
    { path: 'posts/:postHashString', component: PostDetail },
];

const routes = [
    { path: '', component: Home },
    { path: '/herds/create', component: CreateHerd },
    { path: '/herds/private/:password', component: HerdDetail, children: herd_routes },
    { path: '/herds/:listingHashString', component: HerdDetail, children: herd_routes },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`,
});

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