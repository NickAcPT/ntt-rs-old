<template>
  <nav id="navBar">
    <div class="tab">
      <router-link to="schedules">
        <font-awesome-icon icon="fa-solid fa-calendar" /><span>Schedule</span>
      </router-link>
    </div>
    <div class="tab">
      <router-link to="schedules">
        <font-awesome-icon icon="fa-solid fa-gear" /> <span>Settings</span>
      </router-link>
    </div>
  </nav>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import type { User } from "@/types/user";

export default defineComponent({
  name: "NavBar",
  props: {
    user: {
      required: false,
      type: Object as () => User,
    },
  },
  setup() {
    return {};
  },
});
</script>

<style scoped lang="scss">
#navBar {
  display: flex;
  flex-direction: column;
  background-color: #282b30;
  padding: 0 1rem;
  color: white;
  .tab {
    display: flex;
    background: #424549;
    border-radius: 5px;
    padding: 0.5rem 1rem;
    margin: 0.5rem 0;
    cursor: pointer;
    text-align: center;
    transition: all 0.2s ease-in-out;
    font-size: large;
    &:hover {
      background-color: #3a3f48;
    }
    svg {
      margin-right: 0.25rem;
    }
    a {
      color: white;
      text-decoration: none;
    }
  }
  .tab:first-child {
    margin-top: 1rem;
  }
  .tab:last-child {
    // Make the last tab be at the bottom of the nav bar
    margin-top: auto;
  }
}
</style>
