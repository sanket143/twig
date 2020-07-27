<template>
  <div class="home">
    <h1>Home</h1>
    <input
      v-model="msg"
      class="border p-1"
      type="text"
    >
    <br>
    <button
      class="border my-2 p-1"
      @click="sendMessage"
    >
      Send
    </button>
    <button
      class="border my-2 p-1"
      @click="signIn"
    >
      Sign In
    </button>
  </div>
</template>

<script>
import { emit, listen } from "tauri/api/event"

export default {
  name: "Home",
  data(){
    return {
      msg: ""
    }
  },
  mounted(){
    listen("rust-event", (response) => {
      console.log(response.payload)
    })
  },
  methods: {
    sendMessage(){
      emit("js-event", this.msg)
    },
    signIn(){
      emit("sign-in", this.msg)
    }
  }
}
</script>
