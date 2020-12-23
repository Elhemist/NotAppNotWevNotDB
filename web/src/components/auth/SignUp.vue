<template>
  <div class="w-50 border rounded p-3 mx-auto">
    <div class="form-group required">
      <label for="first_name">Имя</label>
      <b-input v-model="first_name" type="text" maxlength="30"></b-input>
      <p><small class="text-muted">Введите имя</small></p>
    </div>
    <div class="form-group required">
    <label for="middle_name">Отчество</label>
      <b-input v-model="middle_name" type="text" maxlength="30"></b-input>
      <p><small class="text-muted">Введите не имя</small></p>
    </div>
    <div class="form-group required">
    <label for="last_name">Фамилия</label>
      <b-input v-model="last_name" type="text" maxlength="30"></b-input>
      <p><small class="text-muted">Введите другое не имя</small></p>
    </div>
    <b-form @submit="register">
      <div class="form-group required">
        <label class="control-label" for="phone">Номер телефона:</label>
        <b-input v-model.number="phone" type="text" id="phone" v-imask="phoneNumberMask" placeholder="+7(921)123-45-67" @keypress="isNumber" @accept="onAccept" @complete="onComplete" maxlength="16"></b-input>
        <p><small class="text-muted">Введите номер в формате: +7(921)123-45-67</small></p>
      </div>
      <div class="form-group required">
        <label class="control-label" for="customerOrExecutor">Вы заказчик или исполнитель?</label>
        <b-select v-model="customerOrExecutor" :options="customerOrExecutorOptions" type="customerOrExecutor" id="customerOrExecutor">
        </b-select>
      </div>
      <div class="form-group required">
        <label class="control-label" for="password">Пароль:</label>
        <b-input v-model="password" type="password" id="password" placeholder="Пароль..."></b-input>
        <p><small class="text-muted">Минимальная длина пароля 6 символов</small></p>
      </div>
      <div class="form-group required">
        <label class="control-label" for="repeatPassword">Повторите пароль:</label>
        <b-input v-model="repeatPassword" type="password" id="repeatPassword" placeholder="Повторите пароль..."></b-input>
      </div>
      <p class="text-danger" v-if="!$v.password.minLength">Длина пароля меньше 6 символов</p>
      <p class="text-danger" v-if="$v.password.required && $v.repeatPassword.required && !$v.repeatPassword.sameAs">Введённые пароли не совпадают</p>
      <b-button variant="primary" type="submit" :disabled="formValid">Регистрация</b-button>
      <p class="mt-2"><small class="text-muted">Все поля отмеченные <span class="text-danger">*</span> обязательны для заполнения.</small></p>
      <p class="mt-3">Уже есть аккаунт? <router-link to="/login">Вход</router-link>
      </p>
    </b-form>
  </div>
</template>
<script>
import { required, minLength, sameAs } from 'vuelidate/lib/validators'
import { IMaskDirective } from 'vue-imask'
import axios from "axios";
import VueAxios from 'vue-axios'
import Vue from 'vue'

Vue.use(axios)
Vue.use(VueAxios, axios)

export default {
  name: "SignUp",
  data() {
    return {
      username: "",
      password: "",
      repeatPassword: "",
      first_name: "",
      middle_name:"",
      last_name:"",
      phone: "",
      userPhone: "",
      customerOrExecutor: "",
      customerOrExecutorOptions: [
        { text: 'Выберите...', value: '', disabled: true, selected: true },
        { text: 'Заказчик', value: 'customer' },
        { text: 'Исполнитель', value: 'executor' }
      ],
      phoneNumberMask: {
        mask: '0000000000',
        lazy: true
      }
    }
  },
  validations: {
    password: {
      required,
      minLength: minLength(6)
    },
    repeatPassword: {
      required,
      sameAs: sameAs('password')
    },
    phone: {
      required
    },
    customerOrExecutor: {
      required
    }
  },
  computed: {
    formValid() {
      return this.$v.$invalid
    }
  },
  methods: {
    register(event) {
      console.log(this.phone);
      event.preventDefault();

      // логика регистрации
      axios
        .post(`https://elhemist.orius.dev/api/users`, { 'phone': parseInt(this.phone), 'password': this.password, 'last_name': this.last_name, 'middle_name':this.middle_name, 'first_name':this.first_name })
        .then(response => {
          console.log(response);
          this.$router.push('/login')
        })
        .catch(err => {
          console.error(err);
          this.err = err
        })
    },
    onAccept(e) {
      const maskRef = e.detail
      this.phone = maskRef.value
    },
    onComplete(e) {
      const maskRef = e.detail
      this.userPhone = maskRef.unmaskedValue
    },
    isNumber(e) {
      let regex = /[0-9]/

      if (!regex.test(e.key)) {
        e.returnValue = false;
        if (e.preventDefault) e.preventDefault();
      }
    }
  },
  directives: {
    imask: IMaskDirective
  }
};

</script>
<style>
.form-group.required .control-label:after {
  content:" *";
  color:red;
}
</style>
