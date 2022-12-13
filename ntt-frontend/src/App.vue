<template>
  <div id="content">
    <NarBar />
    <RouterView />
  </div>
</template>
<script lang="ts">
import { defineComponent } from "vue";
import NarBar from "@/components/NavBar.vue";
export default defineComponent({
  name: "App",
  components: { NarBar },
  setup() {
    return {};
  },
});
</script>
<style lang="scss" scoped>
#content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  height: 100vh;
  nav {
    height: 100vh;
  }
}
</style>
