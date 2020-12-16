<template>
    <div class="cart">
        <cart_item/>
    </div>
</template>
<script>
import cart_item from './cart_item'
export default {
    name: "cart",
    components:{
        cart_item
    },
    data(){
        return{}
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
<style scoped >


</style>