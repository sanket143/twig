<template>
  <div>
    <h1>Page {{ params.id }}</h1>
  </div>
</template>

<script>
import * as tauriEvent from 'tauri/api/event'

window.tauriEvent = tauriEvent

export default {
  name: "Page",
  data(){
    return {
      params: {}
    }
  },
  mounted(){
    this.params = this.$route.params 
    tauriEvent.emit("js-event", "Hello from JS")
    console.log("page mounted")
  }
}
</script>
