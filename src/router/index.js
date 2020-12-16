import Vue from 'vue';
import Router from 'vue-router';
import SignIn from '@/components/auth/SignIn';
import SignUp from '@/components/auth/SignUp';
import Lolce from '@/components/start/lolce';
import Products from '@/components/products/products';
import Cart from '@/components/cart/cart'
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
      path: '/products',
      name: 'products',
      component: Products,
    },
    {
      path: '/cart',
      name: 'cart',
      component: Cart,
      props: true
    }
  ],
});
