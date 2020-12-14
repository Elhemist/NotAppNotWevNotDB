<template>

  <div class="w-25 mx-auto border p-3 rounded">
    <b-form @submit="login">
      <div class="form-group">
        <label for="username">Логин:</label>
        <b-input v-model="username" type="text" id="username" placeholder="Логин..."></b-input>
      </div>
      <div class="form-group">
        <label for="password">Пароль:</label>
        <b-input v-model="password" type="password" id="password" placeholder="Пароль..."></b-input>
      </div>
      <b-button variant="primary" type="submit">Войти</b-button>
      <p class="mt-3">Ещё не зарегистрированы? <router-link to="/auth/signup">Регистрация</router-link>
      </p>
    </b-form>
  </div>
</template>
<script>

</script>
<style>
</style>