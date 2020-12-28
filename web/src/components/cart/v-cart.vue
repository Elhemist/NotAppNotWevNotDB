<template>
  <div class='v-cart'>
    <div >
      <button @click="ClearCart" class="clear" >Очиститесь огнём</button>
      
        <router-link :to="{name: 'catalog'}">
          <div class="v-catalog__link_to_cart">Вернуться в каталог</div>
        </router-link>
      
    </div>
    <h1>Cart</h1>
    <p v-if="!cart_data.length">Пустота холодна...</p>
    <v-cart-item
        v-for="(item, index) in cart_data"
        :key="item.id"
        :cart_item_data="item"
        @deleteFromCart="deleteFromCart(index)"
        @increment="increment(index, item.id, item.quantity)"
        @decrement="decrement(index, item.id, item.quantity)"
    />
    <div class="v-cart__total">
      <textarea v-model="street" placeholder="введите название улицы"></textarea>
      <textarea v-model="home" placeholder="введите номер дома"></textarea>
      <button @click="CreateOrder()" class="total__name">Заказать:{{cartTotalCost | toFix | formattedPrice}}</button>
    </div>
  </div>
</template>

<script>
  import vCartItem from './v-cart-item'
  import toFix from "../../filters/toFix";
  import formattedPrice from "../../filters/price-format";
  import {mapActions, mapGetters} from 'vuex'

  export default {
    name: "v-cart",
    components: {
      vCartItem
    },
    props: {
      cart_data: {
        type: Array,
        default() {
          return []
        }
      }
    },
    data() {
      return {
        street: "",
        home: ""
      }
    },
    filters: {
      formattedPrice,
      toFix
    },
    computed: {
      cartTotalCost() {
        let result = []

        if (this.cart_data.length) {
          for (let item of this.cart_data) {
            result.push(item.price * item.quantity)
          }

          result = result.reduce(function (sum, el) {
            return sum + el;
          })
          return result;
        } else {
          return 0
        }
      }
    },
    methods: {
      ...mapActions([
        'DELETE_FROM_CART',
        'INCREMENT_CART_ITEM',
        'DECREMENT_CART_ITEM',
        'CLEAR_CART',
        'CREATE_ORDER'
        
      ]),
      ...mapGetters([
        'CART']),
      increment(index, id, quantity) {
        var nya = [index, id, quantity+1]
        console.log('nya');
        console.log(nya);
        this.INCREMENT_CART_ITEM(nya)
      },
      decrement(index, id, quantity) {
        var nya = [index, id, quantity-1]
        console.log('nya');
        console.log(nya);
        this.DECREMENT_CART_ITEM(nya)
      },
      deleteFromCart(index) {
        this.DELETE_FROM_CART(index);
      },
      
      ClearCart() {
        this.CLEAR_CART()
      },
      CreateOrder(){
        var index= [this.street, this.home]
        this.CREATE_ORDER(index);
      }
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
