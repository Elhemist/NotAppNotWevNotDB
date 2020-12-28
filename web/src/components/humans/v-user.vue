<template>
  <div class='v-cart'>
    <div >
      <button @click="ClearCart" class="clear" >Очиститесь огнём</button>
      
        <router-link :to="{name: 'catalog'}">
          <div class="v-catalog__link_to_cart">Вернуться в каталог</div>
        </router-link>
      
    </div>
    <button @click="show()"/>
    <h1>Cart</h1>
    <p v-if="!order_data.length">Пустота холодна...</p>
    <v-order-item
        v-for="(item) in then.$store.state.orders"
        :key="item.id"
        :cart_item_data="item"
    />
  </div>
</template>

<script>
  import vOrderItem from './v-order-item'
  import {mapActions, mapGetters} from 'vuex'
  export default {
    name: "v-user",
    components: {
      vOrderItem
    },
    props: {
        order_data: {
        type: Array,
        default() {
          return []
        }
      }
      
    },
    computed: {

    },
    methods: {
      ...mapActions([
        'GET_ORDERS_FROM_API',
      ]),
      ...mapGetters([
        'ORDERS']),
      show(){
          console.log(this.orders);
        this.$forceUpdate()
      },
      ClearCart() {

      
      },
    },
    mounted(){
        this.GET_ORDERS_FROM_API()
    }
  }
</script>

<style lang="scss">
  .v-cart {
    margin-bottom: 100px;
    &__total {
      position: fixed;
      bottom: 0;
      right: 0;
      left: 0;
      padding: $padding*2 $padding*3;
      display: flex;
      justify-content: center;
      background: $green-bg;
      color: #ffffff;
      font-size: 20px;
    }

    .total__name {
      margin-right: $margin*2;
    }
    .clear{
          position: fixed;
      top: 150px;
      right: 10px;
      width: 185px;
      padding: $padding*2;
      border: solid 1px #aeaeae;
      background: #ffffff;
    }
  }
</style>
