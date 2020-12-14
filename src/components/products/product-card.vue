<template>
<div class="product-card">
    <div>
        <img class="product-card_image" src="[http://gallery.greedykidz.net/get/481438/1.jpg?g2_serialNumber=2]" alt="ничего">
        <p class="product-card_name"> {{product_data.name}}</p>
        <p class="product-card_price">Цена: {{product_data.price}} Рyб.</p>
        <button 
        class="product-card_but btn" 
        @click="SendToCart">
        Добавь в корзину пидрила!
        </button>
    </div>
</div>
</template>
<script>

export default {
  name: "product-card",
    data() 
    {
        return {
            /*
            products:
            [
                {name: 'Яблоко', price:6},
                {name: 'Груша', price:66},
                {name: 'Плесень', price:666},
                {name: 'Моль', price:999}
            ]
            */
        }
    },
    props:{
        product_data:{
            type: Object,
            default(){
                return{}
            }
        }
    },
    methods:{
        SendToCart(){
            this.$emit( 'SendId', this.product_data.id)
        }
    }
}
</script>
<style>
.product-card{
    flex-basis: 25%;
    box-shadow: 0 0 8px 0 #e0e0e0;
    padding: 16;
    margin-bottom: 16;
}
.product-card_image{
    width: 100px;       
}
</style>
