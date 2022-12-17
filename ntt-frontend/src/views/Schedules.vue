<template>
  <ul id="timeTables">
    <li class="timeTable" v-for="timeTable in timeTables" :key="timeTable.id">
      <TimeTable :timeTable="timeTable" />
    </li>
  </ul>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import type { TimeTableType } from "@/types/TimeTables";
import TimeTable from "@/components/TimeTable.vue";

export default defineComponent({
  name: "Schedules",
  components: { TimeTable },
  data() {
    const timeTables: TimeTableType[] = [
      {
        id: 1,
        name: "School Schedule",
        owner: "Wyatt",
        events: [
          {
            name: "MTH 264",
            starts_on: new Date("January 17, 2023"),
            start_time: { hour: 9, minute: 0 },
            ends_time: { hour: 10, minute: 45 },
            where: "T303",
          },
          {
            name: "BIO 102",
            starts_on: new Date("January 18, 2023"),
            start_time: { hour: 9, minute: 30 },
            ends_time: { hour: 10, minute: 45 },

            where: "H318",
          },
          {
            name: "PHY 241",
            starts_on: new Date("January 17, 2023"),
            start_time: { hour: 3, minute: 30 },
            ends_time: { hour: 10, minute: 45 },
            where: "H301",
          },
        ],
      },
    ];
    return {
      timeTables,
    };
  },
  setup() {
    return {};
  },
  methods: {},
});
</script>

<style scoped lang="scss">
#timeTables {
  list-style: none;
}
</style>
