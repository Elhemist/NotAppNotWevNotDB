<template>
  <div class='v-main-page'>
    <h1>Отличный онлайн сервис по доставке продуктов</h1>
    <p>
      
      <router-link :to="{name: 'catalog'}">
        Приветствую тебя покупатель!
      </router-link>
      
    </p>
    <h1>
        Купи
      </h1>
  </div>
</template>

<script>
  export default {
    name: "v-main-page",
    props: {},
    data() {
      return {}
    },
    computed: {}
  }
</script>

<style scoped>

</style>
