import Vue from 'vue'
import App from './App.vue'
import store from './vuex/store'
import router from "./router/router";

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

import { BootstrapVue } from 'bootstrap-vue'
import Vuelidate from 'vuelidate'
import './assets/styles/styles.scss'
import 'material-design-icons-iconfont'

Vue.config.productionTip = false
Vue.use(BootstrapVue)
Vue.use(Vuelidate)

new Vue({
  render: h => h(App),
  store,
  router
}).$mount('#app')
