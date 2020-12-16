<template>

<div class="flexboxes">
    <p> nya</p>
    <div class="products">
        <product-card 
            v-for="product in PRODUCTS"
            :key="product.id"
            :product_data="product"
            @SendId="SendToCart"
        />

    </div>
    

</div>
</template>
<script>
import ProductCard from './product-card'
import {mapActions, mapGetters} from 'vuex'
export default {
  name: "products",
   components:{
      ProductCard
   },
   props:
   {
   },
    computed: {
        ...mapGetters([
            'PRODUCTS',
            'ADD_TO_CART'
        ]),
    },
    methods:{
        ...mapActions([
            'GET_PRODUCTS_FROM_API'
        ]),
        SendToCart(data){
            this.ADD_TO_CART(data)
        }
    },
    mounted(){
        this.GET_PRODUCTS_FROM_API()
        .then((response)=>{
            if(response.data){
                console.log('Data arrived!')
            }
        })
    }



}
</script>
<style>

.products{
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
}
</style>
