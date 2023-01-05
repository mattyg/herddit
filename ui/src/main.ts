import { createApp } from 'vue'
import {createRouter, createWebHashHistory} from 'vue-router';
import './style.css'
import App from './App.vue'
import AllPosts from './posts/posts/AllPosts.vue';
import CreatePost from './posts/posts/CreatePost.vue';
import PostDetail from './posts/posts/PostDetail.vue';
import EditPost from './posts/posts/EditPost.vue';

const routes = [
    { path: '', component: AllPosts },
    { path: '/posts/create', component: CreatePost },
    { path: '/posts/:postHashString', component: PostDetail, props: true },
    { path: '/posts/:originalPostHash/edit', component: EditPost, props: true },

    //{ path: '/herds', component: CreatePost },
    //{ path: '/authors/:author', component: MyPosts, props: true },
];

const router = createRouter({
    // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
});

const app = createApp(App);
app.use(router);
app.mount('#app');