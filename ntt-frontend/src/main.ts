import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import "@/styles/app.scss";

import { faCalendar, faGear } from "@fortawesome/free-solid-svg-icons";
import { library } from "@fortawesome/fontawesome-svg-core";

/* add icons to the library */
library.add(faGear);
library.add(faCalendar);
const app = createApp(App);
app.component("font-awesome-icon", FontAwesomeIcon);

app.use(createPinia());
app.use(router);

app.mount("#app");
