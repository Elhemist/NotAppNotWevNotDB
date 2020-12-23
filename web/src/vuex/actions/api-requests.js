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
  POST_Register_To_API(phone, password,first_name,middle_name,last_name) {
    return axios({
      method: 'post',
      url: 'https://elhemist.orius.dev/api/users',
      data: {
        phone: phone,
        password: password,
        first_name: first_name,
        middle_name: middle_name,
        last_name: last_name
      }
    })
    .then(response => console.log(response.data))
    .catch(error => console.log(error));
  },
  POST_LOGIN_To_API(phone, password) {
    return axios({
      method: 'post',
      url: 'https://elhemist.orius.dev/api/users/login',
      data: {
        phone: phone,
        password: password
      }
    })
    .then(response => console.log(response.data))
    .catch(error => console.log(error));
  },
}