<template>
  <div class="timeTableSummary">
    <div class="timeTableHeader">
      <h2>{{ timeTable.name }}</h2>
    </div>

    <div class="nextEvent">
      <h3>Next Event</h3>
      <h4>Name: {{ nextEvent.name }}</h4>
      <span>Where: {{ nextEvent.where }}</span>
      <div v-if="!isToday">
        <span
          >Start:
          {{
            nextEvent.start.toLocaleTimeString([], {
              hour: "2-digit",
              minute: "2-digit",
            })
          }}
          - End:
          {{
            nextEvent.end.toLocaleTimeString([], {
              hour: "2-digit",
              minute: "2-digit",
            })
          }}</span
        >
        <span class="futureDate">On: January 17th 2023</span>
      </div>
      <div v-else>
        <span>Today at 9:30am til 10:45am</span>
      </div>
    </div>
    <div class="timeTableOptions">
      <button>View Calendar</button>
      <button>Edit</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import type { TimeTableType } from "@/types/TimeTables";
import { getNextEvent } from "@/types/TimeTables";

export default defineComponent({
  name: "TimeTable",
  props: {
    timeTable: {
      required: true,
      type: Object as () => TimeTableType,
    },
  },
  setup(props) {
    const nextEvent = getNextEvent(props.timeTable);
    const today = new Date();
    const isToday =
      nextEvent.start.getDate() === today.getDate() &&
      nextEvent.start.getMonth() === today.getMonth() &&
      nextEvent.start.getFullYear() === today.getFullYear();
    return { nextEvent, isToday };
  },
});
</script>

<style scoped lang="scss">
.timeTableSummary {
  color: white;
  background: #3a3f48;
  width: 300px;
  height: 25vh;
  display: flex;
  flex-direction: column;
}
.timeTableOptions {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin: auto 2px 0.5rem;
  button {
    width: 30%;
    background-color: #7289da;
    border: none;
    border-radius: 5px;
    color: white;
    padding: 5px 10px;
    text-align: center;
    text-decoration: none;
    transition: all 0.2s ease-in-out;
    &:hover {
      background-color: #5f73bc;
      cursor: pointer;
    }
  }
}
.timeTableHeader {
  h2 {
    padding: 2px;
    margin: 2px;
    text-align: center;
  }
  span {
    padding: 2px;
  }
  border-bottom: #282b30;
  border-bottom-style: solid;
  border-bottom-width: 1px;
  margin-bottom: 10px;
  padding-bottom: 10px;
}
.nextEvent {
  padding-left: 10px;
  text-align: left;
  h3 {
    border-bottom: #282b30;
    border-bottom-style: solid;
    border-bottom-width: 1px;
    margin: 0 0 10px;
    padding: 0 0 10px;
  }
  h4 {
    margin: 0;
  }
  span {
    display: block;
  }
}
</style>
