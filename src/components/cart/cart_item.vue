<template>
  <div class='cart-item'>
    <img class="cart-item__image" src="" alt="">
    <div class="cart-item__info">
      <p>{{cart_item_data.name}}</p>
      <p>{{cart_item_data.price | toFix | formattedPrice}}</p>
      <p>{{cart_item_data.article}}</p>
    </div>
    <div class="cart-item__quantity">
      <p>Qty:</p>
      <span class="quantity__tools">
        <span class="quantity__btn" @click="decrementItem">-</span>
        {{cart_item_data.quantity}}
        <span class="quantity__btn" @click="incrementItem">+</span>
      </span>
    </div>
    <button @click="deleteFromCart">Delete</button>
  </div>
</template>

<script>
import toFix from '../../filters/toFix'
import formattedPrice from "../../filters/price-format";
  export default {
    name: "cart-item",
    props: {
      cart_item_data: {
        type: Object,
        default() {
          return {}
        }
      }
    },
    data() {
      return {}
    },
    filters: {
      toFix,
      formattedPrice
    },
    computed: {},
    methods: {
      decrementItem() {
        this.$emit('decrement')
      },
      incrementItem() {
        this.$emit('increment')
      },
      deleteFromCart() {
        this.$emit('deleteFromCart')
      }
    }
  }
</script>

<style>
  .cart-item {
    display: flex;
    flex-wrap: nowrap;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 0 8px 0 #e0e0e0;
    padding: $padding*2;
    margin-bottom: $margin*2;
    &__image {
      max-width: 50px;
    }
    .quantity__btn {
      cursor: pointer;
    }
    .quantity__tools {
      user-select: none;
    }
  }
</style>