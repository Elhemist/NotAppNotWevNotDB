<template>
  <div class='v-catalog-item'>
    <p> Продукт </p>
    <v-popup
        v-if="isInfoPopupVisible"
        rightBtnTitle="Добавить в корзину"
        :popupTitle="product_data.name"
        @closePopup="closeInfoPopup"
        @rightBtnAction="addToCart"
    >
      <img class="v-catalog-item__image" :src="product_data.image_url" alt="img">
      <div>
        <p class="v-catalog-item__name">Описание: {{product_data.description}}</p>
      </div>
    </v-popup>
    <center>
    <div class="im">
      <img class="v-catalog-item__image " :src="product_data.image_url" alt="img" @click="productClick">
    </div>
    </center>
    <p class="v-catalog-item__name">{{product_data.name}}</p>
    <p class="v-catalog-item__price">Стоимость: {{product_data.price | toFix | formattedPrice}}</p>
    <button
        class="v-catalog-item__show-info"
        @click="showPopupInfo"
    >
      Подробная информация
    </button>
    <br>
    <button
        class="v-catalog-item__add_to_cart_btn btn"
        @click="addToCart"
    > Добавить в корзину 
    </button>
  </div>
</template>

<script>
  import vPopup from '../popup/v-popup'
  import toFix from '../../filters/toFix'
  import formattedPrice from "../../filters/price-format";

  export default {
    name: "v-catalog-item",
    components: {
      vPopup
    },
    props: {
      product_data: {
        type: Object,
        default() {
          return {}
        }
      }
    },
    data() {
      return {
        isInfoPopupVisible: false
      }
    },
    filters: {
      toFix,
      formattedPrice
    },
    computed: {},
    methods: {
      productClick() {
        this.$emit('productClick', this.product_data.id)
      },
      showPopupInfo() {
        this.isInfoPopupVisible = true;
      },
      closeInfoPopup() {
        this.isInfoPopupVisible = false;
      },
      addToCart() {
        this.$emit('addToCart', this.product_data);
      }
    },
    mounted() {
      this.$set(this.product_data, 'quantity', 1)
    }
  }
</script>

<style lang="scss">
  .v-catalog-item {
    flex-basis: 25%;
    border-radius: 10px;
    box-shadow: 0 0 8px 0 #969595;
    padding: $padding*2;
    background: rgb(190, 190, 190);
    margin-bottom: $margin*2;
    min-width: 280px;
    &__image {
      width: 100px;
    }
  }
  .v-catalog-item__show-info{
    margin-bottom: $margin;
    background: #9b9999;
  }
  .v-catalog-item__add_to_cart_btn{
    width: 202.6px;
  }
  .im{
    width: 100px;
    height: 100px;
  }
</style>
