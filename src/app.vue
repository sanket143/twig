<template>
  <div id="view" class="h-full grid">
    <Navbar />
    <div id="main" class="flex">
      <div id="sidebar" class="p-2 bg-green-600 text-white">
        <h1>Sidebar</h1>
      </div>
      <div id="main" class="p-2">
        <router-view/>
      </div>
    </div>
  </div>
</template>

<script>
import Navbar from "@/components/navbar.vue"

export default {
  components: {
    Navbar
  }
}
</script>

<style>
@font-face {
  font-family: 'Noto Sans';
  font-style: normal;
  font-display: swap;
  src: local('Noto Sans'), format('ttf');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}

body, html {
  font-family: 'Noto Sans', sans-serif;
}

body, html, #app {
  height: 100%;
}

#view {
  grid-template-rows: 40px auto;
}

#sidebar {
  width: 350px;
}
</style>
