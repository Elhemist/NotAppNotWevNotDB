import axios from "axios";

export default {
  GET_PRODUCTS_FROM_API({commit}) {
    return axios('https://elhemist.orius.dev/api/products', {
      method: "GET"
    })
      .then((products) => {
        commit('SET_PRODUCTS_TO_STATE', products.data.data);
        return products.data;
      })
      .catch((error) => {
        console.log(error)
        return error;
      })
  },
  DELETE_FROM_CART({commit}, index) {
    return axios({
      headers:{'x-session-id': localStorage.token},
      method: 'post',
      url: 'https://elhemist.orius.dev/api/cart',
      data: {
        product_id: index,
        quantity: 0
      }
    })
    .then(
    commit('REMOVE_FROM_CART', index))
    .catch(error => console.log(error));
  },
  
  DECREMENT_CART_ITEM({commit}, index) {
    var notnya=index[0];
    var nya1=index[1];
    var nya2=index[2];
    commit('DECREMENT', notnya);
    return axios({
      headers:{'x-session-id': localStorage.token},
      method: 'post',
      url: 'https://elhemist.orius.dev/api/cart',
      data: {
        "product_id": nya1,
        "quantity": nya2
      }
    })
  },
  INCREMENT_CART_ITEM({commit}, index) {
    var notnya=index[0];
    var nya1=index[1];
    var nya2=index[2];
    commit('INCREMENT', notnya);
    return axios({
      headers:{'x-session-id': localStorage.token},
      method: 'post',
      url: 'https://elhemist.orius.dev/api/cart',
      data: {
        "product_id": nya1,
        "quantity": nya2
      }
    })
  },
  CLEAR_CART({commit}){
    commit('Clear');
    return axios({
      headers:{'x-session-id': localStorage.token},
      method: 'delete',
      url: 'https://elhemist.orius.dev/api/cart',
      data: {
      }
    }).then(
      commit('Clear'))
  },
  GET_ORDERS_FROM_API({commit}) {
    return axios('https://elhemist.orius.dev/api/orders', {
      headers:{'x-session-id': localStorage.token},
      method: "GET"
  }).then((orders) => {
  commit('SET_ODERS_TO_STATE', orders.data.data);
  localStorage.setItem('orderid', orders.data.data.id);
  return orders.data.data;
})
  },
  CREATE_ORDER({commit},index) {
    var nya1=index[0];
    var nya2=index[1];
    return axios({
      headers:{'x-session-id': localStorage.token},
      method: 'post',
      url: 'https://elhemist.orius.dev/api/orders',
      data: {
        "street": nya1,
        "home": nya2
      }
    }).then(
      commit('Clear'))
},
}