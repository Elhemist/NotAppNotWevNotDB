<template>

<div class="products">
    <p> nya</p>
    <div class="product-card">
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
   /*
   data() 
    {
        return {
            
            products:
            [
                {
                    id: 1,
                    name: 'Яблоко',
                    price:6
                },
                {
                    id: 2,
                    name: 'Груша',
                    price:66
                },
                {
                    id: 3,
                    name: 'Плесень',
                    price:666
                },
                {
                    id: 4,
                    name: 'Моль',
                    price:999
                }
            ]
            
        }
    },*/
    computed: {
        ...mapGetters([
            'PRODUCTS'
        ]),
    },
    methods:{
        ...mapActions([
            'GET_PRODUCTS_FROM_API'
        ]),
        SendToCart(data){
            console.log(data)
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
/*
.products{
    &_list {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    }
}
*/
</style>
