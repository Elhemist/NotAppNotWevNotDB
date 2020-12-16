<template>
<div class="product-card col-md-4">
        <div class="card mb-4 box-shadow" margin="5">
            <img class="card-img-top product-card_image" src="https://avatars.mds.yandex.net/get-zen_doc/29485/pub_5d48790a35c8d800ad19bd17_5d4879546c300400ae436e00/scale_1200">
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
    
    props:{
        product_data:{
            type: Object,
            default(){
                return{}
            }
        }
    },
    data() 
    {
        return{

        }
    },
    methods:{
        SendToCart(){
            this.$emit( 'SendId', this.product_data)
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
    min-width: 250px;
}
.product-card_image{
    width: 100px;       
}
.card{
padding:  5px;
}
</style>
