import Vue from 'vue'
import Vuex from 'vuex'
import axios from 'axios'

Vue.use(Vuex);

let store = new Vuex.Store({
    state:{
        products:[]
    },
    mutations:{
        Set_Products_To_state:(state, products)=>{
            state.products = products;
        }
    },
    actions:{
        GET_PRODUCTS_FROM_API({commit}){
            return axios('https://elhemist.orius.dev/', {
                method: "GET"
            }).then((products)=>{
                commit('Set_Products_To_state', products.data);
                return products;
            })
            .catch((error)=> {
                console.log(error)
                return error;
            })
        }
    },
    getters:{
        PRODUCTS(state){
            return state.products;
        }
    }
});

export default store;