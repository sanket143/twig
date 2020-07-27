<template>
  <div class="home">
    <h1>Home</h1>
    <form @submit.prevent="signIn">
      <input
        v-model="msg"
        class="border p-1"
        type="text"
      >
    </form>
    <br>
    <Mail />
  </div>
</template>

<script>
import { emit } from "tauri/api/event"
import Mail from "@/components/mail.vue"

export default {
  name: "Home",
  components: {
    Mail
  },
  data(){
    return {
      msg: ""
    }
  },
  methods: {
    signIn(){
      console.log(this.msg)
      emit("sign-in", this.msg)
    }
  }
}
</script>
