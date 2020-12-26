import Vue from 'vue'
import Router from 'vue-router'

import vCatalog from '../components/catalog/v-catalog'
import vCart from '../components/cart/v-cart'
import vMainPage from '../components/main-page/v-main-page'
import vProductPage from '../components/catalog/v-product-page'
import vSignUp from '../components/auth/SignUp'
import vSignIn from '../components/auth/SignIn'
import vUser from '../components/humans/v-user'

Vue.use(Router);

let router = new Router({
  routes: [
    {
      path: '/',
      name: 'mainPage',
      component: vMainPage
    },{
      path: '/user',
      name: 'v-user',
      component: vUser
    },
    {
      path: '/register',
      name: 'SignUp',
      component: vSignUp
    },
    {
      path: '/login',
      name: 'vSignIn',
      component: vSignIn
    },
    {
      path: '/catalog',
      name: 'catalog',
      component: vCatalog
    },
    {
      path: '/product',
      name: 'product',
      component: vProductPage
    },
    {
      path: '/cart',
      name: 'cart',
      component: vCart,
      props: true
    }
  ]
})

export default router;
