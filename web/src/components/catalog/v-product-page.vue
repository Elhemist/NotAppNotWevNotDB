<template>
  <div class='v-product-page'>
    <img class="v-catalog-item__image" :src="product.image_url" alt="img">
    <p>Наименование продукта: {{product.name}}</p>
    <p>id продукта: {{product.id}}</p>
    <p>Стоимость: {{product.price | toFix | formattedPrice}}</p>
    <p>Описание: {{product.description}}</p>
    
    <button
        class="v-catalog-item__add_to_cart_btn btn"
        @click="addToCart"
    >Добавить в корзину
    </button>
  </div>
</template>

<script>
  import {mapGetters, mapActions} from 'vuex'
  import toFix from "../../filters/toFix";
  import formattedPrice from "../../filters/price-format";

  export default {
    name: "v-product-page",
    props: {},
    data() {
      return {}
    },
    filters: {
      formattedPrice,
      toFix
    },
    computed: {
      ...mapGetters([
        'PRODUCTS'
      ]),
      product() {
        let result = {}
        let vm = this;
        this.PRODUCTS.find(function (item) {
          if (item.id === vm.$route.query.product) {
            result = item;
          }
        })
        return result;
      }
    },
    methods: {
      ...mapActions([
        'GET_PRODUCTS_FROM_API',
        'ADD_TO_CART'
      ]),
      addToCart(data) {
        
        console.log(this.product)
        console.log('data')
        console.log(data)
        this.ADD_TO_CART(this.product)
          .then(() => {
            let timeStamp = Date.now().toLocaleString();
            this.messages.unshift(
              {name: 'Товар добавлен в корзину', icon: 'check_circle', id: timeStamp}
            )
          })
      },
    },
    mounted() {
      if (!this.PRODUCTS.length) {
        this.GET_PRODUCTS_FROM_API()
      }
    }
  }
</script>

<style scoped>
.v-product-page{
  background: bisque;
  margin: 20%;
  padding: 10px;
}
</style>
