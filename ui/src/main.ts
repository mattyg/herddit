import { createApp } from 'vue'
import {createRouter, createWebHashHistory, createWebHistory} from 'vue-router';
import './style.css'
import App from './App.vue';
import AllPosts from './herd/posts/AllPosts.vue';
import CreatePost from './herd/posts/CreatePost.vue';
import PostDetail from './herd/posts/PostDetail.vue';
import EditPost from './herd/posts/EditPost.vue';
import CreateHerd from './herd/herds/CreateHerd.vue';
import Home from './herd/herds/Home.vue';
import HerdDetail from './herd/herds/HerdDetail.vue';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';

const routes = [
    { path: '', component: Home },
    { path: '/herds/create', component: CreateHerd },
    { path: '/herds/:listingHashString', component: HerdDetail, 
        children: [
            { path: '', component: AllPosts },
            { path: 'posts/create', component: CreatePost },
            { path: 'posts/:postHashString', component: PostDetail },
        ]
    },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`,
});

const app = createApp(App);

app.use(router);
app.use(Vue3Toastify, {
    autoClose: 2000,
    position: 'bottom-left',
    style: {
      opacity: '1',
      userSelect: 'initial',
    },
  } as ToastContainerOptions);

app.mount('#app');