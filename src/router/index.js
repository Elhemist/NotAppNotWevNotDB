import Vue from 'vue';
import Router from 'vue-router';
import SignIn from '@/components/auth/SignIn';
import SignUp from '@/components/auth/SignUp';
import Lolce from '@/components/start/lolce';
import Products from '@/components/products/products';

Vue.use(Router);

export default new Router({
  mode: 'history',
  routes: [{
      path: '/auth/signin',
      name: 'SignIn',
      component: SignIn,
    },
    {
      path: '/auth/signup',
      name: 'SignUp',
      component: SignUp,
    },
    {
      path: '/start/lolce',
      name: 'lolce',
      component: Lolce,
    },
    {
      path: '/products/products',
      name: 'products',
      component: Products,
    },
  ],
});
