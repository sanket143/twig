<template>
  <div>
    <div v-html="mail_body" />
  </div>
</template>

<script>
import { listen } from "tauri/api/event"

export default {
  name: "Mail",
  data(){
    return {
      mail_body: ""
    }
  },
  mounted(){
    listen("mail-fetch", (response) => {
      const payload = JSON.parse(response.payload)
      this.mail_body = payload.data
    })
  }
}
</script>
