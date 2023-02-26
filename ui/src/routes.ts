import AllPosts from './herd/posts/AllPosts.vue';
import CreatePost from './herd/posts/CreatePost.vue';
import PostDetail from './herd/posts/PostDetail.vue';
import CreateHerd from './herd/herds/CreateHerd.vue';
import WateringHole from './herd/herds/WateringHole.vue';
import HerdDetail from './herd/herds/HerdDetail.vue';
import AgentProfileDetail from './herd/profiles/AgentProfileDetail.vue';

const herd_routes = [
  { path: '', component: AllPosts },
  { path: 'posts/create', component: CreatePost },
  { path: 'posts/:postHashString', component: PostDetail },
];

export default [
  { path: '', component: WateringHole },
  { path: '/agents/:agentPubKeyString', component: AgentProfileDetail },
  { path: '/herds/create', component: CreateHerd },
  { path: '/herds/private/:password', component: HerdDetail, children: herd_routes },
  { path: '/herds/:listingHashString', component: HerdDetail, children: herd_routes },
];
