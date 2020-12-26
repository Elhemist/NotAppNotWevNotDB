
export default {
  GET_SEARCH_VALUE_TO_VUEX ({commit}, value) {
    commit('SET_SEARCH_VALUE_TO_VUEX', value)
  },
  SET_MOBILE({commit}) {
    commit('SWITCH_MOBILE')
  },
  SET_DESKTOP({commit}) {
    commit('SWITCH_DESKTOP')
  },
  ADD_TO_CART({commit}, product) {
    commit('SET_CART', product);
  },

}
