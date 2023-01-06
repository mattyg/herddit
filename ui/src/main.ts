import { createApp } from 'vue'
import {createRouter, createWebHashHistory} from 'vue-router';
import './style.css'
import App from './App.vue'
import AllPosts from './herd/posts/AllPosts.vue';
import CreatePost from './herd/posts/CreatePost.vue';
import PostDetail from './herd/posts/PostDetail.vue';
import EditPost from './herd/posts/EditPost.vue';
import CreateHerd from './herd/herds/CreateHerd.vue';
import Home from './herd/herds/Home.vue';
import HerdDetail from './herd/herds/HerdDetail.vue';

const routes = [
    { path: '', component: Home },
    { path: '/herds/create', component: CreateHerd },
    { path: '/herds/:listingHashString', component: HerdDetail, 
        children: [
            { path: '', component: AllPosts },
            { path: 'posts/create', component: CreatePost },
            { path: 'posts/:postHashString', component: PostDetail },
            { path: 'posts/:postHashString/edit', component: EditPost }
        ]
    },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
});

const app = createApp(App);
app.use(router);
app.mount('#app');