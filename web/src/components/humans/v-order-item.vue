<template>
  <div class='v-cart-item'>
      <p>{{cart_item_data.id}}</p>
  </div>
</template>

<script>


  export default {
    name: "v-order-item",
    props: {
      order_data: {
        type: Object,
        default() {
          return {}
        }
      }
    },
    data() {
      return {}
    },
    computed: {},
    methods: {
    }
  }
</script>

<style lang="scss">
  .v-cart-item {
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
